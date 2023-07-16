plain
[01:18:18] ....................................................................................................
[01:18:34] ....................................................................................................
[01:18:47] ....................................................................................................
[01:19:07] ....................................................................................................
[01:19:24] .........................iF..................................................................
[01:19:24] 
[01:19:24] ---- str/mod.rs - str::str::split_ascii_whitespace (line 3202) stdout ----
[01:19:24] ---- str/mod.rs - str::str::split_ascii_whitespace (line 3202) stdout ----
[01:19:24]  error[E0658]: use of unstable library feature 'split_ascii_whitespace' (see issue #48656)
[01:19:24]  --> str/mod.rs:3203:30
[01:19:24]   |
[01:19:24] 4 | let mut iter = "A few words".split_ascii_whitespace();
[01:19:24]   |
[01:19:24]   |
[01:19:24]   = help: add #![feature(split_ascii_whitespace)] to the crate attributes to enable
[01:19:24] thread 'str/mod.rs - str::str::split_ascii_whitespace (line 3202)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:19:24] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:19:24] 
[01:19:24] 
---
[01:19:24] 
[01:19:24] error: test failed, to rerun pass '--doc'
[01:19:24] 
[01:19:24] 
[01:19:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:19:24] 
[01:19:24] 
[01:19:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:24] Build completed unsuccessfully in 0:34:43
[01:19:24] Build completed unsuccessfully in 0:34:43
[01:19:24] make: *** [check] Error 1
[01:19:24] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:099320f7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
