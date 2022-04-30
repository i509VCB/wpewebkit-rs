// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::PermissionRequest;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitGeolocationPermissionRequest")]
    pub struct GeolocationPermissionRequest(Object<ffi::WebKitGeolocationPermissionRequest, ffi::WebKitGeolocationPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_geolocation_permission_request_get_type(),
    }
}

impl GeolocationPermissionRequest {
    pub const NONE: Option<&'static GeolocationPermissionRequest> = None;
}

impl fmt::Display for GeolocationPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GeolocationPermissionRequest")
    }
}
