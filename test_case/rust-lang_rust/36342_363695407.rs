rust

#[doc(hidden)]
#[macro_export]
macro_rules! export_c_symbol {
    (fn $name:ident($( $arg:ident : $type:ty ),*) -> $ret:ty) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($( $arg : $type),*) -> $ret {
            $crate::error_handling::$name($( $arg ),*)
        }
    };
    (fn $name:ident($( $arg:ident : $type:ty ),*)) => {
        export_c_symbol!(fn $name($( $arg : $type),*) -> ());
    }
}

/// As a workaround for rust-lang/rust#6342, you can use this macro to make sure
/// the symbols for `ffi_utils`'s error handling are correctly exported in your
/// `cdylib`.
#[macro_export]
macro_rules! export_error_handling_functions {
    () => {
    export_c_symbol!(fn clear_last_error());
    export_c_symbol!(fn last_error_length() -> ::libc::c_int);
    export_c_symbol!(fn last_error_length_utf16() -> ::libc::c_int);
    export_c_symbol!(fn error_message_utf8(buf: *mut ::libc::c_char, length: ::libc::c_int) -> ::libc::c_int);
    export_c_symbol!(fn error_message_utf16(buf: *mut u16, length: ::libc::c_int) -> ::libc::c_int);
    };
}
