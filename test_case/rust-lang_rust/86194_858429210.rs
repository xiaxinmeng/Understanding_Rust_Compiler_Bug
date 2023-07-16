plain
.................................................................................................... 1900/11973
....i............................................................................................... 2000/11973
.................................................................................................... 2100/11973
.................................................................................................... 2200/11973
............................................F...............F.........F............................. 2300/11973
.................F............F.........................................F........................... 2400/11973
................................................................................F................... 2500/11973
.................................................................................................... 2600/11973
..................................FF............F................................................... 2700/11973
.................................................................................................... 2900/11973
...............................iiiii................................................................ 3000/11973
.................................................................................................... 3100/11973
.................................................................................................... 3200/11973
---
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
2   --> $DIR/dangling.rs:8:16
3    |
4 LL | / const TEST: () = { unsafe {
9 LL | |
10 LL | | } };
11    | |____-
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
16 
17 error: aborting due to previous error
18 
---
To only update this specific test, also pass `--test-args consts/const-eval/dangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/dangling.rs:8:16
   |
LL | / const TEST: () = { unsafe {
LL | |     let slice: *const [u8] = mem::transmute((1usize, usize::MAX));
LL | |     let _val = &*slice; //~ ERROR: any use of this value will cause an error
   | |                ^^^^^^^ invalid metadata in wide pointer: slice is bigger than largest supported object
LL | |     //~| slice is bigger than largest supported object
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | | } };

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
2   --> $DIR/alloc_intrinsic_errors.rs:10:17
3    |
4 LL | const FOO: i32 = foo();

10    |                 align has to be a power of 2, `3` is not a power of 2
11    |                 inside `foo` at $DIR/alloc_intrinsic_errors.rs:10:17
12    |                 inside `FOO` at $DIR/alloc_intrinsic_errors.rs:7:18
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
17 
18 error: aborting due to previous error
19 
---
To only update this specific test, also pass `--test-args consts/const-eval/heap/alloc_intrinsic_errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs:10:17
   |
LL | const FOO: i32 = foo();
...
...
LL |         let _ = intrinsics::const_allocate(4, 3) as * mut i32;
   |                 |
   |                 |
   |                 align has to be a power of 2, `3` is not a power of 2
   |                 inside `foo` at /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs:10:17
   |                 inside `FOO` at /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs:7:18
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.

---
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
2   --> $DIR/issue-49296.rs:19:16
3    |
4 LL | const X: u64 = *wat(42);
5    | ---------------^^^^^^^^-
6    |                |
6    |                |
7    |                pointer to alloc1 was dereferenced after this allocation got freed
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
12 
13 error: aborting due to previous error
14 
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-49296.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-49296.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/issue-49296.rs:19:16
   |
LL | const X: u64 = *wat(42);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                |
   |                |
   |                pointer to alloc1 was dereferenced after this allocation got freed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.

---
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
13   --> $DIR/ub-nonnull.rs:19:30
14    |
15 LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
21 LL | |     mem::transmute(out_of_bounds_ptr)
22 LL | | } };
23    | |____-
-    |
-    |
- note: the lint level is defined here
-   --> $DIR/ub-nonnull.rs:15:8
-    |
- LL | #[deny(const_err)] // this triggers a `const_err` so validation does not even happen
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
32 
33 error[E0080]: it is undefined behavior to use this value
33 error[E0080]: it is undefined behavior to use this value
34   --> $DIR/ub-nonnull.rs:24:1


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:12:1
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
   |
LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
LL | |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
LL | |     // Use address-of-element for pointer arithmetic. This could wrap around to null!
LL | |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR any use of this value will cause an error
   | |                              ^^^^^^^^ memory access failed: pointer must be in-bounds at offset 256, but is outside bounds of alloc10 which has size 1
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | |     mem::transmute(out_of_bounds_ptr)
LL | | } };

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:1
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:26:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:26:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:34:1
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:34:1
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .0, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               __                                              │ ░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:42:1
   |
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:48:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 7 previous errors

---
- warning: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
2   --> $DIR/validate_uninhabited_zsts.rs:5:14
3    |
4 LL |     unsafe { std::mem::transmute(()) }
10 ...
10 ...
11 LL | const FOO: [Empty; 3] = [foo(); 3];
-    |
- note: the lint level is defined here
-   --> $DIR/validate_uninhabited_zsts.rs:14:8
-    |
-    |
- LL | #[warn(const_err)]
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
21 
22 error[E0080]: it is undefined behavior to use this value
22 error[E0080]: it is undefined behavior to use this value
23   --> $DIR/validate_uninhabited_zsts.rs:18:1

