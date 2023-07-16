plain
.................................................................................................... 1900/11994
.................i.................................................................................. 2000/11994
.................................................................................................... 2100/11994
.................................................................................................... 2200/11994
............................................................F...........F..........F................ 2300/11994
.............................F..F...............F.......................................F........... 2400/11994
...............................................................................................F.... 2500/11994
.................................................................................................... 2600/11994
..................................................FF...................F............................ 2700/11994
.................................................................................................... 2900/11994
................................................iiiii............................................... 3000/11994
.................................................................................................... 3100/11994
.................................................................................................... 3200/11994
---
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/dangling.rs:8:16
3    |
- LL | / const TEST: () = { unsafe {
- LL | |     let slice: *const [u8] = mem::transmute((1usize, usize::MAX));
- LL | |     let _val = &*slice;
-    | |                ^^^^^^^ invalid metadata in wide pointer: slice is bigger than largest supported object
- LL | |
- LL | |
- LL | | } };
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     let _val = &*slice;
+    |                ^^^^^^^ invalid metadata in wide pointer: slice is bigger than largest supported object
17 error: aborting due to previous error
18 

+ For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/const-eval/dangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dangling/auxiliary"
------------------------------------------

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/dangling.rs:8:16
   |
LL |     let _val = &*slice; //~ ERROR: any use of this value will cause an error
   |                ^^^^^^^ invalid metadata in wide pointer: slice is bigger than largest supported object
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.

---
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/alloc_intrinsic_errors.rs:10:17
3    |
4 LL | const FOO: i32 = foo();
-    | -----------------------
+    |                  ----- inside `FOO` at $DIR/alloc_intrinsic_errors.rs:7:18
6 ...
6 ...
7 LL |         let _ = intrinsics::const_allocate(4, 3) as * mut i32;

9    |                 |
9    |                 |
10    |                 align has to be a power of 2, `3` is not a power of 2
11    |                 inside `foo` at $DIR/alloc_intrinsic_errors.rs:10:17
-    |                 inside `FOO` at $DIR/alloc_intrinsic_errors.rs:7:18
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs:10:17
   |
LL | const FOO: i32 = foo();
   |                  ----- inside `FOO` at /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs:7:18
...
LL |         let _ = intrinsics::const_allocate(4, 3) as * mut i32;
   |                 |
   |                 |
   |                 align has to be a power of 2, `3` is not a power of 2
   |                 inside `foo` at /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_errors.rs:10:17
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.

---
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/issue-49296.rs:19:16
3    |
4 LL | const X: u64 = *wat(42);
-    | ---------------^^^^^^^^-
-    |                |
-    |                |
-    |                pointer to alloc1 was dereferenced after this allocation got freed
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                ^^^^^^^^ pointer to alloc1 was dereferenced after this allocation got freed
12 
13 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-49296.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-49296.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/issue-49296.rs:19:16
   |
LL | const X: u64 = *wat(42);
   |                ^^^^^^^^ pointer to alloc1 was dereferenced after this allocation got freed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.

---
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/ub-incorrect-vtable.rs:19:14
3    |
- LL | / const INVALID_VTABLE_ALIGNMENT: &dyn Trait =
- LL | |     unsafe { std::mem::transmute((&92u8, &[0usize, 1usize, 1000usize])) };
-    | |______________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^__-
-    |                |
-    |                invalid vtable: alignment `1000` is not a power of 2
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     unsafe { std::mem::transmute((&92u8, &[0usize, 1usize, 1000usize])) };
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid vtable: alignment `1000` is not a power of 2
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
15   --> $DIR/ub-incorrect-vtable.rs:25:14
16    |
16    |
- LL | / const INVALID_VTABLE_SIZE: &dyn Trait =
- LL | |     unsafe { std::mem::transmute((&92u8, &[1usize, usize::MAX, 1usize])) };
-    | |______________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^__-
-    |                |
-    |                invalid vtable: size is bigger than largest supported object
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     unsafe { std::mem::transmute((&92u8, &[1usize, usize::MAX, 1usize])) };
+    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid vtable: size is bigger than largest supported object
26 error[E0080]: it is undefined behavior to use this value
27   --> $DIR/ub-incorrect-vtable.rs:36:1



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-incorrect-vtable/ub-incorrect-vtable.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-incorrect-vtable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-incorrect-vtable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-incorrect-vtable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-incorrect-vtable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-incorrect-vtable.rs:19:14
   |
LL |     unsafe { std::mem::transmute((&92u8, &[0usize, 1usize, 1000usize])) };
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid vtable: alignment `1000` is not a power of 2
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-incorrect-vtable.rs:25:14
   |
   |
LL |     unsafe { std::mem::transmute((&92u8, &[1usize, usize::MAX, 1usize])) };
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid vtable: size is bigger than largest supported object
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-incorrect-vtable.rs:36:1
   |
   |
