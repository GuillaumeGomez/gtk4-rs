// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::RenderNode;
use glib::translate::*;
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GskCrossFadeNode")]
    pub struct CrossFadeNode(Shared<ffi::GskCrossFadeNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl glib::StaticType for CrossFadeNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_cross_fade_node_get_type()) }
    }
}

impl CrossFadeNode {
    #[doc(alias = "gsk_cross_fade_node_new")]
    pub fn new(
        start: impl AsRef<RenderNode>,
        end: impl AsRef<RenderNode>,
        progress: f32,
    ) -> CrossFadeNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_cross_fade_node_new(
                start.as_ref().to_glib_none().0,
                end.as_ref().to_glib_none().0,
                progress,
            ))
        }
    }

    #[doc(alias = "gsk_cross_fade_node_get_end_child")]
    #[doc(alias = "get_end_child")]
    pub fn end_child(&self) -> RenderNode {
        unsafe {
            from_glib_none(ffi::gsk_cross_fade_node_get_end_child(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_cross_fade_node_get_progress")]
    #[doc(alias = "get_progress")]
    pub fn progress(&self) -> f32 {
        unsafe { ffi::gsk_cross_fade_node_get_progress(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_cross_fade_node_get_start_child")]
    #[doc(alias = "get_start_child")]
    pub fn start_child(&self) -> RenderNode {
        unsafe {
            from_glib_none(ffi::gsk_cross_fade_node_get_start_child(
                self.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for CrossFadeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CrossFadeNode")
    }
}
