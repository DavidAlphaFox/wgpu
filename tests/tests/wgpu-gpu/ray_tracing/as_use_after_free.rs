use crate::ray_tracing::acceleration_structure_limits;
use std::{iter, mem};
use wgpu::{
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    AccelerationStructureFlags, AccelerationStructureGeometryFlags,
    AccelerationStructureUpdateMode, BindGroupDescriptor, BindGroupEntry, BindingResource,
    BlasBuildEntry, BlasGeometries, BlasGeometrySizeDescriptors, BlasTriangleGeometry,
    BlasTriangleGeometrySizeDescriptor, BufferAddress, BufferUsages, CommandEncoderDescriptor,
    ComputePassDescriptor, ComputePipelineDescriptor, CreateBlasDescriptor, CreateTlasDescriptor,
    PollType, TlasInstance, VertexFormat,
};
use wgpu_macros::gpu_test;
use wgpu_test::{FailureCase, GpuTestConfiguration, TestParameters, TestingContext};

fn required_features() -> wgpu::Features {
    wgpu::Features::EXPERIMENTAL_RAY_QUERY
        | wgpu::Features::EXPERIMENTAL_RAY_TRACING_ACCELERATION_STRUCTURE
}

/// This test creates a blas, puts a reference to it in a tlas instance inside a tlas package,
/// drops the blas, and ensures it gets kept alive by the tlas instance. Then it uses the built
/// package in a bindgroup, drops it, and checks that it is kept alive by the bindgroup by
/// executing a shader using that bindgroup.
fn acceleration_structure_use_after_free(ctx: TestingContext) {
    // Dummy vertex buffer.
    let vertices = ctx.device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: &[0; mem::size_of::<[[f32; 3]; 3]>()],
        usage: BufferUsages::BLAS_INPUT,
    });

    // Create a BLAS with a single triangle.
    let blas_size = BlasTriangleGeometrySizeDescriptor {
        vertex_format: VertexFormat::Float32x3,
        vertex_count: 3,
        index_format: None,
        index_count: None,
        flags: AccelerationStructureGeometryFlags::empty(),
    };

    let blas = ctx.device.create_blas(
        &CreateBlasDescriptor {
            label: Some("blas use after free"),
            flags: AccelerationStructureFlags::PREFER_FAST_TRACE,
            update_mode: AccelerationStructureUpdateMode::Build,
        },
        BlasGeometrySizeDescriptors::Triangles {
            descriptors: vec![blas_size.clone()],
        },
    );
    // Create the TLAS
    let mut tlas = ctx.device.create_tlas(&CreateTlasDescriptor {
        label: Some("tlas use after free"),
        max_instances: 1,
        flags: AccelerationStructureFlags::PREFER_FAST_TRACE,
        update_mode: AccelerationStructureUpdateMode::Build,
    });

    tlas[0] = Some(TlasInstance::new(
        &blas,
        [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0],
        0,
        0xFF,
    ));

    // Actually build the BLAS.
    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor::default());
    encoder.build_acceleration_structures(
        iter::once(&BlasBuildEntry {
            blas: &blas,
            geometry: BlasGeometries::TriangleGeometries(vec![BlasTriangleGeometry {
                size: &blas_size,
                vertex_buffer: &vertices,
                first_vertex: 0,
                vertex_stride: mem::size_of::<[f32; 3]>() as BufferAddress,
                index_buffer: None,
                first_index: None,
                transform_buffer: None,
                transform_buffer_offset: None,
            }]),
        }),
        iter::empty(),
    );
    ctx.queue.submit(Some(encoder.finish()));

    // Drop the blas and ensure that if it was going to die, it is dead.
    drop(blas);
    ctx.device.poll(PollType::Wait).unwrap();

    // build the tlas package to ensure the blas is dropped
    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor::default());
    encoder.build_acceleration_structures(iter::empty(), iter::once(&tlas));
    ctx.queue.submit(Some(encoder.finish()));

    // Create a compute shader that uses an AS.
    let shader = ctx
        .device
        .create_shader_module(include_wgsl!("shader.wgsl"));
    let compute_pipeline = ctx
        .device
        .create_compute_pipeline(&ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: Some("basic_usage"),
            compilation_options: Default::default(),
            cache: None,
        });

    let bind_group = ctx.device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &compute_pipeline.get_bind_group_layout(0),
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::AccelerationStructure(&tlas),
        }],
    });

    // Drop the TLAS package and ensure that if it was going to die, it is dead.
    drop(tlas);
    ctx.device.poll(PollType::Wait).unwrap();

    // Run the pass with the bind group that references the TLAS package.
    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&compute_pipeline);
        pass.set_bind_group(0, Some(&bind_group), &[]);
        pass.dispatch_workgroups(1, 1, 1)
    }
    ctx.queue.submit(Some(encoder.finish()));
}

#[gpu_test]
static ACCELERATION_STRUCTURE_USE_AFTER_FREE: GpuTestConfiguration = GpuTestConfiguration::new()
    .parameters(
        TestParameters::default()
            .test_features_limits()
            .limits(acceleration_structure_limits())
            .features(required_features())
            // https://github.com/gfx-rs/wgpu/issues/6727
            .skip(FailureCase::backend_adapter(wgpu::Backends::VULKAN, "AMD")),
    )
    .run_sync(acceleration_structure_use_after_free);
