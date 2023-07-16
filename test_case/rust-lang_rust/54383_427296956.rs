plain
[00:48:39] ................................................................................................i... 2200/4551
[00:48:43] .................................................................................................... 2300/4551
[00:48:46] .................................................................................................... 2400/4551
[00:48:50] .................................................................................................... 2500/4551
[00:48:53] ........iiiiiiiii................................................................................... 2600/4551
[00:48:59] .................................................................................................... 2800/4551
[00:49:03] .................................................................................................... 2900/4551
[00:49:05] ...........................i........................................................................ 3000/4551
[00:49:08] .......................................................................................i.i..ii...... 3100/4551
---
[01:27:54] 
[01:27:54] failures:
[01:27:54] 
[01:27:54] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0378 (line 5580) stdout ----
[01:27:54] error[E0412]: cannot find type `PhantomData` in this scope
[01:27:54]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5586:15
[01:27:54]   |
[01:27:54] 8 |     _phantom: PhantomData<()>,
[01:27:54] help: possible candidate is found in another module, you can import it into scope
[01:27:54]   |
[01:27:54] 3 | use std::marker::PhantomData;
[01:27:54]   |
[01:27:54]   |
[01:27:54] 
[01:27:54] error[E0378]: the trait `DispatchFromDyn` may only be implemented for structs containing the field being coerced, `PhantomData` fields, and nothing else
[01:27:54]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5589:1
[01:27:54]    |
[01:27:54] 11 | / impl<T, U> DispatchFromDyn<Wrapper<U>> for Wrapper<T>
[01:27:54] 12 | | where
[01:27:54] 13 | |     T: DispatchFromDyn<U>,
[01:27:54] 14 | | {}
[01:27:54]    |
[01:27:54]    |
[01:27:54]    = note: extra field `_phantom` of type `[type error]` is not allowed
[01:27:54] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0378 (line 5580)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:27:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:27:54] 
[01:27:54] 
---
[01:27:54] 
[01:27:54] 
[01:27:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:27:54] Build completed unsuccessfully in 0:43:45
[01:27:54] Makefile:58: recipe for target 'check' failed
[01:27:54] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c170ff0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1dbd56d0:start=1538730462552182258,finish=1538730462683663231,duration=131480973
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:012a87f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08e65a34
$ dmesg | grep -i kill
