
[01:10:57] test net::tcp::tests::debug ... ok
[01:10:58] test net::tcp::tests::double_bind ... ok
[01:10:58] test net::tcp::tests::fast_rebind ... ok
[01:10:58] test net::tcp::tests::listen_localhost ... ok
[01:10:59] died due to signal 11
[01:10:59] [m[m[31m[1merror:[m test failed, to rerun pass '--lib'
[01:10:59] 
[01:10:59] 
[01:10:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std:0.0.0" "-p" "unwind:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "alloc:0.0.0" "-p" "alloc_system:0.0.0" "-p" "std_unicode:0.0.0" "-p" "libc:0.0.0" "-p" "panic_abort:0.0.0" "-p" "core:0.0.0" "--"
[01:10:59] expected success, got: exit code: 3
[01:10:59] 
[01:10:59] 
[01:10:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:10:59] Build completed unsuccessfully in 1:08:10
