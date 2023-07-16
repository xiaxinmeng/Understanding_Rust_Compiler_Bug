
[01:06:47] failures:
[01:06:47] 
[01:06:47] ---- thread/mod.rs - thread::spawn (line 445) stdout ----
[01:06:47] 	error[E0425]: cannot find value `v` in this scope
[01:06:47]   --> <anon>:12:16
[01:06:47]    |
[01:06:47] 12 | println!("{}", v);
[01:06:47]    |                ^ not found in this scope
[01:06:47] 
[01:06:47] error: aborting due to previous error(s)
[01:06:47] 
[01:06:47] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:217
[01:06:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:47] 
[01:06:47] ---- thread/mod.rs - thread::spawn (line 422) stdout ----
[01:06:47] 	error: unused result which must be used
[01:06:47]   --> <anon>:10:5
[01:06:47]    |
[01:06:47] 10 |     tx.send("Hello, thread".to_owned());
[01:06:47]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:06:47]    |
[01:06:47]    = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:06:47] note: lint level defined here
[01:06:47]   --> <anon>:1:9
[01:06:47]    |
[01:06:47] 1  | #![deny(warnings)]
[01:06:47]    |         ^^^^^^^^
[01:06:47] 
[01:06:47] error: unused result which must be used
[01:06:47]   --> <anon>:17:1
[01:06:47]    |
[01:06:47] 17 | sender.join();
[01:06:47]    | ^^^^^^^^^^^^^^
[01:06:47]    |
[01:06:47]    = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:06:47] 
[01:06:47] error: unused result which must be used
[01:06:47]   --> <anon>:18:1
[01:06:47]    |
[01:06:47] 18 | receiver.join();
[01:06:47]    | ^^^^^^^^^^^^^^^^
[01:06:47]    |
[01:06:47]    = note: #[deny(unused_must_use)] implied by #[deny(warnings)]
[01:06:47] 
[01:06:47] error: aborting due to previous error(s)
[01:06:47] 
[01:06:47] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:217
[01:06:47] 
[01:06:47] 
[01:06:47] failures:
[01:06:47]     thread/mod.rs - thread::spawn (line 422)
[01:06:47]     thread/mod.rs - thread::spawn (line 445)
[01:06:47] 
[01:06:47] test result: FAILED. 797 passed; 2 failed; 22 ignored; 0 measured
[01:06:47] 
[01:06:47] error: test failed, to rerun pass '--doc'
[01:06:47] 
[01:06:47] 
[01:06:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std:0.0.0" "-p" "alloc_system:0.0.0" "-p" "libc:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "rustc_asan:0.0.0" "-p" "rustc_lsan:0.0.0" "-p" "unwind:0.0.0" "-p" "core:0.0.0" "-p" "rand:0.0.0" "-p" "rustc_msan:0.0.0" "-p" "panic_abort:0.0.0" "-p" "alloc:0.0.0" "-p" "collections:0.0.0" "-p" "rustc_tsan:0.0.0" "-p" "std_unicode:0.0.0" "--"
[01:06:47] expected success, got: exit code: 101
[01:06:47] 
[01:06:47] 
[01:06:47] Build completed unsuccessfully in 0:27:07
[01:06:47] Makefile:54: recipe for target 'check' failed
[01:06:47] make: *** [check] Error 1
