// Copyright 2018-2025 the Deno authors. MIT license.

use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

use deno_core::cppgc::Ptr;
use deno_core::op2;
use deno_core::v8;
use deno_core::webidl::{IntOptions, WebIdlConverter, WebIdlError};
use deno_core::GarbageCollected;
use deno_core::WebIDL;
use deno_error::JsErrorBox;
use wgpu_core::command::PassChannel;
use wgpu_types::{BufferAddress, TexelCopyBufferInfo};

use crate::buffer::GPUBuffer;
use crate::command_buffer::GPUCommandBuffer;
use crate::compute_pass::GPUComputePassEncoder;
use crate::queue::GPUTexelCopyTextureInfo;
use crate::render_pass::GPULoadOp;
use crate::render_pass::GPURenderPassEncoder;
use crate::webidl::GPUExtent3D;
use crate::Instance;

pub struct GPUCommandEncoder {
    pub instance: Instance,
    pub error_handler: super::error::ErrorHandler,

    pub id: wgpu_core::id::CommandEncoderId,
    pub label: String,

    pub finished: AtomicBool,
}

impl Drop for GPUCommandEncoder {
    fn drop(&mut self) {
        // Command encoders and command buffers are both the same wgpu object.
        // At the time `finished` is set, ownership of the id (and
        // responsibility for dropping it) transfers from the encoder to the
        // buffer.
        if !self.finished.load(Ordering::SeqCst) {
            self.instance.command_encoder_drop(self.id);
        }
    }
}

impl GarbageCollected for GPUCommandEncoder {}

#[op2]
impl GPUCommandEncoder {
    #[getter]
    #[string]
    fn label(&self) -> String {
        self.label.clone()
    }
    #[setter]
    #[string]
    fn label(&self, #[webidl] _label: String) {
        // TODO(@crowlKats): no-op, needs wpgu to implement changing the label
    }

    #[required(1)]
    #[cppgc]
    fn begin_render_pass(
        &self,
        #[webidl] descriptor: crate::render_pass::GPURenderPassDescriptor,
    ) -> Result<GPURenderPassEncoder, JsErrorBox> {
        let color_attachments = Cow::Owned(
            descriptor
                .color_attachments
                .into_iter()
                .map(|attachment| {
                    attachment.into_option().map(|attachment| {
                        wgpu_core::command::RenderPassColorAttachment {
                            view: attachment.view.id,
                            depth_slice: attachment.depth_slice,
                            resolve_target: attachment.resolve_target.map(|target| target.id),
                            load_op: attachment
                                .load_op
                                .with_default_value(attachment.clear_value.map(Into::into)),
                            store_op: attachment.store_op.into(),
                        }
                    })
                })
                .collect::<Vec<_>>(),
        );

        let depth_stencil_attachment = descriptor
            .depth_stencil_attachment
            .map(|attachment| {
                if attachment
                    .depth_load_op
                    .as_ref()
                    .is_some_and(|op| matches!(op, GPULoadOp::Clear))
                    && attachment.depth_clear_value.is_none()
                {
                    return Err(JsErrorBox::type_error(
                        r#"'depthClearValue' must be specified when 'depthLoadOp' is "clear""#,
                    ));
                }

                Ok(wgpu_core::command::RenderPassDepthStencilAttachment {
                    view: attachment.view.id,
                    depth: PassChannel {
                        load_op: attachment
                            .depth_load_op
                            .map(|load_op| load_op.with_value(attachment.depth_clear_value)),
                        store_op: attachment.depth_store_op.map(Into::into),
                        read_only: attachment.depth_read_only,
                    },
                    stencil: PassChannel {
                        load_op: attachment.stencil_load_op.map(|load_op| {
                            load_op.with_value(Some(attachment.stencil_clear_value))
                        }),
                        store_op: attachment.stencil_store_op.map(Into::into),
                        read_only: attachment.stencil_read_only,
                    },
                })
            })
            .transpose()?;

        let timestamp_writes = descriptor.timestamp_writes.map(|timestamp_writes| {
            wgpu_core::command::PassTimestampWrites {
                query_set: timestamp_writes.query_set.id,
                beginning_of_pass_write_index: timestamp_writes.beginning_of_pass_write_index,
                end_of_pass_write_index: timestamp_writes.end_of_pass_write_index,
            }
        });

        let wgpu_descriptor = wgpu_core::command::RenderPassDescriptor {
            label: crate::transform_label(descriptor.label.clone()),
            color_attachments,
            depth_stencil_attachment: depth_stencil_attachment.as_ref(),
            timestamp_writes: timestamp_writes.as_ref(),
            occlusion_query_set: descriptor.occlusion_query_set.map(|query_set| query_set.id),
        };

        let (render_pass, err) = self
            .instance
            .command_encoder_begin_render_pass(self.id, &wgpu_descriptor);

        self.error_handler.push_error(err);

        Ok(GPURenderPassEncoder {
            instance: self.instance.clone(),
            error_handler: self.error_handler.clone(),
            render_pass: RefCell::new(render_pass),
            label: descriptor.label,
        })
    }

