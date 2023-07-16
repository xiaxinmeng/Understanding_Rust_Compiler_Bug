
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
[...]
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
[...]
[01:18:09] error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/deps/coretests-968a4ca77273901e` (signal: 11, SIGSEGV: invalid memory reference)

[01:18:09] 

[01:18:09] To learn more, run the command again with --verbose.

[01:18:09] 

[01:18:09] 

[01:18:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "-j" "4" "--target" "i686-unknown-linux-musl" "--release" "--locked" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std:0.0.0" "-p" "alloc:0.0.0" "-p" "core:0.0.0" "-p" "panic_abort:0.0.0" "-p" "std_unicode:0.0.0" "-p" "rustc_asan:0.0.0" "-p" "rustc_lsan:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "rand:0.0.0" "-p" "alloc_system:0.0.0" "-p" "rustc_tsan:0.0.0" "-p" "libc:0.0.0" "-p" "unwind:0.0.0" "-p" "collections:0.0.0" "-p" "rustc_msan:0.0.0" "--"

[01:18:09] expected success, got: exit code: 101

[01:18:09] 

[01:18:09] 

[01:18:09] Build completed unsuccessfully in 1:17:25
