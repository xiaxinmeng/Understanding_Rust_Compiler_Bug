plain
   Compiling libc v0.2.98
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: the feature `bindings_after_at` has been stable since 1.54.0 and no longer requires an attribute to enable
    |
116 | #![feature(bindings_after_at)]
    |            ^^^^^^^^^^^^^^^^^
    |
    |
    = note: `-D stable-features` implied by `-D warnings`
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:04:43
