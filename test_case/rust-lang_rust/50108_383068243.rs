plain
[00:41:05] Documenting book redirect pages (x86_64-unknown-linux-gnu)
[00:41:06] Documenting stage2 std (x86_64-unknown-linux-gnu)
[00:41:06]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:41:06]  Documenting core v0.0.0 (file:///checkout/src/libcore)
[00:41:16] thread 'main' panicked at 'assertion failed: *old == value', librustc_data_structures/sync.rs:241:42
[00:41:17] error: Could not compile `core`.
[00:41:17] warning: build failed, waiting for other jobs to finish...
[00:41:55] warning: [1] cannot be resolved, ignoring it...
[00:41:55] 
---
[00:41:58] 
[00:42:03] error: build failed
[00:42:03] 
[00:42:03] 
[00:42:03] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:42:03] 
[00:42:03] 
[00:42:03] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:42:03] Build completed unsuccessfully in 0:05:33
[00:42:03] Build completed unsuccessfully in 0:05:33
[00:42:03] Makefile:28: recipe for target 'all' failed
[00:42:03] make: *** [all] Error 1
2173528 ./obj
2173496 ./obj/build
1567336 ./obj/build/x86_64-unknown-linux-gnu
721964 ./src
---
148620 ./.git/modules/src
144080 ./obj/build/bootstrap/debug/incremental
127844 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
123124 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv
123120 ./obj/build/bootstrap/debug/incremental/bootstrap-351vorei3hhuv/s-f0aho0r06e-npvqy9-2s5jtoq90mq3o
122132 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
104564 ./obj/build/x86_64-unknown-linux-gnu/crate-docs
102808 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102356 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