51    |
52    = note: enums with no variants have no valid value
- error: aborting due to previous error; 3 warnings emitted
+ error: aborting due to 2 previous errors; 2 warnings emitted
55 
56 For more information about this error, try `rustc --explain E0080`.
56 For more information about this error, try `rustc --explain E0080`.
57 


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
   |
LL |     unsafe { std::mem::transmute(()) }
   |              |
   |              transmuting to uninhabited type
   |              inside `foo` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
   |              inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:15:26
   |              inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:15:26
...
LL | const FOO: [Empty; 3] = [foo(); 3];

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:18:1
   |
   |
LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Empty at [0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}

warning: the type `!` does not permit zero-initialization
   |
   |
LL |     unsafe { std::mem::transmute(()) }
   |              |
   |              |
   |              this code causes undefined behavior when executed
   |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: `#[warn(invalid_value)]` on by default
   = note: the `!` type has no valid value

warning: the type `Empty` does not permit zero-initialization
   |
   |
LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |                                   |
   |                                   |
   |                                   this code causes undefined behavior when executed
   |                                   help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
   |
   = note: enums with no variants have no valid value
error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0080`.

---
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
2   --> $DIR/const-int-unchecked.rs:15:29
3    |
4 LL | const SHL_U8: u8 = unsafe { intrinsics::unchecked_shl(5_u8, 8) };
5    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
6    |                             |
6    |                             |
7    |                             overflowing shift by 8 in `unchecked_shl`
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
12 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
14   --> $DIR/const-int-unchecked.rs:18:31
15    |
16 LL | const SHL_U16: u16 = unsafe { intrinsics::unchecked_shl(5_u16, 16) };
17    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
18    |                               |
18    |                               |
19    |                               overflowing shift by 16 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
23 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
25   --> $DIR/const-int-unchecked.rs:21:31
26    |
27 LL | const SHL_U32: u32 = unsafe { intrinsics::unchecked_shl(5_u32, 32) };
28    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
29    |                               |
29    |                               |
30    |                               overflowing shift by 32 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
34 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
36   --> $DIR/const-int-unchecked.rs:24:31
37    |
38 LL | const SHL_U64: u64 = unsafe { intrinsics::unchecked_shl(5_u64, 64) };
39    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
40    |                               |
40    |                               |
41    |                               overflowing shift by 64 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
45 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
47   --> $DIR/const-int-unchecked.rs:27:33
48    |
49 LL | const SHL_U128: u128 = unsafe { intrinsics::unchecked_shl(5_u128, 128) };
50    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
51    |                                 |
51    |                                 |
52    |                                 overflowing shift by 128 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
56 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
58   --> $DIR/const-int-unchecked.rs:33:29
59    |
60 LL | const SHL_I8: i8 = unsafe { intrinsics::unchecked_shl(5_i8, 8) };
61    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
62    |                             |
62    |                             |
63    |                             overflowing shift by 8 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
67 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
69   --> $DIR/const-int-unchecked.rs:36:31
70    |
71 LL | const SHL_I16: i16 = unsafe { intrinsics::unchecked_shl(5_16, 16) };
72    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
73    |                               |
73    |                               |
74    |                               overflowing shift by 16 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
78 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
80   --> $DIR/const-int-unchecked.rs:39:31
81    |
82 LL | const SHL_I32: i32 = unsafe { intrinsics::unchecked_shl(5_i32, 32) };
83    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
84    |                               |
84    |                               |
85    |                               overflowing shift by 32 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
89 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
91   --> $DIR/const-int-unchecked.rs:42:31
92    |
93 LL | const SHL_I64: i64 = unsafe { intrinsics::unchecked_shl(5_i64, 64) };
94    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
95    |                               |
95    |                               |
96    |                               overflowing shift by 64 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
100 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
102   --> $DIR/const-int-unchecked.rs:45:33
103    |
104 LL | const SHL_I128: i128 = unsafe { intrinsics::unchecked_shl(5_i128, 128) };
105    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
106    |                                 |
106    |                                 |
107    |                                 overflowing shift by 128 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
111 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
113   --> $DIR/const-int-unchecked.rs:51:33
114    |
115 LL | const SHL_I8_NEG: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -1) };
116    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
117    |                                 |
117    |                                 |
118    |                                 overflowing shift by 255 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
122 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
124   --> $DIR/const-int-unchecked.rs:54:35
125    |
126 LL | const SHL_I16_NEG: i16 = unsafe { intrinsics::unchecked_shl(5_16, -1) };
127    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
128    |                                   |
128    |                                   |
129    |                                   overflowing shift by 65535 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
133 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
135   --> $DIR/const-int-unchecked.rs:57:35
136    |
137 LL | const SHL_I32_NEG: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -1) };
138    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
139    |                                   |
139    |                                   |
140    |                                   overflowing shift by 4294967295 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
144 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
146   --> $DIR/const-int-unchecked.rs:60:35
147    |
148 LL | const SHL_I64_NEG: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -1) };
149    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
150    |                                   |
150    |                                   |
151    |                                   overflowing shift by 18446744073709551615 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
155 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
157   --> $DIR/const-int-unchecked.rs:63:37
158    |
159 LL | const SHL_I128_NEG: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -1) };
160    | ------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
161    |                                     |
161    |                                     |
162    |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
166 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
168   --> $DIR/const-int-unchecked.rs:70:40
169    |
170 LL | const SHL_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -6) };
171    | ---------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
172    |                                        |
172    |                                        |
173    |                                        overflowing shift by 250 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
177 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
179   --> $DIR/const-int-unchecked.rs:73:42
180    |
181 LL | const SHL_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shl(5_16, -13) };
182    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
183    |                                          |
183    |                                          |
184    |                                          overflowing shift by 65523 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
188 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
190   --> $DIR/const-int-unchecked.rs:76:42
191    |
192 LL | const SHL_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -25) };
193    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
194    |                                          |
194    |                                          |
195    |                                          overflowing shift by 4294967271 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
199 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
201   --> $DIR/const-int-unchecked.rs:79:42
202    |
203 LL | const SHL_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -30) };
204    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
205    |                                          |
205    |                                          |
206    |                                          overflowing shift by 18446744073709551586 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
210 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
212   --> $DIR/const-int-unchecked.rs:82:44
213    |
214 LL | const SHL_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -93) };
215    | -------------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
216    |                                            |
216    |                                            |
217    |                                            overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
221 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
223   --> $DIR/const-int-unchecked.rs:90:29
224    |
225 LL | const SHR_U8: u8 = unsafe { intrinsics::unchecked_shr(5_u8, 8) };
226    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
227    |                             |
227    |                             |
228    |                             overflowing shift by 8 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
232 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
234   --> $DIR/const-int-unchecked.rs:93:31
235    |
236 LL | const SHR_U16: u16 = unsafe { intrinsics::unchecked_shr(5_u16, 16) };
237    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
238    |                               |
238    |                               |
239    |                               overflowing shift by 16 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
243 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
245   --> $DIR/const-int-unchecked.rs:96:31
246    |
247 LL | const SHR_U32: u32 = unsafe { intrinsics::unchecked_shr(5_u32, 32) };
248    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
249    |                               |
249    |                               |
250    |                               overflowing shift by 32 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
254 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
256   --> $DIR/const-int-unchecked.rs:99:31
257    |
258 LL | const SHR_U64: u64 = unsafe { intrinsics::unchecked_shr(5_u64, 64) };
259    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
260    |                               |
260    |                               |
261    |                               overflowing shift by 64 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
265 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
267   --> $DIR/const-int-unchecked.rs:102:33
268    |
269 LL | const SHR_U128: u128 = unsafe { intrinsics::unchecked_shr(5_u128, 128) };
270    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
271    |                                 |
271    |                                 |
272    |                                 overflowing shift by 128 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
276 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
278   --> $DIR/const-int-unchecked.rs:108:29
279    |
280 LL | const SHR_I8: i8 = unsafe { intrinsics::unchecked_shr(5_i8, 8) };
281    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
282    |                             |
282    |                             |
283    |                             overflowing shift by 8 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
287 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
289   --> $DIR/const-int-unchecked.rs:111:31
290    |
291 LL | const SHR_I16: i16 = unsafe { intrinsics::unchecked_shr(5_16, 16) };
292    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
293    |                               |
293    |                               |
294    |                               overflowing shift by 16 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
298 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
300   --> $DIR/const-int-unchecked.rs:114:31
301    |
302 LL | const SHR_I32: i32 = unsafe { intrinsics::unchecked_shr(5_i32, 32) };
303    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
304    |                               |
304    |                               |
305    |                               overflowing shift by 32 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
309 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
311   --> $DIR/const-int-unchecked.rs:117:31
312    |
313 LL | const SHR_I64: i64 = unsafe { intrinsics::unchecked_shr(5_i64, 64) };
314    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
315    |                               |
315    |                               |
316    |                               overflowing shift by 64 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
320 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
322   --> $DIR/const-int-unchecked.rs:120:33
323    |
324 LL | const SHR_I128: i128 = unsafe { intrinsics::unchecked_shr(5_i128, 128) };
325    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
326    |                                 |
326    |                                 |
327    |                                 overflowing shift by 128 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
331 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
333   --> $DIR/const-int-unchecked.rs:126:33
334    |
335 LL | const SHR_I8_NEG: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -1) };
336    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
337    |                                 |
337    |                                 |
338    |                                 overflowing shift by 255 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
342 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
344   --> $DIR/const-int-unchecked.rs:129:35
345    |
346 LL | const SHR_I16_NEG: i16 = unsafe { intrinsics::unchecked_shr(5_16, -1) };
347    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
348    |                                   |
348    |                                   |
349    |                                   overflowing shift by 65535 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
353 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
355   --> $DIR/const-int-unchecked.rs:132:35
356    |
357 LL | const SHR_I32_NEG: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -1) };
358    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
359    |                                   |
359    |                                   |
360    |                                   overflowing shift by 4294967295 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
364 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
366   --> $DIR/const-int-unchecked.rs:135:35
367    |
368 LL | const SHR_I64_NEG: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -1) };
369    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
370    |                                   |
370    |                                   |
371    |                                   overflowing shift by 18446744073709551615 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
375 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
377   --> $DIR/const-int-unchecked.rs:138:37
378    |
379 LL | const SHR_I128_NEG: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -1) };
380    | ------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
381    |                                     |
381    |                                     |
382    |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
386 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
388   --> $DIR/const-int-unchecked.rs:145:40
389    |
390 LL | const SHR_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -6) };
391    | ---------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
392    |                                        |
392    |                                        |
393    |                                        overflowing shift by 250 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
397 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
399   --> $DIR/const-int-unchecked.rs:148:42
400    |
401 LL | const SHR_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shr(5_16, -13) };
402    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
403    |                                          |
403    |                                          |
404    |                                          overflowing shift by 65523 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
408 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
410   --> $DIR/const-int-unchecked.rs:151:42
411    |
412 LL | const SHR_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -25) };
413    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
414    |                                          |
414    |                                          |
415    |                                          overflowing shift by 4294967271 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
419 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
421   --> $DIR/const-int-unchecked.rs:154:42
422    |
423 LL | const SHR_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -30) };
---
To only update this specific test, also pass `--test-args consts/const-int-unchecked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:15:29
   |
