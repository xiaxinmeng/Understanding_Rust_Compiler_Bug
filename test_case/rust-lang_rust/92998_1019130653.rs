plain
test thread::local::tests::join_orders_after_tls_destructors ... ok
test sync::mpsc::sync_tests::stress_recv_timeout_shared ... ok
test sync::mpsc::sync_tests::stress_recv_timeout_two_threads ... ok
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok
died due to signal 9
error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:40:18
