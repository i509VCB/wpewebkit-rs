// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitEditorState")]
    pub struct EditorState(Object<ffi::WebKitEditorState, ffi::WebKitEditorStateClass>);

    match fn {
        type_ => || ffi::webkit_editor_state_get_type(),
    }
}

impl EditorState {
    pub const NONE: Option<&'static EditorState> = None;
}

pub trait EditorStateExt: 'static {
    #[doc(alias = "webkit_editor_state_get_typing_attributes")]
    #[doc(alias = "get_typing_attributes")]
    fn typing_attributes(&self) -> u32;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    #[doc(alias = "webkit_editor_state_is_copy_available")]
    fn is_copy_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    #[doc(alias = "webkit_editor_state_is_cut_available")]
    fn is_cut_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    #[doc(alias = "webkit_editor_state_is_paste_available")]
    fn is_paste_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    #[doc(alias = "webkit_editor_state_is_redo_available")]
    fn is_redo_available(&self) -> bool;

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    #[doc(alias = "webkit_editor_state_is_undo_available")]
    fn is_undo_available(&self) -> bool;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    #[doc(alias = "typing-attributes")]
    fn connect_typing_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EditorState>> EditorStateExt for O {
    fn typing_attributes(&self) -> u32 {
        unsafe { ffi::webkit_editor_state_get_typing_attributes(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    fn is_copy_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_copy_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    fn is_cut_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_cut_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    fn is_paste_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_paste_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    fn is_redo_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_redo_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
    fn is_undo_available(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_editor_state_is_undo_available(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_10")))]
    fn connect_typing_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_typing_attributes_trampoline<
            P: IsA<EditorState>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitEditorState,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(EditorState::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::typing-attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_typing_attributes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EditorState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EditorState")
    }
}
