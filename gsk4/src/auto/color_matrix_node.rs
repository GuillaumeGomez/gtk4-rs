// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskColorMatrixNode")]
    pub struct ColorMatrixNode(Shared<ffi::GskColorMatrixNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for ColorMatrixNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_color_matrix_node_get_type()) }
    }
}

impl ColorMatrixNode {
    #[doc(alias = "gsk_color_matrix_node_new")]
    pub fn new(
        child: impl AsRef<RenderNode>,
        color_matrix: &graphene::Matrix,
        color_offset: &graphene::Vec4,
    ) -> ColorMatrixNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_color_matrix_node_new(
                child.as_ref().to_glib_none().0,
                color_matrix.to_glib_none().0,
                color_offset.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_color_matrix_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_color_matrix_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_color_matrix_node_get_color_matrix")]
    #[doc(alias = "get_color_matrix")]
    pub fn color_matrix(&self) -> graphene::Matrix {
        unsafe {
            from_glib_none(ffi::gsk_color_matrix_node_get_color_matrix(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_color_matrix_node_get_color_offset")]
    #[doc(alias = "get_color_offset")]
    pub fn color_offset(&self) -> graphene::Vec4 {
        unsafe {
            from_glib_none(ffi::gsk_color_matrix_node_get_color_offset(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ColorMatrixNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ColorMatrixNode")
    }
}
