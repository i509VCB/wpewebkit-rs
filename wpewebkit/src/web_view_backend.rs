#![cfg(any(feature = "v2_20", feature = "dox"))]
#![cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]

use crate::WebViewBackend;

pub trait WebViewBackendManual: 'static {
    // TODO: Constructor with higher level parameters
}

impl WebViewBackendManual for WebViewBackend {}
