plain
........................................................................................  2728/14808
........................................................................................  2816/14808
........................................................................................  2904/14808
........................................................................................  2992/14808
....................F.........F.........................................................  3080/14808
..................F.....................................................................  3256/14808
........................................................................................  3344/14808
........................................................................................  3432/14808
..........i......................................................i......................  3520/14808
---
-   --> $DIR/raw-bytes.rs:201:1
+ error[E0080]: evaluation of constant value failed
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
462    |
- LL | const LAYOUT_INVALID_ZERO: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
+    = note: the evaluated program panicked at 'Alignment::new_unchecked requires a power of two', $SRC_DIR/core/src/panicking.rs:LL:COL
465    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-            }
-            }
+ note: inside `core::panicking::panic_nounwind_fmt::comptime`
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
+ note: inside `std::ptr::Alignment::new_unchecked`
+   --> $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ note: inside `Layout::from_size_align_unchecked`
+   --> $SRC_DIR/core/src/alloc/layout.rs:LL:COL
+ note: inside `LAYOUT_INVALID_ZERO`
+    |
+    |
+ LL | const LAYOUT_INVALID_ZERO: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
+    |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: this error originates in the macro `crate::panic::debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)
- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/raw-bytes.rs:205:1
+ error[E0080]: evaluation of constant value failed
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
473    |
- LL | const LAYOUT_INVALID_THREE: Layout = unsafe { Layout::from_size_align_unchecked(9, 3) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x0000000000000003, but expected a valid enum tag
+    = note: the evaluated program panicked at 'Alignment::new_unchecked requires a power of two', $SRC_DIR/core/src/panicking.rs:LL:COL
476    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-            }
-            }
+ note: inside `core::panicking::panic_nounwind_fmt::comptime`
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
+ note: inside `std::ptr::Alignment::new_unchecked`
+   --> $SRC_DIR/core/src/ptr/alignment.rs:LL:COL
+ note: inside `Layout::from_size_align_unchecked`
+   --> $SRC_DIR/core/src/alloc/layout.rs:LL:COL
+ note: inside `LAYOUT_INVALID_THREE`
+    |
+    |
+ LL | const LAYOUT_INVALID_THREE: Layout = unsafe { Layout::from_size_align_unchecked(9, 3) };
+    |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    = note: this error originates in the macro `crate::panic::debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)
482 error[E0080]: it is undefined behavior to use this value
483   --> $DIR/raw-bytes.rs:209:1



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes/raw-bytes.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/raw-bytes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/raw-bytes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:20:1
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:28:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:28:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:42:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:42:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:44:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:44:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:50:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:50:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:54:1
   |
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:57:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:57:1
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:59:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:59:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:65:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:65:1
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:71:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:74:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:74:1
   |
LL | const NULL_FAT_PTR: NonNull<dyn Send> = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               00 00 00 00 00 00 00 00 ╾───────alloc28───────╼ │ ........╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:82:1
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (0x539[noalloc] has no provenance)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (0x539[noalloc] has no provenance)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
   |
LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered null pointer, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
   |
LL | const DANGLING_FN_PTR: fn() = unsafe { mem::transmute(13usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0xd[noalloc], but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               0d 00 00 00 00 00 00 00                         │ ........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:106:1
   |
   |
LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered alloc55, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:137:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:137:1
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc61───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:139:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc67───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:141:1
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc73───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:144:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:146:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:146:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:148:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:148:1
   |
LL | const MYSTR_NO_INIT_ISSUE83182: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:152:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:152:1
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc93───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:154:1
   |
   |
LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc99───────╼ ff ff ff ff ff ff ff 7f │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:157:1
   |
   |
LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────alloc104───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:160:1
   |
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:160:40
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:160:40
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:166:1
   |
   |
LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:166:42
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:166:42
   |
LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:170:1
   |
   |
LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.1[0]: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:170:42
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:170:42
   |
LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:175:1
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_1: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u8))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered alloc125, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────alloc123───────╼ ╾──────alloc125───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:179:1
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_2: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u64))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered alloc133, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────alloc131───────╼ ╾──────alloc133───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:183:1
   |
   |
LL | const TRAIT_OBJ_INT_VTABLE: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, 4usize))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered 0x4[noalloc], but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:186:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:186:1
   |
