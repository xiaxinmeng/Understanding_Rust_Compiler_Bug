plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `const_eval_select` is not yet stable as a const fn
    |
    |
157 |         crate::intrinsics::const_eval_select((data,), noop, rt_check);
    |
    = help: const-stable functions can only call other const-stable functions

error: could not compile `core` due to previous error
