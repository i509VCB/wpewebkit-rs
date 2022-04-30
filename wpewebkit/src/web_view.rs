// TODO: Gate to v2_20

use glib::translate::*;
use glib::IsA;

use crate::{Settings, UserContentManager, WebContext, WebView, WebViewBackend};

pub trait WebViewManual: 'static {
    #[doc(alias = "webkit_web_view_new")]
    fn new(backend: &WebViewBackend) -> WebView;

    #[doc(alias = "webkit_web_view_new_with_context")]
    #[doc(alias = "new_with_context")]
    fn with_context(backend: &WebViewBackend, context: &impl IsA<WebContext>) -> WebView;

    #[doc(alias = "webkit_web_view_new_with_related_view")]
    #[doc(alias = "new_with_related_view")]
    fn with_related_view(backend: &WebViewBackend, web_view: &impl IsA<WebView>) -> WebView;

    #[doc(alias = "webkit_web_view_new_with_settings")]
    #[doc(alias = "new_with_settings")]
    fn with_settings(backend: &WebViewBackend, settings: &impl IsA<Settings>) -> WebView;

    #[doc(alias = "webkit_web_view_new_with_user_content_manager")]
    #[doc(alias = "new_with_user_content_manager")]
    fn with_user_content_manager(
        backend: &WebViewBackend,
        user_content_manager: &impl IsA<UserContentManager>,
    ) -> WebView;
}

impl<O: IsA<WebView>> WebViewManual for O {
    fn new(backend: &WebViewBackend) -> WebView {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_web_view_new(backend.to_glib_full() as *mut _)) }
    }

    fn with_context(backend: &WebViewBackend, context: &impl IsA<WebContext>) -> WebView {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_web_view_new_with_context(
                backend.to_glib_full() as *mut _,
                context.as_ref().to_glib_none().0,
            ))
        }
    }

    fn with_related_view(backend: &WebViewBackend, web_view: &impl IsA<WebView>) -> WebView {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_web_view_new_with_related_view(
                backend.to_glib_full() as *mut _,
                web_view.as_ref().to_glib_none().0,
            ))
        }
    }

    fn with_settings(backend: &WebViewBackend, settings: &impl IsA<Settings>) -> WebView {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_web_view_new_with_settings(
                backend.to_glib_full() as *mut _,
                settings.as_ref().to_glib_none().0,
            ))
        }
    }

    fn with_user_content_manager(
        backend: &WebViewBackend,
        user_content_manager: &impl IsA<UserContentManager>,
    ) -> WebView {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::webkit_web_view_new_with_user_content_manager(
                backend.to_glib_full() as *mut _,
                user_content_manager.as_ref().to_glib_none().0,
            ))
        }
    }
}
