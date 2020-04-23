// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_10", feature = "dox"))]
use gio;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gio_sys;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::object::IsA;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::ToValue;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gobject_sys;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gtk;
use gtk_source_sys;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::pin::Pin;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::ptr;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use Buffer;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use SearchSettings;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use Style;

glib_wrapper! {
    pub struct SearchContext(Object<gtk_source_sys::GtkSourceSearchContext, gtk_source_sys::GtkSourceSearchContextClass, SearchContextClass>);

    match fn {
        get_type => || gtk_source_sys::gtk_source_search_context_get_type(),
    }
}

impl SearchContext {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new<P: IsA<Buffer>, Q: IsA<SearchSettings>>(
        buffer: &P,
        settings: Option<&Q>,
    ) -> SearchContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_search_context_new(
                buffer.as_ref().to_glib_none().0,
                settings.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct SearchContextBuilder {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    buffer: Option<Buffer>,
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    highlight: Option<bool>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    match_style: Option<Style>,
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    settings: Option<SearchSettings>,
}

impl SearchContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SearchContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v3_10", feature = "dox"))]
        {
            if let Some(ref buffer) = self.buffer {
                properties.push(("buffer", buffer));
            }
        }
        #[cfg(any(feature = "v3_10", feature = "dox"))]
        {
            if let Some(ref highlight) = self.highlight {
                properties.push(("highlight", highlight));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref match_style) = self.match_style {
                properties.push(("match-style", match_style));
            }
        }
        #[cfg(any(feature = "v3_10", feature = "dox"))]
        {
            if let Some(ref settings) = self.settings {
                properties.push(("settings", settings));
            }
        }
        glib::Object::new(SearchContext::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn buffer<P: IsA<Buffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = Some(highlight);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn match_style(mut self, match_style: &Style) -> Self {
        self.match_style = Some(match_style.clone());
        self
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn settings<P: IsA<SearchSettings>>(mut self, settings: &P) -> Self {
        self.settings = Some(settings.clone().upcast());
        self
    }
}

pub const NONE_SEARCH_CONTEXT: Option<&SearchContext> = None;

pub trait SearchContextExt: 'static {
    #[cfg_attr(feature = "v3_22", deprecated)]
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn backward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter), glib::Error>) + Send + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward_async_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(gtk::TextIter, gtk::TextIter), glib::Error>>
                + 'static,
        >,
    >;

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn backward_finish2(&self, result: /*Ignored*/&gio::AsyncResult) -> Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>;

