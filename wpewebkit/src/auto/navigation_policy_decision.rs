// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
use crate::NavigationAction;
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
    #[doc(alias = "WebKitNavigationPolicyDecision")]
    pub struct NavigationPolicyDecision(Object<ffi::WebKitNavigationPolicyDecision, ffi::WebKitNavigationPolicyDecisionClass>) @extends PolicyDecision;

    match fn {
        type_ => || ffi::webkit_navigation_policy_decision_get_type(),
    }
}

impl NavigationPolicyDecision {
    pub const NONE: Option<&'static NavigationPolicyDecision> = None;
}

pub trait NavigationPolicyDecisionExt: 'static {
    #[doc(alias = "webkit_navigation_policy_decision_get_frame_name")]
    #[doc(alias = "get_frame_name")]
    fn frame_name(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    #[doc(alias = "webkit_navigation_policy_decision_get_navigation_action")]
    #[doc(alias = "get_navigation_action")]
    fn navigation_action(&self) -> Option<NavigationAction>;

    #[doc(alias = "frame-name")]
    fn connect_frame_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    #[doc(alias = "navigation-action")]
    fn connect_navigation_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NavigationPolicyDecision>> NavigationPolicyDecisionExt for O {
    fn frame_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_policy_decision_get_frame_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    fn navigation_action(&self) -> Option<NavigationAction> {
        unsafe {
            from_glib_none(
                ffi::webkit_navigation_policy_decision_get_navigation_action(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn connect_frame_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_frame_name_trampoline<
            P: IsA<NavigationPolicyDecision>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitNavigationPolicyDecision,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::frame-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_frame_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_6")))]
    fn connect_navigation_action_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_navigation_action_trampoline<
            P: IsA<NavigationPolicyDecision>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitNavigationPolicyDecision,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(NavigationPolicyDecision::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::navigation-action\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_navigation_action_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for NavigationPolicyDecision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("NavigationPolicyDecision")
    }
}
