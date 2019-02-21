// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::object::IsA;
use glib::translate::*;
use gtk;

use CompletionInfo;

impl CompletionInfo {
    pub fn move_to_iter<'a, P: IsA<gtk::TextView>, Q: Into<Option<&'a mut gtk::TextIter>>>(&self, view: &P, iter: Q) {
        let mut iter = iter.into();
        let iter = iter.to_glib_none_mut();
        unsafe {
            ffi::gtk_source_completion_info_move_to_iter(self.to_glib_none().0, view.as_ref().to_glib_none().0, iter.0);
        }
    }
}
