plain

---- [ui] tests/ui/consts/const-eval/raw-bytes.rs stdout ----
diff of 32bit.stderr:

290 LL | const MYSTR_NO_INIT_ISSUE83182: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };
291    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
295    = note: the raw bytes of the constant (size: 8, align: 4) {
296                ╾ALLOC_ID╼ 01 00 00 00                         │ ╾──╼....

+    = help: this code performed an operation that depends on the underlying bytes representing a pointer
+    = help: this code performed an operation that depends on the underlying bytes representing a pointer
+    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
299 error[E0080]: it is undefined behavior to use this value
300   --> $DIR/raw-bytes.rs:152:1


529 LL | pub static S5: &[u8] = unsafe { from_raw_parts((&D3) as *const _ as _, mem::size_of::<&u32>()) };
530    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
534    = note: the raw bytes of the constant (size: 8, align: 4) {
535                ╾ALLOC_ID╼ 04 00 00 00                         │ ╾──╼....

+    = help: this code performed an operation that depends on the underlying bytes representing a pointer
+    = help: this code performed an operation that depends on the underlying bytes representing a pointer
+    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
538 error[E0080]: it is undefined behavior to use this value
539   --> $DIR/raw-bytes.rs:221:1


574 LL | pub static R5: &[u8] = unsafe {
575    | ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: this code performed an operation that depends on the underlying bytes representing a pointer
-    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
579    = note: the raw bytes of the constant (size: 8, align: 4) {
580                ╾ALLOC_ID╼ 04 00 00 00                         │ ╾──╼....

+    = help: this code performed an operation that depends on the underlying bytes representing a pointer
+    = help: this code performed an operation that depends on the underlying bytes representing a pointer
+    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
583 error[E0080]: it is undefined behavior to use this value
584   --> $DIR/raw-bytes.rs:242:1



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes/raw-bytes.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/raw-bytes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/raw-bytes.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes/auxiliary"
stdout: none
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:20:1
   |
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x00000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:28:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:28:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x00000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
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
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type `Never`
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
   = note: the raw bytes of the constant (size: 4, align: 4) {
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
   = note: the raw bytes of the constant (size: 4, align: 4) {
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
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:82:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:82:1
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (0x539[noalloc] has no provenance)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (0x539[noalloc] has no provenance)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
   |
LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered null pointer, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
   |
LL | const DANGLING_FN_PTR: fn() = unsafe { mem::transmute(13usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0xd[noalloc], but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               0d 00 00 00                                     │ ....

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:106:1
   |
   |
LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered alloc55, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:137:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:137:1
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─alloc61─╼ e7 03 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:139:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─alloc67─╼ ff ff ff ff                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:141:1
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─alloc73─╼ ff ff ff ff                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:144:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:146:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:146:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.0: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:148:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:148:1
   |
LL | const MYSTR_NO_INIT_ISSUE83182: &MyStr = unsafe { mem::transmute::<&[_], _>(&[&()]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:152:1
   |
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─alloc93─╼ e7 03 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:154:1
   |
   |
LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾─alloc99─╼ ff ff ff 7f                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:157:1
   |
   |
LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾alloc104─╼ e7 03 00 00                         │ ╾──╼....

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:160:1
   |
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>[0]: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
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
   = note: the raw bytes of the constant (size: 4, align: 4) {
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
   = note: the raw bytes of the constant (size: 4, align: 4) {
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
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾alloc123─╼ ╾alloc125─╼                         │ ╾──╼╾──╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:179:1
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_2: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u64))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered alloc133, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾alloc131─╼ ╾alloc133─╼                         │ ╾──╼╾──╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:183:1
   |
   |
LL | const TRAIT_OBJ_INT_VTABLE: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, 4usize))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered 0x4[noalloc], but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:186:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:186:1
   |
LL | const TRAIT_OBJ_BAD_DROP_FN_NOT_FN_PTR: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &[&42u8; 8]))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered alloc148, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾alloc145─╼ ╾alloc148─╼                         │ ╾──╼╾──╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:190:1
   |
   |
LL | const TRAIT_OBJ_CONTENT_INVALID: &dyn Trait = unsafe { mem::transmute::<_, &bool>(&3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<deref>.<dyn-downcast>: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾alloc154─╼ ╾alloc156─╼                         │ ╾──╼╾──╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:194:1
   |
   |
