// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Language;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct LanguageManager(Object<ffi::GtkSourceLanguageManager, ffi::GtkSourceLanguageManagerClass, LanguageManagerClass>);

    match fn {
        get_type => || ffi::gtk_source_language_manager_get_type(),
    }
}

impl LanguageManager {
    pub fn new() -> LanguageManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_source_language_manager_new())
        }
    }

    pub fn get_default() -> Option<LanguageManager> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_source_language_manager_get_default())
        }
    }
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_LANGUAGE_MANAGER: Option<&LanguageManager> = None;

pub trait LanguageManagerExt: 'static {
    fn get_language(&self, id: &str) -> Option<Language>;

    fn get_language_ids(&self) -> Vec<GString>;

    fn get_search_path(&self) -> Vec<GString>;

    fn guess_language(&self, filename: Option<&str>, content_type: Option<&str>) -> Option<Language>;

    fn set_search_path(&self, dirs: &[&str]);

    fn connect_property_language_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LanguageManager>> LanguageManagerExt for O {
    fn get_language(&self, id: &str) -> Option<Language> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_manager_get_language(self.as_ref().to_glib_none().0, id.to_glib_none().0))
        }
    }

    fn get_language_ids(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_language_manager_get_language_ids(self.as_ref().to_glib_none().0))
        }
    }

    fn get_search_path(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_language_manager_get_search_path(self.as_ref().to_glib_none().0))
        }
    }

    fn guess_language(&self, filename: Option<&str>, content_type: Option<&str>) -> Option<Language> {
        unsafe {
            from_glib_none(ffi::gtk_source_language_manager_guess_language(self.as_ref().to_glib_none().0, filename.to_glib_none().0, content_type.to_glib_none().0))
        }
    }

    fn set_search_path(&self, dirs: &[&str]) {
        unsafe {
            ffi::gtk_source_language_manager_set_search_path(self.as_ref().to_glib_none().0, dirs.to_glib_none().0);
        }
    }

    fn connect_property_language_ids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::language-ids\0".as_ptr() as *const _,
                Some(transmute(notify_language_ids_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_search_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::search-path\0".as_ptr() as *const _,
                Some(transmute(notify_search_path_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_language_ids_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkSourceLanguageManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LanguageManager> {
    let f: &F = &*(f as *const F);
    f(&LanguageManager::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_search_path_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkSourceLanguageManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LanguageManager> {
    let f: &F = &*(f as *const F);
    f(&LanguageManager::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for LanguageManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LanguageManager")
    }
}