LL | const SHL_U8: u8 = unsafe { intrinsics::unchecked_shl(5_u8, 8) };
   |                             |
   |                             |
   |                             overflowing shift by 8 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:18:31
   |
   |
LL | const SHL_U16: u16 = unsafe { intrinsics::unchecked_shl(5_u16, 16) };
   |                               |
   |                               |
   |                               overflowing shift by 16 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:21:31
   |
   |
LL | const SHL_U32: u32 = unsafe { intrinsics::unchecked_shl(5_u32, 32) };
   |                               |
   |                               |
   |                               overflowing shift by 32 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:24:31
   |
   |
LL | const SHL_U64: u64 = unsafe { intrinsics::unchecked_shl(5_u64, 64) };
   |                               |
   |                               |
   |                               overflowing shift by 64 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:27:33
   |
   |
LL | const SHL_U128: u128 = unsafe { intrinsics::unchecked_shl(5_u128, 128) };
   |                                 |
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:33:29
   |
   |
LL | const SHL_I8: i8 = unsafe { intrinsics::unchecked_shl(5_i8, 8) };
   |                             |
   |                             |
   |                             overflowing shift by 8 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:36:31
   |
   |
LL | const SHL_I16: i16 = unsafe { intrinsics::unchecked_shl(5_16, 16) };
   |                               |
   |                               |
   |                               overflowing shift by 16 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:39:31
   |
   |
