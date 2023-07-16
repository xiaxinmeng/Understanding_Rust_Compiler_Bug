plain
[01:16:23] 
[01:16:23]    Doc-tests core
[01:16:35] 
[01:16:35] running 2079 tests
[01:16:48] .........................................F..............F...........................................
[01:17:20] ....................................................................................................
[01:17:37] ...............................i....................................................................
[01:17:49] ....................................................................................................
[01:18:02] ....................................................................................................
---
[01:21:17] ....................................................................................................
[01:21:30] .........i.............................................i.......................
[01:21:30] failures:
[01:21:30] 
[01:21:30] ---- cell.rs - cell::Ref<'b, T>::map_split (line 1139) stdout ----
[01:21:30] error[E0658]: use of unstable library feature 'refcell_map_split'
[01:21:30]  --> cell.rs:1144:20
[01:21:30]   |
[01:21:30] 8 | let (begin, end) = Ref::map_split(borrow, |a| a.split_at(2));
[01:21:30]   |
[01:21:30]   |
[01:21:30]   = help: add #![feature(refcell_map_split)] to the crate attributes to enable
[01:21:30] 
[01:21:30] thread 'cell.rs - cell::Ref<'b, T>::map_split (line 1139)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:21:30] 
[01:21:30] ---- cell.rs - cell::RefMut<'b, T>::map_split (line 1220) stdout ----
[01:21:30] ---- cell.rs - cell::RefMut<'b, T>::map_split (line 1220) stdout ----
[01:21:30] error[E0658]: use of unstable library feature 'refcell_map_split'
[01:21:30]  --> cell.rs:1226:24
[01:21:30]   |
[01:21:30] 9 |     let (begin, end) = RefMut::map_split(borrow, |a| a.split_at_mut(2));
[01:21:30]   |
[01:21:30]   |
[01:21:30]   = help: add #![feature(refcell_map_split)] to the crate attributes to enable
[01:21:30] 
[01:21:30] thread 'cell.rs - cell::RefMut<'b, T>::map_split (line 1220)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:21:30] 
[01:21:30] failures:
[01:21:30]     cell.rs - cell::Ref<'b, T>::map_split (line 1139)
[01:21:30]     cell.rs - cell::RefMut<'b, T>::map_split (line 1220)
[01:21:30]     cell.rs - cell::RefMut<'b, T>::map_split (line 1220)
[01:21:30] 
[01:21:30] test result: FAILED. 2074 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out
[01:21:30] 
[01:21:30] error: test failed, to rerun pass '--doc'
[01:21:30] 
[01:21:30] 
[01:21:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:21:30] 
[01:21:30] 
[01:21:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:30] Build completed unsuccessfully in 0:34:19
[01:21:30] Build completed unsuccessfully in 0:34:19
[01:21:30] make: *** [check] Error 1
[01:21:30] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a85adb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
