// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Sorter};

glib::wrapper! {
    #[doc(alias = "GtkCustomSorter")]
    pub struct CustomSorter(Object<ffi::GtkCustomSorter, ffi::GtkCustomSorterClass>) @extends Sorter;

    match fn {
        type_ => || ffi::gtk_custom_sorter_get_type(),
    }
}

impl CustomSorter {}