LL | / const INVALID_VTABLE_ALIGNMENT_UB: W<&dyn Trait> =
LL | |     unsafe { std::mem::transmute((&92u8, &(drop_me as fn(*mut usize), 1usize, 1000usize))) };
   | |_____________________________________________________________________________________________^ type validation failed at .0: encountered invalid vtable: alignment `1000` is not a power of 2
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc17───────╼ ╾───────alloc20───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-incorrect-vtable.rs:41:1
   |
   |
LL | / const INVALID_VTABLE_SIZE_UB: W<&dyn Trait> =
LL | |     unsafe { std::mem::transmute((&92u8, &(drop_me as fn(*mut usize), usize::MAX, 1usize))) };
   | |______________________________________________________________________________________________^ type validation failed at .0: encountered invalid vtable: size is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc25───────╼ ╾───────alloc27───────╼ │ ╾──────╼╾──────╼

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0080`.
---
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
13   --> $DIR/ub-nonnull.rs:19:30
14    |
- LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
- LL | |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
- LL | |     // Use address-of-element for pointer arithmetic. This could wrap around to null!
- LL | |     let out_of_bounds_ptr = &ptr[255];
- LL | |
- LL | |     mem::transmute(out_of_bounds_ptr)
- LL | |     mem::transmute(out_of_bounds_ptr)
- LL | | } };
-    |
- note: the lint level is defined here
-   --> $DIR/ub-nonnull.rs:15:8
-    |
-    |
- LL | #[deny(const_err)] // this triggers a `const_err` so validation does not even happen
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     let out_of_bounds_ptr = &ptr[255];
32 
33 error[E0080]: it is undefined behavior to use this value
34   --> $DIR/ub-nonnull.rs:24:1



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
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

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:19:30
   |
LL |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR any use of this value will cause an error

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
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
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
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/validate_uninhabited_zsts.rs:5:14
3    |
4 LL |     unsafe { std::mem::transmute(()) }
6    |              |
7    |              transmuting to uninhabited type
8    |              inside `foo` at $DIR/validate_uninhabited_zsts.rs:5:14
-    |              inside `FOO` at $DIR/validate_uninhabited_zsts.rs:15:26
-    |              inside `FOO` at $DIR/validate_uninhabited_zsts.rs:15:26
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
+    |                          ----- inside `FOO` at $DIR/validate_uninhabited_zsts.rs:15:26
21 
21 
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
   |
LL |     unsafe { std::mem::transmute(()) }
   |              |
   |              transmuting to uninhabited type
   |              inside `foo` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
...
...
LL | const FOO: [Empty; 3] = [foo(); 3];
   |                          ----- inside `FOO` at /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:15:26
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:18:1
   |
   |
LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at [0]: encountered a value of uninhabited type Empty
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
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/const-int-unchecked.rs:15:29
3    |
4 LL | const SHL_U8: u8 = unsafe { intrinsics::unchecked_shl(5_u8, 8) };
-    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                             |
-    |                             |
-    |                             overflowing shift by 8 in `unchecked_shl`
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
14   --> $DIR/const-int-unchecked.rs:18:31
15    |
15    |
16 LL | const SHL_U16: u16 = unsafe { intrinsics::unchecked_shl(5_u16, 16) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 16 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
25   --> $DIR/const-int-unchecked.rs:21:31
26    |
26    |
27 LL | const SHL_U32: u32 = unsafe { intrinsics::unchecked_shl(5_u32, 32) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 32 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
36   --> $DIR/const-int-unchecked.rs:24:31
37    |
37    |
38 LL | const SHL_U64: u64 = unsafe { intrinsics::unchecked_shl(5_u64, 64) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 64 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
47   --> $DIR/const-int-unchecked.rs:27:33
48    |
48    |
49 LL | const SHL_U128: u128 = unsafe { intrinsics::unchecked_shl(5_u128, 128) };
-    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                 |
-    |                                 |
-    |                                 overflowing shift by 128 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
58   --> $DIR/const-int-unchecked.rs:33:29
59    |
59    |
60 LL | const SHL_I8: i8 = unsafe { intrinsics::unchecked_shl(5_i8, 8) };
-    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                             |
-    |                             |
-    |                             overflowing shift by 8 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
69   --> $DIR/const-int-unchecked.rs:36:31
70    |
70    |
71 LL | const SHL_I16: i16 = unsafe { intrinsics::unchecked_shl(5_16, 16) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 16 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
80   --> $DIR/const-int-unchecked.rs:39:31
81    |
81    |
82 LL | const SHL_I32: i32 = unsafe { intrinsics::unchecked_shl(5_i32, 32) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 32 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
91   --> $DIR/const-int-unchecked.rs:42:31
92    |
92    |
93 LL | const SHL_I64: i64 = unsafe { intrinsics::unchecked_shl(5_i64, 64) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 64 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
102   --> $DIR/const-int-unchecked.rs:45:33
103    |
103    |
104 LL | const SHL_I128: i128 = unsafe { intrinsics::unchecked_shl(5_i128, 128) };
-    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                 |
-    |                                 |
-    |                                 overflowing shift by 128 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
113   --> $DIR/const-int-unchecked.rs:51:33
114    |
114    |
115 LL | const SHL_I8_NEG: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -1) };
-    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                 |
-    |                                 |
-    |                                 overflowing shift by 255 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 255 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
124   --> $DIR/const-int-unchecked.rs:54:35
125    |
125    |
126 LL | const SHL_I16_NEG: i16 = unsafe { intrinsics::unchecked_shl(5_16, -1) };
-    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                   |
-    |                                   |
-    |                                   overflowing shift by 65535 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65535 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
135   --> $DIR/const-int-unchecked.rs:57:35
136    |
136    |
137 LL | const SHL_I32_NEG: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -1) };
-    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                   |
-    |                                   |
-    |                                   overflowing shift by 4294967295 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967295 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
146   --> $DIR/const-int-unchecked.rs:60:35
147    |
147    |
148 LL | const SHL_I64_NEG: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -1) };
-    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                   |
-    |                                   |
-    |                                   overflowing shift by 18446744073709551615 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551615 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
157   --> $DIR/const-int-unchecked.rs:63:37
158    |
158    |
159 LL | const SHL_I128_NEG: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -1) };
-    | ------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                     |
-    |                                     |
-    |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
168   --> $DIR/const-int-unchecked.rs:70:40
169    |
169    |
170 LL | const SHL_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -6) };
-    | ---------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                        |
-    |                                        |
-    |                                        overflowing shift by 250 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 250 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
179   --> $DIR/const-int-unchecked.rs:73:42
180    |
180    |
181 LL | const SHL_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shl(5_16, -13) };
-    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                          |
-    |                                          |
-    |                                          overflowing shift by 65523 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65523 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
190   --> $DIR/const-int-unchecked.rs:76:42
191    |
191    |
192 LL | const SHL_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -25) };
-    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                          |
-    |                                          |
-    |                                          overflowing shift by 4294967271 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967271 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
201   --> $DIR/const-int-unchecked.rs:79:42
202    |
202    |
203 LL | const SHL_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -30) };
-    | -----------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                          |
-    |                                          |
-    |                                          overflowing shift by 18446744073709551586 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551586 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
212   --> $DIR/const-int-unchecked.rs:82:44
213    |
213    |
214 LL | const SHL_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -93) };
-    | -------------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                            |
-    |                                            |
-    |                                            overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shl`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shl`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
223   --> $DIR/const-int-unchecked.rs:90:29
224    |
224    |
225 LL | const SHR_U8: u8 = unsafe { intrinsics::unchecked_shr(5_u8, 8) };
-    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                             |
-    |                             |
-    |                             overflowing shift by 8 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
234   --> $DIR/const-int-unchecked.rs:93:31
235    |
235    |
236 LL | const SHR_U16: u16 = unsafe { intrinsics::unchecked_shr(5_u16, 16) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 16 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
245   --> $DIR/const-int-unchecked.rs:96:31
246    |
246    |
247 LL | const SHR_U32: u32 = unsafe { intrinsics::unchecked_shr(5_u32, 32) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 32 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
256   --> $DIR/const-int-unchecked.rs:99:31
257    |
257    |
258 LL | const SHR_U64: u64 = unsafe { intrinsics::unchecked_shr(5_u64, 64) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 64 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
267   --> $DIR/const-int-unchecked.rs:102:33
268    |
268    |
269 LL | const SHR_U128: u128 = unsafe { intrinsics::unchecked_shr(5_u128, 128) };
-    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                 |
-    |                                 |
-    |                                 overflowing shift by 128 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
278   --> $DIR/const-int-unchecked.rs:108:29
279    |
279    |
280 LL | const SHR_I8: i8 = unsafe { intrinsics::unchecked_shr(5_i8, 8) };
-    | ----------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                             |
-    |                             |
-    |                             overflowing shift by 8 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
289   --> $DIR/const-int-unchecked.rs:111:31
290    |
290    |
291 LL | const SHR_I16: i16 = unsafe { intrinsics::unchecked_shr(5_16, 16) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 16 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
300   --> $DIR/const-int-unchecked.rs:114:31
301    |
301    |
302 LL | const SHR_I32: i32 = unsafe { intrinsics::unchecked_shr(5_i32, 32) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 32 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
311   --> $DIR/const-int-unchecked.rs:117:31
312    |
312    |
313 LL | const SHR_I64: i64 = unsafe { intrinsics::unchecked_shr(5_i64, 64) };
-    | ------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                               |
-    |                               |
-    |                               overflowing shift by 64 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
322   --> $DIR/const-int-unchecked.rs:120:33
323    |
323    |
324 LL | const SHR_I128: i128 = unsafe { intrinsics::unchecked_shr(5_i128, 128) };
-    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                 |
-    |                                 |
-    |                                 overflowing shift by 128 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
333   --> $DIR/const-int-unchecked.rs:126:33
334    |
334    |
335 LL | const SHR_I8_NEG: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -1) };
-    | --------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                 |
-    |                                 |
-    |                                 overflowing shift by 255 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 255 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
344   --> $DIR/const-int-unchecked.rs:129:35
345    |
345    |
346 LL | const SHR_I16_NEG: i16 = unsafe { intrinsics::unchecked_shr(5_16, -1) };
-    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                   |
-    |                                   |
-    |                                   overflowing shift by 65535 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65535 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
355   --> $DIR/const-int-unchecked.rs:132:35
356    |
356    |
357 LL | const SHR_I32_NEG: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -1) };
-    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                   |
-    |                                   |
-    |                                   overflowing shift by 4294967295 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967295 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
366   --> $DIR/const-int-unchecked.rs:135:35
367    |
367    |
368 LL | const SHR_I64_NEG: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -1) };
-    | ----------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                   |
-    |                                   |
-    |                                   overflowing shift by 18446744073709551615 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551615 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
377   --> $DIR/const-int-unchecked.rs:138:37
378    |
378    |
379 LL | const SHR_I128_NEG: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -1) };
-    | ------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                     |
-    |                                     |
-    |                                     overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shr`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shr`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
388   --> $DIR/const-int-unchecked.rs:145:40
389    |
389    |
390 LL | const SHR_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -6) };
-    | ---------------------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                                        |
-    |                                        |
-    |                                        overflowing shift by 250 in `unchecked_shr`
---
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
487   --> $DIR/const-int-unchecked.rs:178:25
488    |
489 LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(i32::MIN, -1) };
-    | ------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                         |
-    |                         |
-    |                         overflow executing `unchecked_div`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_div`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
498   --> $DIR/const-int-unchecked.rs:182:25
499    |
499    |
500 LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(1, 0) };
-    | ------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                         |
-    |                         |
-    |                         calculating the remainder with a divisor of zero
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ calculating the remainder with a divisor of zero
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
509   --> $DIR/const-int-unchecked.rs:185:25
510    |
510    |
511 LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(i32::MIN, -1) };
-    | ------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                         |
-    |                         |
-    |                         overflow executing `unchecked_rem`
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_rem`
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
520   --> $DIR/const-int-unchecked.rs:191:25
521    |
521    |
522 LL | const _: u32 = unsafe { std::intrinsics::ctlz_nonzero(0) };
-    | ------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                         |
-    |                         |
-    |                         `ctlz_nonzero` called on 0
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ctlz_nonzero` called on 0
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
531   --> $DIR/const-int-unchecked.rs:194:25
532    |
532    |
533 LL | const _: u32 = unsafe { std::intrinsics::cttz_nonzero(0) };
-    | ------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
-    |                         |
-    |                         |
-    |                         `cttz_nonzero` called on 0
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `cttz_nonzero` called on 0
541 error: aborting due to 49 previous errors
542 

