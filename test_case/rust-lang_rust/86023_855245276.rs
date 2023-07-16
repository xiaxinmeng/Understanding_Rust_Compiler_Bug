plain
thread panicked while panicking. aborting.
error: test failed, to rerun pass '-p alloc --test collectionstests'

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i586-unknown-linux-gnu/release/deps/collectionstests-b7619a21b3e02624` (signal: 4, SIGILL: illegal instruction)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "i586-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "alloc" "--"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target i586-unknown-linux-gnu,i686-unknown-linux-musl
Build completed unsuccessfully in 0:23:20
