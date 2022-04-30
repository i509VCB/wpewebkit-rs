#![allow(deprecated)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

// Re-export dependencies
pub use gio;
pub use glib;
pub use glib::object as gobject;

#[macro_use]
mod rt;

pub mod color;
pub mod web_view_backend;
pub mod prelude;

pub use prelude::*;

#[macro_use]
#[allow(unused_imports)]
mod auto;
pub use auto::*;

pub use auto::functions::*;