LL | const SHL_I32: i32 = unsafe { intrinsics::unchecked_shl(5_i32, 32) };
   |                               |
   |                               |
   |                               overflowing shift by 32 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:42:31
   |
   |
LL | const SHL_I64: i64 = unsafe { intrinsics::unchecked_shl(5_i64, 64) };
   |                               |
   |                               |
   |                               overflowing shift by 64 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:45:33
   |
   |
LL | const SHL_I128: i128 = unsafe { intrinsics::unchecked_shl(5_i128, 128) };
   |                                 |
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:51:33
   |
   |
LL | const SHL_I8_NEG: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -1) };
   |                                 |
   |                                 |
   |                                 overflowing shift by 255 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:54:35
   |
   |
LL | const SHL_I16_NEG: i16 = unsafe { intrinsics::unchecked_shl(5_16, -1) };
   |                                   |
   |                                   |
   |                                   overflowing shift by 65535 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:57:35
   |
   |
LL | const SHL_I32_NEG: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -1) };
   |                                   |
   |                                   |
   |                                   overflowing shift by 4294967295 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:60:35
   |
   |
LL | const SHL_I64_NEG: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -1) };
   |                                   |
   |                                   |
   |                                   overflowing shift by 18446744073709551615 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:63:37
   |
   |
LL | const SHL_I128_NEG: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -1) };
   |                                     |
   |                                     |
   |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:70:40
   |
   |
