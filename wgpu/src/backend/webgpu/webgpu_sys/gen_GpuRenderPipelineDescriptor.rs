// DO NOT EDIT THIS FILE!
//
// This module part of a subset of web-sys that is used by wgpu's webgpu backend.
//
// These bindings are vendored into wgpu for the sole purpose of letting
// us pin the WebGPU backend to a specific version of the bindings, not
// to enable local changes. There are no provisions to preserve changes
// you make here the next time we re-vendor the bindings.
//
// The `web-sys` crate does not treat breaking changes to the WebGPU API
// as semver breaking changes, as WebGPU is "unstable". This means Cargo
// will not let us mix versions of `web-sys`, pinning WebGPU bindings to
// a specific version, while letting other bindings like WebGL get
// updated. Vendoring WebGPU was the workaround we chose.
//
// Vendoring also allows us to avoid building `web-sys` with
// `--cfg=web_sys_unstable_apis`, needed to get the WebGPU bindings.
//
// If you want to improve the generated code, please submit a PR to the https://github.com/rustwasm/wasm-bindgen repository.
//
// This file was generated by the `cargo xtask vendor-web-sys --version buf-to-buf-copy-size --git-url https://github.com/andyleiserson/wasm-bindgen.git` command.
#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPURenderPipelineDescriptor)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPipelineDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuRenderPipelineDescriptor;

    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &GpuRenderPipelineDescriptor) -> Option<::alloc::string::String>;

    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &GpuRenderPipelineDescriptor, val: &str);

    #[doc = "Get the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "layout")]
    pub fn get_layout(this: &GpuRenderPipelineDescriptor) -> ::wasm_bindgen::JsValue;

    #[doc = "Change the `layout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout(this: &GpuRenderPipelineDescriptor, val: &::wasm_bindgen::JsValue);

    #[doc = "Get the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "depthStencil")]
    pub fn get_depth_stencil(this: &GpuRenderPipelineDescriptor) -> Option<GpuDepthStencilState>;

    #[doc = "Change the `depthStencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDepthStencilState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "depthStencil")]
    pub fn set_depth_stencil(this: &GpuRenderPipelineDescriptor, val: &GpuDepthStencilState);

    #[doc = "Get the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "fragment")]
    pub fn get_fragment(this: &GpuRenderPipelineDescriptor) -> Option<GpuFragmentState>;

    #[doc = "Change the `fragment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "fragment")]
    pub fn set_fragment(this: &GpuRenderPipelineDescriptor, val: &GpuFragmentState);

    #[doc = "Get the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "multisample")]
    pub fn get_multisample(this: &GpuRenderPipelineDescriptor) -> Option<GpuMultisampleState>;

    #[doc = "Change the `multisample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMultisampleState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "multisample")]
    pub fn set_multisample(this: &GpuRenderPipelineDescriptor, val: &GpuMultisampleState);

    #[doc = "Get the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "primitive")]
    pub fn get_primitive(this: &GpuRenderPipelineDescriptor) -> Option<GpuPrimitiveState>;

    #[doc = "Change the `primitive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPrimitiveState`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "primitive")]
    pub fn set_primitive(this: &GpuRenderPipelineDescriptor, val: &GpuPrimitiveState);

    #[doc = "Get the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "vertex")]
    pub fn get_vertex(this: &GpuRenderPipelineDescriptor) -> GpuVertexState;

    #[doc = "Change the `vertex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "vertex")]
    pub fn set_vertex(this: &GpuRenderPipelineDescriptor, val: &GpuVertexState);
}

impl GpuRenderPipelineDescriptor {
    #[doc = "Construct a new `GpuRenderPipelineDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPipelineDescriptor`, `GpuVertexState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(layout: &::wasm_bindgen::JsValue, vertex: &GpuVertexState) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_layout(layout);
        ret.set_vertex(vertex);
        ret
    }

    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }

    #[deprecated = "Use `set_layout()` instead."]
    pub fn layout(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_layout(val);
        self
    }

    #[deprecated = "Use `set_depth_stencil()` instead."]
    pub fn depth_stencil(&mut self, val: &GpuDepthStencilState) -> &mut Self {
        self.set_depth_stencil(val);
        self
    }

    #[deprecated = "Use `set_fragment()` instead."]
    pub fn fragment(&mut self, val: &GpuFragmentState) -> &mut Self {
        self.set_fragment(val);
        self
    }

    #[deprecated = "Use `set_multisample()` instead."]
    pub fn multisample(&mut self, val: &GpuMultisampleState) -> &mut Self {
        self.set_multisample(val);
        self
    }

    #[deprecated = "Use `set_primitive()` instead."]
    pub fn primitive(&mut self, val: &GpuPrimitiveState) -> &mut Self {
        self.set_primitive(val);
        self
    }

    #[deprecated = "Use `set_vertex()` instead."]
    pub fn vertex(&mut self, val: &GpuVertexState) -> &mut Self {
        self.set_vertex(val);
        self
    }
}
