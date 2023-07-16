plain
..........................................................i......................................... 1700/11725
.................................................................................................... 1800/11725
..................................................................................i................. 1900/11725
.................................................................................................... 2000/11725
...................................................F................................................ 2100/11725
........................................................................................F........... 2200/11725
.........F.............................F.......................................F..F...FF.FF.F.FFF... 2300/11725
.F..F..F.............................................................F.............................. 2400/11725
.................................................................................................... 2500/11725
............................F......................................F..........................F....F 2600/11725
........F......................................F....................F.........................i..i.. 2700/11725
.................................................................................................... 2900/11725
...iiiii............................................................................................ 3000/11725
.................................................................................................... 3100/11725
.................................................................................................... 3200/11725
---
.................................................................................................... 9400/11725
.................................................................................................... 9500/11725
...................................................................i.......i........................ 9600/11725
.................................................................................................... 9700/11725
.............iiiiiii..iiiiii.i...................................................................... 9800/11725
.................................................................................................... 10000/11725
.................................................................................................... 10100/11725
.................................................................................................... 10200/11725
.................................................................................................... 10300/11725
---
normalized 64bit.stderr:
error[E0308]: mismatched types
  --> $DIR/invalid-patterns.rs:29:21
   |
LL |   get_flag::<false, 0xFF>();
   |                     ^^^^ expected `char`, found `u8`
error[E0308]: mismatched types
  --> $DIR/invalid-patterns.rs:31:14
   |
   |
LL |   get_flag::<7, 'c'>();
   |              ^ expected `bool`, found integer
error[E0308]: mismatched types
  --> $DIR/invalid-patterns.rs:33:14
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |              ^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> $DIR/invalid-patterns.rs:33:18
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |                  ^^^^^ expected `char`, found `u8`
error[E0080]: it is undefined behavior to use this value
  --> $DIR/invalid-patterns.rs:38:21
   |
   |
LL |   get_flag::<false, { unsafe { char_raw.character } }>();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/invalid-patterns.rs:40:14
   |
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, 'z'>();
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/invalid-patterns.rs:42:14
  --> $DIR/invalid-patterns.rs:42:14
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/invalid-patterns.rs:42:47
  --> $DIR/invalid-patterns.rs:42:47
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0080, E0308.
Some errors have detailed explanations: E0080, E0308.
For more information about an error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/invalid-patterns.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/min_const_generics/invalid-patterns.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/invalid-patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:29:21
   |
LL |   get_flag::<false, 0xFF>();
   |                     ^^^^ expected `char`, found `u8`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:31:14
   |
   |
LL |   get_flag::<7, 'c'>();
   |              ^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:14
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |              ^^ expected `bool`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:33:18
   |
   |
LL |   get_flag::<42, 0x5ad>();
   |                  ^^^^^ expected `char`, found `u8`
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:38:21
   |
   |
LL |   get_flag::<false, { unsafe { char_raw.character } }>();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:40:14
   |
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, 'z'>();
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:42:14
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:42:14
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x42, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:42:47
  --> /checkout/src/test/ui/const-generics/min_const_generics/invalid-patterns.rs:42:47
   |
LL |   get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
   |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0080, E0308.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-err4.rs:9:11
   |
LL |     Boo = [unsafe { Foo { b: () }.a }; 4][3],
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/const-err4.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-err4.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-err4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-err4/auxiliary"
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-err4.rs:9:11
   |
LL |     Boo = [unsafe { Foo { b: () }.a }; 4][3],
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:26:5
   |
LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:41:5
   |
LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc18, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:44:5
  --> $DIR/const-pointer-values-in-various-types.rs:44:5
   |
LL |     const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:59:5
   |
LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc38, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:62:5
  --> $DIR/const-pointer-values-in-various-types.rs:62:5
   |
