// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ..
// from ../gir-files
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type JSCCheckSyntaxMode = c_int;
pub const JSC_CHECK_SYNTAX_MODE_SCRIPT: JSCCheckSyntaxMode = 0;
pub const JSC_CHECK_SYNTAX_MODE_MODULE: JSCCheckSyntaxMode = 1;

pub type JSCCheckSyntaxResult = c_int;
pub const JSC_CHECK_SYNTAX_RESULT_SUCCESS: JSCCheckSyntaxResult = 0;
pub const JSC_CHECK_SYNTAX_RESULT_RECOVERABLE_ERROR: JSCCheckSyntaxResult = 1;
pub const JSC_CHECK_SYNTAX_RESULT_IRRECOVERABLE_ERROR: JSCCheckSyntaxResult = 2;
pub const JSC_CHECK_SYNTAX_RESULT_UNTERMINATED_LITERAL_ERROR: JSCCheckSyntaxResult = 3;
pub const JSC_CHECK_SYNTAX_RESULT_OUT_OF_MEMORY_ERROR: JSCCheckSyntaxResult = 4;
pub const JSC_CHECK_SYNTAX_RESULT_STACK_OVERFLOW_ERROR: JSCCheckSyntaxResult = 5;

pub type JSCOptionType = c_int;
pub const JSC_OPTION_BOOLEAN: JSCOptionType = 0;
pub const JSC_OPTION_INT: JSCOptionType = 1;
pub const JSC_OPTION_UINT: JSCOptionType = 2;
pub const JSC_OPTION_SIZE: JSCOptionType = 3;
pub const JSC_OPTION_DOUBLE: JSCOptionType = 4;
pub const JSC_OPTION_STRING: JSCOptionType = 5;
pub const JSC_OPTION_RANGE_STRING: JSCOptionType = 6;

pub type JSCTypedArrayType = c_int;
pub const JSC_TYPED_ARRAY_NONE: JSCTypedArrayType = 0;
pub const JSC_TYPED_ARRAY_INT8: JSCTypedArrayType = 1;
pub const JSC_TYPED_ARRAY_INT16: JSCTypedArrayType = 2;
pub const JSC_TYPED_ARRAY_INT32: JSCTypedArrayType = 3;
pub const JSC_TYPED_ARRAY_INT64: JSCTypedArrayType = 4;
pub const JSC_TYPED_ARRAY_UINT8: JSCTypedArrayType = 5;
pub const JSC_TYPED_ARRAY_UINT8_CLAMPED: JSCTypedArrayType = 6;
pub const JSC_TYPED_ARRAY_UINT16: JSCTypedArrayType = 7;
pub const JSC_TYPED_ARRAY_UINT32: JSCTypedArrayType = 8;
pub const JSC_TYPED_ARRAY_UINT64: JSCTypedArrayType = 9;
pub const JSC_TYPED_ARRAY_FLOAT32: JSCTypedArrayType = 10;
pub const JSC_TYPED_ARRAY_FLOAT64: JSCTypedArrayType = 11;

// Constants
pub const JSC_MAJOR_VERSION: c_int = 2;
pub const JSC_MICRO_VERSION: c_int = 1;
pub const JSC_MINOR_VERSION: c_int = 35;
pub const JSC_OPTIONS_USE_DFG: *const c_char = b"useDFGJIT\0" as *const u8 as *const c_char;
pub const JSC_OPTIONS_USE_FTL: *const c_char = b"useFTLJIT\0" as *const u8 as *const c_char;
pub const JSC_OPTIONS_USE_JIT: *const c_char = b"useJIT\0" as *const u8 as *const c_char;
pub const JSC_OPTIONS_USE_LLINT: *const c_char = b"useLLInt\0" as *const u8 as *const c_char;

// Flags
pub type JSCValuePropertyFlags = c_uint;
pub const JSC_VALUE_PROPERTY_CONFIGURABLE: JSCValuePropertyFlags = 1;
pub const JSC_VALUE_PROPERTY_ENUMERABLE: JSCValuePropertyFlags = 2;
pub const JSC_VALUE_PROPERTY_WRITABLE: JSCValuePropertyFlags = 4;

// Callbacks
pub type JSCClassDeletePropertyFunction = Option<
    unsafe extern "C" fn(*mut JSCClass, *mut JSCContext, gpointer, *const c_char) -> gboolean,
