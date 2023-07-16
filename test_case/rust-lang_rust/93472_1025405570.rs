plain
   Compiling getopts v0.2.21
error[E0080]: it is undefined behavior to use this value
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.21/src/lib.rs:927:1
    |
927 | fn is_arg(arg: &str) -> bool {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.<enum-variant(Some)>.0.<deref>: encountered (potentially part of) a pointer, but expected plain (non-pointer) bytes
    |
    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
    = note: the raw bytes of the constant (size: 8, align: 8) {
            }

For more information about this error, try `rustc --explain E0080`.
error: could not compile `getopts` due to previous error
