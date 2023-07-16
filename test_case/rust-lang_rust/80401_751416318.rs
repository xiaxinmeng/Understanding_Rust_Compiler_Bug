plain
.................................................................................................... 2000/11202
.................................................................................................... 2100/11202
.................................................................................................... 2200/11202
.................................................................................................... 2300/11202
...........................................................FF..........F............................ 2400/11202
.................................................................................................... 2600/11202
............................................iiiii................................................... 2700/11202
.................................................................................................... 2800/11202
.................................................................................................... 2900/11202
---
.................................................................................................... 9100/11202
............................................................................................i......i 9200/11202
................................................................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
.... 9300/11202
...............................iiiiii..iiiiii.i..................................................... 9400/11202
.................................................................................................... 9600/11202
.................................................................................................... 9700/11202
.................................................................................................... 9800/11202
.................................................................................................... 9900/11202
---

---- [ui] ui/associated-type-bounds/bounds-on-assoc-in-trait.rs stdout ----
diff of stderr:

1 error[E0277]: `<<Self as Case1>::A as Iterator>::Item` doesn't implement `Debug`
3    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL |       type A: Iterator<Item: Debug>;
-    |                              ^^^^^ `<<Self as Case1>::A as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+ LL |     type A: Iterator<Item: Debug>;
+    |                            ^^^^^ `<<Self as Case1>::A as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    | 
+   ::: $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+    |
+ LL | pub trait Debug {
+    | --------------- required by this bound in `Debug`
6    |
7    = help: the trait `Debug` is not implemented for `<<Self as Case1>::A as Iterator>::Item`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait/bounds-on-assoc-in-trait.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait/bounds-on-assoc-in-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/bounds-on-assoc-in-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `<<Self as Case1>::A as Iterator>::Item` doesn't implement `Debug`
   |
   |
LL |     type A: Iterator<Item: Debug>;
   |                            ^^^^^ `<<Self as Case1>::A as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  ::: /checkout/library/core/src/fmt/mod.rs:552:1
   |
LL | pub trait Debug {
LL | pub trait Debug {
   | --------------- required by this bound in `Debug`
   |
   = help: the trait `Debug` is not implemented for `<<Self as Case1>::A as Iterator>::Item`
   |
   |
LL | trait Case1 where <<Self as Case1>::A as Iterator>::Item: Debug {


error[E0277]: the trait bound `<<Self as Foo>::Out as Baz>::Assoc: Default` is not satisfied
   |
   |
LL | pub trait Foo { type Out: Baz<Assoc: Default>; }
   |                                      ^^^^^^^ the trait `Default` is not implemented for `<<Self as Foo>::Out as Baz>::Assoc`
  ::: /checkout/library/core/src/default.rs:84:1
   |
   |
LL | pub trait Default: Sized {
   | ------------------------ required by this bound in `Default`
help: consider further restricting the associated type
   |
   |
LL | pub trait Foo where <<Self as Foo>::Out as Baz>::Assoc: Default { type Out: Baz<Assoc: Default>; }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/consts/offset_from_ub.rs stdout ----
diff of stderr:

1 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                    |
+    |                    |
+    |                    ptr_offset_from cannot compute offset of pointers into different allocations.
+    |                    inside `ptr::const_ptr::<impl *const Struct>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                    inside `DIFFERENT_ALLOC` at $DIR/offset_from_ub.rs:16:27
2    | 
3   ::: $DIR/offset_from_ub.rs:10:1


14    = note: `#[deny(const_err)]` on by default
15 
16 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                    |
+    |                    |
+    |                    unable to turn bytes into a pointer
+    |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                    inside `NOT_PTR` at $DIR/offset_from_ub.rs:22:14
17    | 
18   ::: $DIR/offset_from_ub.rs:20:1

24    | |__-
25 
25 
26 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                    |
+    |                    |
+    |                    exact_div: 1_isize cannot be divided by 2_isize without remainder
+    |                    inside `ptr::const_ptr::<impl *const u16>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                    inside `NOT_MULTIPLE_OF_SIZE` at $DIR/offset_from_ub.rs:30:14
27    | 
28   ::: $DIR/offset_from_ub.rs:25:1

37    | |__-
38 
38 
39 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                    |
+    |                    |
+    |                    inbounds test failed: 0x0 is not a valid pointer
+    |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                    inside `OFFSET_FROM_NULL` at $DIR/offset_from_ub.rs:36:14
40    | 
41   ::: $DIR/offset_from_ub.rs:33:1

48    | |__-
49 
49 
50 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
+    |                    |
+    |                    |
+    |                    unable to turn bytes into a pointer
+    |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                    inside `DIFFERENT_INT` at $DIR/offset_from_ub.rs:43:14
51    | 
52   ::: $DIR/offset_from_ub.rs:39:1


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    |
   |                    ptr_offset_from cannot compute offset of pointers into different allocations.
   |                    inside `ptr::const_ptr::<impl *const Struct>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:374:18
   |                    inside `DIFFERENT_ALLOC` at /checkout/src/test/ui/consts/offset_from_ub.rs:16:27
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:10:1
   |
   |
LL | / pub const DIFFERENT_ALLOC: usize = {
LL | |     //~^ NOTE
LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
LL | |     offset as usize
LL | | };
   | |__-
   |
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
   |
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    |
   |                    unable to turn bytes into a pointer
   |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:374:18
   |                    inside `NOT_PTR` at /checkout/src/test/ui/consts/offset_from_ub.rs:22:14
  ::: /checkout/src/test/ui/consts/offset_from_ub.rs:20:1
   |
   |
LL | / pub const NOT_PTR: usize = {
LL | |     //~^ NOTE
LL | |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
LL | | };


error: any use of this value will cause an error
   |
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    |
   |                    exact_div: 1_isize cannot be divided by 2_isize without remainder
   |                    inside `ptr::const_ptr::<impl *const u16>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:374:18
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


error: any use of this value will cause an error
   |
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    |
   |                    inbounds test failed: 0x0 is not a valid pointer
   |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:374:18
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
   |
   |
LL |           unsafe { intrinsics::ptr_offset_from(self, origin) }
   |                    |
   |                    |
   |                    unable to turn bytes into a pointer
   |                    inside `ptr::const_ptr::<impl *const u8>::offset_from` at /checkout/library/core/src/ptr/const_ptr.rs:374:18
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

error: aborting due to 5 previous errors



------------------------------------------


---- [ui] ui/consts/offset_ub.rs stdout ----
diff of stderr:

1 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  overflowing in-bounds pointer arithmetic
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `BEFORE_START` at $DIR/offset_ub.rs:7:46
2    | 
3   ::: $DIR/offset_ub.rs:7:1


8    = note: `#[deny(const_err)]` on by default
9 
10 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  inbounds test failed: pointer must be in-bounds at offset 2, but is outside bounds of allocN which has size 1
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `AFTER_END` at $DIR/offset_ub.rs:8:43
11    | 
12   ::: $DIR/offset_ub.rs:8:1

15    | --------------------------------------------------------------------------
16 
16 
17 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  inbounds test failed: pointer must be in-bounds at offset 101, but is outside bounds of allocN which has size 100
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `AFTER_ARRAY` at $DIR/offset_ub.rs:9:45
18    | 
19   ::: $DIR/offset_ub.rs:9:1

22    | ------------------------------------------------------------------------------
23 
23 
24 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  overflowing in-bounds pointer arithmetic
+    |                  inside `ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `OVERFLOW` at $DIR/offset_ub.rs:11:43
25    | 
26   ::: $DIR/offset_ub.rs:11:1

29    | ----------------------------------------------------------------------------------
30 
30 
31 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  overflowing in-bounds pointer arithmetic
+    |                  inside `ptr::const_ptr::<impl *const u16>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `UNDERFLOW` at $DIR/offset_ub.rs:12:44
32    | 
33   ::: $DIR/offset_ub.rs:12:1

36    | -----------------------------------------------------------------------------------
37 
37 
38 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  overflowing in-bounds pointer arithmetic
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `OVERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:13:56
39    | 
40   ::: $DIR/offset_ub.rs:13:1

43    | ---------------------------------------------------------------------------------------------
44 
44 
45 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  overflowing in-bounds pointer arithmetic
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `UNDERFLOW_ADDRESS_SPACE` at $DIR/offset_ub.rs:14:57
46    | 
47   ::: $DIR/offset_ub.rs:14:1

50    | --------------------------------------------------------------------------------------
51 
51 
52 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  inbounds test failed: pointer must be in-bounds at offset 1, but is outside bounds of allocN which has size 0
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `ZERO_SIZED_ALLOC` at $DIR/offset_ub.rs:16:50
53    | 
54   ::: $DIR/offset_ub.rs:16:1

57    | -------------------------------------------------------------------------------
58 
58 
59 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) as *mut T }
+    |                  |
+    |                  |
+    |                  unable to turn bytes into a pointer
+    |                  inside `ptr::mut_ptr::<impl *mut u8>::offset` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+    |                  inside `DANGLING` at $DIR/offset_ub.rs:17:42
60    | 
61   ::: $DIR/offset_ub.rs:17:1

64    | ---------------------------------------------------------------------------------------------
65 
65 
66 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  inbounds test failed: 0x0 is not a valid pointer
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `NULL_OFFSET_ZERO` at $DIR/offset_ub.rs:20:50
67    | 
68   ::: $DIR/offset_ub.rs:20:1

71    | -------------------------------------------------------------------------------
72 
72 
73 error: any use of this value will cause an error
+   --> $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |
+ LL |         unsafe { intrinsics::offset(self, count) }
+    |                  |
+    |                  |
+    |                  unable to turn bytes into a pointer
+    |                  inside `ptr::const_ptr::<impl *const u8>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |                  inside `UNDERFLOW_ABS` at $DIR/offset_ub.rs:23:47
74    | 
75   ::: $DIR/offset_ub.rs:23:1


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/offset_ub.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/offset_ub.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  |
   |                  |
   |                  overflowing in-bounds pointer arithmetic
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:225:18
   |                  inside `BEFORE_START` at /checkout/src/test/ui/consts/offset_ub.rs:7:46
  ::: /checkout/src/test/ui/consts/offset_ub.rs:7:1
   |
   |
LL | pub const BEFORE_START: *const u8 = unsafe { (&0u8 as *const u8).offset(-1) }; //~NOTE
   |
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  |
   |                  |
   |                  inbounds test failed: pointer must be in-bounds at offset 2, but is outside bounds of alloc5 which has size 1
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:225:18
   |                  inside `AFTER_END` at /checkout/src/test/ui/consts/offset_ub.rs:8:43
  ::: /checkout/src/test/ui/consts/offset_ub.rs:8:1
   |
   |
LL | pub const AFTER_END: *const u8 = unsafe { (&0u8 as *const u8).offset(2) }; //~NOTE


error: any use of this value will cause an error
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  |
   |                  |
   |                  inbounds test failed: pointer must be in-bounds at offset 101, but is outside bounds of alloc8 which has size 100
   |                  inside `ptr::const_ptr::<impl *const u8>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:225:18
   |                  inside `AFTER_ARRAY` at /checkout/src/test/ui/consts/offset_ub.rs:9:45
  ::: /checkout/src/test/ui/consts/offset_ub.rs:9:1
   |
   |
LL | pub const AFTER_ARRAY: *const u8 = unsafe { [0u8; 100].as_ptr().offset(101) }; //~NOTE
---
test result: FAILED. 11114 passed; 4 failed; 84 ignored; 0 measured; 0 filtered out; finished in 166.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:29
