
test atomic::int_nand ... ok
test atomic::int_or ... ok
test atomic::int_xor ... ok
test atomic::static_init ... died due to signal 11
error: test failed, to rerun pass '--test coretests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-j" "8" "--release" "--locked" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--"
expected success, got: exit code: 3


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
