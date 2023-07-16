plain
[00:54:26] ....................................................................................................
[00:54:28] ......................................................i.............................................
[00:54:31] ....................................................................................................
[00:54:34] ....................................................................................................
[00:54:37] ..iiiiiiiii.........................................................................................
[00:54:43] ....................................................................................................
[00:54:46] ......................................................................................i.............
[00:54:49] ....................................................................................................
[00:54:52] .........................................i.i..ii....................................................
---
[01:27:28] 
[01:27:28] failures:
[01:27:28] 
[01:27:28] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0378 (line 5565) stdout ----
[01:27:28] error[E0405]: cannot find trait `CoerceUnsized` in this scope
[01:27:28]   --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5574:28
[01:27:28]    |
[01:27:28] 11 | impl<T: ?Sized, U: ?Sized> CoerceUnsized<Ptr<U>> for Ptr<T>
[01:27:28]    |                            ^^^^^^^^^^^^^ did you mean `CoerceSized`?
[01:27:28]    |
[01:27:28]    |
[01:27:28] 3  | use std::ops::CoerceUnsized;
[01:27:28] 
[01:27:28] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0378 (line 5565)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:27:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:27:28] 
[01:27:28] 
[01:27:28] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0378 (line 5585) stdout ----
[01:27:28] error[E0412]: cannot find type `PhantomData` in this scope
[01:27:28]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:5591:15
[01:27:28]   |
[01:27:28] 8 |     _phantom: PhantomData<()>,
[01
travis_time:end:04993b6f:start=1537510713754336308,finish=1537515962628450007,duration=5248874113699

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
---
travis_time:end:1a9d4020:start=1537515964439982332,finish=1537515964443983046,duration=4000714
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2caf179e
$ ln -s . checkout && for CORE in obj/cores/core.
