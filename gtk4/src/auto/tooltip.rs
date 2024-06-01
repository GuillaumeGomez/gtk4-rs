// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Widget};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GtkTooltip")]
    pub struct Tooltip(Object<ffi::GtkTooltip>);

    match fn {
        type_ => || ffi::gtk_tooltip_get_type(),
    }
}

impl Tooltip {
    #[doc(alias = "gtk_tooltip_set_custom")]
    pub fn set_custom(&self, custom_widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_tooltip_set_custom(
                self.to_glib_none().0,
                custom_widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tooltip_set_icon")]
    pub fn set_icon(&self, paintable: Option<&impl IsA<gdk::Paintable>>) {
        unsafe {
            ffi::gtk_tooltip_set_icon(
                self.to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tooltip_set_icon_from_gicon")]
    pub fn set_icon_from_gicon(&self, gicon: Option<&impl IsA<gio::Icon>>) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_gicon(
                self.to_glib_none().0,
                gicon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tooltip_set_icon_from_icon_name")]
    pub fn set_icon_from_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tooltip_set_markup")]
    pub fn set_markup(&self, markup: Option<&str>) {
        unsafe {
            ffi::gtk_tooltip_set_markup(self.to_glib_none().0, markup.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tooltip_set_text")]
    pub fn set_text(&self, text: Option<&str>) {
        unsafe {
            ffi::gtk_tooltip_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_tooltip_set_tip_area")]
    pub fn set_tip_area(&self, rect: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_tooltip_set_tip_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }
}
