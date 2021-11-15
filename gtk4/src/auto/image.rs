// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::IconSize;
use crate::ImageType;
use crate::LayoutManager;
use crate::Overflow;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkImage")]
    pub struct Image(Object<ffi::GtkImage>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_image_get_type(),
    }
}

impl Image {
    #[doc(alias = "gtk_image_new")]
    pub fn new() -> Image {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_image_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_image_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file(filename: impl AsRef<std::path::Path>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_file(
                filename.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_gicon")]
    #[doc(alias = "new_from_gicon")]
    pub fn from_gicon(icon: &impl IsA<gio::Icon>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_gicon(
                icon.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_icon_name")]
    #[doc(alias = "new_from_icon_name")]
    pub fn from_icon_name(icon_name: Option<&str>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_icon_name(
                icon_name.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_paintable")]
    #[doc(alias = "new_from_paintable")]
    pub fn from_paintable(paintable: Option<&impl IsA<gdk::Paintable>>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_paintable(
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_pixbuf")]
    #[doc(alias = "new_from_pixbuf")]
    pub fn from_pixbuf(pixbuf: Option<&gdk_pixbuf::Pixbuf>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_pixbuf(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource(resource_path: &str) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_resource(
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Image`] objects.
    ///
    /// This method returns an instance of [`ImageBuilder`] which can be used to create [`Image`] objects.
    pub fn builder() -> ImageBuilder {
        ImageBuilder::default()
    }

    #[doc(alias = "gtk_image_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::gtk_image_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_get_gicon")]
    #[doc(alias = "get_gicon")]
    pub fn gicon(&self) -> Option<gio::Icon> {
        unsafe { from_glib_none(ffi::gtk_image_get_gicon(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_image_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_icon_size")]
    #[doc(alias = "get_icon_size")]
    pub fn icon_size(&self) -> IconSize {
        unsafe { from_glib(ffi::gtk_image_get_icon_size(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_paintable")]
    #[doc(alias = "get_paintable")]
    pub fn paintable(&self) -> Option<gdk::Paintable> {
        unsafe { from_glib_none(ffi::gtk_image_get_paintable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_pixel_size")]
    #[doc(alias = "get_pixel_size")]
    pub fn pixel_size(&self) -> i32 {
        unsafe { ffi::gtk_image_get_pixel_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_image_get_storage_type")]
    #[doc(alias = "get_storage_type")]
    pub fn storage_type(&self) -> ImageType {
        unsafe { from_glib(ffi::gtk_image_get_storage_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_set_from_file")]
    pub fn set_from_file(&self, filename: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::gtk_image_set_from_file(
                self.to_glib_none().0,
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_image_set_from_gicon")]
    pub fn set_from_gicon(&self, icon: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::gtk_image_set_from_gicon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_from_icon_name")]
    pub fn set_from_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_from_paintable")]
    pub fn set_from_paintable(&self, paintable: Option<&impl IsA<gdk::Paintable>>) {
        unsafe {
            ffi::gtk_image_set_from_paintable(
                self.to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_image_set_from_pixbuf")]
    pub fn set_from_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_image_set_from_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_from_resource")]
    pub fn set_from_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_image_set_from_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_icon_size")]
    pub fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_image_set_icon_size(self.to_glib_none().0, icon_size.into_glib());
        }
    }

    #[doc(alias = "gtk_image_set_pixel_size")]
    pub fn set_pixel_size(&self, pixel_size: i32) {
        unsafe {
            ffi::gtk_image_set_pixel_size(self.to_glib_none().0, pixel_size);
        }
    }

    pub fn file(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "file")
    }

    pub fn set_file(&self, file: Option<&str>) {
        glib::ObjectExt::set_property(self, "file", &file)
    }

    pub fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>) {
        glib::ObjectExt::set_property(self, "gicon", &gicon)
    }

    #[doc(alias = "icon-name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        glib::ObjectExt::set_property(self, "icon-name", &icon_name)
    }

    pub fn set_paintable<P: IsA<gdk::Paintable>>(&self, paintable: Option<&P>) {
        glib::ObjectExt::set_property(self, "paintable", &paintable)
    }

    pub fn resource(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self, "resource")
    }

    pub fn set_resource(&self, resource: Option<&str>) {
        glib::ObjectExt::set_property(self, "resource", &resource)
    }

    #[doc(alias = "use-fallback")]
    pub fn uses_fallback(&self) -> bool {
        glib::ObjectExt::property(self, "use-fallback")
    }

    #[doc(alias = "use-fallback")]
    pub fn set_use_fallback(&self, use_fallback: bool) {
        glib::ObjectExt::set_property(self, "use-fallback", &use_fallback)
    }

    #[doc(alias = "file")]
    pub fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_file_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "gicon")]
    pub fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::gicon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-size")]
    pub fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "paintable")]
    pub fn connect_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paintable_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::paintable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_paintable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixel-size")]
    pub fn connect_pixel_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixel_size_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::pixel-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixel_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resource")]
    pub fn connect_resource_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resource_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::resource\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resource_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "storage-type")]
    pub fn connect_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_storage_type_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::storage-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_storage_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-fallback")]
    pub fn connect_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_fallback_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::use-fallback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_fallback_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Image`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ImageBuilder {
    file: Option<String>,
    gicon: Option<gio::Icon>,
    icon_name: Option<String>,
    icon_size: Option<IconSize>,
    paintable: Option<gdk::Paintable>,
    pixel_size: Option<i32>,
    resource: Option<String>,
    use_fallback: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
}

impl ImageBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ImageBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Image`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> Image {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref gicon) = self.gicon {
            properties.push(("gicon", gicon));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref icon_size) = self.icon_size {
            properties.push(("icon-size", icon_size));
        }
        if let Some(ref paintable) = self.paintable {
            properties.push(("paintable", paintable));
        }
        if let Some(ref pixel_size) = self.pixel_size {
            properties.push(("pixel-size", pixel_size));
        }
        if let Some(ref resource) = self.resource {
            properties.push(("resource", resource));
        }
        if let Some(ref use_fallback) = self.use_fallback {
            properties.push(("use-fallback", use_fallback));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        glib::Object::new::<Image>(&properties).expect("Failed to create an instance of Image")
    }

    pub fn file(mut self, file: &str) -> Self {
        self.file = Some(file.to_string());
        self
    }

    pub fn gicon(mut self, gicon: &impl IsA<gio::Icon>) -> Self {
        self.gicon = Some(gicon.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn paintable(mut self, paintable: &impl IsA<gdk::Paintable>) -> Self {
        self.paintable = Some(paintable.clone().upcast());
        self
    }

    pub fn pixel_size(mut self, pixel_size: i32) -> Self {
        self.pixel_size = Some(pixel_size);
        self
    }

    pub fn resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn use_fallback(mut self, use_fallback: bool) -> Self {
        self.use_fallback = Some(use_fallback);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Image")
    }
}
