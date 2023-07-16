plain
[01:08:37] ....................................................................................................
[01:08:49] ....................................................................................................
[01:09:02] ....................................................................................................
[01:09:17] ....................................................................................................
[01:09:32] ......................................................................................F.............
[01:10:04] ....................................................................................................
[01:10:21] .......................i...............................................i.......................
[01:10:21] failures:
[01:10:21] 
[01:10:21] 
[01:10:21] ---- option.rs - option::Option<T>::replace (line 857) stdout ----
[01:10:21] error[E0658]: use of unstable library feature 'option_replace' (see issue #51998)
[01:10:21]  --> option.rs:859:13
[01:10:21]   |
[01:10:21] 5 | let old = x.replace(5);
[01:10:21]   |
[01:10:21]   |
[01:10:21]   = help: add #![feature(option_replace)] to the crate attributes to enable
[01:10:21] error[E0658]: use of unstable library feature 'option_replace' (see issue #51998)
[01:10:21]   --> option.rs:864:13
[01:10:21]    |
[01:10:21]    |
[01:10:21] 10 | let old = x.replace(3);
[01:10:21]    |
[01:10:21]    |
[01:10:21]    = help: add #![feature(option_replace)] to the crate attributes to enable
[01:10:21] thread 'option.rs - option::Option<T>::replace (line 857)' panicked at 'couldn't compile the test', librustdoc/test.rs:327:17
[01:10:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:10:21] 
[01:10:21] 
---
[01:10:21] 
[01:10:21] error: test failed, to rerun pass '--doc'
[01:10:21] 
[01:10:21] 
[01:10:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:10:21] 
[01:10:21] 
[01:10:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:21] Build completed unsuccessfully in 0:30:49
[01:10:21] Build completed unsuccessfully in 0:30:49
[01:10:21] make: *** [check] Error 1
[01:10:21] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:003ca5cf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1069ff86:start=1530569463395771721,finish=1530569463403315532,duration=7543811
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1da2b9fa
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0593cc28
$ dmesg | grep -i kill
