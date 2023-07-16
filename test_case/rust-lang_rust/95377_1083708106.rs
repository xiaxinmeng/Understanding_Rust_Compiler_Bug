plain

---- [ui] ui/consts/const-eval/ub-nonnull.rs stdout ----
diff of 64bit.stderr:

28 LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
29    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
- error[E0080]: evaluation of constant value failed
+ error[E0080]: it is undefined behavior to use this value
32   --> $DIR/ub-nonnull.rs:42:1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
33    |
34 LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
35    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
+    |
+    |
+    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
+    = note: the raw bytes of the constant (size: 4, align: 4) {
+                2a 00 00 00                                     │ *...
36 
- error[E0080]: evaluation of constant value failed
+ error[E0080]: it is undefined behavior to use this value
38   --> $DIR/ub-nonnull.rs:48:1
38   --> $DIR/ub-nonnull.rs:48:1
39    |
40 LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };

41    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
+    |
+    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
+    = note: the raw bytes of the constant (size: 4, align: 4) {
+            }
42 
43 error: aborting due to 7 previous errors
44 
44 


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   |                                        ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:17:27
   |
   |
LL |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
   |                           ^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:37
   |
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   |                                     ^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:26:43
   |
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   |                                           ^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:34:36
   |
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
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

