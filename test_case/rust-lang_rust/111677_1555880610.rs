plain
........................................................................................  2728/15027
........................................................................................  2816/15027
........................................................................................  2904/15027
.............................................................i..........................  2992/15027
................................................................................F.ii...F  3080/15027
........................F...............................................................  3168/15027
...............i........................................................................  3344/15027
.....i.i...........................i.................i.............................i....  3432/15027
.....................................................................i..................  3520/15027
....................................i........................................i..........  3608/15027
---
diff of 32bit.stderr:

35   --> $DIR/raw-bytes.rs:44:1
36    |
37 LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type `Never`
39    |
40    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
41    = note: the raw bytes of the constant (size: 1, align: 1) {

The actual 32bit.stderr differed from the expected 32bit.stderr.
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
Build completed unsuccessfully in 0:01:44
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
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:152:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:152:1
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
LL | const RAW_TRAIT_OBJ_VTABLE_NULL: *const dyn Trait = unsafe { mem::transmute((&92u8, 0usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered null pointer, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:196:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:196:1
   |
LL | const RAW_TRAIT_OBJ_VTABLE_INVALID: *const dyn Trait = unsafe { mem::transmute((&92u8, &3u64)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered alloc166, but expected a vtable pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               ╾alloc164─╼ ╾alloc166─╼                         │ ╾──╼╾──╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:201:1
   |
   |
LL | const LAYOUT_INVALID_ZERO: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x00000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:205:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:205:1
   |
LL | const LAYOUT_INVALID_THREE: Layout = unsafe { Layout::from_size_align_unchecked(9, 3) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x00000003, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
---
diff of 32bit.stderr:

86   --> $DIR/ub-enum.rs:83:1
87    |
88 LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type `Never`
90    |
91    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
92    = note: the raw bytes of the constant (size: $SIZE, align: $ALIGN) {
108   --> $DIR/ub-enum.rs:96:77
109    |
109    |
110 LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
-    |                                                                             ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Ok)>.0.1: encountered a value of uninhabited type Never
+    |                                                                             ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Ok)>.0.1: encountered a value of uninhabited type `Never`
113 error[E0080]: evaluation of constant value failed
114   --> $DIR/ub-enum.rs:98:77



The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
stdout: none
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:27:1
   |
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x00000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:30:1
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:30:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:33:1
   |
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:45:1
   |
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x00000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:47:1
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:47:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:50:1
   |
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:59:42
   |
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:64:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:81:1
   |
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:83:1
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:83:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type `Never`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:91:1
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:91:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:96:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Ok)>.0.1: encountered a value of uninhabited type `Never`
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/ub-enum.rs:98:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Ok)>.0.1: encountered a value of the never type `!`
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/consts/const-eval/validate_uninhabited_zsts.rs stdout ----
diff of 32bit.stderr:

28   --> $DIR/validate_uninhabited_zsts.rs:21:42
29    |
30 LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
-    |                                          ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered a value of uninhabited type empty::Void
+    |                                          ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered a value of uninhabited type `Void`
32 
33 warning: the type `empty::Empty` does not permit zero-initialization


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the type `!` does not permit zero-initialization
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:4:14
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^ this code causes undefined behavior when executed
   |
   |
   = note: the `!` type has no valid value

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:4:14
   |
   |
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a value of the never type `!`
note: inside `foo`
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:4:14
   |
LL |     unsafe { std::mem::transmute(()) }
LL |     unsafe { std::mem::transmute(()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^
note: inside `FOO`
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:19:33
   |
LL | const FOO: [empty::Empty; 3] = [foo(); 3];

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:21:42
   |
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .0: encountered a value of uninhabited type `Void`

warning: the type `empty::Empty` does not permit zero-initialization
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:21:42
   |
LL | const BAR: [empty::Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
   |
note: in this struct field
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:16:22
   |
   |
LL |     pub struct Empty(Void);
   |                      ^^^^
note: enums with no inhabited variants have no valid value
  --> fake-test-src-base/consts/const-eval/validate_uninhabited_zsts.rs:13:5
LL |     enum Void {}
   |     ^^^^^^^^^

error: aborting due to 2 previous errors; 2 warnings emitted
