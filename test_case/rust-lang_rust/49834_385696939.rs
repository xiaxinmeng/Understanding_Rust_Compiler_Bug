plain
[00:37:03] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[00:37:04] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:37:04]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:37:04]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:37:17] overwriting key/trait_ref: Binder(<isize as marker::Freeze>) with EvaluationResult WithDepNode { dep_node: 150890, cached_value: EvaluatedToOk }
[00:37:17]  old value: WithDepNode { dep_node: 150889, cached_value: EvaluatedToOk }
[00:37:17] thread 'main' panicked at 'assertion failed: *old == value', librustc_data_structures/sync.rs:241:42
[00:37:17] 
[00:37:17] error: internal compiler error: unexpected panic
[00:37:17] 
[00:37:17] 
[00:37:17] note: the compiler unexpectedly panicked. this is a bug.
[00:37:17] 
[00:37:17] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:37:17] 
[00:37:17] note: rustc 1.27.0-dev running on x86_64-unknown-linux-gnu
[00:37:17] 
[00:37:17] note: compiler flags: -Z force-unstable-if-unmarked -C debug-assertions=off -C overflow-checks=on -C incremental -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:37:17] 
[00:37:17] note: some of the compiler flags provided by cargo are hidden
[00:37:17] error: Could not compile `core`.
[00:37:17] warning: build failed, waiting for other jobs to finish...
[00:37:49] warning: [1] cannot be resolved, ignoring it...
[00:37:49] 
---
[00:37:51] 
[00:37:55] error: build failed
[00:37:55] 
[00:37:55] 
[00:37:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:37:55] 
[00:37:55] 
[00:37:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:37:55] Build completed unsuccessfully in 0:05:15
[00:37:55] Build completed unsuccessfully in 0:05:15
[00:37:55] Makefile:28: recipe for target 'all' failed
[00:37:55] make: *** [all] Error 1
2174820 ./obj
2174788 ./obj/build
1566936 ./obj/build/x86_64-unknown-linux-gnu
725072 ./src
---
149120 ./src/llvm-emscripten/test
144660 ./obj/build/bootstrap/debug/incremental
128084 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
123692 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d
123688 ./obj/build/bootstrap/debug/incremental/bootstrap-1wl4zjaz72e5d/s-f0msmdn1je-1ia77dq-6gvofocobazn
121820 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
105360 ./obj/build/x86_64-unknown-linux-gnu/crate-docs
103452 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
103448 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
