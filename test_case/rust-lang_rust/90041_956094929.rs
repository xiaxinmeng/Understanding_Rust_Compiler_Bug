plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `const_eval_select` is not yet stable as a const fn
     |
     |
2077 |         const_eval_select((src, dst, count), compiletime_check, runtime_check);
     |
     = help: add `#![feature(const_eval_select)]` to the crate attributes to enable


error: `const_eval_select` is not yet stable as a const fn
     |
     |
2171 |         const_eval_select((src, dst, count), compiletime_check, runtime_check);
     |
     = help: add `#![feature(const_eval_select)]` to the crate attributes to enable

error: could not compile `core` due to 2 previous errors
