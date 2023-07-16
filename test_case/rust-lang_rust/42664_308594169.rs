
[00:49:58] make[1]: warning: jobserver unavailable: using -j1.  Add '+' to parent make rule.
[00:49:58] error[E0464]: multiple matching crates for `libc`
[00:49:58]   --> foo.rs:15:1
[00:49:58]    |
[00:49:58] 15 | extern crate libc;
[00:49:58]    | ^^^^^^^^^^^^^^^^^^
[00:49:58]    |
[00:49:58]    = note: candidates:
[00:49:58]    = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-6545a7b8b89ca177.rlib
[00:49:58]    = note: crate name: libc
[00:49:58]    = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-6ecacccb5bdc4911.rlib
[00:49:58]    = note: crate name: libc
[00:49:58] 
[00:49:58] error[E0463]: can't find crate for `libc`
[00:49:58]   --> foo.rs:15:1
[00:49:58]    |
[00:49:58] 15 | extern crate libc;
[00:49:58]    | ^^^^^^^^^^^^^^^^^^ can't find crate
[00:49:58] 
[00:49:58] error: aborting due to previous error(s)
[00:49:58] 
[00:49:58] make[1]: *** [all] Error 101
