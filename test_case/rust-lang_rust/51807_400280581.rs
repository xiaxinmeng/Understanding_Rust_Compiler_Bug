plain
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:52]    Compiling libc v0.2.40
[01:04:53]    Compiling rand v0.4.2
[01:04:55]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:05:00] error: use of deprecated item 'core::str::<impl str>::slice_unchecked': duplicates `get_unchecked`
[01:05:00]     |
[01:05:00]     |
[01:05:00] 180 |     assert_eq!("ab", unsafe {"abc".slice_unchecked(0, 2)});
[01:05:00]     |
[01:05:00]     |
[01:05:00]     = note: `-D deprecated` implied by `-D warnings`
[01:05:00] 
[01:05:00] error: use of deprecated item 'core::str::<impl str>::slice_unchecked': duplicates `get_unchecked`
[01:05:00]     |
[01:05:00]     |
[01:05:00] 181 |     assert_eq!("bc", unsafe {"abc".slice_unchecked(1, 3)});
[01:05:00] 
[01:05:00] 
[01:05:00] error: use of deprecated item 'core::str::<impl str>::slice_unchecked': duplicates `get_unchecked`
[01:05:00]     |
[01:05:00]     |
[01:05:00] 182 |     assert_eq!("", unsafe {"abc".slice_unchecked(1, 1)});
[01:05:00] 
[01:05:00] 
[01:05:00] error: use of deprecated item 'core::str::<impl str>::slice_unchecked': duplicates `get_unchecked`
[01:05:00]     |
[01:05:00]     |
[01:05:00] 203 |         unsafe { letters.slice_unchecked(0, 500000)});
[01:05:00] 
[01:05:03] error: aborting due to 4 previous errors
[01:05:03] 
[01:05:03] error: Could not compile `alloc`.
[01:05:03] error: Could not compile `alloc`.
[01:05:03] warning: build failed, waiting for other jobs to finish...
[01:05:04] error: build failed
[01:05:04] 
[01:05:04] 
[01:05:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:05:04] 
[01:05:04] 
[01:05:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:04] Build completed unsuccessfully in 0:23:49
[01:05:04] Build completed unsuccessfully in 0:23:49
[01:05:04] Makefile:58: recipe for target 'check' failed
[01:05:04] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2aca81aa
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
