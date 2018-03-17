// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct UndoManager(Object<ffi::GtkSourceUndoManager, ffi::GtkSourceUndoManagerIface>);

    match fn {
        get_type => || ffi::gtk_source_undo_manager_get_type(),
    }
}

pub trait UndoManagerExt {
    fn begin_not_undoable_action(&self);

    fn can_redo(&self) -> bool;

    fn can_redo_changed(&self);

    fn can_undo(&self) -> bool;

    fn can_undo_changed(&self);

    fn end_not_undoable_action(&self);

    fn redo(&self);

    fn undo(&self);

    fn connect_can_redo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_can_redo_changed(&self);

    fn connect_can_undo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_can_undo_changed(&self);
}

impl<O: IsA<UndoManager> + IsA<glib::object::Object> + glib::object::ObjectExt> UndoManagerExt for O {
    fn begin_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_begin_not_undoable_action(self.to_glib_none().0);
        }
    }

    fn can_redo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_undo_manager_can_redo(self.to_glib_none().0))
        }
    }

    fn can_redo_changed(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_can_redo_changed(self.to_glib_none().0);
        }
    }

    fn can_undo(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_undo_manager_can_undo(self.to_glib_none().0))
        }
    }

    fn can_undo_changed(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_can_undo_changed(self.to_glib_none().0);
        }
    }

    fn end_not_undoable_action(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_end_not_undoable_action(self.to_glib_none().0);
        }
    }

    fn redo(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_redo(self.to_glib_none().0);
        }
    }

    fn undo(&self) {
        unsafe {
            ffi::gtk_source_undo_manager_undo(self.to_glib_none().0);
        }
    }

    fn connect_can_redo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "can-redo-changed",
                transmute(can_redo_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_can_redo_changed(&self) {
        let _ = self.emit("can-redo-changed", &[]).unwrap();
    }

    fn connect_can_undo_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "can-undo-changed",
                transmute(can_undo_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_can_undo_changed(&self) {
        let _ = self.emit("can-undo-changed", &[]).unwrap();
    }
}

unsafe extern "C" fn can_redo_changed_trampoline<P>(this: *mut ffi::GtkSourceUndoManager, f: glib_ffi::gpointer)
where P: IsA<UndoManager> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&UndoManager::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn can_undo_changed_trampoline<P>(this: *mut ffi::GtkSourceUndoManager, f: glib_ffi::gpointer)
where P: IsA<UndoManager> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&UndoManager::from_glib_borrow(this).downcast_unchecked())
}
