#![allow(deprecated)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

// Re-export dependencies
pub use gio;
pub use glib;
pub use glib::object as gobject;

#[macro_use]
mod rt;

pub mod web_view_backend;
pub mod prelude;

pub use prelude::*;

#[macro_use]
mod auto;
pub use auto::*;