LL | const SHL_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -6) };
   |                                        |
   |                                        |
   |                                        overflowing shift by 250 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:73:42
   |
   |
LL | const SHL_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shl(5_16, -13) };
   |                                          |
   |                                          |
   |                                          overflowing shift by 65523 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:76:42
   |
   |
LL | const SHL_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -25) };
   |                                          |
   |                                          |
   |                                          overflowing shift by 4294967271 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:79:42
   |
   |
LL | const SHL_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -30) };
   |                                          |
   |                                          |
   |                                          overflowing shift by 18446744073709551586 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:82:44
   |
   |
LL | const SHL_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -93) };
   |                                            |
   |                                            |
   |                                            overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shl`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:90:29
   |
   |
LL | const SHR_U8: u8 = unsafe { intrinsics::unchecked_shr(5_u8, 8) };
   |                             |
   |                             |
   |                             overflowing shift by 8 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:93:31
   |
   |
LL | const SHR_U16: u16 = unsafe { intrinsics::unchecked_shr(5_u16, 16) };
   |                               |
   |                               |
   |                               overflowing shift by 16 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:96:31
   |
   |
LL | const SHR_U32: u32 = unsafe { intrinsics::unchecked_shr(5_u32, 32) };
   |                               |
   |                               |
   |                               overflowing shift by 32 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:99:31
   |
   |
LL | const SHR_U64: u64 = unsafe { intrinsics::unchecked_shr(5_u64, 64) };
   |                               |
   |                               |
   |                               overflowing shift by 64 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:102:33
   |
   |
LL | const SHR_U128: u128 = unsafe { intrinsics::unchecked_shr(5_u128, 128) };
   |                                 |
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:108:29
   |
   |
LL | const SHR_I8: i8 = unsafe { intrinsics::unchecked_shr(5_i8, 8) };
   |                             |
   |                             |
   |                             overflowing shift by 8 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:111:31
   |
   |
LL | const SHR_I16: i16 = unsafe { intrinsics::unchecked_shr(5_16, 16) };
   |                               |
   |                               |
   |                               overflowing shift by 16 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:114:31
   |
   |
LL | const SHR_I32: i32 = unsafe { intrinsics::unchecked_shr(5_i32, 32) };
   |                               |
   |                               |
   |                               overflowing shift by 32 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:117:31
   |
   |
LL | const SHR_I64: i64 = unsafe { intrinsics::unchecked_shr(5_i64, 64) };
   |                               |
   |                               |
   |                               overflowing shift by 64 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:120:33
   |
   |
LL | const SHR_I128: i128 = unsafe { intrinsics::unchecked_shr(5_i128, 128) };
   |                                 |
   |                                 |
   |                                 overflowing shift by 128 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:126:33
   |
   |
LL | const SHR_I8_NEG: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -1) };
   |                                 |
   |                                 |
   |                                 overflowing shift by 255 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:129:35
   |
   |
LL | const SHR_I16_NEG: i16 = unsafe { intrinsics::unchecked_shr(5_16, -1) };
   |                                   |
   |                                   |
   |                                   overflowing shift by 65535 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:132:35
   |
   |
LL | const SHR_I32_NEG: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -1) };
   |                                   |
   |                                   |
   |                                   overflowing shift by 4294967295 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:135:35
   |
   |
LL | const SHR_I64_NEG: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -1) };
   |                                   |
   |                                   |
   |                                   overflowing shift by 18446744073709551615 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:138:37
   |
   |
LL | const SHR_I128_NEG: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -1) };
   |                                     |
   |                                     |
   |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:145:40
   |
   |
LL | const SHR_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -6) };
   |                                        |
   |                                        |
   |                                        overflowing shift by 250 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:148:42
   |
   |
LL | const SHR_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shr(5_16, -13) };
   |                                          |
   |                                          |
   |                                          overflowing shift by 65523 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:151:42
   |
   |
LL | const SHR_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -25) };
   |                                          |
   |                                          |
   |                                          overflowing shift by 4294967271 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:154:42
   |
   |
LL | const SHR_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -30) };
   |                                          |
   |                                          |
   |                                          overflowing shift by 18446744073709551586 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:157:44
   |
   |
LL | const SHR_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -93) };
   |                                            |
   |                                            |
   |                                            overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shr`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:163:25
   |
   |
