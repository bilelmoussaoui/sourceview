// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use View;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Map(Object<ffi::GtkSourceMap, ffi::GtkSourceMapClass, MapClass>) @extends View, gtk::TextView, gtk::Container, gtk::Widget, @implements gtk::Buildable, gtk::Scrollable;

    match fn {
        get_type => || ffi::gtk_source_map_get_type(),
    }
}

impl Map {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    pub fn new() -> Map {
        assert_initialized_main_thread!();
        unsafe {
            gtk::Widget::from_glib_none(ffi::gtk_source_map_new()).unsafe_cast()
        }
    }
}

#[cfg(any(feature = "v3_18", feature = "dox"))]
impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MAP: Option<&Map> = None;

pub trait MapExt: 'static {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_view(&self) -> Option<View>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_view<P: IsA<View>>(&self, view: &P);

    fn get_property_view(&self) -> Option<View>;

    fn set_property_view(&self, view: Option<&View>);

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Map>> MapExt for O {
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_view(&self) -> Option<View> {
        unsafe {
            from_glib_none(ffi::gtk_source_map_get_view(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_view<P: IsA<View>>(&self, view: &P) {
        unsafe {
            ffi::gtk_source_map_set_view(self.as_ref().to_glib_none().0, view.as_ref().to_glib_none().0);
        }
    }

    fn get_property_view(&self) -> Option<View> {
        unsafe {
            let mut value = Value::from_type(<View as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"view\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_view(&self, view: Option<&View>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"view\0".as_ptr() as *const _, Value::from(view).to_glib_none().0);
        }
    }

    fn connect_property_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::view\0".as_ptr() as *const _,
                Some(transmute(notify_view_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_view_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkSourceMap, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Map> {
    let f: &F = &*(f as *const F);
    f(&Map::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Map")
    }
}
