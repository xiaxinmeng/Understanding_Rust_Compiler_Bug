plain
[01:03:10] ....................................................................................................
[01:03:17] ....................................................................................................
[01:03:24] ....................................................................................................
[01:03:33] ....................................................................................................
[01:03:41] ........................i.............FFF.F......F..FF.........................i....................
[01:03:42] failures:
[01:03:42] 
[01:03:42] ---- str/mod.rs - str::str::trim_end (line 3642) stdout ----
[01:03:42] ---- str/mod.rs - str::str::trim_end (line 3642) stdout ----
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 6 | assert_eq!(" Hello\tworld", s.trim_end());
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] thread 'str/mod.rs - str::str::trim_end (line 3642)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:42] 
[01:03:42] ---- str/mod.rs - str::str::trim_end (line 3650) stdout ----
[01:03:42] ---- str/mod.rs - str::str::trim_end (line 3650) stdout ----
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 5 | assert!(Some('h') == s.trim_end().chars().rev().next());
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 8 | assert!(Some('ת') == s.trim_end().chars().rev().next());
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] thread 'str/mod.rs - str::str::trim_end (line 3650)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:42] 
[01:03:42] ---- str/mod.rs - str::str::trim_end_matches (line 3840) stdout ----
[01:03:42] ---- str/mod.rs - str::str::trim_end_matches (line 3840) stdout ----
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 4 | assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 5 | assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 8 | assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] thread 'str/mod.rs - str::str::trim_end_matches (line 3840)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:42] ---- str/mod.rs - str::str::trim_end_matches (line 3850) stdout ----
[01:03:42] ---- str/mod.rs - str::str::trim_end_matches (line 3850) stdout ----
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 4 | assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] thread 'str/mod.rs - str::str::trim_end_matches (line 3850)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:42] ---- str/mod.rs - str::str::trim_start (line 3606) stdout ----
[01:03:42] ---- str/mod.rs - str::str::trim_start (line 3606) stdout ----
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 6 | assert_eq!("Hello\tworld\t", s.trim_start());
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] thread 'str/mod.rs - str::str::trim_start (line 3606)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:42] 
[01:03:42] ---- str/mod.rs - str::str::trim_start (line 3614) stdout ----
[01:03:42] ---- str/mod.rs - str::str::trim_start (line 3614) stdout ----
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 5 | assert!(Some('E') == s.trim_start().chars().next());
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 8 | assert!(Some('ע') == s.trim_start().chars().next());
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] thread 'str/mod.rs - str::str::trim_start (line 3614)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:42] 
[01:03:42] ---- str/mod.rs - str::str::trim_start_matches (line 3801) stdout ----
[01:03:42] ---- str/mod.rs - str::str::trim_start_matches (line 3801) stdout ----
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 4 | assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 5 | assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] 
[01:03:42] error[E0658]: use of unstable library feature 'trim_direction' (see issue #30459)
[01:03:42]   |
[01:03:42]   |
[01:03:42] 8 | assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
[01:03:42]   |
[01:03:42]   |
[01:03:42]   = help: add #![feature(trim_direction)] to the crate attributes to enable
[01:03:42] thread 'str/mod.rs - str::str::trim_start_matches (line 3801)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:03:42] 
[01:03:42] 
[01:03:42] failures:
---
[01:03:42] 
[01:03:42] error: test failed, to rerun pass '--doc'
[01:03:42] 
[01:03:42] 
[01:03:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:03:42] 
[01:03:42] 
[01:03:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:42] Build completed unsuccessfully in 0:20:37
[01:03:42] Build completed unsuccessfully in 0:20:37
[01:03:42] make: *** [check] Error 1
[01:03:42] Makefile:58: recipe for target 'check' failed
60840 ./src/llvm-emscripten/lib
59588 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
55548 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
55268 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