LL |     const I32_REF_I128_UNION: i128 = unsafe { Nonsense { int_32_ref: &3 }.int_128 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_F32_UNION: f32 = unsafe { Nonsense { int_32_ref: &3 }.float_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:69:5
   |
LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc50, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
   |                                               |
   |                                               |
   |                                               unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_CHAR_UNION: char = unsafe { Nonsense { int_32_ref: &3 }.character };
   |                                               |
   |                                               |
   |                                               unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_U8_UNION: u8 = unsafe { Nonsense { stringy: "3" }.uint_8 };
   |                                       |
   |                                       |
   |                                       unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_U16_UNION: u16 = unsafe { Nonsense { stringy: "3" }.uint_16 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_U32_UNION: u32 = unsafe { Nonsense { stringy: "3" }.uint_32 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:92:5
   |
LL |     const STR_U64_UNION: u64 = unsafe { Nonsense { stringy: "3" }.uint_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc71, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
   |
   |
LL |     const STR_U128_UNION: u128 = unsafe { Nonsense { stringy: "3" }.uint_128 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_I8_UNION: i8 = unsafe { Nonsense { stringy: "3" }.int_8 };
   |                                       |
   |                                       |
   |                                       unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_I16_UNION: i16 = unsafe { Nonsense { stringy: "3" }.int_16 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_I32_UNION: i32 = unsafe { Nonsense { stringy: "3" }.int_32 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:111:5
   |
LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc86, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
   |
   |
LL |     const STR_I128_UNION: i128 = unsafe { Nonsense { stringy: "3" }.int_128 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_F32_UNION: f32 = unsafe { Nonsense { stringy: "3" }.float_32 };
   |                                         |
   |                                         |
   |                                         unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-pointer-values-in-various-types.rs:122:5
   |
LL |     const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc95, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
   |
   |
LL |     const STR_BOOL_UNION: bool = unsafe { Nonsense { stringy: "3" }.truthy_falsey };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const STR_CHAR_UNION: char = unsafe { Nonsense { stringy: "3" }.character };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 29 previous errors
error: aborting due to 29 previous errors

For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/const-pointer-values-in-various-types.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-pointer-values-in-various-types.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:26:5
   |
LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:41:5
   |
LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc18, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:44:5
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:44:5
   |
LL |     const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
   |                                           |
   |                                           |
   |                                           unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:59:5
   |
LL |     const I32_REF_I64_UNION: i64 = unsafe { Nonsense { int_32_ref: &3 }.int_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc38, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:62:5
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:62:5
   |
LL |     const I32_REF_I128_UNION: i128 = unsafe { Nonsense { int_32_ref: &3 }.int_128 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_F32_UNION: f32 = unsafe { Nonsense { int_32_ref: &3 }.float_32 };
   |                                             |
   |                                             |
   |                                             unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:69:5
   |
LL |     const I32_REF_F64_UNION: f64 = unsafe { Nonsense { int_32_ref: &3 }.float_64 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc50, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
   |                                               |
   |                                               |
   |                                               unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL |     const I32_REF_CHAR_UNION: char = unsafe { Nonsense { int_32_ref: &3 }.character };
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/alloc_intrinsic_uninit.rs:9:1
   |
LL | const BAR: &i32 = unsafe { &*(intrinsics::const_allocate(4, 4) as *mut i32) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<deref>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit/alloc_intrinsic_uninit.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/heap/alloc_intrinsic_uninit.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_uninit.rs:9:1
   |
LL | const BAR: &i32 = unsafe { &*(intrinsics::const_allocate(4, 4) as *mut i32) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<deref>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: aborting due to previous error

---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ref_to_int_match.rs:26:1
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: could not evaluate constant pattern
  --> $DIR/ref_to_int_match.rs:8:14
  --> $DIR/ref_to_int_match.rs:8:14
   |
LL |         10..=BAR => {},

error: could not evaluate constant pattern
  --> $DIR/ref_to_int_match.rs:8:14
   |
   |
LL |         10..=BAR => {},

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:26:1
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f }; //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc2, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:8:14
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:8:14
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:8:14
   |
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/transmute-const.rs:4:1
   |
LL | static FOO: bool = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const/transmute-const.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/transmute-const.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/transmute-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/transmute-const.rs:4:1
   |
LL | static FOO: bool = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error: aborting due to previous error

---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-int-array.rs:14:1
   |
LL | / const UNINIT_INT_0: [u32; 3] = unsafe {
LL | |
LL | |
LL | |     [
LL | |     ]
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at [0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 12, align: 4) {
               __ __ __ __ 01 00 00 00 02 00 00 00             │ ░░░░........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-int-array.rs:23:1
   |
   |
LL | / const UNINIT_INT_1: [u32; 3] = unsafe {
LL | |
LL | |
LL | |     mem::transmute(
LL | |     )
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at [1]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 12, align: 4) {
               00 00 00 00 01 __ 01 01 02 02 __ 02             │ .....░....░.

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-int-array.rs:43:1
   |
   |
LL | / const UNINIT_INT_2: [u32; 3] = unsafe {
LL | |
LL | |
LL | |     mem::transmute(
LL | |     )
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at [2]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 12, align: 4) {
               00 00 00 00 01 01 01 01 02 02 02 __             │ ...........░

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-int-array/ub-int-array.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-int-array.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-int-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-int-array" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-int-array/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-int-array.rs:14:1
   |
LL | / const UNINIT_INT_0: [u32; 3] = unsafe {
LL | | //~^ ERROR it is undefined behavior to use this value
LL | | //~| type validation failed: encountered uninitialized bytes at [0]
LL | |     [
LL | |     ]
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at [0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 12, align: 4) {
               __ __ __ __ 01 00 00 00 02 00 00 00             │ ░░░░........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-int-array.rs:23:1
   |
   |
LL | / const UNINIT_INT_1: [u32; 3] = unsafe {
LL | | //~^ ERROR it is undefined behavior to use this value
LL | | //~| type validation failed: encountered uninitialized bytes at [1]
LL | |     mem::transmute(
LL | |     )
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at [1]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 12, align: 4) {
               00 00 00 00 01 __ 01 01 02 02 __ 02             │ .....░....░.

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-int-array.rs:43:1
   |
   |
LL | / const UNINIT_INT_2: [u32; 3] = unsafe {
LL | | //~^ ERROR it is undefined behavior to use this value
LL | | //~| type validation failed: encountered uninitialized bytes at [2]
LL | |     mem::transmute(
LL | |     )
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at [2]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 12, align: 4) {
               00 00 00 00 01 01 01 01 02 02 02 __             │ ...........░

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-nonnull.rs:12:1
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: any use of this value will cause an error
  --> $DIR/ub-nonnull.rs:19:30
   |
LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
LL | |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
LL | |     // Use address-of-element for pointer arithmetic. This could wrap around to NULL!
LL | |     let out_of_bounds_ptr = &ptr[255];
LL | |
LL | |     mem::transmute(out_of_bounds_ptr)
LL | | } };
   | |____-
   | |____-
   |
note: the lint level is defined here
  --> $DIR/ub-nonnull.rs:15:8
   |
LL | #[deny(const_err)] // this triggers a `const_err` so validation does not even happen
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-nonnull.rs:24:1
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-nonnull.rs:26:1
  --> $DIR/ub-nonnull.rs:26:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-nonnull.rs:34:1
  --> $DIR/ub-nonnull.rs:34:1
   |
LL | const UNINIT: NonZeroU8 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .0, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               __                                              │ ░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-nonnull.rs:42:1
   |
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-nonnull.rs:48:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: aborting due to 7 previous errors


For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/ub-nonnull.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-nonnull.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-nonnull/auxiliary"
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


error: any use of this value will cause an error
   |
   |
LL | / const OUT_OF_BOUNDS_PTR: NonNull<u8> = { unsafe {
LL | |     let ptr: &[u8; 256] = mem::transmute(&0u8); // &0 gets promoted so it does not dangle
LL | |     // Use address-of-element for pointer arithmetic. This could wrap around to NULL!
LL | |     let out_of_bounds_ptr = &ptr[255]; //~ ERROR any use of this value will cause an error
   | |                              ^^^^^^^^ memory access failed: pointer must be in-bounds at offset 256, but is outside bounds of alloc10 which has size 1
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | |     mem::transmute(out_of_bounds_ptr)
LL | | } };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:15:8
   |
   |
LL | #[deny(const_err)] // this triggers a `const_err` so validation does not even happen
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-nonnull.rs:24:1
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
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-upvars.rs:6:1
   |
LL | / const BAD_UPVAR: &dyn FnOnce() = &{
LL | |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
LL | |     let another_var = 13;
LL | |     move || { let _ = bad_ref; let _ = another_var; }
LL | | };
   | |__^ type validation failed: encountered a NULL reference at .<deref>.<dyn-downcast>.<captured-var(bad_ref)>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc2────────╼ ╾───────alloc3────────╼ │ ╾──────╼╾──────╼

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/ub-upvars.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-upvars.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-upvars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-upvars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-upvars.rs:6:1
   |
LL | / const BAD_UPVAR: &dyn FnOnce() = &{ //~ ERROR it is undefined behavior to use this value
LL | |     let bad_ref: &'static u16 = unsafe { mem::transmute(0usize) };
LL | |     let another_var = 13;
LL | |     move || { let _ = bad_ref; let _ = another_var; }
LL | | };
   | |__^ type validation failed: encountered a NULL reference at .<deref>.<dyn-downcast>.<captured-var(bad_ref)>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc2────────╼ ╾───────alloc3────────╼ │ ╾──────╼╾──────╼

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:24:1
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x0000000000000001 at .<enum-tag>, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:27:1
  --> $DIR/ub-enum.rs:27:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc8 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:30:1
  --> $DIR/ub-enum.rs:30:1
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc12 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:42:1
  --> $DIR/ub-enum.rs:42:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x0000000000000000 at .<enum-tag>, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:44:1
  --> $DIR/ub-enum.rs:44:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc18 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:47:1
  --> $DIR/ub-enum.rs:47:1
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc22 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:56:1
  --> $DIR/ub-enum.rs:56:1
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:60:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc28 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:77:1
  --> $DIR/ub-enum.rs:77:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(B)>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:79:1
  --> $DIR/ub-enum.rs:79:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(D)>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:87:1
  --> $DIR/ub-enum.rs:87:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0xffffffff at .<enum-variant(Some)>.0.1, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:92:1
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(Ok)>.0.1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-enum.rs:94:1
  --> $DIR/ub-enum.rs:94:1
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(Ok)>.0.1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error: aborting due to 13 previous errors


For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:24:1
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x0000000000000001 at .<enum-tag>, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:27:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc8 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc12 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:42:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:42:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x0000000000000000 at .<enum-tag>, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:44:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:44:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc18 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc22 at .0.<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:56:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:56:1
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:60:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc28 at .<enum-tag>, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:77:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:77:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(B)>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:79:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:79:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(D)>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:87:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:87:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0xffffffff at .<enum-variant(Some)>.0.1, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:92:1
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Never at .<enum-variant(Ok)>.0.1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:94:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:94:1
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<enum-variant(Ok)>.0.1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error: aborting due to 13 previous errors

---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:13:1
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:17:1
  --> $DIR/ub-ref-ptr.rs:17:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:21:1
  --> $DIR/ub-ref-ptr.rs:21:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:24:1
  --> $DIR/ub-ref-ptr.rs:24:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:30:1
  --> $DIR/ub-ref-ptr.rs:30:1
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc14, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:33:1
  --> $DIR/ub-ref-ptr.rs:33:1
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:36:1
  --> $DIR/ub-ref-ptr.rs:36:1
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:39:1
  --> $DIR/ub-ref-ptr.rs:39:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (created from integer)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:42:1
  --> $DIR/ub-ref-ptr.rs:42:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (created from integer)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:45:1
  --> $DIR/ub-ref-ptr.rs:45:1
   |
LL | const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-ref-ptr.rs:47:1
   |
   |
LL | const UNINIT_FN_PTR: fn() = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/ub-ref-ptr.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-ref-ptr.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:13:1
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a NULL box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:30:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:30:1
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc14, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:33:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:33:1
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:36:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:36:1
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer at .<deref>, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:39:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:39:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (created from integer)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:42:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:42:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (created from integer)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:45:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:45:1
   |
LL | const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:47:1
   |
   |
LL | const UNINIT_FN_PTR: fn() = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/union-ice.rs:15:1
   |
LL | const FIELD3: Field3 = unsafe { UNION.field3 };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/union-ice.rs:17:1
   |
   |
LL | / const FIELD_PATH: Struct = Struct {
LL | |     a: 42,
LL | |     b: unsafe { UNION.field3 },
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at .b, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ 2a __ __ __ __ __ __ __ │ ░░░░░░░░*░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/union-ice.rs:27:1
   |
   |
LL | / const FIELD_PATH2: Struct2 = Struct2 {
LL | |     b: [
LL | |         21,
LL | |         unsafe { UNION.field3 },
LL | |     a: 42,
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at .b[1]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 40, align: 8) {
               0x00 │ 15 00 00 00 00 00 00 00 __ __ __ __ __ __ __ __ │ ........░░░░░░░░
               0x10 │ 17 00 00 00 00 00 00 00 18 00 00 00 00 00 00 00 │ ................
               0x20 │ 2a __ __ __ __ __ __ __                         │ *░░░░░░░

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/union-ice.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/union-ice.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:15:1
   |
LL | const FIELD3: Field3 = unsafe { UNION.field3 }; //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:17:1
   |
   |
LL | / const FIELD_PATH: Struct = Struct { //~ ERROR it is undefined behavior to use this value
LL | |     a: 42,
LL | |     b: unsafe { UNION.field3 },
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at .b, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ 2a __ __ __ __ __ __ __ │ ░░░░░░░░*░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ice.rs:27:1
   |
   |
LL | / const FIELD_PATH2: Struct2 = Struct2 { //~ ERROR it is undefined behavior to use this value
LL | |     b: [
LL | |         21,
LL | |         unsafe { UNION.field3 },
LL | |     a: 42,
LL | | };
LL | | };
   | |__^ type validation failed: encountered uninitialized bytes at .b[1]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 40, align: 8) {
               0x00 │ 15 00 00 00 00 00 00 00 __ __ __ __ __ __ __ __ │ ........░░░░░░░░
               0x10 │ 17 00 00 00 00 00 00 00 18 00 00 00 00 00 00 00 │ ................
               0x20 │ 2a __ __ __ __ __ __ __                         │ *░░░░░░░

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/union-const-eval-field.rs:29:5
   |
LL |     const FIELD3: Field3 = unsafe { UNION.field3 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field/union-const-eval-field.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/union-const-eval-field.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-const-eval-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-const-eval-field/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-const-eval-field.rs:29:5
   |
LL |     const FIELD3: Field3 = unsafe { UNION.field3 };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected initialized plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               __ __ __ __ __ __ __ __                         │ ░░░░░░░░

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-uninhabit.rs:15:1
   |
LL | const BAD_BAD_BAD: Bar = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-uninhabit.rs:18:1
   |
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar at .<deref>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-uninhabit.rs:21:1
  --> $DIR/ub-uninhabit.rs:21:1
   |
LL | const BAD_BAD_ARRAY: [Bar; 1] = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar at [0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.




The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/ub-uninhabit.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-uninhabit.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-uninhabit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:15:1
   |
LL | const BAD_BAD_BAD: Bar = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:18:1
   |
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar at .<deref>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:21:1
  --> /checkout/src/test/ui/consts/const-eval/ub-uninhabit.rs:21:1
   |
LL | const BAD_BAD_ARRAY: [Bar; 1] = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of uninhabited type Bar at [0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 0, align: 1) {}
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.

---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/union-ub.rs:33:1
   |
LL | const BAD_BOOL: bool = unsafe { DummyUnion { u8: 42 }.bool};
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x2a, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               2a                                              │ *

error[E0080]: it is undefined behavior to use this value
  --> $DIR/union-ub.rs:35:1
   |
   |
LL | const UNINIT_BOOL: bool = unsafe { DummyUnion { unit: () }.bool};
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               __                                              │ ░

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ub/union-ub.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/union-ub.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union-ub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union-ub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ub.rs:33:1
   |
LL | const BAD_BOOL: bool = unsafe { DummyUnion { u8: 42 }.bool};
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x2a, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               2a                                              │ *

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/union-ub.rs:35:1
   |
   |
LL | const UNINIT_BOOL: bool = unsafe { DummyUnion { unit: () }.bool};
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized bytes, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
               __                                              │ ░

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:38:1
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN────────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:40:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object at .0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:43:1
   |
   |
LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ ╾───────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:46:1
   |
   |
LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ ╾───────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:48:1
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:52:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized data in `str` at .<deref>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:55:1
   |
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized data in `str` at .<deref>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ 01 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:62:1
   |
   |
LL | / const SLICE_LENGTH_UNINIT: &[u8] = unsafe {
LL | |
LL | |     let uninit_len = MaybeUninit::<usize> { uninit: () };
LL | |     mem::transmute((42, uninit_len))
LL | | };
   | |__^ type validation failed: encountered uninitialized reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:68:1
   |
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:71:1
   |
   |
LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ ╾───────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:74:1
   |
   |
LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:77:1
   |
   |
LL | const SLICE_LENGTH_PTR_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────allocN───────╼ ╾───────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:81:1
   |
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03 at .<deref>[0], but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               ╾───────allocN───────╼                         │ ╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:87:1
   |
   |
LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03 at .<deref>.0, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               ╾───────allocN───────╼                         │ ╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:90:1
   |
   |
LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03 at .<deref>.1[0], but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               ╾───────allocN───────╼                         │ ╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:97:1
   |
   |
LL | / const RAW_SLICE_LENGTH_UNINIT: *const [u8] = unsafe {
LL | |
LL | |     let uninit_len = MaybeUninit::<usize> { uninit: () };
LL | |     mem::transmute((42, uninit_len))
LL | | };
   | |__^ type validation failed: encountered uninitialized raw pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:105:1
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_1: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u8))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too small vtable at .0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:108:1
   |
   |
LL | const TRAIT_OBJ_SHORT_VTABLE_2: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &3u64))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too small vtable at .0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:111:1
   |
   |
LL | const TRAIT_OBJ_INT_VTABLE: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, 4usize))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling vtable pointer in wide pointer at .0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ 04 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:113:1
   |
   |
LL | const TRAIT_OBJ_UNALIGNED_VTABLE: &dyn Trait = unsafe { mem::transmute((&92u8, &[0u8; 128])) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered unaligned vtable pointer in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:115:1
   |
   |
LL | const TRAIT_OBJ_BAD_DROP_FN_NULL: &dyn Trait = unsafe { mem::transmute((&92u8, &[0usize; 8])) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:117:1
   |
   |
LL | const TRAIT_OBJ_BAD_DROP_FN_INT: &dyn Trait = unsafe { mem::transmute((&92u8, &[1usize; 8])) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:119:1
   |
   |
LL | const TRAIT_OBJ_BAD_DROP_FN_NOT_FN_PTR: W<&dyn Trait> = unsafe { mem::transmute(W((&92u8, &[&42u8; 8]))) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid drop function pointer in vtable (not pointing to a function) at .0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:123:1
   |
   |
LL | const TRAIT_OBJ_CONTENT_INVALID: &dyn Trait = unsafe { mem::transmute::<_, &bool>(&3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03 at .<deref>.<dyn-downcast>, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:127:1
   |
   |
LL | const RAW_TRAIT_OBJ_VTABLE_NULL: *const dyn Trait = unsafe { mem::transmute((&92u8, 0usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered dangling vtable pointer in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ 00 00 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> $DIR/ub-wide-ptr.rs:129:1
   |
   |
LL | const RAW_TRAIT_OBJ_VTABLE_INVALID: *const dyn Trait = unsafe { mem::transmute((&92u8, &3u64)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered too small vtable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾──────allocN───────╼ ╾──────allocN───────╼ │ ╾──────╼╾──────╼

error[E0080]: could not evaluate static initializer
  --> $DIR/ub-wide-ptr.rs:135:5
   |
   |
LL |     mem::transmute::<_, &dyn Trait>((&92u8, 0usize))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ inbounds test failed: 0x0 is not a valid pointer
error[E0080]: could not evaluate static initializer
  --> $DIR/ub-wide-ptr.rs:139:5
   |
   |
LL |     mem::transmute::<_, &dyn Trait>((&92u8, &3u64))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ memory access failed: pointer must be in-bounds at offset N, but is outside bounds of allocN which has size N
error: aborting due to 28 previous errors

For more information about this error, try `rustc --explain E0080`.




The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/ub-wide-ptr.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/ub-wide-ptr.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:38:1
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc7────────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:40:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object at .0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc13───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:43:1
   |
   |
LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc19───────╼ ╾───────alloc21───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:46:1
   |
   |
LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc26───────╼ ╾───────alloc28───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:48:1
   |
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc33───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:52:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized data in `str` at .<deref>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:55:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:55:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized data in `str` at .<deref>.0
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:62:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:62:1
   |
LL | / const SLICE_LENGTH_UNINIT: &[u8] = unsafe {
LL | | //~^ ERROR it is undefined behavior to use this value
LL | |     let uninit_len = MaybeUninit::<usize> { uninit: () };
LL | |     mem::transmute((42, uninit_len))
LL | | };
   | |__^ type validation failed: encountered uninitialized reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:68:1
   |
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc58───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:71:1
   |
   |
LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc63───────╼ ╾───────alloc65───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:74:1
   |
   |
LL | const SLICE_TOO_LONG_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, 999usize)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (going beyond the bounds of its allocation)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc70───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:77:1
   |
   |
LL | const SLICE_LENGTH_PTR_BOX: Box<[u8]> = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered non-integer slice length in wide pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc75───────╼ ╾───────alloc77───────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:81:1
   |
   |
LL | const SLICE_CONTENT_INVALID: &[bool] = &[unsafe { mem::transmute(3u8) }];
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03 at .<deref>[0], but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:87:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:87:1
   |
LL | const MYSLICE_PREFIX_BAD: &MySliceBool = &MySlice(unsafe { mem::transmute(3u8) }, [false]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03 at .<deref>.0, but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:90:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:90:1
   |
LL | const MYSLICE_SUFFIX_BAD: &MySliceBool = &MySlice(true, [unsafe { mem::transmute(3u8) }]);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0x03 at .<deref>.1[0], but expected a boolean
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:97:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:97:1
   |
LL | / const RAW_SLICE_LENGTH_UNINIT: *const [u8] = unsafe {
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const-points-to-static.rs:6:1
   |
LL | const TEST: &u8 = &MY_STATIC;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL | const TEST: &u8 = &MY_STATIC;

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-points-to-static/const-points-to-static.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-points-to-static.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-points-to-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-points-to-static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-points-to-static/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-points-to-static.rs:6:1
   |
LL | const TEST: &u8 = &MY_STATIC;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL | const TEST: &u8 = &MY_STATIC;

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/issue-63952.rs:17:1
   |
LL | / const SLICE_WAY_TOO_LONG: &[u8] = unsafe {
LL | |     SliceTransmute {
LL | |         repr: SliceRepr {
LL | |             ptr: &42,
LL | |     .slice
LL | | };
LL | | };
   | |__^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc3────────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-63952/issue-63952.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-63952.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-63952.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-63952" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-63952/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/issue-63952.rs:17:1
   |
LL | / const SLICE_WAY_TOO_LONG: &[u8] = unsafe { //~ ERROR: it is undefined behavior to use this value
LL | |     SliceTransmute {
LL | |         repr: SliceRepr {
LL | |             ptr: &42,
LL | |     .slice
LL | | };
LL | | };
   | |__^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc3────────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/issue-79690.rs:30:1
   |
LL | const G: Fat = unsafe { Transmute { t: FOO }.u };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered (potentially part of) a pointer at .1.<deref>.size.foo, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc2────────╼ ╾───────alloc3────────╼ │ ╾──────╼╾──────╼

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79690/issue-79690.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-79690.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-79690.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79690" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-79690/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/issue-79690.rs:30:1
   |
LL | const G: Fat = unsafe { Transmute { t: FOO }.u };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered (potentially part of) a pointer at .1.<deref>.size.foo, but expected plain (non-pointer) bytes
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc2────────╼ ╾───────alloc3────────╼ │ ╾──────╼╾──────╼

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const_refers_to_static2.rs:11:1
   |
LL | / const REF_INTERIOR_MUT: &usize = {
LL | |
LL | |
LL | |
LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
LL | |     unsafe { &*(&FOO as *const _ as *const usize) }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/const_refers_to_static2.rs:20:1
  --> $DIR/const_refers_to_static2.rs:20:1
   |
LL | / const READ_IMMUT: &usize = {
LL | |
LL | |
LL | |
LL | |     static FOO: usize = 0;
LL | |     &FOO
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
help: skipping check for `const_raw_ptr_deref` feature
  --> $DIR/const_refers_to_static2.rs:16:14
   |
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
LL |     &FOO
   |      ^^^


error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2/const_refers_to_static2.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:11:1
   |
LL | / const REF_INTERIOR_MUT: &usize = { //~ ERROR undefined behavior to use this value
LL | | //~| NOTE encountered a reference pointing to a static variable
LL | | //~| NOTE undefined behavior
LL | | //~| NOTE the raw bytes of the constant
LL | |     static FOO: AtomicUsize = AtomicUsize::new(0);
LL | |     unsafe { &*(&FOO as *const _ as *const usize) }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:20:1
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:20:1
   |
LL | / const READ_IMMUT: &usize = { //~ ERROR it is undefined behavior to use this value
LL | | //~| NOTE encountered a reference pointing to a static variable
LL | | //~| NOTE undefined behavior
LL | | //~| NOTE the raw bytes of the constant
LL | |     static FOO: usize = 0;
LL | |     &FOO
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
help: skipping check for `const_raw_ptr_deref` feature
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static2.rs:16:14
   |
   |
LL |     unsafe { &*(&FOO as *const _ as *const usize) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
LL |     &FOO
   |      ^^^

---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/mutable_references_err.rs:17:1
   |
LL | / const MUH: Meh = Meh {
LL | |     x: &UnsafeCell::new(42),
LL | | };
   | |__^ type validation failed: encountered `UnsafeCell` in a `const` at .x.<deref>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/mutable_references_err.rs:27:1
  --> $DIR/mutable_references_err.rs:27:1
   |
LL | const SNEAKY: &dyn Sync = &Synced { x: UnsafeCell::new(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered `UnsafeCell` in a `const` at .<deref>.<dyn-downcast>.x
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc6────────╼ ╾───────alloc7────────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> $DIR/mutable_references_err.rs:31:1
   |
   |
LL | const BLUNT: &mut i32 = &mut 42;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered mutable reference in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL |     x: &UnsafeCell::new(42),
   |        ^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL | const SNEAKY: &dyn Sync = &Synced { x: UnsafeCell::new(42) };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL | const BLUNT: &mut i32 = &mut 42;

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_err/mutable_references_err.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references_err.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references_err/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:17:1
   |
LL | / const MUH: Meh = Meh { //~ ERROR: it is undefined behavior to use this value
LL | |     x: &UnsafeCell::new(42),
LL | | };
   | |__^ type validation failed: encountered `UnsafeCell` in a `const` at .x.<deref>
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:27:1
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:27:1
   |
LL | const SNEAKY: &dyn Sync = &Synced { x: UnsafeCell::new(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered `UnsafeCell` in a `const` at .<deref>.<dyn-downcast>.x
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc6────────╼ ╾───────alloc7────────╼ │ ╾──────╼╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references_err.rs:31:1
   |
   |
LL | const BLUNT: &mut i32 = &mut 42;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered mutable reference in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL |     x: &UnsafeCell::new(42),
   |        ^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL | const SNEAKY: &dyn Sync = &Synced { x: UnsafeCell::new(42) };
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL | const BLUNT: &mut i32 = &mut 42;

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/const_refers_to_static_cross_crate.rs:12:1
   |
LL | / const SLICE_MUT: &[u8; 1] = {
LL | |
LL | |
LL | |
LL | |     unsafe { &static_cross_crate::ZERO }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:47:9
  --> $DIR/const_refers_to_static_cross_crate.rs:47:9
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error[E0080]: it is undefined behavior to use this value
  --> $DIR/const_refers_to_static_cross_crate.rs:19:1
   |
LL | / const U8_MUT: &u8 = {
LL | |
LL | |
LL | |
LL | |     unsafe { &static_cross_crate::ZERO[0] }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:56:9
  --> $DIR/const_refers_to_static_cross_crate.rs:56:9
   |
LL |         U8_MUT => true,


warning: any use of this value will cause an error
   |
   |
LL | / const U8_MUT2: &u8 = {
LL | |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |
LL | |
LL | |
LL | |
LL | | };
   |
note: the lint level is defined here
  --> $DIR/const_refers_to_static_cross_crate.rs:27:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:67:9
   |
LL |         U8_MUT2 => true,


warning: any use of this value will cause an error
   |
   |
LL | / const U8_MUT3: &u8 = {
LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   | |                                                   ^^^^^^^^^^^ constant accesses static
LL | |
LL | |
LL | |
LL | | };
   | |__-
   |
   |
note: the lint level is defined here
  --> $DIR/const_refers_to_static_cross_crate.rs:35:8
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:75:9
   |
LL |         U8_MUT3 => true,

error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:47:9
   |
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:56:9
   |
LL |         U8_MUT => true,

error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:67:9
   |
   |
LL |         U8_MUT2 => true,

error: could not evaluate constant pattern
  --> $DIR/const_refers_to_static_cross_crate.rs:75:9
   |
   |
LL |         U8_MUT3 => true,

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
LL |     unsafe { &static_cross_crate::ZERO }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
LL |     unsafe { &static_cross_crate::ZERO }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check for `const_panic` feature
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                                                                             ^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors; 3 warnings emitted


For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/const_refers_to_static_cross_crate.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static_cross_crate.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:12:1
   |
LL | / const SLICE_MUT: &[u8; 1] = { //~ ERROR undefined behavior to use this value
LL | | //~| NOTE encountered a reference pointing to a static variable
LL | | //~| NOTE undefined behavior
LL | | //~| NOTE the raw bytes of the constant
LL | |     unsafe { &static_cross_crate::ZERO }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:47:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:47:9
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:19:1
   |
LL | / const U8_MUT: &u8 = { //~ ERROR undefined behavior to use this value
LL | | //~| NOTE encountered a reference pointing to a static variable
LL | | //~| NOTE undefined behavior
LL | | //~| NOTE the raw bytes of the constant
LL | |     unsafe { &static_cross_crate::ZERO[0] }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:56:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:56:9
   |
LL |         U8_MUT => true,


warning: any use of this value will cause an error
   |
   |
LL | / const U8_MUT2: &u8 = { //~ NOTE
LL | |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| NOTE constant accesses static
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | |     //~| NOTE
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:27:8
   |
   |
LL | #[warn(const_err)] //~ NOTE
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:67:9
   |
LL |         U8_MUT2 => true,


warning: any use of this value will cause an error
   |
   |
LL | / const U8_MUT3: &u8 = { //~ NOTE
LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   | |                                                   ^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| NOTE constant accesses static
...  |
LL | |     //~| NOTE
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:35:8
   |
   |
LL | #[warn(const_err)] //~ NOTE
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:75:9
   |
LL |         U8_MUT3 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:47:9
   |
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:56:9
   |
LL |         U8_MUT => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:67:9
   |
   |
LL |         U8_MUT2 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:75:9
   |
   |
LL |         U8_MUT3 => true,

warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
LL |     unsafe { &static_cross_crate::ZERO }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
LL |     unsafe { &static_cross_crate::ZERO }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check for `const_panic` feature
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   |                                                                             ^^^^^^^^
help: skipping check that does not even have a feature gate
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors; 3 warnings emitted

---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/alloc.rs:8:1
   |
LL | const LAYOUT_INVALID: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0 at .align_, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc/alloc.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/std/alloc.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/std/alloc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/std/alloc.rs:8:1
   |
LL | const LAYOUT_INVALID: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 0 at .align_, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error: aborting due to previous error

---
normalized 64bit.stderr:
error[E0080]: it is undefined behavior to use this value
  --> $DIR/validate_never_arrays.rs:4:1
   |
LL | const _: &[!; 1] = unsafe { &*(1_usize as *const [!; 1]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<deref>[0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/validate_never_arrays.rs:7:1
  --> $DIR/validate_never_arrays.rs:7:1
   |
LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 1]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<deref>[0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> $DIR/validate_never_arrays.rs:8:1
  --> $DIR/validate_never_arrays.rs:8:1
   |
LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 42]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<deref>[0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               01 00 00 00 00 00 00 00 2a 00 00 00 00 00 00 00 │ ........*.......

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.



The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/validate_never_arrays/validate_never_arrays.64bit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/validate_never_arrays.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/validate_never_arrays.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/validate_never_arrays" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/validate_never_arrays/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:4:1
   |
LL | const _: &[!; 1] = unsafe { &*(1_usize as *const [!; 1]) }; //~ ERROR undefined behavior
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<deref>[0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:7:1
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:7:1
   |
LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 1]) }; //~ ERROR undefined behavior
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<deref>[0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:8:1
  --> /checkout/src/test/ui/consts/validate_never_arrays.rs:8:1
   |
LL | const _: &[!] = unsafe { &*(1_usize as *const [!; 42]) }; //~ ERROR undefined behavior
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of the never type `!` at .<deref>[0]
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               01 00 00 00 00 00 00 00 2a 00 00 00 00 00 00 00 │ ........*.......

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.
---
test result: FAILED. 11604 passed; 25 failed; 96 ignored; 0 measured; 0 filtered out; finished in 135.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:00
