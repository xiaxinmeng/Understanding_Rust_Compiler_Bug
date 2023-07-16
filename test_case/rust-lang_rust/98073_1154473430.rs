plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `validity_invariants_of::validity_invariants_of` is not yet stable as a const fn
     |
     |
2158 |     let invariants: &'static [u8] = validity_invariants_of::<T>();
     |
     |
     = help: add `#![feature(validity_invariants_of)]` to the crate attributes to enable
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:04:22
