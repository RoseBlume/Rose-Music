// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, Bin, Buildable, CellArea, CellEditable, CellLayout, ComboBox, Container, ResizeMode,
    SensitivityType, TreeModel, Widget,
};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkComboBoxText")]
    pub struct ComboBoxText(Object<ffi::GtkComboBoxText, ffi::GtkComboBoxTextClass>) @extends ComboBox, Bin, Container, Widget, @implements Buildable, CellEditable, CellLayout;

    match fn {
        type_ => || ffi::gtk_combo_box_text_get_type(),
    }
}

impl ComboBoxText {
    pub const NONE: Option<&'static ComboBoxText> = None;

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
    /// This method returns an instance of [`ComboBoxTextBuilder`](crate::builders::ComboBoxTextBuilder) which can be used to create [`ComboBoxText`] objects.
    pub fn builder() -> ComboBoxTextBuilder {
        ComboBoxTextBuilder::new()
    }
}

impl Default for ComboBoxText {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ComboBoxText`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ComboBoxTextBuilder {
    builder: glib::object::ObjectBuilder<'static, ComboBoxText>,
}

impl ComboBoxTextBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn active(self, active: i32) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn active_id(self, active_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("active-id", active_id.into()),
        }
    }

    pub fn button_sensitivity(self, button_sensitivity: SensitivityType) -> Self {
        Self {
            builder: self
                .builder
                .property("button-sensitivity", button_sensitivity),
        }
    }

    pub fn cell_area(self, cell_area: &impl IsA<CellArea>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-area", cell_area.clone().upcast()),
        }
    }

    pub fn column_span_column(self, column_span_column: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("column-span-column", column_span_column),
        }
    }

    pub fn entry_text_column(self, entry_text_column: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("entry-text-column", entry_text_column),
        }
    }

    pub fn has_entry(self, has_entry: bool) -> Self {
        Self {
            builder: self.builder.property("has-entry", has_entry),
        }
    }

    pub fn has_frame(self, has_frame: bool) -> Self {
        Self {
            builder: self.builder.property("has-frame", has_frame),
        }
    }

    pub fn id_column(self, id_column: i32) -> Self {
        Self {
            builder: self.builder.property("id-column", id_column),
        }
    }

    pub fn model(self, model: &impl IsA<TreeModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    pub fn popup_fixed_width(self, popup_fixed_width: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("popup-fixed-width", popup_fixed_width),
        }
    }

    pub fn row_span_column(self, row_span_column: i32) -> Self {
        Self {
            builder: self.builder.property("row-span-column", row_span_column),
        }
    }

    pub fn wrap_width(self, wrap_width: i32) -> Self {
        Self {
            builder: self.builder.property("wrap-width", wrap_width),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
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

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
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

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
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

    pub fn editing_canceled(self, editing_canceled: bool) -> Self {
        Self {
            builder: self.builder.property("editing-canceled", editing_canceled),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ComboBoxText`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ComboBoxText {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ComboBoxText>> Sealed for T {}
}

pub trait ComboBoxTextExt: IsA<ComboBoxText> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_combo_box_text_append")]
    fn append(&self, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_append_text")]
    fn append_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_append_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_get_active_text")]
    #[doc(alias = "get_active_text")]
    fn active_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_combo_box_text_get_active_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_combo_box_text_insert")]
    fn insert(&self, position: i32, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert(
                self.as_ref().to_glib_none().0,
                position,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_insert_text")]
    fn insert_text(&self, position: i32, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_insert_text(
                self.as_ref().to_glib_none().0,
                position,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_prepend")]
    fn prepend(&self, id: Option<&str>, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_prepend_text")]
    fn prepend_text(&self, text: &str) {
        unsafe {
            ffi::gtk_combo_box_text_prepend_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_combo_box_text_remove")]
    fn remove(&self, position: i32) {
        unsafe {
            ffi::gtk_combo_box_text_remove(self.as_ref().to_glib_none().0, position);
        }
    }

    #[doc(alias = "gtk_combo_box_text_remove_all")]
    fn remove_all(&self) {
        unsafe {
            ffi::gtk_combo_box_text_remove_all(self.as_ref().to_glib_none().0);
        }
    }
}

impl<O: IsA<ComboBoxText>> ComboBoxTextExt for O {}

impl fmt::Display for ComboBoxText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ComboBoxText")
    }
}