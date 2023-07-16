
DEBUG:rustc_trans::type_of: type_of &mut str::Chars
DEBUG:rustc_trans::type_of: --> mapped t=&mut str::Chars to llty=%"str::Chars"*
DEBUG:rustc_trans::type_of: type_of &mut str::Bytes
DEBUG:rustc_trans::type_of: type_of str::Bytes
DEBUG:rustc_trans::type_of: --> mapped t=str::Bytes to llty=%"str::Bytes" = type opaque
DEBUG:rustc_trans::type_of: --> mapped t=&mut str::Bytes to llty=%"str::Bytes"*
DEBUG:rustc_trans::type_of: type_of result::Result<(), str::Utf8Error>
DEBUG:rustc_trans::type_of: --> mapped t=result::Result<(), str::Utf8Error> to llty=%"result::Result<(), str::Utf8Error>" = type { i64, [0 x i64], [2 x i64] }
DEBUG:rustc_trans::type_of: type_of [u8; <unevaluated[]>]
error: internal compiler error: unexpected panic
