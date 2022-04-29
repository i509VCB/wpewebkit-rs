// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::PolicyDecision;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitResponsePolicyDecision")]
    pub struct ResponsePolicyDecision(Object<ffi::WebKitResponsePolicyDecision, ffi::WebKitResponsePolicyDecisionClass>) @extends PolicyDecision;

    match fn {
        type_ => || ffi::webkit_response_policy_decision_get_type(),
    }
}

impl ResponsePolicyDecision {
        pub const NONE: Option<&'static ResponsePolicyDecision> = None;
    
}

pub trait ResponsePolicyDecisionExt: 'static {
    //#[doc(alias = "webkit_response_policy_decision_get_request")]
    //#[doc(alias = "get_request")]
    //fn request(&self) -> /*Ignored*/Option<URIRequest>;

    //#[doc(alias = "webkit_response_policy_decision_get_response")]
    //#[doc(alias = "get_response")]
    //fn response(&self) -> /*Ignored*/Option<URIResponse>;

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    #[doc(alias = "webkit_response_policy_decision_is_mime_type_supported")]
    fn is_mime_type_supported(&self) -> bool;

    #[doc(alias = "request")]
    fn connect_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "response")]
    fn connect_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ResponsePolicyDecision>> ResponsePolicyDecisionExt for O {
    //fn request(&self) -> /*Ignored*/Option<URIRequest> {
    //    unsafe { TODO: call ffi:webkit_response_policy_decision_get_request() }
    //}

    //fn response(&self) -> /*Ignored*/Option<URIResponse> {
    //    unsafe { TODO: call ffi:webkit_response_policy_decision_get_response() }
    //}

    #[cfg(any(feature = "v2_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_4")))]
    fn is_mime_type_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_response_policy_decision_is_mime_type_supported(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_request_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_request_trampoline<P: IsA<ResponsePolicyDecision>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitResponsePolicyDecision, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ResponsePolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::request\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_request_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_response_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_response_trampoline<P: IsA<ResponsePolicyDecision>, F: Fn(&P) + 'static>(this: *mut ffi::WebKitResponsePolicyDecision, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ResponsePolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::response\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_response_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ResponsePolicyDecision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ResponsePolicyDecision")
    }
}