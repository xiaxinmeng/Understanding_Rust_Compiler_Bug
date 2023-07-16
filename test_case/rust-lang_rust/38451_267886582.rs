
executing step Step { name: "check-std-all", stage: 2, host: "x86_64-unknown-openbsd", target: "x86_64-unknown-openbsd" }
Testing libstd stage2 (x86_64-unknown-openbsd -> x86_64-unknown-openbsd)
running: "/data/semarie/obj/rust-installs/beta/bin/cargo" "test" "-j" "4" "--target" "x86_64-unknown-openbsd" "-v" "--release" "--frozen" "--manifest-path" "/data/semarie/src/rust/src/rustc/std_shim/Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std_shim" "-p" "std" "-p" "rand" "-p" "libc" "-p" "compiler_builtins" "-p"
"alloc" "-p" "collections" "-p" "unwind" "-p" "std_unicode" "-p" "panic_abort" "-p" "build_helper" "-p" "alloc_system" "-p" "core" "--"
error: Package `build_helper v0.1.0 (file:///data/semarie/src/rust/src/build_helper)` does not have these features: `backtrace, panic-unwind, jemalloc`


command did not execute successfully: "/data/semarie/obj/rust-installs/beta/bin/cargo" "test" "-j" "4" "--target" "x86_64-unknown-openbsd" "-v" "--release" "--frozen" "--manifest-path" "/data/semarie/src/rust/src/rustc/std_shim/Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std_shim" "-p" "std" "-p" "rand" "-p" "libc" "-p" "compiler_builtins" "-p" "alloc" "-p" "collections" "-p" "unwind" "-p" "std_unicode" "-p" "panic_abort" "-p"
"build_helper" "-p" "alloc_system" "-p" "core" "--"
expected success, got: exit code: 101
