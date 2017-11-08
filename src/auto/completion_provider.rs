// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use CompletionActivation;
use CompletionContext;
use CompletionInfo;
use CompletionProposal;
use ffi;
use gdk_pixbuf;
#[cfg(any(feature = "v3_18", feature = "dox"))]
use gio;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct CompletionProvider(Object<ffi::GtkSourceCompletionProvider, ffi::GtkSourceCompletionProviderIface>);

    match fn {
        get_type => || ffi::gtk_source_completion_provider_get_type(),
    }
}

pub trait CompletionProviderExt {
    fn activate_proposal<P: IsA<CompletionProposal>>(&self, proposal: &P, iter: &mut gtk::TextIter) -> bool;

    fn get_activation(&self) -> CompletionActivation;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_gicon(&self) -> Option<gio::Icon>;

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_icon_name(&self) -> Option<String>;

    fn get_info_widget<P: IsA<CompletionProposal>>(&self, proposal: &P) -> Option<gtk::Widget>;

    fn get_interactive_delay(&self) -> i32;

    fn get_name(&self) -> Option<String>;

    fn get_priority(&self) -> i32;

    fn get_start_iter<P: IsA<CompletionProposal>>(&self, context: &CompletionContext, proposal: &P) -> Option<gtk::TextIter>;

    fn match_(&self, context: &CompletionContext) -> bool;

    fn populate(&self, context: &CompletionContext);

    fn update_info<P: IsA<CompletionProposal>>(&self, proposal: &P, info: &CompletionInfo);
}

impl<O: IsA<CompletionProvider>> CompletionProviderExt for O {
    fn activate_proposal<P: IsA<CompletionProposal>>(&self, proposal: &P, iter: &mut gtk::TextIter) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_provider_activate_proposal(self.to_glib_none().0, proposal.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    fn get_activation(&self) -> CompletionActivation {
        unsafe {
            from_glib(ffi::gtk_source_completion_provider_get_activation(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_provider_get_gicon(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_provider_get_icon(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_provider_get_icon_name(self.to_glib_none().0))
        }
    }

    fn get_info_widget<P: IsA<CompletionProposal>>(&self, proposal: &P) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::gtk_source_completion_provider_get_info_widget(self.to_glib_none().0, proposal.to_glib_none().0))
        }
    }

    fn get_interactive_delay(&self) -> i32 {
        unsafe {
            ffi::gtk_source_completion_provider_get_interactive_delay(self.to_glib_none().0)
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_source_completion_provider_get_name(self.to_glib_none().0))
        }
    }

    fn get_priority(&self) -> i32 {
        unsafe {
            ffi::gtk_source_completion_provider_get_priority(self.to_glib_none().0)
        }
    }

    fn get_start_iter<P: IsA<CompletionProposal>>(&self, context: &CompletionContext, proposal: &P) -> Option<gtk::TextIter> {
        unsafe {
            let mut iter = gtk::TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_source_completion_provider_get_start_iter(self.to_glib_none().0, context.to_glib_none().0, proposal.to_glib_none().0, iter.to_glib_none_mut().0));
            if ret { Some(iter) } else { None }
        }
    }

    fn match_(&self, context: &CompletionContext) -> bool {
        unsafe {
            from_glib(ffi::gtk_source_completion_provider_match(self.to_glib_none().0, context.to_glib_none().0))
        }
    }

    fn populate(&self, context: &CompletionContext) {
        unsafe {
            ffi::gtk_source_completion_provider_populate(self.to_glib_none().0, context.to_glib_none().0);
        }
    }

    fn update_info<P: IsA<CompletionProposal>>(&self, proposal: &P, info: &CompletionInfo) {
        unsafe {
            ffi::gtk_source_completion_provider_update_info(self.to_glib_none().0, proposal.to_glib_none().0, info.to_glib_none().0);
        }
    }
}
