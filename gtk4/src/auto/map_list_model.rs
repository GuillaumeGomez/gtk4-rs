// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use crate::SectionModel;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
glib::wrapper! {
    #[doc(alias = "GtkMapListModel")]
    pub struct MapListModel(Object<ffi::GtkMapListModel, ffi::GtkMapListModelClass>) @implements gio::ListModel, SectionModel;

    match fn {
        type_ => || ffi::gtk_map_list_model_get_type(),
    }
}

#[cfg(not(any(feature = "v4_12")))]
glib::wrapper! {
    #[doc(alias = "GtkMapListModel")]
    pub struct MapListModel(Object<ffi::GtkMapListModel, ffi::GtkMapListModelClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::gtk_map_list_model_get_type(),
    }
}

impl MapListModel {
    #[doc(alias = "gtk_map_list_model_new")]
    pub fn new<P: Fn(&glib::Object) -> glib::Object + 'static>(
        model: Option<impl IsA<gio::ListModel>>,
        map_func: P,
    ) -> MapListModel {
        assert_initialized_main_thread!();
        let map_func_data: Box_<P> = Box_::new(map_func);
        unsafe extern "C" fn map_func_func<P: Fn(&glib::Object) -> glib::Object + 'static>(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut glib::gobject_ffi::GObject {
            let item = from_glib_full(item);
            let callback = &*(user_data as *mut P);
            (*callback)(&item).to_glib_full()
        }
        let map_func = Some(map_func_func::<P> as _);
        unsafe extern "C" fn user_destroy_func<P: Fn(&glib::Object) -> glib::Object + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(user_destroy_func::<P> as _);
        let super_callback0: Box_<P> = map_func_data;
        unsafe {
            from_glib_full(ffi::gtk_map_list_model_new(
                model.map(|p| p.upcast()).into_glib_ptr(),
                map_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            ))
        }
    }

    #[doc(alias = "gtk_map_list_model_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_map_list_model_get_model(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_map_list_model_has_map")]
    pub fn has_map(&self) -> bool {
        unsafe { from_glib(ffi::gtk_map_list_model_has_map(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_map_list_model_set_map_func")]
    pub fn set_map_func<P: Fn(&glib::Object) -> glib::Object + 'static>(&self, map_func: P) {
        let map_func_data: Box_<P> = Box_::new(map_func);
        unsafe extern "C" fn map_func_func<P: Fn(&glib::Object) -> glib::Object + 'static>(
            item: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
        ) -> *mut glib::gobject_ffi::GObject {
            let item = from_glib_full(item);
            let callback = &*(user_data as *mut P);
            (*callback)(&item).to_glib_full()
        }
        let map_func = Some(map_func_func::<P> as _);
        unsafe extern "C" fn user_destroy_func<P: Fn(&glib::Object) -> glib::Object + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(user_destroy_func::<P> as _);
        let super_callback0: Box_<P> = map_func_data;
        unsafe {
            ffi::gtk_map_list_model_set_map_func(
                self.to_glib_none().0,
                map_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "gtk_map_list_model_set_model")]
    pub fn set_model(&self, model: Option<&impl IsA<gio::ListModel>>) {
        unsafe {
            ffi::gtk_map_list_model_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "has-map")]
    pub fn connect_has_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_map_trampoline<F: Fn(&MapListModel) + 'static>(
            this: *mut ffi::GtkMapListModel,
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
                b"notify::has-map\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_has_map_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
