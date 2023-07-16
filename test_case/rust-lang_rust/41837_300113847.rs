
[00:44:28] ---- /checkout/src/doc/unstable-book/src/language-features/allocator.md - allocator (line 15) stdout ----
[00:44:28] 	error[E0464]: multiple matching crates for `libc`
[00:44:28]   --> <anon>:23:1
[00:44:28]    |
[00:44:28] 23 | extern crate libc;
[00:44:28]    | ^^^^^^^^^^^^^^^^^^
[00:44:28]    |
[00:44:28]    = note: candidates:
[00:44:28]    = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-6ecacccb5bdc4911.rlib
[00:44:28]    = note: crate name: libc
[00:44:28]    = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-008601069b74251c.rlib
[00:44:28]    = note: crate name: libc
[00:44:28] 
[00:44:28] error[E0463]: can't find crate for `libc`
[00:44:28]   --> <anon>:23:1
[00:44:28]    |
[00:44:28] 23 | extern crate libc;
[00:44:28]    | ^^^^^^^^^^^^^^^^^^ can't find crate
[00:44:28] 
[00:44:28] error: aborting due to 2 previous errors
[00:44:28] 
[00:44:28] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:454
[00:44:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:28] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:275
[00:44:28] 
[00:44:28] ---- /checkout/src/doc/unstable-book/src/language-features/allocator.md - _code_allocator__code_ (line 15) stdout ----
[00:44:28] 	error[E0464]: multiple matching crates for `libc`
[00:44:28]   --> <anon>:23:1
[00:44:28]    |
[00:44:28] 23 | extern crate libc;
[00:44:28]    | ^^^^^^^^^^^^^^^^^^
[00:44:28]    |
[00:44:28]    = note: candidates:
[00:44:28]    = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-6ecacccb5bdc4911.rlib
[00:44:28]    = note: crate name: libc
[00:44:28]    = note: path: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-008601069b74251c.rlib
[00:44:28]    = note: crate name: libc
[00:44:28] 
[00:44:28] error[E0463]: can't find crate for `libc`
[00:44:28]   --> <anon>:23:1
[00:44:28]    |
[00:44:28] 23 | extern crate libc;
[00:44:28]    | ^^^^^^^^^^^^^^^^^^ can't find crate
[00:44:28] 
[00:44:28] error: aborting due to 2 previous errors
[00:44:28] 
[00:44:28] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:454
[00:44:28] thread 'rustc' panicked at 'couldn't compile the test', /checkout/src/librustdoc/test.rs:275
[00:44:28] 
[00:44:28] 
[00:44:28] failures:
[00:44:28]     /checkout/src/doc/unstable-book/src/language-features/allocator.md - _code_allocator__code_ (line 15)
[00:44:28]     /checkout/src/doc/unstable-book/src/language-features/allocator.md - allocator (line 15)
[00:44:28] 
[00:44:28] test result: FAILED. 0 passed; 2 failed; 2 ignored; 0 measured
