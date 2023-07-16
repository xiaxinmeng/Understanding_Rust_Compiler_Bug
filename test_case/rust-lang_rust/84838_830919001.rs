plain
.................................................................................................... 9400/11819
.................................................................................................... 9500/11819
.................................................................................................... 9600/11819
.........................................i......i................................................... 9700/11819
.......................................................................................iiiiiii..iiii 9800/11819
.................................................................................................... 10000/11819
.................................................................................................... 10100/11819
.................................................................................................... 10200/11819
.................................................................................................... 10300/11819
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.171 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.44s

 finished in 2.516 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Set({"library/core"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
 finished in 49.260 seconds
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/checkout/library/core)
error[E0599]: the function or associated item `default` exists for array `[DoesNotImplDefault; 0]`, but its trait bounds were not satisfied
    |
    |
288 |     let _arr = <[DoesNotImplDefault; 0]>::default();
    |                                           ^^^^^^^ function or associated item cannot be called on `[DoesNotImplDefault; 0]` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/array/mod.rs:421:5
    |
    |
421 |     pub struct SendToDefault<T, const N: usize>(ZeroToSend<T, N>);
    |     -------------------------------------------------------------- doesn't satisfy `_: Send`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `std::array::array_defaults::SendToDefault<DoesNotImplDefault, 0_usize>: Send`
            which is required by `[DoesNotImplDefault; 0]: Default`

error[E0277]: the trait bound `NotDefault: Default` is not satisfied
    |
    |
375 |         assert!(matches!(<[NotDefault; 0] as Default>::default(), []));
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `NotDefault`
    |
    = note: required because of the requirements on the impl of `Send` for `std::array::array_defaults::SendToDefault<NotDefault, 0_usize>`
    = note: required because of the requirements on the impl of `Default` for `[NotDefault; 0]`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
---
warning: build failed, waiting for other jobs to finish...
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:24
