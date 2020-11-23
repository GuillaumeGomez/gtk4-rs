// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use crate::RoundedRect;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct RoundedClipNode(Object<ffi::GskRoundedClipNode>) @extends RenderNode;

    match fn {
        get_type => || ffi::gsk_rounded_clip_node_get_type(),
    }
}

impl RoundedClipNode {
    pub fn new<P: IsA<RenderNode>>(child: &P, clip: &RoundedRect) -> RoundedClipNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gsk_rounded_clip_node_new(
                child.as_ref().to_glib_none().0,
                clip.to_glib_none().0,
            ))
        }
    }

    pub fn get_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(ffi::gsk_rounded_clip_node_get_child(self.to_glib_none().0)) }
    }

    pub fn get_clip(&self) -> Option<RoundedRect> {
        unsafe { from_glib_none(ffi::gsk_rounded_clip_node_get_clip(self.to_glib_none().0)) }
    }
}

impl fmt::Display for RoundedClipNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RoundedClipNode")
    }
}
