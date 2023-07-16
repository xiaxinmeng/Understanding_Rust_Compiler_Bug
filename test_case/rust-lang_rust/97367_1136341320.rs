plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `run_utf8_validation` is not yet stable as a const fn
   |
   |
88 |     match run_utf8_validation(v) {
   |
   = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to previous error
