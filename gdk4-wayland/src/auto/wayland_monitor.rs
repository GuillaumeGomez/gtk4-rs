// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;

glib::wrapper! {
    #[doc(alias = "GdkWaylandMonitor")]
    pub struct WaylandMonitor(Object<ffi::GdkWaylandMonitor, ffi::GdkWaylandMonitorClass>) @extends gdk::Monitor;

    match fn {
        type_ => || ffi::gdk_wayland_monitor_get_type(),
    }
}

impl WaylandMonitor {}
