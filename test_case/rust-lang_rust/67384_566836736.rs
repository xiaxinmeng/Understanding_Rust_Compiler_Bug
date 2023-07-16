plain
2019-12-18T02:19:39.9035610Z diff of stderr:
2019-12-18T02:19:39.9035710Z 
2019-12-18T02:19:39.9036280Z 152   --> $DIR/ub-wide-ptr.rs:161:5
2019-12-18T02:19:39.9036380Z 153    |
2019-12-18T02:19:39.9036460Z 154 LL |     DynTransmute { repr2: DynRepr2 { ptr: &92, vtable: &3 } }.rust
2019-12-18T02:19:39.9037170Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Memory access failed: pointer must be in-bounds at offset 24, but is outside bounds of allocation N which has size 8
2019-12-18T02:19:39.9037910Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of allocation N which has size 8
2019-12-18T02:19:39.9038160Z 157 error: aborting due to 20 previous errors
2019-12-18T02:19:39.9038230Z 158 
2019-12-18T02:19:39.9038270Z 
2019-12-18T02:19:39.9038320Z 
2019-12-18T02:19:39.9038320Z 
2019-12-18T02:19:39.9038390Z The actual stderr differed from the expected stderr.
2019-12-18T02:19:39.9039060Z Actual stderr saved to /Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/const-eval/ub-wide-ptr/ub-wide-ptr.stderr
2019-12-18T02:19:39.9039650Z To update references, rerun the tests and pass the `--bless` flag
2019-12-18T02:19:39.9040260Z To only update this specific test, also pass `--test-args consts/const-eval/ub-wide-ptr.rs`
2019-12-18T02:19:39.9040410Z error: 1 errors occurred comparing output.
2019-12-18T02:19:39.9040480Z status: exit code: 1
2019-12-18T02:19:39.9040480Z status: exit code: 1
2019-12-18T02:19:39.9041730Z command: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=i686-apple-darwin" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/const-eval/ub-wide-ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui/consts/const-eval/ub-wide-ptr/auxiliary" "-A" "unused"
2019-12-18T02:19:39.9042810Z ------------------------------------------
2019-12-18T02:19:39.9042870Z 
2019-12-18T02:19:39.9043430Z ------------------------------------------
2019-12-18T02:19:39.9043530Z stderr:
2019-12-18T02:19:39.9043530Z stderr:
2019-12-18T02:19:39.9044070Z ------------------------------------------
2019-12-18T02:19:39.9044170Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9044760Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:84:1
2019-12-18T02:19:39.9044880Z    |
2019-12-18T02:19:39.9044980Z LL | const STR_TOO_LONG: &str = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: 999 } }.str};
2019-12-18T02:19:39.9045120Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (not entirely in bounds)
2019-12-18T02:19:39.9045240Z    |
2019-12-18T02:19:39.9045970Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9046170Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9046760Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:87:1
2019-12-18T02:19:39.9046880Z    |
2019-12-18T02:19:39.9046880Z    |
2019-12-18T02:19:39.9046960Z LL | const STR_LENGTH_PTR: &str = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.str};
2019-12-18T02:19:39.9047670Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-12-18T02:19:39.9047820Z    |
2019-12-18T02:19:39.9048660Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9048890Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9049520Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:90:1
2019-12-18T02:19:39.9049620Z    |
2019-12-18T02:19:39.9049620Z    |
2019-12-18T02:19:39.9049720Z LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.my_str};
2019-12-18T02:19:39.9050410Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-12-18T02:19:39.9050570Z    |
2019-12-18T02:19:39.9051330Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9051520Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9052220Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:94:1
2019-12-18T02:19:39.9052340Z    |
2019-12-18T02:19:39.9052340Z    |
2019-12-18T02:19:39.9052410Z LL | const STR_NO_UTF8: &str = unsafe { SliceTransmute { slice: &[0xFF] }.str };
2019-12-18T02:19:39.9053090Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized or non-UTF-8 data in str at .<deref>
2019-12-18T02:19:39.9053400Z    |
2019-12-18T02:19:39.9054140Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9054350Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9054930Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:97:1
2019-12-18T02:19:39.9055050Z    |
2019-12-18T02:19:39.9055050Z    |
2019-12-18T02:19:39.9055130Z LL | const MYSTR_NO_UTF8: &MyStr = unsafe { SliceTransmute { slice: &[0xFF] }.my_str };
2019-12-18T02:19:39.9055820Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized or non-UTF-8 data in str at .<deref>.0
2019-12-18T02:19:39.9055960Z    |
2019-12-18T02:19:39.9056650Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9056860Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9057470Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:104:1
2019-12-18T02:19:39.9057570Z    |
2019-12-18T02:19:39.9057570Z    |
2019-12-18T02:19:39.9057660Z LL | const SLICE_LENGTH_UNINIT: &[u8] = unsafe { SliceTransmute { addr: 42 }.slice};
2019-12-18T02:19:39.9057780Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered undefined pointer
2019-12-18T02:19:39.9057880Z    |
2019-12-18T02:19:39.9058600Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9058780Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9059390Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:107:1
2019-12-18T02:19:39.9059510Z    |
2019-12-18T02:19:39.9059510Z    |
2019-12-18T02:19:39.9059750Z LL | const SLICE_TOO_LONG: &[u8] = unsafe { SliceTransmute { repr: SliceRepr { ptr: &42, len: 999 } }.slice};
2019-12-18T02:19:39.9059950Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling reference (not entirely in bounds)
2019-12-18T02:19:39.9060060Z    |
2019-12-18T02:19:39.9060790Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9060990Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9061660Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:110:1
2019-12-18T02:19:39.9061790Z    |
2019-12-18T02:19:39.9061790Z    |
2019-12-18T02:19:39.9061880Z LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { SliceTransmute { bad: BadSliceRepr { ptr: &42, len: &3 } }.slice};
2019-12-18T02:19:39.9062590Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
2019-12-18T02:19:39.9062730Z    |
2019-12-18T02:19:39.9063430Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9063630Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9064220Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:114:1
2019-12-18T02:19:39.9064320Z    |
2019-12-18T02:19:39.9064320Z    |
2019-12-18T02:19:39.9064610Z LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { BoolTransmute { val: 3 }.bl }];
2019-12-18T02:19:39.9064750Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>[0], but expected something less or equal to 1
2019-12-18T02:19:39.9064870Z    |
2019-12-18T02:19:39.9065600Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9065800Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9066380Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:120:1
2019-12-18T02:19:39.9066500Z    |
2019-12-18T02:19:39.9066500Z    |
2019-12-18T02:19:39.9066590Z LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { BoolTransmute { val: 3 }.bl }, [false]);
2019-12-18T02:19:39.9066750Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.0, but expected something less or equal to 1
2019-12-18T02:19:39.9066870Z    |
2019-12-18T02:19:39.9067580Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9067790Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9068390Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:123:1
2019-12-18T02:19:39.9068490Z    |
2019-12-18T02:19:39.9068490Z    |
2019-12-18T02:19:39.9068580Z LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { BoolTransmute { val: 3 }.bl }]);
2019-12-18T02:19:39.9068720Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.1[0], but expected something less or equal to 1
2019-12-18T02:19:39.9068860Z    |
2019-12-18T02:19:39.9069690Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9069930Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9070550Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:130:1
2019-12-18T02:19:39.9070670Z    |
2019-12-18T02:19:39.9070670Z    |
2019-12-18T02:19:39.9070750Z LL | const RAW_SLICE_LENGTH_UNINIT: *const [u8] = unsafe { SliceTransmute { addr: 42 }.raw_slice};
2019-12-18T02:19:39.9070880Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered undefined pointer
2019-12-18T02:19:39.9070980Z    |
2019-12-18T02:19:39.9071700Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9071910Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9072500Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:135:1
2019-12-18T02:19:39.9072620Z    |
2019-12-18T02:19:39.9072620Z    |
2019-12-18T02:19:39.9072710Z LL | const TRAIT_OBJ_SHORT_VTABLE_1: &dyn Trait = unsafe { DynTransmute { repr: DynRepr { ptr: &92, vtable: &3 } }.rust};
2019-12-18T02:19:39.9072870Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-12-18T02:19:39.9073010Z    |
2019-12-18T02:19:39.9073700Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9074090Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9074730Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:138:1
2019-12-18T02:19:39.9074850Z    |
2019-12-18T02:19:39.9074850Z    |
2019-12-18T02:19:39.9074940Z LL | const TRAIT_OBJ_SHORT_VTABLE_2: &dyn Trait = unsafe { DynTransmute { repr2: DynRepr2 { ptr: &92, vtable: &3 } }.rust};
2019-12-18T02:19:39.9075110Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-12-18T02:19:39.9075230Z    |
2019-12-18T02:19:39.9075940Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9076160Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9076830Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:141:1
2019-12-18T02:19:39.9076930Z    |
2019-12-18T02:19:39.9076930Z    |
2019-12-18T02:19:39.9077040Z LL | const TRAIT_OBJ_INT_VTABLE: &dyn Trait = unsafe { DynTransmute { bad: BadDynRepr { ptr: &92, vtable: 3 } }.rust};
2019-12-18T02:19:39.9077200Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-12-18T02:19:39.9077320Z    |
2019-12-18T02:19:39.9078040Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9078250Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9078840Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:145:1
2019-12-18T02:19:39.9079110Z    |
2019-12-18T02:19:39.9079110Z    |
2019-12-18T02:19:39.9079250Z LL | const TRAIT_OBJ_CONTENT_INVALID: &dyn Trait = &unsafe { BoolTransmute { val: 3 }.bl };
2019-12-18T02:19:39.9079980Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 3 at .<deref>.<dyn-downcast>, but expected something less or equal to 1
2019-12-18T02:19:39.9080130Z    |
2019-12-18T02:19:39.9080820Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9081020Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9081630Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:149:1
2019-12-18T02:19:39.9081730Z    |
2019-12-18T02:19:39.9081730Z    |
2019-12-18T02:19:39.9081850Z LL | const RAW_TRAIT_OBJ_VTABLE_NULL: *const dyn Trait = unsafe { DynTransmute { bad: BadDynRepr { ptr: &92, vtable: 0 } }.raw_rust};
2019-12-18T02:19:39.9082020Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-12-18T02:19:39.9082150Z    |
2019-12-18T02:19:39.9082860Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9083070Z error[E0080]: it is undefined behavior to use this value
2019-12-18T02:19:39.9083870Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:151:1
2019-12-18T02:19:39.9083990Z    |
2019-12-18T02:19:39.9083990Z    |
2019-12-18T02:19:39.9084100Z LL | const RAW_TRAIT_OBJ_VTABLE_INVALID: *const dyn Trait = unsafe { DynTransmute { repr2: DynRepr2 { ptr: &92, vtable: &3 } }.raw_rust};
2019-12-18T02:19:39.9084270Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling or unaligned vtable pointer in wide pointer or too small vtable
2019-12-18T02:19:39.9084400Z    |
2019-12-18T02:19:39.9085110Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2019-12-18T02:19:39.9085320Z error[E0080]: could not evaluate static initializer
2019-12-18T02:19:39.9085920Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:157:5
2019-12-18T02:19:39.9086040Z    |
2019-12-18T02:19:39.9086040Z    |
2019-12-18T02:19:39.9086110Z LL |     DynTransmute { bad: BadDynRepr { ptr: &92, vtable: 0 } }.rust
2019-12-18T02:19:39.9086230Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid use of NULL pointer
2019-12-18T02:19:39.9086370Z error[E0080]: could not evaluate static initializer
2019-12-18T02:19:39.9086960Z   --> /Users/runner/runners/2.163.1/work/1/s/src/test/ui/consts/const-eval/ub-wide-ptr.rs:161:5
2019-12-18T02:19:39.9087080Z    |
2019-12-18T02:19:39.9087080Z    |
2019-12-18T02:19:39.9087150Z LL |     DynTransmute { repr2: DynRepr2 { ptr: &92, vtable: &3 } }.rust
2019-12-18T02:19:39.9087850Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of allocation 113 which has size 8
2019-12-18T02:19:39.9088040Z error: aborting due to 20 previous errors
2019-12-18T02:19:39.9088100Z 
2019-12-18T02:19:39.9088660Z For more information about this error, try `rustc --explain E0080`.
2019-12-18T02:19:39.9088720Z 
---
2019-12-18T02:19:39.9117490Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-18T02:19:39.9117670Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-18T02:19:39.9134030Z 
2019-12-18T02:19:39.9134340Z 
2019-12-18T02:19:39.9137460Z command did not execute successfully: "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/runners/2.163.1/work/1/s/src/test/ui" "--build-base" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/test/ui" "--stage-id" "stage2-i686-apple-darwin" "--mode" "ui" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.79.2\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode_9.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "9.0.0-rust-1.41.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-18T02:19:39.9138480Z 
2019-12-18T02:19:39.9138520Z 
2019-12-18T02:19:39.9147490Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-18T02:19:39.9147650Z Build completed unsuccessfully in 0:51:36
2019-12-18T02:19:39.9147650Z Build completed unsuccessfully in 0:51:36
2019-12-18T02:19:39.9206930Z == clock drift check ==
2019-12-18T02:19:39.9253270Z   local time: Wed Dec 18 02:19:39 UTC 2019
2019-12-18T02:19:39.9789420Z   network time: Wed, 18 Dec 2019 02:19:39 GMT
2019-12-18T02:19:39.9790960Z == end clock drift check ==
2019-12-18T02:19:39.9835090Z 
2019-12-18T02:19:39.9939320Z ##[error]Bash exited with code '1'.
2019-12-18T02:19:39.9989660Z ##[section]Starting: Checkout
2019-12-18T02:19:39.9991970Z ==============================================================================
2019-12-18T02:19:39.9992070Z Task         : Get sources
2019-12-18T02:19:39.9992140Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