LL | const TRAIT_OBJ_BAD_DROP_FN_NOT_FN_PTR: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &[&42u8; 8]))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered alloc148, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────alloc145───────╼ ╾──────alloc148───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:190:1
   |
   |
LL | const TRAIT_OBJ_CONTENT_INVALID: &dyn Trait = unsafe { mem::transmute::<_, &bool>(&3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.<dyn-downcast>: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────alloc154───────╼ ╾──────alloc156───────╼ │ ╾──────╼╾──────╼
---
1 error[E0080]: evaluation of constant value failed
-   --> $SRC_DIR/core/src/slice/index.rs:LL:COL
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
3    |
-    = note: overflow executing `unchecked_sub`
+    = note: the evaluated program panicked at 'slice::get_unchecked requires that the range is within the slice', $SRC_DIR/core/src/panicking.rs:LL:COL
5    |
+ note: inside `core::panicking::panic_nounwind_fmt::comptime`
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
6 note: inside `<std::ops::Range<usize> as SliceIndex<[()]>>::get_unchecked`
7   --> $SRC_DIR/core/src/slice/index.rs:LL:COL
8 note: inside `core::slice::<impl [()]>::get_unchecked::<std::ops::Range<usize>>`
12    |
12    |
13 LL | const B: &[()] = unsafe { A.get_unchecked(3..1) };
+    = note: this error originates in the macro `debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)
15 
16 error: aborting due to previous error
17 
---
To only update this specific test, also pass `--test-args consts/const-eval/ub-slice-get-unchecked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/ub-slice-get-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-slice-get-unchecked" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-slice-get-unchecked/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /rustc/FAKE_PREFIX/library/core/src/panicking.rs:103:9
   |
   |
   = note: the evaluated program panicked at 'slice::get_unchecked requires that the range is within the slice', /rustc/FAKE_PREFIX/library/core/src/panicking.rs:103:9
   |
note: inside `core::panicking::panic_nounwind_fmt::comptime`
  --> /rustc/FAKE_PREFIX/library/core/src/panicking.rs:103:9
note: inside `<std::ops::Range<usize> as SliceIndex<[()]>>::get_unchecked`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/index.rs:362:9
note: inside `core::slice::<impl [()]>::get_unchecked::<std::ops::Range<usize>>`
  --> /rustc/FAKE_PREFIX/library/core/src/slice/mod.rs:406:20
  --> fake-test-src-base/consts/const-eval/ub-slice-get-unchecked.rs:7:27
   |
   |
LL | const B: &[()] = unsafe { A.get_unchecked(3..1) };
   = note: this error originates in the macro `debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---
-   --> $SRC_DIR/core/src/hint.rs:LL:COL
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
3    |
-    = note: entering unreachable code
+    = note: the evaluated program panicked at 'hint::unreachable_unchecked must never be reached', $SRC_DIR/core/src/panicking.rs:LL:COL
5    |
+ note: inside `core::panicking::panic_nounwind_fmt::comptime`
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
6 note: inside `unreachable_unchecked`
7   --> $SRC_DIR/core/src/hint.rs:LL:COL

15    |
15    |
16 LL | const BAR: bool = unsafe { foo(false) };
17    |                            ^^^^^^^^^^
+    = note: this error originates in the macro `crate::panic::debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)
19 error: aborting due to previous error
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/const_unsafe_unreachable_ub.stderr
To only update this specific test, also pass `--test-args consts/const_unsafe_unreachable_ub.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const_unsafe_unreachable_ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_unsafe_unreachable_ub/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /rustc/FAKE_PREFIX/library/core/src/panicking.rs:103:9
   |
   |
   = note: the evaluated program panicked at 'hint::unreachable_unchecked must never be reached', /rustc/FAKE_PREFIX/library/core/src/panicking.rs:103:9
   |
note: inside `core::panicking::panic_nounwind_fmt::comptime`
  --> /rustc/FAKE_PREFIX/library/core/src/panicking.rs:103:9
note: inside `unreachable_unchecked`
  --> /rustc/FAKE_PREFIX/library/core/src/hint.rs:101:5
  --> fake-test-src-base/consts/const_unsafe_unreachable_ub.rs:6:18
   |
LL |         false => std::hint::unreachable_unchecked(),
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `BAR`
  --> fake-test-src-base/consts/const_unsafe_unreachable_ub.rs:10:28
   |
LL | const BAR: bool = unsafe { foo(false) };
   |                            ^^^^^^^^^^
   = note: this error originates in the macro `crate::panic::debug_assert_nounwind` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/print_type_sizes/niche-filling.rs stdout ----
diff of stdout:

+ print-type-size type: `std::fmt::rt::v1::Argument`: 56 bytes, alignment: 8 bytes
+ print-type-size     field `.format`: 48 bytes
+ print-type-size     field `.position`: 8 bytes
+ print-type-size type: `std::fmt::Arguments<'_>`: 48 bytes, alignment: 8 bytes
+ print-type-size     field `.fmt`: 16 bytes
+ print-type-size     field `.pieces`: 16 bytes
+ print-type-size     field `.args`: 16 bytes
+ print-type-size type: `std::fmt::rt::v1::FormatSpec`: 48 bytes, alignment: 8 bytes
+ print-type-size     field `.precision`: 16 bytes
+ print-type-size     field `.width`: 16 bytes
+ print-type-size     field `.flags`: 4 bytes
+ print-type-size     field `.fill`: 4 bytes
+ print-type-size     field `.align`: 1 bytes
+ print-type-size     end padding: 7 bytes
+ print-type-size type: `std::panic::Location<'_>`: 24 bytes, alignment: 8 bytes
+ print-type-size     field `.file`: 16 bytes
+ print-type-size     field `.line`: 4 bytes
+ print-type-size     field `.col`: 4 bytes
+ print-type-size type: `core::fmt::ArgumentV1<'_>`: 16 bytes, alignment: 8 bytes
+ print-type-size     field `.value`: 8 bytes
+ print-type-size     field `.formatter`: 8 bytes
+ print-type-size type: `std::fmt::rt::v1::Count`: 16 bytes, alignment: 8 bytes
+ print-type-size     discriminant: 8 bytes
+ print-type-size     variant `Is`: 8 bytes
+ print-type-size         field `.0`: 8 bytes
+ print-type-size     variant `Param`: 8 bytes
+ print-type-size         field `.0`: 8 bytes
+ print-type-size     variant `Implied`: 0 bytes
+ print-type-size type: `std::option::Option<&[std::fmt::rt::v1::Argument]>`: 16 bytes, alignment: 8 bytes
+ print-type-size     variant `Some`: 16 bytes
+ print-type-size         field `.0`: 16 bytes
+ print-type-size     variant `None`: 0 bytes
1 print-type-size type: `IndirectNonZero`: 12 bytes, alignment: 4 bytes
2 print-type-size     field `.nested`: 8 bytes
3 print-type-size     field `.post`: 2 bytes

110 print-type-size     variant `Less`: 0 bytes
111 print-type-size     variant `Equal`: 0 bytes
112 print-type-size     variant `Greater`: 0 bytes
+ print-type-size type: `std::fmt::rt::v1::Alignment`: 1 bytes, alignment: 1 bytes
+ print-type-size     discriminant: 1 bytes
+ print-type-size     variant `Left`: 0 bytes
+ print-type-size     variant `Right`: 0 bytes
+ print-type-size     variant `Center`: 0 bytes
+ print-type-size     variant `Unknown`: 0 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/niche-filling.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/niche-filling.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/niche-filling.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/print_type_sizes/niche-filling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/niche-filling/auxiliary" "-Z" "print-type-sizes" "--crate-type=lib"
--- stdout -------------------------------
print-type-size type: `std::fmt::rt::v1::Argument`: 56 bytes, alignment: 8 bytes
print-type-size     field `.format`: 48 bytes
print-type-size     field `.position`: 8 bytes
print-type-size type: `std::fmt::Arguments<'_>`: 48 bytes, alignment: 8 bytes
print-type-size     field `.fmt`: 16 bytes
print-type-size     field `.pieces`: 16 bytes
print-type-size     field `.args`: 16 bytes
print-type-size type: `std::fmt::rt::v1::FormatSpec`: 48 bytes, alignment: 8 bytes
print-type-size     field `.precision`: 16 bytes
print-type-size     field `.width`: 16 bytes
print-type-size     field `.flags`: 4 bytes
print-type-size     field `.fill`: 4 bytes
print-type-size     field `.align`: 1 bytes
print-type-size     end padding: 7 bytes
print-type-size type: `std::panic::Location<'_>`: 24 bytes, alignment: 8 bytes
print-type-size     field `.file`: 16 bytes
print-type-size     field `.line`: 4 bytes
print-type-size     field `.col`: 4 bytes
print-type-size type: `core::fmt::ArgumentV1<'_>`: 16 bytes, alignment: 8 bytes
print-type-size     field `.value`: 8 bytes
print-type-size     field `.formatter`: 8 bytes
print-type-size type: `std::fmt::rt::v1::Count`: 16 bytes, alignment: 8 bytes
print-type-size     discriminant: 8 bytes
print-type-size     variant `Is`: 8 bytes
print-type-size         field `.0`: 8 bytes
print-type-size     variant `Param`: 8 bytes
print-type-size         field `.0`: 8 bytes
print-type-size     variant `Implied`: 0 bytes
print-type-size type: `std::option::Option<&[std::fmt::rt::v1::Argument]>`: 16 bytes, alignment: 8 bytes
print-type-size     variant `Some`: 16 bytes
print-type-size         field `.0`: 16 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `IndirectNonZero`: 12 bytes, alignment: 4 bytes
print-type-size     field `.nested`: 8 bytes
print-type-size     field `.post`: 2 bytes
print-type-size     field `.pre`: 1 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `MyOption<IndirectNonZero>`: 12 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 12 bytes
print-type-size         field `.0`: 12 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `EmbeddedDiscr`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Record`: 7 bytes
print-type-size         field `.pre`: 1 bytes
print-type-size         field `.post`: 2 bytes
print-type-size         field `.val`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union1<std::num::NonZeroU32>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union2<std::num::NonZeroU32, std::num::NonZeroU32>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<Union2<std::num::NonZeroU32, u32>>`: 8 bytes, alignment: 4 bytes
print-type-size     discriminant: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `NestedNonZero`: 8 bytes, alignment: 4 bytes
print-type-size     field `.val`: 4 bytes
print-type-size     field `.post`: 2 bytes
print-type-size     field `.pre`: 1 bytes
print-type-size     end padding: 1 bytes
print-type-size type: `Enum4<(), char, (), ()>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Two`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Three`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Four`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyNotNegativeOne`: 4 bytes, alignment: 4 bytes
print-type-size     field `._i`: 4 bytes
print-type-size type: `MyOption<MyNotNegativeOne>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<char>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Some`: 4 bytes
print-type-size         field `.0`: 4 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `Union1<std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union1`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size type: `Union2<std::num::NonZeroU32, std::num::NonZeroU32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union2`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes
print-type-size type: `Union2<std::num::NonZeroU32, u32>`: 4 bytes, alignment: 4 bytes
print-type-size     variant `Union2`: 4 bytes
print-type-size         field `.a`: 4 bytes
print-type-size         field `.b`: 4 bytes, offset: 0 bytes, alignment: 4 bytes
print-type-size type: `std::num::NonZeroU32`: 4 bytes, alignment: 4 bytes
print-type-size     field `.0`: 4 bytes
print-type-size type: `Enum4<(), (), (), MyOption<u8>>`: 2 bytes, alignment: 1 bytes
print-type-size     variant `Four`: 2 bytes
print-type-size         field `.0`: 2 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Two`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Three`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyOption<MyOption<u8>>`: 2 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 2 bytes
print-type-size         field `.0`: 2 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<u8>`: 2 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `Enum4<(), (), bool, ()>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Three`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `One`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Two`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size     variant `Four`: 0 bytes
print-type-size         field `.0`: 0 bytes
print-type-size type: `MyOption<bool>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `MyOption<std::cmp::Ordering>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `Some`: 1 bytes
print-type-size         field `.0`: 1 bytes
print-type-size     variant `None`: 0 bytes
print-type-size type: `std::cmp::Ordering`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Less`: 0 bytes
print-type-size     variant `Equal`: 0 bytes
print-type-size     variant `Greater`: 0 bytes
print-type-size type: `std::fmt::rt::v1::Alignment`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Left`: 0 bytes
print-type-size     variant `Right`: 0 bytes
print-type-size     variant `Center`: 0 bytes
print-type-size     variant `Unknown`: 0 bytes
stderr: none



