// This file was generated by gir (83d5a2f) from gir-files (db49619)
// DO NOT EDIT

use StyleSchemeChooser;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use gtk_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct StyleSchemeChooserButton(Object<ffi::GtkSourceStyleSchemeChooserButton>): [
        gtk::Widget => gtk_ffi::GtkWidget,
        StyleSchemeChooser,
    ];

    match fn {
        get_type => || ffi::gtk_source_style_scheme_chooser_button_get_type(),
    }
}

impl StyleSchemeChooserButton {
    #[cfg(feature = "v3_16")]
    pub fn new() -> StyleSchemeChooserButton {
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_style_scheme_chooser_button_new()).downcast_unchecked()
        }
    }
}

#[cfg(feature = "v3_16")]
impl Default for StyleSchemeChooserButton {
    fn default() -> Self {
        Self::new()
    }
}