>;
pub type JSCClassEnumeratePropertiesFunction =
    Option<unsafe extern "C" fn(*mut JSCClass, *mut JSCContext, gpointer) -> *mut *mut c_char>;
pub type JSCClassGetPropertyFunction = Option<
    unsafe extern "C" fn(*mut JSCClass, *mut JSCContext, gpointer, *const c_char) -> *mut JSCValue,
>;
pub type JSCClassHasPropertyFunction = Option<
    unsafe extern "C" fn(*mut JSCClass, *mut JSCContext, gpointer, *const c_char) -> gboolean,
>;
pub type JSCClassSetPropertyFunction = Option<
    unsafe extern "C" fn(
        *mut JSCClass,
        *mut JSCContext,
        gpointer,
        *const c_char,
        *mut JSCValue,
    ) -> gboolean,
>;
pub type JSCExceptionHandler =
    Option<unsafe extern "C" fn(*mut JSCContext, *mut JSCException, gpointer)>;
pub type JSCOptionsFunc =
    Option<unsafe extern "C" fn(*const c_char, JSCOptionType, *const c_char, gpointer) -> gboolean>;

// Records
#[repr(C)]
pub struct _JSCClassClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type JSCClassClass = *mut _JSCClassClass;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCClassVTable {
    pub get_property: JSCClassGetPropertyFunction,
    pub set_property: JSCClassSetPropertyFunction,
    pub has_property: JSCClassHasPropertyFunction,
    pub delete_property: JSCClassDeletePropertyFunction,
    pub enumerate_properties: JSCClassEnumeratePropertiesFunction,
    pub _jsc_reserved0: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved1: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved2: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved3: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for JSCClassVTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCClassVTable @ {:p}", self))
            .field("get_property", &self.get_property)
            .field("set_property", &self.set_property)
            .field("has_property", &self.has_property)
            .field("delete_property", &self.delete_property)
            .field("enumerate_properties", &self.enumerate_properties)
            .field("_jsc_reserved0", &self._jsc_reserved0)
            .field("_jsc_reserved1", &self._jsc_reserved1)
            .field("_jsc_reserved2", &self._jsc_reserved2)
            .field("_jsc_reserved3", &self._jsc_reserved3)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCContextClass {
    pub parent_class: gobject::GObjectClass,
    pub _jsc_reserved0: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved1: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved2: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved3: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for JSCContextClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCContextClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("_jsc_reserved0", &self._jsc_reserved0)
            .field("_jsc_reserved1", &self._jsc_reserved1)
            .field("_jsc_reserved2", &self._jsc_reserved2)
            .field("_jsc_reserved3", &self._jsc_reserved3)
            .finish()
    }
}

#[repr(C)]
pub struct _JSCContextPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type JSCContextPrivate = *mut _JSCContextPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCExceptionClass {
    pub parent_class: gobject::GObjectClass,
    pub _jsc_reserved0: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved1: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved2: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved3: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for JSCExceptionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCExceptionClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("_jsc_reserved0", &self._jsc_reserved0)
            .field("_jsc_reserved1", &self._jsc_reserved1)
            .field("_jsc_reserved2", &self._jsc_reserved2)
            .field("_jsc_reserved3", &self._jsc_reserved3)
            .finish()
    }
}

#[repr(C)]
pub struct _JSCExceptionPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type JSCExceptionPrivate = *mut _JSCExceptionPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCValueClass {
    pub parent_class: gobject::GObjectClass,
    pub _jsc_reserved0: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved1: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved2: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved3: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for JSCValueClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCValueClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("_jsc_reserved0", &self._jsc_reserved0)
            .field("_jsc_reserved1", &self._jsc_reserved1)
            .field("_jsc_reserved2", &self._jsc_reserved2)
            .field("_jsc_reserved3", &self._jsc_reserved3)
            .finish()
    }
}

#[repr(C)]
pub struct _JSCValuePrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type JSCValuePrivate = *mut _JSCValuePrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCVirtualMachineClass {
    pub parent_class: gobject::GObjectClass,
    pub _jsc_reserved0: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved1: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved2: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved3: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for JSCVirtualMachineClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCVirtualMachineClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("_jsc_reserved0", &self._jsc_reserved0)
            .field("_jsc_reserved1", &self._jsc_reserved1)
            .field("_jsc_reserved2", &self._jsc_reserved2)
            .field("_jsc_reserved3", &self._jsc_reserved3)
            .finish()
    }
}

