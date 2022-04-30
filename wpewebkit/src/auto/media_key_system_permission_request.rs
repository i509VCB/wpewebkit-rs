// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use crate::PermissionRequest;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitMediaKeySystemPermissionRequest")]
    pub struct MediaKeySystemPermissionRequest(Object<ffi::WebKitMediaKeySystemPermissionRequest, ffi::WebKitMediaKeySystemPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_media_key_system_permission_request_get_type(),
    }
}

impl MediaKeySystemPermissionRequest {
        pub const NONE: Option<&'static MediaKeySystemPermissionRequest> = None;
    
}

impl fmt::Display for MediaKeySystemPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MediaKeySystemPermissionRequest")
    }
}