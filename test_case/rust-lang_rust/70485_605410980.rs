
test vec::test_splice_unbounded ... ok
test vec::test_split_at_mut ... ok
test vec::test_split_off ... ok
test vec::test_stable_push_pop ... ok
test vec::test_swap_remove_empty ... ok
test slice::test_sort ... ok
died due to signal 11
error: test failed, to rerun pass '-p alloc --test collectionstests'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "arm-linux-androideabi" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "-p" "alloc" "--"
expected success, got: exit code: 3


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
Build completed unsuccessfully in 2:25:06