LL | const _: u16 = unsafe { std::intrinsics::unchecked_add(40000u16, 30000) };
   |                         |
   |                         |
   |                         overflow executing `unchecked_add`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:167:25
   |
   |
LL | const _: u32 = unsafe { std::intrinsics::unchecked_sub(14u32, 22) };
   |                         |
   |                         |
   |                         overflow executing `unchecked_sub`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:171:25
   |
   |
LL | const _: u16 = unsafe { std::intrinsics::unchecked_mul(300u16, 250u16) };
   |                         |
   |                         |
   |                         overflow executing `unchecked_mul`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:175:25
   |
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(1, 0) };
   |                         |
   |                         dividing by zero

error[E0080]: any use of this value will cause an error
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:178:25
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(i32::MIN, -1) };
   |                         |
   |                         |
   |                         overflow executing `unchecked_div`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:182:25
   |
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(1, 0) };
   |                         |
   |                         |
   |                         calculating the remainder with a divisor of zero
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:185:25
   |
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(i32::MIN, -1) };
   |                         |
   |                         |
   |                         overflow executing `unchecked_rem`
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:191:25
   |
   |
LL | const _: u32 = unsafe { std::intrinsics::ctlz_nonzero(0) };
   |                         |
   |                         |
   |                         `ctlz_nonzero` called on 0
error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:194:25
   |
   |
LL | const _: u32 = unsafe { std::intrinsics::cttz_nonzero(0) };
   |                         |
   |                         |
   |                         `cttz_nonzero` called on 0
error: aborting due to 49 previous errors

For more information about this error, try `rustc --explain E0080`.


------------------------------------------


---- [ui] ui/consts/const_unsafe_unreachable_ub.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL |     unsafe { intrinsics::unreachable() }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              entering unreachable code
   |              inside `unreachable_unchecked` at /checkout/library/core/src/hint.rs:51:14
   |              inside `foo` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:8:18
   |              inside `BAR` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:13:28
  ::: /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:13:1
   |
   |
LL | const BAR: bool = unsafe { foo(false) };

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
2   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
3    |
4 LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
18 LL | |     offset as usize
19 LL | | };
20    | |__-
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
25 
26 error: any use of this value will cause an error
27   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
27   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

41 LL | | };
42    | |__-
43    |
+    = note: `#[deny(const_err)]` on by default
45    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
46 

- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
48   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
49    |
50 LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }

