plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `ptr::metadata::from_raw_parts` is not yet stable as a const fn
    |
260 |     from_raw_parts(data.cast(), len)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
