// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use crate::AuthenticationScheme;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use crate::Credential;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use crate::SecurityOrigin;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use glib::translate::*;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitAuthenticationRequest")]
    pub struct AuthenticationRequest(Object<ffi::WebKitAuthenticationRequest, ffi::WebKitAuthenticationRequestClass>);

    match fn {
        type_ => || ffi::webkit_authentication_request_get_type(),
    }
}

impl AuthenticationRequest {
    pub const NONE: Option<&'static AuthenticationRequest> = None;
}

pub trait AuthenticationRequestExt: 'static {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_can_save_credentials")]
    fn can_save_credentials(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_cancel")]
    fn cancel(&self);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    #[doc(alias = "webkit_authentication_request_get_certificate_pin_flags")]
    #[doc(alias = "get_certificate_pin_flags")]
    fn certificate_pin_flags(&self) -> gio::TlsPasswordFlags;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_get_host")]
    #[doc(alias = "get_host")]
    fn host(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_get_port")]
    #[doc(alias = "get_port")]
    fn port(&self) -> u32;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_get_proposed_credential")]
    #[doc(alias = "get_proposed_credential")]
    fn proposed_credential(&self) -> Option<Credential>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_get_realm")]
    #[doc(alias = "get_realm")]
    fn realm(&self) -> Option<glib::GString>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_get_scheme")]
    #[doc(alias = "get_scheme")]
    fn scheme(&self) -> AuthenticationScheme;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_authentication_request_get_security_origin")]
    #[doc(alias = "get_security_origin")]
    fn security_origin(&self) -> Option<SecurityOrigin>;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_is_for_proxy")]
    fn is_for_proxy(&self) -> bool;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_authentication_request_is_retry")]
    fn is_retry(&self) -> bool;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_authentication_request_set_can_save_credentials")]
    fn set_can_save_credentials(&self, enabled: bool);

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_authentication_request_set_proposed_credential")]
    fn set_proposed_credential(&self, credential: &mut Credential);

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "authenticated")]
    fn connect_authenticated<F: Fn(&Self, &Credential) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "cancelled")]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AuthenticationRequest>> AuthenticationRequestExt for O {
    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn can_save_credentials(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_can_save_credentials(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn cancel(&self) {
        unsafe {
            ffi::webkit_authentication_request_cancel(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    fn certificate_pin_flags(&self) -> gio::TlsPasswordFlags {
        unsafe {
            from_glib(
                ffi::webkit_authentication_request_get_certificate_pin_flags(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn host(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_host(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn port(&self) -> u32 {
        unsafe { ffi::webkit_authentication_request_get_port(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn proposed_credential(&self) -> Option<Credential> {
        unsafe {
            from_glib_full(ffi::webkit_authentication_request_get_proposed_credential(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn realm(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_authentication_request_get_realm(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn scheme(&self) -> AuthenticationScheme {
        unsafe {
            from_glib(ffi::webkit_authentication_request_get_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn security_origin(&self) -> Option<SecurityOrigin> {
        unsafe {
            from_glib_full(ffi::webkit_authentication_request_get_security_origin(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn is_for_proxy(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_for_proxy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn is_retry(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_authentication_request_is_retry(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn set_can_save_credentials(&self, enabled: bool) {
        unsafe {
            ffi::webkit_authentication_request_set_can_save_credentials(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn set_proposed_credential(&self, credential: &mut Credential) {
        unsafe {
            ffi::webkit_authentication_request_set_proposed_credential(
                self.as_ref().to_glib_none().0,
                credential.to_glib_none_mut().0,
            );
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn connect_authenticated<F: Fn(&Self, &Credential) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn authenticated_trampoline<
            P: IsA<AuthenticationRequest>,
            F: Fn(&P, &Credential) + 'static,
        >(
            this: *mut ffi::WebKitAuthenticationRequest,
            credential: *mut ffi::WebKitCredential,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AuthenticationRequest::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(credential),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"authenticated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    authenticated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<
            P: IsA<AuthenticationRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitAuthenticationRequest,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AuthenticationRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancelled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AuthenticationRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AuthenticationRequest")
    }
}
