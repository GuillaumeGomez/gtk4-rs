// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::Buildable;
use crate::CellEditable;
use crate::CellLayout;
use crate::ComboBox;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Overflow;
use crate::SensitivityType;
use crate::TreeModel;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkComboBoxText")]
    pub struct ComboBoxText(Object<ffi::GtkComboBoxText>) @extends ComboBox, Widget, @implements Accessible, Buildable, ConstraintTarget, CellEditable, CellLayout;

    match fn {
        type_ => || ffi::gtk_combo_box_text_get_type(),
    }
}

impl ComboBoxText {
    #[doc(alias = "gtk_combo_box_text_new")]
    pub fn new() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_combo_box_text_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_combo_box_text_new_with_entry")]
    #[doc(alias = "new_with_entry")]
    pub fn with_entry() -> ComboBoxText {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_combo_box_text_new_with_entry()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ComboBoxText`] objects.
    ///
    /// This method returns an instance of [`ComboBoxTextBuilder`] which can be used to create [`ComboBoxText`] objects.
    pub fn builder() -> ComboBoxTextBuilder {
        ComboBoxTextBuilder::default()
    }

    #[doc(alias = "gtk_combo_box_text_append")]
    pub fn append(&self, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append(
                self.to_glib_none().0,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_append_text")]
    pub fn append_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_combo_box_text_get_active_text")]
    #[doc(alias = "get_active_text")]
    pub fn active_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_combo_box_text_get_active_text(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_combo_box_text_insert")]
    pub fn insert(&self, position: i32, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert(
                self.to_glib_none().0,
                position,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_insert_text")]
    pub fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert_text(
                self.to_glib_none().0,
                position,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_prepend")]
    pub fn prepend(&self, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend(
                self.to_glib_none().0,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_prepend_text")]
    pub fn prepend_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_combo_box_text_remove")]
    pub fn remove(&self, position: i32) {
        unsafe {
            ffi::gtk_combo_box_text_remove(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_combo_box_text_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::gtk_combo_box_text_remove_all(self.to_glib_none().0);
        }
    }
}

impl Default for ComboBoxText {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ComboBoxText`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct ComboBoxTextBuilder {
    active: Option<i32>,
    active_id: Option<String>,
    button_sensitivity: Option<SensitivityType>,
    child: Option<Widget>,
    entry_text_column: Option<i32>,
    has_entry: Option<bool>,
    has_frame: Option<bool>,
    id_column: Option<i32>,
    model: Option<TreeModel>,
    popup_fixed_width: Option<bool>,
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
    editing_canceled: Option<bool>,
}

impl ComboBoxTextBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ComboBoxTextBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ComboBoxText`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> ComboBoxText {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref active) = self.active {
            properties.push(("active", active));
        }
        if let Some(ref active_id) = self.active_id {
            properties.push(("active-id", active_id));
        }
        if let Some(ref button_sensitivity) = self.button_sensitivity {
            properties.push(("button-sensitivity", button_sensitivity));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref entry_text_column) = self.entry_text_column {
            properties.push(("entry-text-column", entry_text_column));
        }
        if let Some(ref has_entry) = self.has_entry {
            properties.push(("has-entry", has_entry));
        }
        if let Some(ref has_frame) = self.has_frame {
            properties.push(("has-frame", has_frame));
        }
        if let Some(ref id_column) = self.id_column {
            properties.push(("id-column", id_column));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref popup_fixed_width) = self.popup_fixed_width {
            properties.push(("popup-fixed-width", popup_fixed_width));
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
        if let Some(ref editing_canceled) = self.editing_canceled {
            properties.push(("editing-canceled", editing_canceled));
        }
        glib::Object::new::<ComboBoxText>(&properties)
            .expect("Failed to create an instance of ComboBoxText")
    }

    pub fn active(mut self, active: i32) -> Self {
        self.active = Some(active);
        self
    }

    pub fn active_id(mut self, active_id: &str) -> Self {
        self.active_id = Some(active_id.to_string());
        self
    }

    pub fn button_sensitivity(mut self, button_sensitivity: SensitivityType) -> Self {
        self.button_sensitivity = Some(button_sensitivity);
        self
    }

    pub fn child(mut self, child: &impl IsA<Widget>) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn entry_text_column(mut self, entry_text_column: i32) -> Self {
        self.entry_text_column = Some(entry_text_column);
        self
    }

    pub fn has_entry(mut self, has_entry: bool) -> Self {
        self.has_entry = Some(has_entry);
        self
    }

    pub fn has_frame(mut self, has_frame: bool) -> Self {
        self.has_frame = Some(has_frame);
        self
    }

    pub fn id_column(mut self, id_column: i32) -> Self {
        self.id_column = Some(id_column);
        self
    }

    pub fn model(mut self, model: &impl IsA<TreeModel>) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }

    pub fn popup_fixed_width(mut self, popup_fixed_width: bool) -> Self {
        self.popup_fixed_width = Some(popup_fixed_width);
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

    pub fn editing_canceled(mut self, editing_canceled: bool) -> Self {
        self.editing_canceled = Some(editing_canceled);
        self
    }
}

impl fmt::Display for ComboBoxText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ComboBoxText")
    }
}
