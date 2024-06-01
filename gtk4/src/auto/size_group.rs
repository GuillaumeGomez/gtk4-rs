// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Buildable, SizeGroupMode, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSizeGroup")]
    pub struct SizeGroup(Object<ffi::GtkSizeGroup>) @implements Buildable;

    match fn {
        type_ => || ffi::gtk_size_group_get_type(),
    }
}

impl SizeGroup {
    #[doc(alias = "gtk_size_group_new")]
    pub fn new(mode: SizeGroupMode) -> SizeGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_size_group_new(mode.into_glib())) }
    }

    #[doc(alias = "gtk_size_group_add_widget")]
    pub fn add_widget(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_size_group_add_widget(self.to_glib_none().0, widget.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_size_group_get_mode")]
    #[doc(alias = "get_mode")]
    pub fn mode(&self) -> SizeGroupMode {
        unsafe { from_glib(ffi::gtk_size_group_get_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_size_group_get_widgets")]
    #[doc(alias = "get_widgets")]
    pub fn widgets(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_size_group_get_widgets(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_size_group_remove_widget")]
    pub fn remove_widget(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_size_group_remove_widget(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_size_group_set_mode")]
    pub fn set_mode(&self, mode: SizeGroupMode) {
        unsafe {
            ffi::gtk_size_group_set_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "mode")]
    pub fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&SizeGroup) + 'static>(
            this: *mut ffi::GtkSizeGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
