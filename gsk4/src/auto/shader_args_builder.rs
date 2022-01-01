// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::GLShader;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderArgsBuilder(Shared<ffi::GskShaderArgsBuilder>);

    match fn {
        ref => |ptr| ffi::gsk_shader_args_builder_ref(ptr),
        unref => |ptr| ffi::gsk_shader_args_builder_unref(ptr),
        type_ => || ffi::gsk_shader_args_builder_get_type(),
    }
}

impl ShaderArgsBuilder {
    #[doc(alias = "gsk_shader_args_builder_new")]
    pub fn new(shader: &GLShader, initial_values: Option<&glib::Bytes>) -> ShaderArgsBuilder {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_shader_args_builder_new(
                shader.to_glib_none().0,
                initial_values.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_shader_args_builder_set_bool")]
    pub fn set_bool(&self, idx: i32, value: bool) {
        unsafe {
            ffi::gsk_shader_args_builder_set_bool(self.to_glib_none().0, idx, value.into_glib());
        }
    }

    #[doc(alias = "gsk_shader_args_builder_set_float")]
    pub fn set_float(&self, idx: i32, value: f32) {
        unsafe {
            ffi::gsk_shader_args_builder_set_float(self.to_glib_none().0, idx, value);
        }
    }

    #[doc(alias = "gsk_shader_args_builder_set_int")]
    pub fn set_int(&self, idx: i32, value: i32) {
        unsafe {
            ffi::gsk_shader_args_builder_set_int(self.to_glib_none().0, idx, value);
        }
    }

    #[doc(alias = "gsk_shader_args_builder_set_uint")]
    pub fn set_uint(&self, idx: i32, value: u32) {
        unsafe {
            ffi::gsk_shader_args_builder_set_uint(self.to_glib_none().0, idx, value);
        }
    }

    #[doc(alias = "gsk_shader_args_builder_set_vec2")]
    pub fn set_vec2(&self, idx: i32, value: &graphene::Vec2) {
        unsafe {
            ffi::gsk_shader_args_builder_set_vec2(
                self.to_glib_none().0,
                idx,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gsk_shader_args_builder_set_vec3")]
    pub fn set_vec3(&self, idx: i32, value: &graphene::Vec3) {
        unsafe {
            ffi::gsk_shader_args_builder_set_vec3(
                self.to_glib_none().0,
                idx,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gsk_shader_args_builder_set_vec4")]
    pub fn set_vec4(&self, idx: i32, value: &graphene::Vec4) {
        unsafe {
            ffi::gsk_shader_args_builder_set_vec4(
                self.to_glib_none().0,
                idx,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gsk_shader_args_builder_to_args")]
    pub fn to_args(&self) -> glib::Bytes {
        unsafe { from_glib_full(ffi::gsk_shader_args_builder_to_args(self.to_glib_none().0)) }
    }
}
