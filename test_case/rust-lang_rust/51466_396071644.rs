plain
[01:08:57] 
[01:08:57]    Doc-tests core
[01:09:09] 
[01:09:09] running 2079 tests
[01:09:21] .......................................................F............................................
[01:09:50] ....................................................................................................
[01:10:06] ...............................i....................................................................
[01:10:17] ....................................................................................................
[01:10:29] ....................................................................................................
---
[01:13:31] ....................................................................................................
[01:13:44] .........i.............................................i.......................
[01:13:44] failures:
[01:13:44] 
[01:13:44] ---- cell.rs - cell::RefMut<'b, T>::map_split (line 1221) stdout ----
[01:13:44] error[E0596]: cannot borrow immutable local variable `begin` as mutable
[01:13:44]   --> cell.rs:1230:1
[01:13:44]    |
[01:13:44] 9  | let (begin, end) = RefMut::map_split(borrow, |slice| slice.split_at_mut(2));
[01:13:44]    |      ----- consider changing this to `mut begin`
[01:13:44] ...
[01:13:44] 12 | begin.copy_from_slice(&[4, 3]);
[01:13:44]    | ^^^^^ cannot borrow mutably
[01:13:44] 
[01:13:44] error[E0596]: cannot borrow immutable local variable `end` as mutable
[01:13:44]   --> cell.rs:1231:1
[01:13:44]    |
[01:13:44] 9  | let (begin, end) = RefMut::map_split(borrow, |slice| slice.split_at_mut(2));
[01:13:44]    |             --- consider changing this to `mut end`
[01:13:44] ...
[01:13:44] 13 | end.copy_from_slice(&[2, 1]);
[01:13:44]    | ^^^ cannot borrow mutably
[01:13:44] 
[01:13:44] thread 'cell.rs - cell::RefMut<'b, T>::map_split (line 1221)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:13:44] 
[01:13:44] 
[01:13:44] failures:
[01:13:44]     cell.rs - cell::RefMut<'b, T>::map_split (line 1221)
[01:13:44]     cell.rs - cell::RefMut<'b, T>::map_split (line 1221)
[01:13:44] 
[01:13:44] test result: FAILED. 2075 passed; 1 failed; 3 ignored; 0 measured; 0 filtered out
[01:13:44] 
[01:13:44] error: test failed, to rerun pass '--doc'
[01:13:44] 
[01:13:44] 
[01:13:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:13:44] 
[01:13:44] 
[01:13:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:13:44] Build completed unsuccessfully in 0:31:05
[01:13:44] Build completed unsuccessfully in 0:31:05
[01:13:44] Makefile:58: recipe for target 'check' failed
[01:13:44] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05b30a45
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1d0f04c0:start=1528656269896282280,finish=1528656269903843575,duration=7561295
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e7ba5c0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08a51600
$ dmesg | grep -i kill
