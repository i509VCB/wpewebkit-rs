#![cfg(any(feature = "v2_20", feature = "dox"))]
#![cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]

use std::ffi::CStr;

use glib::translate::from_glib_full;
use wpe::{vtable, ViewBackend};

use crate::WebViewBackend;

pub trait WebViewBackendManual: 'static {
    // TODO: Constructor with higher level parameters
    fn new<T: ViewBackend>(backend: T) -> Self;
}

impl WebViewBackendManual for WebViewBackend {
    fn new<T: ViewBackend>(backend: T) -> Self {
        // FIXME: Memory leak of interface, this should be destroyed in notify
        let interface = Box::into_raw(Box::new(vtable::backend_interface_with_type::<T>()));
        println!("{:p}", interface);
        let backend = Box::into_raw(Box::new(backend));

        // TODO: Properly configure this
        unsafe {
            let name = CStr::from_bytes_with_nul_unchecked(b"libWPEBackend-fdo-1.0.so\0");
            // /usr/lib/libWPEBackend-fdo-1.0.so
            wpe_sys::wpe_loader_init(name.as_ptr());
        }

        unsafe extern "C" fn destroy_interface(ptr: glib_sys::gpointer) {
            println!("{:p}", ptr);
            let _ = Box::from_raw(ptr);
        }

        unsafe {
            let view_backend = wpe_sys::wpe_view_backend_create_with_backend_interface(
                interface,
                backend as *mut _,
            );
            from_glib_full(ffi::webkit_web_view_backend_new(
                view_backend,
                Some(destroy_interface),
                interface as *mut _,
            ))
        }
    }
}
