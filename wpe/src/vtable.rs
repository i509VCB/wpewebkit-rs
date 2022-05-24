#![forbid(unsafe_op_in_unsafe_fn)]

use std::{ffi::c_void, os::unix::prelude::RawFd, ptr};

use crate::ViewBackend;

pub fn backend_interface_with_type<T: ViewBackend>() -> wpe_sys::wpe_view_backend_interface {
    wpe_sys::wpe_view_backend_interface {
        create: Some(create::<T>),
        destroy: Some(destroy::<T>),
        initialize: Some(initialize::<T>),
        get_renderer_host_fd: Some(get_renderer_host_fd::<T>),
        _wpe_reserved0: None,
        _wpe_reserved1: None,
        _wpe_reserved2: None,
        _wpe_reserved3: None,
    }
}

unsafe extern "C" fn create<T: ViewBackend>(
    user_data: *mut c_void,
    backend: *mut wpe_sys::wpe_view_backend,
) -> *mut c_void {
    println!("backend create {:?}, {:?}", user_data, backend);
    // User data is already initialized
    ptr::null_mut()
}

unsafe extern "C" fn initialize<T: ViewBackend>(user_data: *mut c_void) {
    // User data is already initialized
    println!("backend init {:?}", user_data);
}

unsafe extern "C" fn destroy<T: ViewBackend>(user_data: *mut c_void) {
    // SAFETY: The T was created using Box::into_raw.
    println!("backend destroy");
    let _ = unsafe { Box::from_raw(user_data as *mut T) };
}

unsafe extern "C" fn get_renderer_host_fd<T: ViewBackend>(_user_data: *mut c_void) -> RawFd {
    // Not supported
    println!("backend renderer host");
    -1
}
