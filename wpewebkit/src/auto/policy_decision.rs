// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitPolicyDecision")]
    pub struct PolicyDecision(Object<ffi::WebKitPolicyDecision, ffi::WebKitPolicyDecisionClass>);

    match fn {
        type_ => || ffi::webkit_policy_decision_get_type(),
    }
}

impl PolicyDecision {
        pub const NONE: Option<&'static PolicyDecision> = None;
    
}

pub trait PolicyDecisionExt: 'static {
    #[doc(alias = "webkit_policy_decision_download")]
    fn download(&self);

    #[doc(alias = "webkit_policy_decision_ignore")]
    fn ignore(&self);

    #[doc(alias = "webkit_policy_decision_use")]
    #[doc(alias = "use")]
    fn use_(&self);

    //#[cfg(any(feature = "v2_30", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    //#[doc(alias = "webkit_policy_decision_use_with_policies")]
    //fn use_with_policies(&self, policies: /*Ignored*/&WebsitePolicies);
}

impl<O: IsA<PolicyDecision>> PolicyDecisionExt for O {
    fn download(&self) {
        unsafe {
            ffi::webkit_policy_decision_download(self.as_ref().to_glib_none().0);
        }
    }

    fn ignore(&self) {
        unsafe {
            ffi::webkit_policy_decision_ignore(self.as_ref().to_glib_none().0);
        }
    }

    fn use_(&self) {
        unsafe {
            ffi::webkit_policy_decision_use(self.as_ref().to_glib_none().0);
        }
    }

    //#[cfg(any(feature = "v2_30", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    //fn use_with_policies(&self, policies: /*Ignored*/&WebsitePolicies) {
    //    unsafe { TODO: call ffi:webkit_policy_decision_use_with_policies() }
    //}
}

impl fmt::Display for PolicyDecision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PolicyDecision")
    }
}
