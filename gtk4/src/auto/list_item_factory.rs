// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;

glib::wrapper! {
    #[doc(alias = "GtkListItemFactory")]
    pub struct ListItemFactory(Object<ffi::GtkListItemFactory, ffi::GtkListItemFactoryClass>);

    match fn {
        type_ => || ffi::gtk_list_item_factory_get_type(),
    }
}

impl ListItemFactory {
    pub const NONE: Option<&'static ListItemFactory> = None;
}
