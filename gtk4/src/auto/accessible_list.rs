// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Accessible};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccessibleList(Boxed<ffi::GtkAccessibleList>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::gtk_accessible_list_get_type(), ptr as *mut _) as *mut ffi::GtkAccessibleList,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::gtk_accessible_list_get_type(), ptr as *mut _),
        type_ => || ffi::gtk_accessible_list_get_type(),
    }
}

impl AccessibleList {
    #[doc(alias = "gtk_accessible_list_new_from_array")]
    #[doc(alias = "new_from_array")]
    pub fn from_array(accessibles: &[Accessible]) -> AccessibleList {
        assert_initialized_main_thread!();
        let n_accessibles = accessibles.len() as _;
        unsafe {
            from_glib_full(ffi::gtk_accessible_list_new_from_array(
                accessibles.to_glib_none().0,
                n_accessibles,
            ))
        }
    }

    #[doc(alias = "gtk_accessible_list_get_objects")]
    #[doc(alias = "get_objects")]
    pub fn objects(&self) -> Vec<Accessible> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_accessible_list_get_objects(
                mut_override(self.to_glib_none().0),
            ))
        }
    }
}