+ For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args consts/const-int-unchecked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:15:29
   |
LL | const SHL_U8: u8 = unsafe { intrinsics::unchecked_shl(5_u8, 8) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:18:31
   |
   |
LL | const SHL_U16: u16 = unsafe { intrinsics::unchecked_shl(5_u16, 16) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:21:31
   |
   |
LL | const SHL_U32: u32 = unsafe { intrinsics::unchecked_shl(5_u32, 32) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:24:31
   |
   |
LL | const SHL_U64: u64 = unsafe { intrinsics::unchecked_shl(5_u64, 64) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:27:33
   |
   |
LL | const SHL_U128: u128 = unsafe { intrinsics::unchecked_shl(5_u128, 128) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:33:29
   |
   |
LL | const SHL_I8: i8 = unsafe { intrinsics::unchecked_shl(5_i8, 8) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:36:31
   |
   |
LL | const SHL_I16: i16 = unsafe { intrinsics::unchecked_shl(5_16, 16) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:39:31
   |
   |
LL | const SHL_I32: i32 = unsafe { intrinsics::unchecked_shl(5_i32, 32) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:42:31
   |
   |
LL | const SHL_I64: i64 = unsafe { intrinsics::unchecked_shl(5_i64, 64) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:45:33
   |
   |
LL | const SHL_I128: i128 = unsafe { intrinsics::unchecked_shl(5_i128, 128) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:51:33
   |
   |
LL | const SHL_I8_NEG: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -1) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 255 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:54:35
   |
   |
LL | const SHL_I16_NEG: i16 = unsafe { intrinsics::unchecked_shl(5_16, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65535 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:57:35
   |
   |
LL | const SHL_I32_NEG: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967295 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:60:35
   |
   |
LL | const SHL_I64_NEG: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551615 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:63:37
   |
   |
LL | const SHL_I128_NEG: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -1) };
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:70:40
   |
   |
LL | const SHL_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shl(5_i8, -6) };
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 250 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:73:42
   |
   |
LL | const SHL_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shl(5_16, -13) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65523 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:76:42
   |
   |
LL | const SHL_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shl(5_i32, -25) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967271 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:79:42
   |
   |
LL | const SHL_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shl(5_i64, -30) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551586 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:82:44
   |
   |
LL | const SHL_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shl(5_i128, -93) };
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shl`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:90:29
   |
   |
LL | const SHR_U8: u8 = unsafe { intrinsics::unchecked_shr(5_u8, 8) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:93:31
   |
   |
LL | const SHR_U16: u16 = unsafe { intrinsics::unchecked_shr(5_u16, 16) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:96:31
   |
   |
LL | const SHR_U32: u32 = unsafe { intrinsics::unchecked_shr(5_u32, 32) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:99:31
   |
   |
LL | const SHR_U64: u64 = unsafe { intrinsics::unchecked_shr(5_u64, 64) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:102:33
   |
   |
LL | const SHR_U128: u128 = unsafe { intrinsics::unchecked_shr(5_u128, 128) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:108:29
   |
   |
LL | const SHR_I8: i8 = unsafe { intrinsics::unchecked_shr(5_i8, 8) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 8 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:111:31
   |
   |
LL | const SHR_I16: i16 = unsafe { intrinsics::unchecked_shr(5_16, 16) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 16 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:114:31
   |
   |
LL | const SHR_I32: i32 = unsafe { intrinsics::unchecked_shr(5_i32, 32) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 32 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:117:31
   |
   |
LL | const SHR_I64: i64 = unsafe { intrinsics::unchecked_shr(5_i64, 64) };
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 64 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:120:33
   |
   |
LL | const SHR_I128: i128 = unsafe { intrinsics::unchecked_shr(5_i128, 128) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 128 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:126:33
   |
   |
LL | const SHR_I8_NEG: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -1) };
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 255 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:129:35
   |
   |
LL | const SHR_I16_NEG: i16 = unsafe { intrinsics::unchecked_shr(5_16, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65535 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:132:35
   |
   |
LL | const SHR_I32_NEG: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967295 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:135:35
   |
   |
LL | const SHR_I64_NEG: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -1) };
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551615 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:138:37
   |
   |
LL | const SHR_I128_NEG: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -1) };
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211455 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:145:40
   |
   |
LL | const SHR_I8_NEG_RANDOM: i8 = unsafe { intrinsics::unchecked_shr(5_i8, -6) };
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 250 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:148:42
   |
   |
LL | const SHR_I16_NEG_RANDOM: i16 = unsafe { intrinsics::unchecked_shr(5_16, -13) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 65523 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:151:42
   |
   |
LL | const SHR_I32_NEG_RANDOM: i32 = unsafe { intrinsics::unchecked_shr(5_i32, -25) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 4294967271 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:154:42
   |
   |
LL | const SHR_I64_NEG_RANDOM: i64 = unsafe { intrinsics::unchecked_shr(5_i64, -30) };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 18446744073709551586 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:157:44
   |
   |
LL | const SHR_I128_NEG_RANDOM: i128 = unsafe { intrinsics::unchecked_shr(5_i128, -93) };
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflowing shift by 340282366920938463463374607431768211363 in `unchecked_shr`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:163:25
   |
   |
LL | const _: u16 = unsafe { std::intrinsics::unchecked_add(40000u16, 30000) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_add`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:167:25
   |
   |
LL | const _: u32 = unsafe { std::intrinsics::unchecked_sub(14u32, 22) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_sub`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:171:25
   |
   |
LL | const _: u16 = unsafe { std::intrinsics::unchecked_mul(300u16, 250u16) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_mul`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:175:25
   |
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(1, 0) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dividing by zero
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:178:25
   |
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_div(i32::MIN, -1) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_div`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:182:25
   |
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(1, 0) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ calculating the remainder with a divisor of zero
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:185:25
   |
   |
LL | const _: i32 = unsafe { std::intrinsics::unchecked_rem(i32::MIN, -1) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow executing `unchecked_rem`
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:191:25
   |
   |
LL | const _: u32 = unsafe { std::intrinsics::ctlz_nonzero(0) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `ctlz_nonzero` called on 0
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-int-unchecked.rs:194:25
   |
   |
LL | const _: u32 = unsafe { std::intrinsics::cttz_nonzero(0) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `cttz_nonzero` called on 0
error: aborting due to 49 previous errors

For more information about this error, try `rustc --explain E0080`.


------------------------------------------


---- [ui] ui/consts/const_unsafe_unreachable_ub.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL |     unsafe { intrinsics::unreachable() }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              entering unreachable code
   |              inside `unreachable_unchecked` at /checkout/library/core/src/hint.rs:51:14
  ::: /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:8:18
   |
LL |         false => std::hint::unreachable_unchecked(),
   |                  ---------------------------------- inside `foo` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:8:18
   |                  ---------------------------------- inside `foo` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:8:18
...
LL | const BAR: bool = unsafe { foo(false) };
   |                            ---------- inside `BAR` at /checkout/src/test/ui/consts/const_unsafe_unreachable_ub.rs:13:28
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.

---
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
2   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
3    |
- LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
-    |                    |
-    |                    |
-    |                    ptr_offset_from cannot compute offset of pointers into different allocations.
-    |                    inside `ptr::const_ptr::<impl *const Struct>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                    inside `DIFFERENT_ALLOC` at $DIR/offset_from_ub.rs:16:27
+ LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                  |
+    |                  |
+    |                  ptr_offset_from cannot compute offset of pointers into different allocations.
+    |                  inside `ptr::const_ptr::<impl *const Struct>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
10    | 
-   ::: $DIR/offset_from_ub.rs:10:1
+   ::: $DIR/offset_from_ub.rs:16:27
- LL | / pub const DIFFERENT_ALLOC: usize = {
- LL | |
- LL | |
- LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
- LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
- LL | |     offset as usize
- LL | | };
-    | |__-
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     let offset = unsafe { field_ptr.offset_from(base_ptr) };
+    |                           ------------------------------- inside `DIFFERENT_ALLOC` at $DIR/offset_from_ub.rs:16:27
26 error: any use of this value will cause an error
27   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

41 LL | | };
41 LL | | };
42    | |__-
43    |
+    = note: `#[deny(const_err)]` on by default
45    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
46 

- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
48   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
49    |
- LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
-    |                    |
-    |                    |
-    |                    exact_div: 1_isize cannot be divided by 2_isize without remainder
-    |                    inside `ptr::const_ptr::<impl *const u16>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                    inside `NOT_MULTIPLE_OF_SIZE` at $DIR/offset_from_ub.rs:30:14
+ LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                  |
+    |                  |
+    |                  exact_div: 1_isize cannot be divided by 2_isize without remainder
+    |                  inside `ptr::const_ptr::<impl *const u16>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
56    | 
-   ::: $DIR/offset_from_ub.rs:25:1
+   ::: $DIR/offset_from_ub.rs:30:14
58    |
- LL | / pub const NOT_MULTIPLE_OF_SIZE: isize = {
- LL | |
- LL | |     let data = [5u8, 6, 7];
- LL | |     let base_ptr = data.as_ptr();
- LL | |     let field_ptr = &data[1] as *const u8 as *const u16;
- LL | |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
- LL | | };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
+    |              --------------------------------------------- inside `NOT_MULTIPLE_OF_SIZE` at $DIR/offset_from_ub.rs:30:14
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
72   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
73    |
73    |
- LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
-    |                    |
-    |                    null pointer is not a valid pointer for this operation
-    |                    null pointer is not a valid pointer for this operation
-    |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                    inside `OFFSET_FROM_NULL` at $DIR/offset_from_ub.rs:36:14
+ LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                  |
+    |                  null pointer is not a valid pointer for this operation
+    |                  null pointer is not a valid pointer for this operation
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
80    | 
-   ::: $DIR/offset_from_ub.rs:33:1
+   ::: $DIR/offset_from_ub.rs:36:14
82    |
- LL | / pub const OFFSET_FROM_NULL: isize = {
- LL | |
- LL | |     let ptr = 0 as *const u8;
- LL | |     unsafe { ptr.offset_from(ptr) }
- LL | | };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     unsafe { ptr.offset_from(ptr) }
---
118 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                  |
   |                  |
   |                  ptr_offset_from cannot compute offset of pointers into different allocations.
   |                  inside `ptr::const_ptr::<impl *const Struct>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:16:27
   |
   |
LL |     let offset = unsafe { field_ptr.offset_from(base_ptr) };
   |                           ------------------------------- inside `DIFFERENT_ALLOC` at /checkout/src/test/ui/consts/offset_from_ub.rs:16:27
error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
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

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                  |
   |                  |
   |                  exact_div: 1_isize cannot be divided by 2_isize without remainder
   |                  inside `ptr::const_ptr::<impl *const u16>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:30:14
   |
   |
LL |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
   |              --------------------------------------------- inside `NOT_MULTIPLE_OF_SIZE` at /checkout/src/test/ui/consts/offset_from_ub.rs:30:14
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
   |
   |
LL |         unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                  |
   |                  null pointer is not a valid pointer for this operation
   |                  null pointer is not a valid pointer for this operation
   |                  inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:391:18
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:36:14
   |
LL |     unsafe { ptr.offset_from(ptr) }
   |              -------------------- inside `OFFSET_FROM_NULL` at /checkout/src/test/ui/consts/offset_from_ub.rs:36:14
   |              -------------------- inside `OFFSET_FROM_NULL` at /checkout/src/test/ui/consts/offset_from_ub.rs:36:14

error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:391:18
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

6    |                  |
7    |                  overflowing in-bounds pointer arithmetic
8    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `BEFORE_START` at $DIR/offset_ub.rs:6:46
10    | 
-   ::: $DIR/offset_ub.rs:6:1
+   ::: $DIR/offset_ub.rs:6:46
12    |
13 LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) };
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                              ------------------------------ inside `BEFORE_START` at $DIR/offset_ub.rs:6:46
19 
- error: any use of this value will cause an error
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
21   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
22    |
23 LL |         unsafe { intrinsics::offset(self, count) }

25    |                  |
26    |                  pointer arithmetic failed: pointer must be in-bounds at offset 2, but is outside bounds of allocN which has size 1
27    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `AFTER_END` at $DIR/offset_ub.rs:7:43
29    | 
-   ::: $DIR/offset_ub.rs:7:1
+   ::: $DIR/offset_ub.rs:7:43
31    |
32 LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                           ----------------------------- inside `AFTER_END` at $DIR/offset_ub.rs:7:43
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
39   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
40    |
40    |
41 LL |         unsafe { intrinsics::offset(self, count) }

43    |                  |
44    |                  pointer arithmetic failed: pointer must be in-bounds at offset 101, but is outside bounds of allocN which has size 100
45    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `AFTER_ARRAY` at $DIR/offset_ub.rs:8:45
47    | 
-   ::: $DIR/offset_ub.rs:8:1
+   ::: $DIR/offset_ub.rs:8:45
49    |
50 LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                             ------------------------------- inside `AFTER_ARRAY` at $DIR/offset_ub.rs:8:45
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
57   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
58    |
58    |
59 LL |         unsafe { intrinsics::offset(self, count) }

61    |                  |
62    |                  overflowing in-bounds pointer arithmetic
63    |                  inside `ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `OVERFLOW` at $DIR/offset_ub.rs:10:43
65    | 
-   ::: $DIR/offset_ub.rs:10:1
+   ::: $DIR/offset_ub.rs:10:43
67    |
68 LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                           ------------------------------------- inside `OVERFLOW` at $DIR/offset_ub.rs:10:43
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
75   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
76    |
76    |
77 LL |         unsafe { intrinsics::offset(self, count) }

79    |                  |
80    |                  overflowing in-bounds pointer arithmetic
81    |                  inside `ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `UNDERFLOW` at $DIR/offset_ub.rs:11:44
83    | 
-   ::: $DIR/offset_ub.rs:11:1
+   ::: $DIR/offset_ub.rs:11:44
85    |
86 LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                            ------------------------------------- inside `UNDERFLOW` at $DIR/offset_ub.rs:11:44
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
93   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
94    |
94    |
95 LL |         unsafe { intrinsics::offset(self, count) }

97    |                  |
98    |                  overflowing in-bounds pointer arithmetic
99    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `OVERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:12:56
101    | 
-   ::: $DIR/offset_ub.rs:12:1
+   ::: $DIR/offset_ub.rs:12:56
103    |
104 LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                                        ----------------------------------- inside `OVERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:12:56
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
111   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
112    |
112    |
113 LL |         unsafe { intrinsics::offset(self, count) }

115    |                  |
116    |                  overflowing in-bounds pointer arithmetic
117    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `UNDERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:13:57
119    | 
-   ::: $DIR/offset_ub.rs:13:1
+   ::: $DIR/offset_ub.rs:13:57
121    |
122 LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                                         --------------------------- inside `UNDERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:13:57
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
129   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
130    |
130    |
131 LL |         unsafe { intrinsics::offset(self, count) }

133    |                  |
134    |                  pointer arithmetic failed: pointer must be in-bounds at offset 1, but is outside bounds of allocN which has size 0
135    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `ZERO_SIZED_ALLOC` at $DIR/offset_ub.rs:15:50
137    | 
-   ::: $DIR/offset_ub.rs:15:1
+   ::: $DIR/offset_ub.rs:15:50
139    |
140 LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                                  --------------------------- inside `ZERO_SIZED_ALLOC` at $DIR/offset_ub.rs:15:50
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
+ error[E0080]: evaluation of constant value failed
165   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
166    |
167 LL |         unsafe { intrinsics::offset(self, count) }

169    |                  |
170    |                  pointer arithmetic failed: 0x0 is not a valid pointer
171    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `NULL_OFFSET_ZERO` at $DIR/offset_ub.rs:19:50
173    | 
-   ::: $DIR/offset_ub.rs:19:1
+   ::: $DIR/offset_ub.rs:19:50
175    |
176 LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) };
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                                  --------------------------- inside `NULL_OFFSET_ZERO` at $DIR/offset_ub.rs:19:50
182 error: any use of this value will cause an error
183   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

199 
199 
200 error: aborting due to 11 previous errors
201 
+ For more information about this error, try `rustc --explain E0080`.
202 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/offset_ub.stderr
To only update this specific test, also pass `--test-args consts/offset_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:6:46
   |
   |
LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) }; //~NOTE
   |                                              ------------------------------ inside `BEFORE_START` at /checkout/src/test/ui/consts/offset_ub.rs:6:46
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: pointer must be in-bounds at offset 2, but is outside bounds of alloc5 which has size 1
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:7:43
   |
   |
LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) }; //~NOTE
   |                                           ----------------------------- inside `AFTER_END` at /checkout/src/test/ui/consts/offset_ub.rs:7:43
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: pointer must be in-bounds at offset 101, but is outside bounds of alloc8 which has size 100
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:8:45
   |
   |
LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) }; //~NOTE
   |                                             ------------------------------- inside `AFTER_ARRAY` at /checkout/src/test/ui/consts/offset_ub.rs:8:45
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:10:43
   |
   |
LL | pub const OVERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MAX) }; //~NOTE
   |                                           ------------------------------------- inside `OVERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:10:43
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u16>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:11:44
   |
   |
LL | pub const UNDERFLOW: *const u16 = unsafe { [0u16; 1].as_ptr().offset(isize::MIN) }; //~NOTE
   |                                            ------------------------------------- inside `UNDERFLOW` at /checkout/src/test/ui/consts/offset_ub.rs:11:44
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:12:56
   |
   |
LL | pub const OVERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (usize::MAX as *const u8).offset(2) }; //~NOTE
   |                                                        ----------------------------------- inside `OVERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:12:56
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:13:57
   |
   |
LL | pub const UNDERFLOW_ADDRESS_SPACE: *const u8 = unsafe { (1 as *const u8).offset(-2) }; //~NOTE
   |                                                         --------------------------- inside `UNDERFLOW_ADDRESS_SPACE` at /checkout/src/test/ui/consts/offset_ub.rs:13:57
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: pointer must be in-bounds at offset 1, but is outside bounds of alloc23 which has size 0
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:15:50
   |
   |
LL | pub const ZERO_SIZED_ALLOC: *const u8 = unsafe { [0u8; 0].as_ptr().offset(1) }; //~NOTE
   |                                                  --------------------------- inside `ZERO_SIZED_ALLOC` at /checkout/src/test/ui/consts/offset_ub.rs:15:50
error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/mut_ptr.rs:242:18
   |
LL |         unsafe { intrinsics::offset(self, count) as *mut T }
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

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  |
   |                  pointer arithmetic failed: 0x0 is not a valid pointer
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
  ::: /checkout/src/test/ui/consts/offset_ub.rs:19:50
   |
   |
LL | pub const NULL_OFFSET_ZERO: *const u8 = unsafe { ptr::null::<u8>().offset(0) }; //~NOTE
   |                                                  --------------------------- inside `NULL_OFFSET_ZERO` at /checkout/src/test/ui/consts/offset_ub.rs:19:50
error: any use of this value will cause an error
  --> /checkout/library/core/src/ptr/const_ptr.rs:235:18
   |
LL |         unsafe { intrinsics::offset(self, count) }
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

6    |                  |
7    |                  pointer arithmetic failed: pointer must be in-bounds at offset $TWO_WORDS, but is outside bounds of alloc2 which has size $WORD
8    |                  inside `ptr::const_ptr::<impl *const usize>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `_` at $DIR/ptr_comparisons.rs:61:34
-   ::: $DIR/ptr_comparisons.rs:61:1
+   ::: $DIR/ptr_comparisons.rs:61:34
12    |
12    |
13 LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
-    |
-    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+    |                                  ------------------------------- inside `_` at $DIR/ptr_comparisons.rs:61:34
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
21   --> $DIR/ptr_comparisons.rs:64:33
22    |
22    |
- LL | / const _: *const u8 =
- LL | |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
-    | |_________________________________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^___-
-    |                                   |
-    |                                   memory access failed: pointer must be in-bounds at offset 1000, but is outside bounds of alloc2 which has size $WORD
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
+    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ memory access failed: pointer must be in-bounds at offset 1000, but is outside bounds of alloc2 which has size $WORD
32 error: any use of this value will cause an error
33   --> $DIR/ptr_comparisons.rs:68:27

37    |                           |
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |                  inside `ptr::const_ptr::<impl *const usize>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:235:18
   | 
  ::: /checkout/src/test/ui/consts/ptr_comparisons.rs:61:34
   |
LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
   |                                  ------------------------------- inside `_` at /checkout/src/test/ui/consts/ptr_comparisons.rs:61:34
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:64:33
   |
   |
LL |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/ptr_comparisons.rs:68:27
   |
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
test result: FAILED. 11882 passed; 11 failed; 101 ignored; 0 measured; 0 filtered out; finished in 119.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:41