    #[cppgc]
    fn begin_compute_pass(
        &self,
        #[webidl] descriptor: crate::compute_pass::GPUComputePassDescriptor,
    ) -> GPUComputePassEncoder {
        let timestamp_writes = descriptor.timestamp_writes.map(|timestamp_writes| {
            wgpu_core::command::PassTimestampWrites {
                query_set: timestamp_writes.query_set.id,
                beginning_of_pass_write_index: timestamp_writes.beginning_of_pass_write_index,
                end_of_pass_write_index: timestamp_writes.end_of_pass_write_index,
            }
        });

        let wgpu_descriptor = wgpu_core::command::ComputePassDescriptor {
            label: crate::transform_label(descriptor.label.clone()),
            timestamp_writes,
        };

        let (compute_pass, err) = self
            .instance
            .command_encoder_begin_compute_pass(self.id, &wgpu_descriptor);

        self.error_handler.push_error(err);

        GPUComputePassEncoder {
            instance: self.instance.clone(),
            error_handler: self.error_handler.clone(),
            compute_pass: RefCell::new(compute_pass),
            label: descriptor.label,
        }
    }

    #[required(2)]
    fn copy_buffer_to_buffer<'a>(
        &self,
        scope: &mut v8::HandleScope<'a>,
        #[webidl] source: Ptr<GPUBuffer>,
        arg2: v8::Local<'a, v8::Value>,
        arg3: v8::Local<'a, v8::Value>,
        arg4: v8::Local<'a, v8::Value>,
        arg5: v8::Local<'a, v8::Value>,
    ) -> Result<(), WebIdlError> {
        let prefix = "Failed to execute 'GPUCommandEncoder.copyBufferToBuffer'";
        let int_options = IntOptions {
            clamp: false,
            enforce_range: true,
        };

        let source_offset: BufferAddress;
        let destination: Ptr<GPUBuffer>;
        let destination_offset: BufferAddress;
        let size: Option<BufferAddress>;
        // Note that the last argument to either overload of `copy_buffer_to_buffer`
        // is optional, so `arg5.is_undefined()` would not work here.
        if arg4.is_undefined() {
            // 3-argument overload
            source_offset = 0;
            destination = Ptr::<GPUBuffer>::convert(
                scope,
                arg2,
                Cow::Borrowed(prefix),
                (|| Cow::Borrowed("destination")).into(),
                &(),
            )?;
            destination_offset = 0;
            size = <Option<u64>>::convert(
                scope,
                arg3,
                Cow::Borrowed(prefix),
                (|| Cow::Borrowed("size")).into(),
                &int_options,
            )?;
        } else {
            // 5-argument overload
            source_offset = u64::convert(
                scope,
                arg2,
                Cow::Borrowed(prefix),
                (|| Cow::Borrowed("sourceOffset")).into(),
                &int_options,
            )?;
            destination = Ptr::<GPUBuffer>::convert(
                scope,
                arg3,
                Cow::Borrowed(prefix),
                (|| Cow::Borrowed("destination")).into(),
                &(),
            )?;
            destination_offset = u64::convert(
                scope,
                arg4,
                Cow::Borrowed(prefix),
                (|| Cow::Borrowed("destinationOffset")).into(),
                &int_options,
            )?;
            size = <Option<u64>>::convert(
                scope,
                arg5,
                Cow::Borrowed(prefix),
                (|| Cow::Borrowed("size")).into(),
                &int_options,
            )?;
        }

        let err = self
            .instance
            .command_encoder_copy_buffer_to_buffer(
                self.id,
                source.id,
                source_offset,
                destination.id,
                destination_offset,
                size,
            )
            .err();

        self.error_handler.push_error(err);

        Ok(())
    }

    #[required(3)]
    fn copy_buffer_to_texture(
        &self,
        #[webidl] source: GPUTexelCopyBufferInfo,
        #[webidl] destination: GPUTexelCopyTextureInfo,
        #[webidl] copy_size: GPUExtent3D,
    ) {
        let source = TexelCopyBufferInfo {
            buffer: source.buffer.id,
            layout: wgpu_types::TexelCopyBufferLayout {
                offset: source.offset,
                bytes_per_row: source.bytes_per_row,
                rows_per_image: source.rows_per_image,
            },
        };
        let destination = wgpu_types::TexelCopyTextureInfo {
            texture: destination.texture.id,
            mip_level: destination.mip_level,
            origin: destination.origin.into(),
            aspect: destination.aspect.into(),
        };

        let err = self
            .instance
            .command_encoder_copy_buffer_to_texture(
                self.id,
                &source,
                &destination,
                &copy_size.into(),
            )
            .err();

        self.error_handler.push_error(err);
    }

    #[required(3)]
    fn copy_texture_to_buffer(
        &self,
        #[webidl] source: GPUTexelCopyTextureInfo,
        #[webidl] destination: GPUTexelCopyBufferInfo,
        #[webidl] copy_size: GPUExtent3D,
    ) {
        let source = wgpu_types::TexelCopyTextureInfo {
            texture: source.texture.id,
            mip_level: source.mip_level,
            origin: source.origin.into(),
            aspect: source.aspect.into(),
        };
        let destination = TexelCopyBufferInfo {
            buffer: destination.buffer.id,
            layout: wgpu_types::TexelCopyBufferLayout {
                offset: destination.offset,
                bytes_per_row: destination.bytes_per_row,
                rows_per_image: destination.rows_per_image,
            },
        };

        let err = self
            .instance
            .command_encoder_copy_texture_to_buffer(
                self.id,
                &source,
                &destination,
                &copy_size.into(),
            )
            .err();

        self.error_handler.push_error(err);
    }

    #[required(3)]
    fn copy_texture_to_texture(
        &self,
        #[webidl] source: GPUTexelCopyTextureInfo,
        #[webidl] destination: GPUTexelCopyTextureInfo,
        #[webidl] copy_size: GPUExtent3D,
    ) {
        let source = wgpu_types::TexelCopyTextureInfo {
            texture: source.texture.id,
            mip_level: source.mip_level,
            origin: source.origin.into(),
            aspect: source.aspect.into(),
        };
        let destination = wgpu_types::TexelCopyTextureInfo {
            texture: destination.texture.id,
            mip_level: destination.mip_level,
            origin: destination.origin.into(),
            aspect: destination.aspect.into(),
        };

        let err = self
            .instance
            .command_encoder_copy_texture_to_texture(
                self.id,
                &source,
                &destination,
                &copy_size.into(),
            )
            .err();

        self.error_handler.push_error(err);
    }

    #[required(1)]
    fn clear_buffer(
        &self,
        #[webidl] buffer: Ptr<GPUBuffer>,
        #[webidl(default = 0, options(enforce_range = true))] offset: u64,
        #[webidl(options(enforce_range = true))] size: Option<u64>,
    ) {
        let err = self
            .instance
            .command_encoder_clear_buffer(self.id, buffer.id, offset, size)
            .err();
        self.error_handler.push_error(err);
    }

    #[required(5)]
    fn resolve_query_set(
        &self,
        #[webidl] query_set: Ptr<super::query_set::GPUQuerySet>,
        #[webidl(options(enforce_range = true))] first_query: u32,
        #[webidl(options(enforce_range = true))] query_count: u32,
        #[webidl] destination: Ptr<GPUBuffer>,
        #[webidl(options(enforce_range = true))] destination_offset: u64,
    ) {
        let err = self
            .instance
            .command_encoder_resolve_query_set(
                self.id,
                query_set.id,
                first_query,
                query_count,
                destination.id,
                destination_offset,
            )
            .err();

        self.error_handler.push_error(err);
    }

    #[cppgc]
    fn finish(
        &self,
        #[webidl] descriptor: crate::command_buffer::GPUCommandBufferDescriptor,
    ) -> Result<GPUCommandBuffer, JsErrorBox> {
        let wgpu_descriptor = wgpu_types::CommandBufferDescriptor {
            label: crate::transform_label(descriptor.label.clone()),
        };

        // TODO(https://github.com/gfx-rs/wgpu/issues/7812): This is not right,
        // it should be a validation error, and it would be nice if we can just
        // let wgpu generate it for us. The problem is that if the encoder was
        // already finished, we transferred ownership of the id to a command
        // buffer, so we have to bail out before we mint a duplicate command
        // buffer with the same id below.
        if self.finished.fetch_or(true, Ordering::SeqCst) {
            return Err(JsErrorBox::type_error(
                "The command encoder has already finished.",
            ));
        }

        let (id, err) = self
            .instance
            .command_encoder_finish(self.id, &wgpu_descriptor);

        self.error_handler.push_error(err);

        Ok(GPUCommandBuffer {
            instance: self.instance.clone(),
            id,
            label: descriptor.label,
        })
    }

    fn push_debug_group(&self, #[webidl] group_label: String) {
        let err = self
            .instance
            .command_encoder_push_debug_group(self.id, &group_label)
            .err();
        self.error_handler.push_error(err);
    }

    #[fast]
    fn pop_debug_group(&self) {
        let err = self.instance.command_encoder_pop_debug_group(self.id).err();
        self.error_handler.push_error(err);
    }

    fn insert_debug_marker(&self, #[webidl] marker_label: String) {
        let err = self
            .instance
            .command_encoder_insert_debug_marker(self.id, &marker_label)
            .err();
        self.error_handler.push_error(err);
    }
}

#[derive(WebIDL)]
#[webidl(dictionary)]
pub(crate) struct GPUCommandEncoderDescriptor {
    #[webidl(default = String::new())]
    pub label: String,
}

#[derive(WebIDL)]
#[webidl(dictionary)]
pub(crate) struct GPUTexelCopyBufferInfo {
    pub buffer: Ptr<GPUBuffer>,
    #[webidl(default = 0)]
    #[options(enforce_range = true)]
    offset: u64,
    #[options(enforce_range = true)]
    bytes_per_row: Option<u32>,
    #[options(enforce_range = true)]
    rows_per_image: Option<u32>,
}