#[repr(C)]
pub struct _JSCVirtualMachinePrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type JSCVirtualMachinePrivate = *mut _JSCVirtualMachinePrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCWeakValueClass {
    pub parent_class: gobject::GObjectClass,
    pub _jsc_reserved0: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved1: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved2: Option<unsafe extern "C" fn()>,
    pub _jsc_reserved3: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for JSCWeakValueClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCWeakValueClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("_jsc_reserved0", &self._jsc_reserved0)
            .field("_jsc_reserved1", &self._jsc_reserved1)
            .field("_jsc_reserved2", &self._jsc_reserved2)
            .field("_jsc_reserved3", &self._jsc_reserved3)
            .finish()
    }
}

#[repr(C)]
pub struct _JSCWeakValuePrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type JSCWeakValuePrivate = *mut _JSCWeakValuePrivate;

// Classes
#[repr(C)]
pub struct JSCClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for JSCClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCClass @ {:p}", self)).finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCContext {
    pub parent: gobject::GObject,
    pub priv_: *mut JSCContextPrivate,
}

impl ::std::fmt::Debug for JSCContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCContext @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCException {
    pub parent: gobject::GObject,
    pub priv_: *mut JSCExceptionPrivate,
}

impl ::std::fmt::Debug for JSCException {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCException @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCValue {
    pub parent: gobject::GObject,
    pub priv_: *mut JSCValuePrivate,
}

impl ::std::fmt::Debug for JSCValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCValue @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCVirtualMachine {
    pub parent: gobject::GObject,
    pub priv_: *mut JSCVirtualMachinePrivate,
}

impl ::std::fmt::Debug for JSCVirtualMachine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCVirtualMachine @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JSCWeakValue {
    pub parent: gobject::GObject,
    pub priv_: *mut JSCWeakValuePrivate,
}

impl ::std::fmt::Debug for JSCWeakValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("JSCWeakValue @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[link(name = "WPEWebKit-1.1")]
extern "C" {

    //=========================================================================
    // JSCClass
    //=========================================================================
    pub fn jsc_class_get_type() -> GType;
    pub fn jsc_class_add_constructor(
        jsc_class: *mut JSCClass,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
        n_params: c_uint,
        ...
    ) -> *mut JSCValue;
    pub fn jsc_class_add_constructor_variadic(
        jsc_class: *mut JSCClass,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
    ) -> *mut JSCValue;
    pub fn jsc_class_add_constructorv(
        jsc_class: *mut JSCClass,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
        n_parameters: c_uint,
        parameter_types: *mut GType,
    ) -> *mut JSCValue;
    pub fn jsc_class_add_method(
        jsc_class: *mut JSCClass,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
        n_params: c_uint,
        ...
    );
    pub fn jsc_class_add_method_variadic(
        jsc_class: *mut JSCClass,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
    );
    pub fn jsc_class_add_methodv(
        jsc_class: *mut JSCClass,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
        n_parameters: c_uint,
        parameter_types: *mut GType,
    );
    pub fn jsc_class_add_property(
        jsc_class: *mut JSCClass,
        name: *const c_char,
        property_type: GType,
        getter: gobject::GCallback,
        setter: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
    );
    pub fn jsc_class_get_name(jsc_class: *mut JSCClass) -> *const c_char;
    pub fn jsc_class_get_parent(jsc_class: *mut JSCClass) -> *mut JSCClass;

    //=========================================================================
    // JSCContext
    //=========================================================================
    pub fn jsc_context_get_type() -> GType;
    pub fn jsc_context_new() -> *mut JSCContext;
    pub fn jsc_context_new_with_virtual_machine(vm: *mut JSCVirtualMachine) -> *mut JSCContext;
    pub fn jsc_context_get_current() -> *mut JSCContext;
    pub fn jsc_context_check_syntax(
        context: *mut JSCContext,
        code: *const c_char,
        length: ssize_t,
        mode: JSCCheckSyntaxMode,
        uri: *const c_char,
        line_number: c_uint,
        exception: *mut *mut JSCException,
    ) -> JSCCheckSyntaxResult;
    pub fn jsc_context_clear_exception(context: *mut JSCContext);
    pub fn jsc_context_evaluate(
        context: *mut JSCContext,
        code: *const c_char,
        length: ssize_t,
    ) -> *mut JSCValue;
    pub fn jsc_context_evaluate_in_object(
        context: *mut JSCContext,
        code: *const c_char,
        length: ssize_t,
        object_instance: gpointer,
        object_class: *mut JSCClass,
        uri: *const c_char,
        line_number: c_uint,
        object: *mut *mut JSCValue,
    ) -> *mut JSCValue;
    pub fn jsc_context_evaluate_with_source_uri(
        context: *mut JSCContext,
        code: *const c_char,
        length: ssize_t,
        uri: *const c_char,
        line_number: c_uint,
    ) -> *mut JSCValue;
    pub fn jsc_context_get_exception(context: *mut JSCContext) -> *mut JSCException;
    pub fn jsc_context_get_global_object(context: *mut JSCContext) -> *mut JSCValue;
    pub fn jsc_context_get_value(context: *mut JSCContext, name: *const c_char) -> *mut JSCValue;
    pub fn jsc_context_get_virtual_machine(context: *mut JSCContext) -> *mut JSCVirtualMachine;
    pub fn jsc_context_pop_exception_handler(context: *mut JSCContext);
    pub fn jsc_context_push_exception_handler(
        context: *mut JSCContext,
        handler: JSCExceptionHandler,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
    );
    pub fn jsc_context_register_class(
        context: *mut JSCContext,
        name: *const c_char,
        parent_class: *mut JSCClass,
        vtable: *mut JSCClassVTable,
        destroy_notify: glib::GDestroyNotify,
    ) -> *mut JSCClass;
    pub fn jsc_context_set_value(
        context: *mut JSCContext,
        name: *const c_char,
        value: *mut JSCValue,
    );
    pub fn jsc_context_throw(context: *mut JSCContext, error_message: *const c_char);
    pub fn jsc_context_throw_exception(context: *mut JSCContext, exception: *mut JSCException);
    pub fn jsc_context_throw_printf(context: *mut JSCContext, format: *const c_char, ...);
    pub fn jsc_context_throw_with_name(
        context: *mut JSCContext,
        error_name: *const c_char,
        error_message: *const c_char,
    );
    pub fn jsc_context_throw_with_name_printf(
        context: *mut JSCContext,
        error_name: *const c_char,
        format: *const c_char,
        ...
    );

    //=========================================================================
    // JSCException
    //=========================================================================
    pub fn jsc_exception_get_type() -> GType;
    pub fn jsc_exception_new(context: *mut JSCContext, message: *const c_char)
        -> *mut JSCException;
    pub fn jsc_exception_new_printf(
        context: *mut JSCContext,
        format: *const c_char,
        ...
    ) -> *mut JSCException;
    //pub fn jsc_exception_new_vprintf(context: *mut JSCContext, format: *const c_char, args: /*Unimplemented*/va_list) -> *mut JSCException;
    pub fn jsc_exception_new_with_name(
        context: *mut JSCContext,
        name: *const c_char,
        message: *const c_char,
    ) -> *mut JSCException;
    pub fn jsc_exception_new_with_name_printf(
        context: *mut JSCContext,
        name: *const c_char,
        format: *const c_char,
        ...
    ) -> *mut JSCException;
    //pub fn jsc_exception_new_with_name_vprintf(context: *mut JSCContext, name: *const c_char, format: *const c_char, args: /*Unimplemented*/va_list) -> *mut JSCException;
    pub fn jsc_exception_get_backtrace_string(exception: *mut JSCException) -> *const c_char;
    pub fn jsc_exception_get_column_number(exception: *mut JSCException) -> c_uint;
    pub fn jsc_exception_get_line_number(exception: *mut JSCException) -> c_uint;
    pub fn jsc_exception_get_message(exception: *mut JSCException) -> *const c_char;
    pub fn jsc_exception_get_name(exception: *mut JSCException) -> *const c_char;
    pub fn jsc_exception_get_source_uri(exception: *mut JSCException) -> *const c_char;
    pub fn jsc_exception_report(exception: *mut JSCException) -> *mut c_char;
    pub fn jsc_exception_to_string(exception: *mut JSCException) -> *mut c_char;

    //=========================================================================
    // JSCValue
    //=========================================================================
    pub fn jsc_value_get_type() -> GType;
    pub fn jsc_value_new_array(
        context: *mut JSCContext,
        first_item_type: GType,
        ...
    ) -> *mut JSCValue;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_new_array_buffer(
        context: *mut JSCContext,
        data: gpointer,
        size: size_t,
        destroy_notify: glib::GDestroyNotify,
        user_data: gpointer,
    ) -> *mut JSCValue;
    pub fn jsc_value_new_array_from_garray(
        context: *mut JSCContext,
        array: *mut glib::GPtrArray,
    ) -> *mut JSCValue;
    pub fn jsc_value_new_array_from_strv(
        context: *mut JSCContext,
        strv: *const *const c_char,
    ) -> *mut JSCValue;
    pub fn jsc_value_new_boolean(context: *mut JSCContext, value: gboolean) -> *mut JSCValue;
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub fn jsc_value_new_from_json(context: *mut JSCContext, json: *const c_char) -> *mut JSCValue;
    pub fn jsc_value_new_function(
        context: *mut JSCContext,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
        n_params: c_uint,
        ...
    ) -> *mut JSCValue;
    pub fn jsc_value_new_function_variadic(
        context: *mut JSCContext,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
    ) -> *mut JSCValue;
    pub fn jsc_value_new_functionv(
        context: *mut JSCContext,
        name: *const c_char,
        callback: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
        return_type: GType,
        n_parameters: c_uint,
        parameter_types: *mut GType,
    ) -> *mut JSCValue;
    pub fn jsc_value_new_null(context: *mut JSCContext) -> *mut JSCValue;
    pub fn jsc_value_new_number(context: *mut JSCContext, number: c_double) -> *mut JSCValue;
    pub fn jsc_value_new_object(
        context: *mut JSCContext,
        instance: gpointer,
        jsc_class: *mut JSCClass,
    ) -> *mut JSCValue;
    pub fn jsc_value_new_string(context: *mut JSCContext, string: *const c_char) -> *mut JSCValue;
    pub fn jsc_value_new_string_from_bytes(
        context: *mut JSCContext,
        bytes: *mut glib::GBytes,
    ) -> *mut JSCValue;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_new_typed_array(
        context: *mut JSCContext,
        type_: JSCTypedArrayType,
        length: size_t,
    ) -> *mut JSCValue;
    pub fn jsc_value_new_undefined(context: *mut JSCContext) -> *mut JSCValue;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_array_buffer_get_data(value: *mut JSCValue, size: *mut size_t) -> gpointer;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_array_buffer_get_size(value: *mut JSCValue) -> size_t;
    pub fn jsc_value_constructor_call(
        value: *mut JSCValue,
        first_parameter_type: GType,
        ...
    ) -> *mut JSCValue;
    pub fn jsc_value_constructor_callv(
        value: *mut JSCValue,
        n_parameters: c_uint,
        parameters: *mut *mut JSCValue,
    ) -> *mut JSCValue;
    pub fn jsc_value_function_call(
        value: *mut JSCValue,
        first_parameter_type: GType,
        ...
    ) -> *mut JSCValue;
    pub fn jsc_value_function_callv(
        value: *mut JSCValue,
        n_parameters: c_uint,
        parameters: *mut *mut JSCValue,
    ) -> *mut JSCValue;
    pub fn jsc_value_get_context(value: *mut JSCValue) -> *mut JSCContext;
    pub fn jsc_value_is_array(value: *mut JSCValue) -> gboolean;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_is_array_buffer(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_boolean(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_constructor(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_function(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_null(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_number(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_object(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_string(value: *mut JSCValue) -> gboolean;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_is_typed_array(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_is_undefined(value: *mut JSCValue) -> gboolean;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_new_typed_array_with_buffer(
        array_buffer: *mut JSCValue,
        type_: JSCTypedArrayType,
        offset: size_t,
        length: ssize_t,
    ) -> *mut JSCValue;
    pub fn jsc_value_object_define_property_accessor(
        value: *mut JSCValue,
        property_name: *const c_char,
        flags: JSCValuePropertyFlags,
        property_type: GType,
        getter: gobject::GCallback,
        setter: gobject::GCallback,
        user_data: gpointer,
        destroy_notify: glib::GDestroyNotify,
    );
    pub fn jsc_value_object_define_property_data(
        value: *mut JSCValue,
        property_name: *const c_char,
        flags: JSCValuePropertyFlags,
        property_value: *mut JSCValue,
    );
    pub fn jsc_value_object_delete_property(value: *mut JSCValue, name: *const c_char) -> gboolean;
    pub fn jsc_value_object_enumerate_properties(value: *mut JSCValue) -> *mut *mut c_char;
    pub fn jsc_value_object_get_property(
        value: *mut JSCValue,
        name: *const c_char,
    ) -> *mut JSCValue;
    pub fn jsc_value_object_get_property_at_index(
        value: *mut JSCValue,
        index: c_uint,
    ) -> *mut JSCValue;
    pub fn jsc_value_object_has_property(value: *mut JSCValue, name: *const c_char) -> gboolean;
    pub fn jsc_value_object_invoke_method(
        value: *mut JSCValue,
        name: *const c_char,
        first_parameter_type: GType,
        ...
    ) -> *mut JSCValue;
    pub fn jsc_value_object_invoke_methodv(
        value: *mut JSCValue,
        name: *const c_char,
        n_parameters: c_uint,
        parameters: *mut *mut JSCValue,
    ) -> *mut JSCValue;
    pub fn jsc_value_object_is_instance_of(value: *mut JSCValue, name: *const c_char) -> gboolean;
    pub fn jsc_value_object_set_property(
        value: *mut JSCValue,
        name: *const c_char,
        property: *mut JSCValue,
    );
    pub fn jsc_value_object_set_property_at_index(
        value: *mut JSCValue,
        index: c_uint,
        property: *mut JSCValue,
    );
    pub fn jsc_value_to_boolean(value: *mut JSCValue) -> gboolean;
    pub fn jsc_value_to_double(value: *mut JSCValue) -> c_double;
    pub fn jsc_value_to_int32(value: *mut JSCValue) -> i32;
    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    pub fn jsc_value_to_json(value: *mut JSCValue, indent: c_uint) -> *mut c_char;
    pub fn jsc_value_to_string(value: *mut JSCValue) -> *mut c_char;
    pub fn jsc_value_to_string_as_bytes(value: *mut JSCValue) -> *mut glib::GBytes;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_typed_array_get_buffer(value: *mut JSCValue) -> *mut JSCValue;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_typed_array_get_data(value: *mut JSCValue, length: *mut size_t) -> gpointer;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_typed_array_get_length(value: *mut JSCValue) -> size_t;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_typed_array_get_offset(value: *mut JSCValue) -> size_t;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_typed_array_get_size(value: *mut JSCValue) -> size_t;
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_38")))]
    pub fn jsc_value_typed_array_get_type(value: *mut JSCValue) -> JSCTypedArrayType;

    //=========================================================================
    // JSCVirtualMachine
    //=========================================================================
    pub fn jsc_virtual_machine_get_type() -> GType;
    pub fn jsc_virtual_machine_new() -> *mut JSCVirtualMachine;

    //=========================================================================
    // JSCWeakValue
    //=========================================================================
    pub fn jsc_weak_value_get_type() -> GType;
    pub fn jsc_weak_value_new(value: *mut JSCValue) -> *mut JSCWeakValue;
    pub fn jsc_weak_value_get_value(weak_value: *mut JSCWeakValue) -> *mut JSCValue;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn jsc_get_major_version() -> c_uint;
    pub fn jsc_get_micro_version() -> c_uint;
    pub fn jsc_get_minor_version() -> c_uint;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_foreach(function: JSCOptionsFunc, user_data: gpointer);
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_boolean(option: *const c_char, value: *mut gboolean) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_double(option: *const c_char, value: *mut c_double) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_int(option: *const c_char, value: *mut c_int) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_option_group() -> *mut glib::GOptionGroup;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_range_string(option: *const c_char, value: *mut *mut c_char)
        -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_size(option: *const c_char, value: *mut size_t) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_string(option: *const c_char, value: *mut *mut c_char) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_get_uint(option: *const c_char, value: *mut c_uint) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_set_boolean(option: *const c_char, value: gboolean) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_set_double(option: *const c_char, value: c_double) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_set_int(option: *const c_char, value: c_int) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_set_range_string(option: *const c_char, value: *const c_char) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_set_size(option: *const c_char, value: size_t) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_set_string(option: *const c_char, value: *const c_char) -> gboolean;
    #[cfg(any(feature = "v2_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
    pub fn jsc_options_set_uint(option: *const c_char, value: c_uint) -> gboolean;

}
