// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::PermissionRequest;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitUserMediaPermissionRequest")]
    pub struct UserMediaPermissionRequest(Object<ffi::WebKitUserMediaPermissionRequest, ffi::WebKitUserMediaPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_user_media_permission_request_get_type(),
    }
}

impl UserMediaPermissionRequest {
    pub const NONE: Option<&'static UserMediaPermissionRequest> = None;
}

pub trait UserMediaPermissionRequestExt: 'static {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "is-for-audio-device")]
    fn is_for_audio_device(&self) -> bool;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "is-for-video-device")]
    fn is_for_video_device(&self) -> bool;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "is-for-audio-device")]
    fn connect_is_for_audio_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "is-for-video-device")]
    fn connect_is_for_video_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UserMediaPermissionRequest>> UserMediaPermissionRequestExt for O {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn is_for_audio_device(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-for-audio-device")
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn is_for_video_device(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-for-video-device")
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_is_for_audio_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_for_audio_device_trampoline<
            P: IsA<UserMediaPermissionRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitUserMediaPermissionRequest,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UserMediaPermissionRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-for-audio-device\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_for_audio_device_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn connect_is_for_video_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_for_video_device_trampoline<
            P: IsA<UserMediaPermissionRequest>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitUserMediaPermissionRequest,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UserMediaPermissionRequest::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-for-video-device\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_for_video_device_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for UserMediaPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UserMediaPermissionRequest")
    }
}
