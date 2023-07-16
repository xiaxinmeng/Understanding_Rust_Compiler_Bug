plain
[01:09:05] 
[01:09:05]      Running build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-005c3aa196393c84
[01:09:05] 
[01:09:05] running 490 tests
[01:09:06] error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-005c3aa196393c84 --quiet` (signal: 11, SIGSEGV: invalid memory reference)
[01:09:06] 
[01:09:06] 
[01:09:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:09:06] 
[01:09:06] 
[01:09:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:06] Build completed unsuccessfully in 0:27:53
[01:09:06] Build completed unsuccessfully in 0:27:53
[01:09:06] Makefile:58: recipe for target 'check' failed
[01:09:06] make: *** [chec4425960 .
3153108 ./obj/build
2386872 ./obj/build/x86_64-unknown-linux-gnu
725068 ./src
572752 ./obj/build/bootstrap
