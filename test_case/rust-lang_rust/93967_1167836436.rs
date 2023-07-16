plain
........................................................................................ 2200/13117
........................................................................................ 2288/13117
........................................................................................ 2376/13117
..............................................................i......................... 2464/13117
...............................................................F...........ii..F...F.... 2552/13117
.........F.............................................................................. 2640/13117
...i................................................................i.i................. 2816/13117
..........i................i..............................i.........................F... 2904/13117
.....................................i.................................................. 2992/13117
...i.................................................................................... 3080/13117
---
diff of 32bit.stderr:

27   --> $DIR/ref_to_int_match.rs:25:27
28    |
29 LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
-    |                           |
-    |                           |
-    |                           unable to turn pointer into raw bytes
+    | --------------            ^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
33    |
34    = note: `#[deny(const_err)]` on by default
thread 'main' panicked at 'I/O failure during tests: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/tools/compiletest/src/main.rs:433:13


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/ref_to_int_match.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ref_to_int_match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ref_to_int_match/auxiliary"
stdout: none
--- stderr -------------------------------
error: any use of this value will cause an error
   |
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
   | --------------            ^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/const-eval/ref_to_int_match.rs:7:14
   |
   |
LL |         10..=BAR => {}, //~ ERROR could not evaluate constant pattern

error: aborting due to 3 previous errors


Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const BAR: Int = unsafe { Foo { r: &42 }.f };
   | --------------            ^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/const-eval/ub-enum.rs stdout ----
diff of 32bit.stderr:

130   --> $DIR/ub-enum.rs:26:1
131    |
132 LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
134    |
135    = note: `#[deny(const_err)]` on by default

141   --> $DIR/ub-enum.rs:30:1
142    |
142    |
143 LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
145    |
146    = note: `#[deny(const_err)]` on by default

152   --> $DIR/ub-enum.rs:45:1
153    |
153    |
154 LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
156    |
157    = note: `#[deny(const_err)]` on by default

163   --> $DIR/ub-enum.rs:49:1
164    |
164    |
165 LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
167    |
168    = note: `#[deny(const_err)]` on by default

174   --> $DIR/ub-enum.rs:63:1
175    |
175    |
176 LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
178    |
179    = note: `#[deny(const_err)]` on by default


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered 0x00000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:26:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:26:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:43:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-tag>: encountered 0x00000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:45:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:45:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:49:1
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:59:1
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   |
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:63:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:81:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:83:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:83:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:91:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:91:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:96:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:98:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
   |
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:30:1
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:45:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:49:1
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

Future breakage diagnostic:
error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:63:1
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------



---- [ui] src/test/ui/consts/const-eval/ub-ref-ptr.rs stdout ----
diff of 32bit.stderr:

173   --> $DIR/ub-ref-ptr.rs:31:1
174    |
175 LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
177    |
178    = note: `#[deny(const_err)]` on by default

184   --> $DIR/ub-ref-ptr.rs:35:39
185    |
185    |
186 LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
-    |                                       |
-    |                                       |
-    |                                       unable to turn pointer into raw bytes
+    | ----------------------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
190    |
191    = note: `#[deny(const_err)]` on by default

197   --> $DIR/ub-ref-ptr.rs:35:38
198    |
198    |
199 LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
-    |                                      |
-    |                                      referenced constant has errors
+    | ----------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
203    |
203    |
204    = note: `#[deny(const_err)]` on by default

210   --> $DIR/ub-ref-ptr.rs:41:86
211    |
211    |
212 LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
-    |                                                                                      |
-    |                                                                                      |
-    |                                                                                      unable to turn pointer into raw bytes
+    | ------------------------------------------                                           ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
216    |
217    = note: `#[deny(const_err)]` on by default

223   --> $DIR/ub-ref-ptr.rs:41:85
224    |
224    |
225 LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
-    |                                                                                     |
-    |                                                                                     referenced constant has errors
+    | ------------------------------------------                                          ^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
229    |
229    |
230    = note: `#[deny(const_err)]` on by default


The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/ub-ref-ptr.32bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-ref-ptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:17:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:21:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^ type validation failed: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:24:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a null box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:31:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:31:1
   |
LL | const REF_AS_USIZE: usize = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:39
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:39
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   | ----------------------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:35:38
   |
LL | const REF_AS_USIZE_SLICE: &[usize] = &[unsafe { mem::transmute(&0) }];
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:41:86
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   | ------------------------------------------                                           ^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:41:85
   |
LL | const REF_AS_USIZE_BOX_SLICE: Box<[usize]> = unsafe { mem::transmute::<&[usize], _>(&[mem::transmute(&0)]) };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:47:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling reference (address 0x539 is unallocated)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:50:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:50:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a dangling box (address 0x539 is unallocated)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:53:1
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:53:1
   |
LL | const UNINIT_PTR: *const i32 = unsafe { MaybeUninit { uninit: () }.init };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered uninitialized raw pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               __ __ __ __                                     │ ░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs:56:1
   |
   |
LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
   |
