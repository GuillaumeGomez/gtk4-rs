// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, Editable, LayoutManager,
    Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkSearchBar")]
    pub struct SearchBar(Object<ffi::GtkSearchBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_search_bar_get_type(),
    }
}

impl SearchBar {
    #[doc(alias = "gtk_search_bar_new")]
    pub fn new() -> SearchBar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_search_bar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SearchBar`] objects.
    ///
    /// This method returns an instance of [`SearchBarBuilder`](crate::builders::SearchBarBuilder) which can be used to create [`SearchBar`] objects.
    pub fn builder() -> SearchBarBuilder {
        SearchBarBuilder::new()
    }

    #[doc(alias = "gtk_search_bar_connect_entry")]
    pub fn connect_entry(&self, entry: &impl IsA<Editable>) {
        unsafe {
            ffi::gtk_search_bar_connect_entry(
                self.to_glib_none().0,
                entry.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_search_bar_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_search_bar_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_search_bar_get_key_capture_widget")]
    #[doc(alias = "get_key_capture_widget")]
    pub fn key_capture_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_search_bar_get_key_capture_widget(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_search_bar_get_search_mode")]
    #[doc(alias = "get_search_mode")]
    pub fn is_search_mode(&self) -> bool {
        unsafe { from_glib(ffi::gtk_search_bar_get_search_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_search_bar_get_show_close_button")]
    #[doc(alias = "get_show_close_button")]
    pub fn shows_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_bar_get_show_close_button(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_search_bar_set_child")]
    pub fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_search_bar_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_search_bar_set_key_capture_widget")]
    pub fn set_key_capture_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_search_bar_set_key_capture_widget(
                self.to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_search_bar_set_search_mode")]
    pub fn set_search_mode(&self, search_mode: bool) {
        unsafe {
            ffi::gtk_search_bar_set_search_mode(self.to_glib_none().0, search_mode.into_glib());
        }
    }

    #[doc(alias = "gtk_search_bar_set_show_close_button")]
    pub fn set_show_close_button(&self, visible: bool) {
        unsafe {
            ffi::gtk_search_bar_set_show_close_button(self.to_glib_none().0, visible.into_glib());
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&SearchBar) + 'static>(
            this: *mut ffi::GtkSearchBar,
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
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "key-capture-widget")]
    pub fn connect_key_capture_widget_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_key_capture_widget_trampoline<F: Fn(&SearchBar) + 'static>(
            this: *mut ffi::GtkSearchBar,
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
                b"notify::key-capture-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_key_capture_widget_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "search-mode-enabled")]
    pub fn connect_search_mode_enabled_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_mode_enabled_trampoline<F: Fn(&SearchBar) + 'static>(
            this: *mut ffi::GtkSearchBar,
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
                b"notify::search-mode-enabled\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_search_mode_enabled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-close-button")]
    pub fn connect_show_close_button_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_close_button_trampoline<F: Fn(&SearchBar) + 'static>(
            this: *mut ffi::GtkSearchBar,
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
                b"notify::show-close-button\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_close_button_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SearchBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SearchBarBuilder {
    builder: glib::object::ObjectBuilder<'static, SearchBar>,
}

impl SearchBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn key_capture_widget(self, key_capture_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("key-capture-widget", key_capture_widget.clone().upcast()),
        }
    }

    pub fn search_mode_enabled(self, search_mode_enabled: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("search-mode-enabled", search_mode_enabled),
        }
    }

    pub fn show_close_button(self, show_close_button: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-close-button", show_close_button),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SearchBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SearchBar {
        self.builder.build()
    }
}
