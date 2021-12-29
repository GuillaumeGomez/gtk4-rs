// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use glib::StaticType;

glib::wrapper! {
    #[doc(alias = "GdkTouchEvent")]
    pub struct TouchEvent(Shared<ffi::GdkTouchEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent),
        unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
    }
}
impl glib::StaticType for TouchEvent {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gdk_touch_event_get_type()) }
    }
}

impl TouchEvent {
    #[doc(alias = "gdk_touch_event_get_emulating_pointer")]
    #[doc(alias = "get_emulating_pointer")]
    pub fn is_emulating_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_touch_event_get_emulating_pointer(
                self.to_glib_none().0,
            ))
        }
    }
}