    #[cfg_attr(feature = "v3_22", deprecated)]
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn forward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter), glib::Error>) + Send + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&P>,
        callback: Q,
    );

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward_async_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(gtk::TextIter, gtk::TextIter), glib::Error>>
                + 'static,
        >,
    >;

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn forward_finish2(&self, result: /*Ignored*/&gio::AsyncResult) -> Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_highlight(&self) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_match_style(&self) -> Option<Style>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrence_position(
        &self,
        match_start: &gtk::TextIter,
        match_end: &gtk::TextIter,
    ) -> i32;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrences_count(&self) -> i32;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_regex_error(&self) -> Option<glib::Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_settings(&self) -> Option<SearchSettings>;

    #[cfg_attr(feature = "v3_22", deprecated)]
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace(
        &self,
        match_start: &gtk::TextIter,
        match_end: &gtk::TextIter,
        replace: &str,
    ) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn replace2(
        &self,
        match_start: &mut gtk::TextIter,
        match_end: &mut gtk::TextIter,
        replace: &str,
    ) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace_all(&self, replace: &str) -> Result<u32, glib::Error>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_highlight(&self, highlight: bool);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_match_style(&self, match_style: Option<&Style>);

    #[cfg_attr(feature = "v3_24", deprecated)]
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_settings<P: IsA<SearchSettings>>(&self, settings: Option<&P>);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_highlight_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_match_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_occurrences_count_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_regex_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchContext>> SearchContextExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let ret = from_glib(gtk_source_sys::gtk_source_search_context_backward(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
            ));
            if ret {
                Some((match_start, match_end))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn backward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_source_sys::gtk_source_search_context_backward2(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
            ));
            let has_wrapped_around = has_wrapped_around.assume_init();
            if ret {
                Some((match_start, match_end, from_glib(has_wrapped_around)))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter), glib::Error>) + Send + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn backward_async_trampoline<
            Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let _ = gtk_source_sys::gtk_source_search_context_backward_finish(
                _source_object as *mut _,
                res,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((match_start, match_end))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = backward_async_trampoline::<Q>;
        unsafe {
            gtk_source_sys::gtk_source_search_context_backward_async(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn backward_async_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(gtk::TextIter, gtk::TextIter), glib::Error>>
                + 'static,
        >,
    > {
        let iter = iter.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.backward_async(&iter, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn backward_finish2(&self, result: /*Ignored*/&gio::AsyncResult) -> Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error> {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_search_context_backward_finish2() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let ret = from_glib(gtk_source_sys::gtk_source_search_context_forward(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
            ));
            if ret {
                Some((match_start, match_end))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn forward2(&self, iter: &gtk::TextIter) -> Option<(gtk::TextIter, gtk::TextIter, bool)> {
        unsafe {
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let mut has_wrapped_around = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_source_sys::gtk_source_search_context_forward2(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                has_wrapped_around.as_mut_ptr(),
            ));
            let has_wrapped_around = has_wrapped_around.assume_init();
            if ret {
                Some((match_start, match_end, from_glib(has_wrapped_around)))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter), glib::Error>) + Send + 'static,
    >(
        &self,
        iter: &gtk::TextIter,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn forward_async_trampoline<
            Q: FnOnce(Result<(gtk::TextIter, gtk::TextIter), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut match_start = gtk::TextIter::uninitialized();
            let mut match_end = gtk::TextIter::uninitialized();
            let _ = gtk_source_sys::gtk_source_search_context_forward_finish(
                _source_object as *mut _,
                res,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((match_start, match_end))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = forward_async_trampoline::<Q>;
        unsafe {
            gtk_source_sys::gtk_source_search_context_forward_async(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn forward_async_future(
        &self,
        iter: &gtk::TextIter,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(gtk::TextIter, gtk::TextIter), glib::Error>>
                + 'static,
        >,
    > {
        let iter = iter.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            obj.forward_async(&iter, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    //#[cfg(any(feature = "v3_22", feature = "dox"))]
    //fn forward_finish2(&self, result: /*Ignored*/&gio::AsyncResult) -> Result<(gtk::TextIter, gtk::TextIter, bool), glib::Error> {
    //    unsafe { TODO: call gtk_source_sys:gtk_source_search_context_forward_finish2() }
    //}

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_search_context_get_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_highlight(&self) -> bool {
        unsafe {
            from_glib(gtk_source_sys::gtk_source_search_context_get_highlight(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_match_style(&self) -> Option<Style> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_search_context_get_match_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrence_position(
        &self,
        match_start: &gtk::TextIter,
        match_end: &gtk::TextIter,
    ) -> i32 {
        unsafe {
            gtk_source_sys::gtk_source_search_context_get_occurrence_position(
                self.as_ref().to_glib_none().0,
                match_start.to_glib_none().0,
                match_end.to_glib_none().0,
            )
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_occurrences_count(&self) -> i32 {
        unsafe {
            gtk_source_sys::gtk_source_search_context_get_occurrences_count(
                self.as_ref().to_glib_none().0,
            )
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_regex_error(&self) -> Option<glib::Error> {
        unsafe {
            from_glib_full(gtk_source_sys::gtk_source_search_context_get_regex_error(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_settings(&self) -> Option<SearchSettings> {
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_search_context_get_settings(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace(
        &self,
        match_start: &gtk::TextIter,
        match_end: &gtk::TextIter,
        replace: &str,
    ) -> Result<(), glib::Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_source_sys::gtk_source_search_context_replace(
                self.as_ref().to_glib_none().0,
                match_start.to_glib_none().0,
                match_end.to_glib_none().0,
                replace.to_glib_none().0,
                replace_length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn replace2(
        &self,
        match_start: &mut gtk::TextIter,
        match_end: &mut gtk::TextIter,
        replace: &str,
    ) -> Result<(), glib::Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_source_sys::gtk_source_search_context_replace2(
                self.as_ref().to_glib_none().0,
                match_start.to_glib_none_mut().0,
                match_end.to_glib_none_mut().0,
                replace.to_glib_none().0,
                replace_length,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn replace_all(&self, replace: &str) -> Result<u32, glib::Error> {
        let replace_length = replace.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_source_sys::gtk_source_search_context_replace_all(
                self.as_ref().to_glib_none().0,
                replace.to_glib_none().0,
                replace_length,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_highlight(&self, highlight: bool) {
        unsafe {
            gtk_source_sys::gtk_source_search_context_set_highlight(
                self.as_ref().to_glib_none().0,
                highlight.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_match_style(&self, match_style: Option<&Style>) {
        unsafe {
            gtk_source_sys::gtk_source_search_context_set_match_style(
                self.as_ref().to_glib_none().0,
                match_style.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_settings<P: IsA<SearchSettings>>(&self, settings: Option<&P>) {
        unsafe {
            gtk_source_sys::gtk_source_search_context_set_settings(
                self.as_ref().to_glib_none().0,
                settings.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_highlight_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_highlight_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSearchContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SearchContext>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::highlight\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_highlight_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_match_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_match_style_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSearchContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SearchContext>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::match-style\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_match_style_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_occurrences_count_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_occurrences_count_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSearchContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SearchContext>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::occurrences-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_occurrences_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_regex_error_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_regex_error_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSearchContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SearchContext>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::regex-error\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_regex_error_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_property_settings_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_settings_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_source_sys::GtkSourceSearchContext,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SearchContext>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::settings\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_settings_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SearchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchContext")
    }
}
