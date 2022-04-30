// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct JavascriptResult(Shared<ffi::WebKitJavascriptResult>);

    match fn {
        ref => |ptr| ffi::webkit_javascript_result_ref(ptr),
        unref => |ptr| ffi::webkit_javascript_result_unref(ptr),
        type_ => || ffi::webkit_javascript_result_get_type(),
    }
}

impl JavascriptResult {
    #[cfg(any(feature = "v2_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_22")))]
    #[doc(alias = "webkit_javascript_result_get_js_value")]
    #[doc(alias = "get_js_value")]
    pub fn js_value(&self) -> Option<wpe_java_script_core::Value> {
        unsafe {
            from_glib_none(ffi::webkit_javascript_result_get_js_value(
                self.to_glib_none().0,
            ))
        }
    }
}