64 LL | |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
65 LL | | };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
70 
70 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
72   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
73    |
74 LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
86 LL | |     unsafe { ptr.offset_from(ptr) }
87 LL | | };
88    | |__-
-    |
---
118 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    |
   |                    ptr_offset_from cannot compute offset of pointers into different allocations.
   |                    inside `ptr::const_ptr::<impl *const Struct>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |                    inside `DIFFERENT_ALLOC` at /checkout/src/test/ui/consts/offset_from_ub.rs:16:27
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:10:1
   |
LL | / pub const DIFFERENT_ALLOC: usize = {
LL | / pub const DIFFERENT_ALLOC: usize = {
LL | |     //~^ NOTE
LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
LL | |     offset as usize
LL | | };
   | |__-


error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    unable to turn bytes into a pointer
   |                    unable to turn bytes into a pointer
   |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |                    inside `NOT_PTR` at /checkout/src/test/ui/consts/offset_from_ub.rs:22:14
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:20:1
   |
   |
LL | / pub const NOT_PTR: usize = {
LL | |     //~^ NOTE
LL | |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
LL | | };
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    |
   |                    exact_div: 1_isize cannot be divided by 2_isize without remainder
   |                    inside `ptr::const_ptr::<impl *const u16>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |                    inside `NOT_MULTIPLE_OF_SIZE` at /checkout/src/test/ui/consts/offset_from_ub.rs:30:14
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:25:1
   |
   |
LL | / pub const NOT_MULTIPLE_OF_SIZE: isize = {
LL | |     //~^ NOTE
LL | |     let data = [5u8, 6, 7];
LL | |     let base_ptr = data.as_ptr();
LL | |     let field_ptr = &data[1] as *const u8 as *const u16;
LL | |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
LL | | };

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    null pointer is not a valid pointer for this operation
   |                    null pointer is not a valid pointer for this operation
   |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |                    inside `OFFSET_FROM_NULL` at /checkout/src/test/ui/consts/offset_from_ub.rs:36:14
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:33:1
   |
   |
LL | / pub const OFFSET_FROM_NULL: isize = {
LL | |     //~^ NOTE
LL | |     let ptr = 0 as *const u8;
LL | |     unsafe { ptr.offset_from(ptr) }
LL | | };

error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    unable to turn bytes into a pointer
   |                    unable to turn bytes into a pointer
   |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |                    inside `DIFFERENT_INT` at /checkout/src/test/ui/consts/offset_from_ub.rs:43:14
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:39:1
   |
   |
LL | / pub const DIFFERENT_INT: isize = { // offset_from with two different integers: like DIFFERENT_ALLOC
LL | |     //~^ NOTE
LL | |     let ptr1 = 8 as *const u8;
LL | |     let ptr2 = 16 as *const u8;
LL | |     unsafe { ptr2.offset_from(ptr1) }
LL | | };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

---
3    |
4 LL |         unsafe { intrinsics::offset(self, count) }

12    |
13 LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) };
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
19 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
21   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
22    |
23 LL |         unsafe { intrinsics::offset(self, count) }

31    |
32 LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
37 
37 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
39   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
40    |
41 LL |         unsafe { intrinsics::offset(self, count) }

49    |
50 LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
55 
55 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
57   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
58    |
59 LL |         unsafe { intrinsics::offset(self, count) }

67    |
68 LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
73 
73 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
75   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
76    |
77 LL |         unsafe { intrinsics::offset(self, count) }

85    |
86 LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
91 
91 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
93   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
94    |
95 LL |         unsafe { intrinsics::offset(self, count) }

103    |
104 LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
109 
109 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
111   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
112    |
113 LL |         unsafe { intrinsics::offset(self, count) }

121    |
122 LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
127 
127 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
129   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
130    |
131 LL |         unsafe { intrinsics::offset(self, count) }

139    |
140 LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
145 
145 
146 error: any use of this value will cause an error
147   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL

158 LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) };
160    |
160    |
+    = note: `#[deny(const_err)]` on by default
162    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
163 

- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
165   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
166    |
167 LL |         unsafe { intrinsics::offset(self, count) }

175    |
176 LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
181 
---
202 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/offset_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `BEFORE_START` at /checkout/src/test/ui/consts/offset_ub.rs:6:46
  ::: /checkout/src/test/ui/consts/offset_ub.rs:6:1
   |
   |
LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) }; //~NOTE

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: pointer must be in-bounds at offset 2, but is outside bounds of alloc5 which has size 1
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `AFTER_END` at /checkout/src/test/ui/consts/offset_ub.rs:7:43
  ::: /checkout/src/test/ui/consts/offset_ub.rs:7:1
   |
   |
LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) }; //~NOTE

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: pointer must be in-bounds at offset 101, but is outside bounds of alloc8 which has size 100
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `AFTER_ARRAY` at /checkout/src/test/ui/consts/offset_ub.rs:8:45
  ::: /checkout/src/test/ui/consts/offset_ub.rs:8:1
   |
   |
LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) }; //~NOTE

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `OVERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:10:43
  ::: /checkout/src/test/ui/consts/offset_ub.rs:10:1
   |
   |
LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) }; //~NOTE

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `UNDERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:11:44
  ::: /checkout/src/test/ui/consts/offset_ub.rs:11:1
   |
   |
LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) }; //~NOTE

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `OVERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:12:56
  ::: /checkout/src/test/ui/consts/offset_ub.rs:12:1
   |
   |
LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) }; //~NOTE

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `UNDERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:13:57
  ::: /checkout/src/test/ui/consts/offset_ub.rs:13:1
   |
   |
LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) }; //~NOTE

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: pointer must be in-bounds at offset 1, but is outside bounds of alloc23 which has size 0
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `ZERO_SIZED_ALLOC` at /checkout/src/test/ui/consts/offset_ub.rs:15:50
  ::: /checkout/src/test/ui/consts/offset_ub.rs:15:1
   |
   |
LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/mut_ptr.rs:242:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) as *mut T }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  unable to turn bytes into a pointer
   |                  inside `ptr::mut_ptr::<impl *mut u8>::offset` at /checkout/library/core/src/ptr/mut_ptr.rs:242:18
   |                  inside `DANGLING` at /checkout/src/test/ui/consts/offset_ub.rs:16:42
  ::: /checkout/src/test/ui/consts/offset_ub.rs:16:1
   |
   |
LL | pub const DANGLING: *const u8 = unsafe { ptr::NonNull::<u8>::dangling().as_ptr().offset(4) }; //~NOTE
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: 0x0 is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `NULL_OFFSET_ZERO` at /checkout/src/test/ui/consts/offset_ub.rs:19:50
  ::: /checkout/src/test/ui/consts/offset_ub.rs:19:1
   |
   |
LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) }; //~NOTE

error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  unable to turn bytes into a pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `UNDERFLOW_ABS` at /checkout/src/test/ui/consts/offset_ub.rs:22:47
  ::: /checkout/src/test/ui/consts/offset_ub.rs:22:1
   |
   |
LL | pub const UNDERFLOW_ABS: *const u8 = unsafe { (usize::MAX as *const u8).offset(isize::MIN) }; //~NOTE
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

---
3    |
4 LL |         unsafe { intrinsics::offset(self, count) }

12    |
13 LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
19 
- error: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
+ error[E0080]: any use of this value will cause an error
21   --> $DIR/ptr_comparisons.rs:64:33
22    |
23 LL | / const _: *const u8 =

25    | |_________________________________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^___-
26    |                                   |
27    |                                   memory access failed: pointer must be in-bounds at offset 1000, but is outside bounds of alloc2 which has size $WORD
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
31 
32 error: any use of this value will cause an error
32 error: any use of this value will cause an error
33   --> $DIR/ptr_comparisons.rs:68:27

37    |                           |
38    |                           cannot cast pointer to integer because it was not created by cast from integer
39    |
+    = note: `#[deny(const_err)]` on by default
41    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
42 

53 
---
To only update this specific test, also pass `--test-args consts/ptr_comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: pointer must be in-bounds at offset 16, but is outside bounds of alloc2 which has size 8
   |                  inside `ptr::const_ptr::<impl *const usize>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |                  inside `_` at /checkout/src/test/ui/consts/ptr_comparisons.rs:61:34
  ::: /checkout/src/test/ui/consts/ptr_comparisons.rs:61:1
   |
   |
LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };

error[E0080]: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:64:33
   |
   |
LL | / const _: *const u8 =
LL | |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
   | |_________________________________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^___-
   |                                   memory access failed: pointer must be in-bounds at offset 1000, but is outside bounds of alloc2 which has size 8

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:68:27
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:68:27
   |
LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
   |                           |
   |                           |
   |                           cannot cast pointer to integer because it was not created by cast from integer
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:73:27
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:73:27
   |
LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
   |                           |
   |                           |
   |                           cannot cast pointer to integer because it was not created by cast from integer
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 4 previous errors
---
test result: FAILED. 11862 passed; 10 failed; 101 ignored; 0 measured; 0 filtered out; finished in 120.92s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:43
