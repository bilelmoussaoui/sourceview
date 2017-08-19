// This file was generated by gir (4b09025) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct SearchSettings(Object<ffi::GtkSourceSearchSettings>);

    match fn {
        get_type => || ffi::gtk_source_search_settings_get_type(),
    }
}

impl SearchSettings {
    #[cfg(feature = "v3_10")]
    pub fn new() -> SearchSettings {
        unsafe {
            from_glib_full(ffi::gtk_source_search_settings_new())
        }
    }
}

pub trait SearchSettingsExt {
    #[cfg(feature = "v3_10")]
    fn get_at_word_boundaries(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_case_sensitive(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_regex_enabled(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_search_text(&self) -> Option<String>;

    #[cfg(feature = "v3_10")]
    fn get_wrap_around(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn set_at_word_boundaries(&self, at_word_boundaries: bool);

    #[cfg(feature = "v3_10")]
    fn set_case_sensitive(&self, case_sensitive: bool);

    #[cfg(feature = "v3_10")]
    fn set_regex_enabled(&self, regex_enabled: bool);

    #[cfg(feature = "v3_10")]
    fn set_search_text<'a, P: Into<Option<&'a str>>>(&self, search_text: P);

    #[cfg(feature = "v3_10")]
    fn set_wrap_around(&self, wrap_around: bool);
}

impl<O: IsA<SearchSettings>> SearchSettingsExt for O {
    #[cfg(feature = "v3_10")]
    fn get_at_word_boundaries(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_at_word_boundaries(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_case_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_case_sensitive(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_regex_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_regex_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_search_text(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_search_settings_get_search_text(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_wrap_around(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_search_settings_get_wrap_around(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_at_word_boundaries(&self, at_word_boundaries: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_at_word_boundaries(self.to_glib_none().0, at_word_boundaries.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_case_sensitive(&self, case_sensitive: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_case_sensitive(self.to_glib_none().0, case_sensitive.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_regex_enabled(&self, regex_enabled: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_regex_enabled(self.to_glib_none().0, regex_enabled.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_search_text<'a, P: Into<Option<&'a str>>>(&self, search_text: P) {
        let search_text = search_text.into();
        let search_text = search_text.to_glib_none();
        unsafe {
            ffi::gtk_source_search_settings_set_search_text(self.to_glib_none().0, search_text.0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_wrap_around(&self, wrap_around: bool) {
        unsafe {
            ffi::gtk_source_search_settings_set_wrap_around(self.to_glib_none().0, wrap_around.to_glib());
        }
    }
}
