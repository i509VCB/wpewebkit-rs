// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

mod class;
pub use self::class::Class;

mod context;
pub use self::context::Context;

mod exception;
pub use self::exception::Exception;

mod value;
pub use self::value::Value;

mod virtual_machine;
pub use self::virtual_machine::VirtualMachine;

mod weak_value;
pub use self::weak_value::WeakValue;

mod enums;
pub use self::enums::CheckSyntaxMode;
pub use self::enums::CheckSyntaxResult;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::enums::OptionType;
#[cfg(any(feature = "v2_38", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
pub use self::enums::TypedArrayType;

mod flags;
pub use self::flags::ValuePropertyFlags;

pub mod functions;

mod constants;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::constants::OPTIONS_USE_DFG;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::constants::OPTIONS_USE_FTL;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::constants::OPTIONS_USE_JIT;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
pub use self::constants::OPTIONS_USE_LLINT;

#[doc(hidden)]
pub mod traits {
    pub use super::context::ContextExt;
    pub use super::exception::ExceptionExt;
    pub use super::value::ValueExt;
    pub use super::weak_value::WeakValueExt;
}
#[doc(hidden)]
pub mod builders {
    pub use super::context::ContextBuilder;
    pub use super::value::ValueBuilder;
    pub use super::weak_value::WeakValueBuilder;
}