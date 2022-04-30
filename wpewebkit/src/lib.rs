#![allow(deprecated)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

// Re-export dependencies
pub use gio;
pub use glib;
pub use glib::object as gobject;
// TODO: Reexport as javascriptcore
pub use wpe_java_script_core;

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

// TODO: Not ideal to do manually?
pub use auto::functions::*;
