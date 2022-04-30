// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitFileChooserRequest")]
    pub struct FileChooserRequest(Object<ffi::WebKitFileChooserRequest, ffi::WebKitFileChooserRequestClass>);

    match fn {
        type_ => || ffi::webkit_file_chooser_request_get_type(),
    }
}

impl FileChooserRequest {
    pub const NONE: Option<&'static FileChooserRequest> = None;
}

pub trait FileChooserRequestExt: 'static {
    #[doc(alias = "webkit_file_chooser_request_cancel")]
    fn cancel(&self);

    #[doc(alias = "webkit_file_chooser_request_get_mime_types")]
    #[doc(alias = "get_mime_types")]
    fn mime_types(&self) -> Vec<glib::GString>;

    #[doc(alias = "webkit_file_chooser_request_get_select_multiple")]
    #[doc(alias = "get_select_multiple")]
    fn selects_multiple(&self) -> bool;

    #[doc(alias = "webkit_file_chooser_request_get_selected_files")]
    #[doc(alias = "get_selected_files")]
    fn selected_files(&self) -> Vec<glib::GString>;

    #[doc(alias = "webkit_file_chooser_request_select_files")]
    fn select_files(&self, files: &[&str]);

    #[doc(alias = "mime-types")]
    fn connect_mime_types_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "select-multiple")]
    fn connect_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "selected-files")]
    fn connect_selected_files_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserRequest>> FileChooserRequestExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::webkit_file_chooser_request_cancel(self.as_ref().to_glib_none().0);
        }
    }

    fn mime_types(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_mime_types(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn selects_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_file_chooser_request_get_select_multiple(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn selected_files(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::webkit_file_chooser_request_get_selected_files(self.as_ref().to_glib_none().0),
            )
        }
    }

    fn select_files(&self, files: &[&str]) {
        unsafe {
            ffi::webkit_file_chooser_request_select_files(
                self.as_ref().to_glib_none().0,
                files.to_glib_none().0,
            );
        }
    }

    fn connect_mime_types_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mime_types_trampoline<
            P: IsA<FileChooserRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitFileChooserRequest,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mime-types\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mime_types_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_select_multiple_trampoline<
            P: IsA<FileChooserRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitFileChooserRequest,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::select-multiple\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_select_multiple_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_selected_files_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selected_files_trampoline<
            P: IsA<FileChooserRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitFileChooserRequest,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FileChooserRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selected-files\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_selected_files_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileChooserRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileChooserRequest")
    }
}
