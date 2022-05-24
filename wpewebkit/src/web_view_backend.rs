#![cfg(any(feature = "v2_20", feature = "dox"))]
#![cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]

use std::ffi::CStr;

use glib::translate::from_glib_full;
use wpe::{vtable, ViewBackend};

use crate::WebViewBackend;

impl WebViewBackend {
    pub fn new<T: ViewBackend>(backend: T) -> Self {
        unsafe extern "C" fn destroy_interface(ptr: glib_sys::gpointer) {
            println!("{:p}", ptr);
            let _ = Box::from_raw(ptr);
        }

        // This is a horrifying hack to make things even attempt to work, proper bindings probably needed.
        extern "C" {
            fn wpe_fdo_initialize_shm() -> bool;
        }

        // FIXME: Memory leak of interface, this should be destroyed in notify
        let interface = Box::into_raw(Box::new(vtable::backend_interface_with_type::<T>()));
        println!("{:p}", interface);
        let backend = Box::into_raw(Box::new(backend));

        // TODO: Properly configure this
        unsafe {
            let name = CStr::from_bytes_with_nul_unchecked(b"libWPEBackend-fdo-1.0.so\0");
            // // /usr/lib/libWPEBackend-fdo-1.0.so
            if !wpe_sys::wpe_loader_init(name.as_ptr()) {
                panic!("Failed to init loader");
            }
            // FIXME: Temporary hack
            wpe_fdo_initialize_shm();

            let view_backend = wpe_sys::wpe_view_backend_create_with_backend_interface(
                interface,
                backend as *mut _,
            );
            assert!(!view_backend.is_null(), "failed to create view backend");
            wpe_sys::wpe_view_backend_initialize(view_backend);
            from_glib_full(ffi::webkit_web_view_backend_new(
                view_backend,
                Some(destroy_interface),
                interface as *mut _,
                //view_backend as *mut _
            ))
        }
    }
}
