plain
   Compiling sharded-slab v0.0.9
   Compiling itertools v0.9.0
   Compiling rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
   Compiling getopts v0.2.21
error: the feature `min_const_generics` has been stable since 1.51.0 and no longer requires an attribute to enable
  --> compiler/rustc_arena/src/lib.rs:19:12
   |
19 | #![feature(min_const_generics)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_arena`

