plain
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:24:1
+   --> $DIR/ub-enum.rs:25:1
3    |
4 LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
5    | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
10            }
11 
12 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:27:1
-   --> $DIR/ub-enum.rs:27:1
+   --> $DIR/ub-enum.rs:28:1
14    |
15 LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
16    | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

22    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
24 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:31:1
+   --> $DIR/ub-enum.rs:32:1
26    |
26    |
27 LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
28    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

33    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
35 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:44:1
+   --> $DIR/ub-enum.rs:45:1
37    |
37    |
38 LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
39    | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
44            }
45 
46 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:46:1
-   --> $DIR/ub-enum.rs:46:1
+   --> $DIR/ub-enum.rs:47:1
48    |
49 LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
50    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

55    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
57 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:50:1
+   --> $DIR/ub-enum.rs:51:1
59    |
59    |
60 LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
61    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

66    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
68 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-enum.rs:60:42
+   --> $DIR/ub-enum.rs:61:42
70    |
70    |
71 LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
72    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
73 
74 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:65:1
+   --> $DIR/ub-enum.rs:66:1
+   --> $DIR/ub-enum.rs:66:1
76    |
77 LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
78    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

83    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
85 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:83:1
+   --> $DIR/ub-enum.rs:84:1
87    |
87    |
88 LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
89    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
94            }
95 
96 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:85:1
-   --> $DIR/ub-enum.rs:85:1
+   --> $DIR/ub-enum.rs:86:1
98    |
99 LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
100    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
105            }
106 
107 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/ub-enum.rs:93:1
-   --> $DIR/ub-enum.rs:93:1
+   --> $DIR/ub-enum.rs:94:1
109    |
110 LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
111    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
116            }
117 
118 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-enum.rs:98:77
-   --> $DIR/ub-enum.rs:98:77
+   --> $DIR/ub-enum.rs:99:77
120    |
121 LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
122    |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
123 
124 error[E0080]: evaluation of constant value failed
-   --> $DIR/ub-enum.rs:100:77
+   --> $DIR/ub-enum.rs:101:77
+   --> $DIR/ub-enum.rs:101:77
126    |
127 LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
128    |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
132 For more information about this error, try `rustc --explain E0080`.
133 Future incompatibility report: Future breakage diagnostic:
134 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:27:1
-   --> $DIR/ub-enum.rs:27:1
+   --> $DIR/ub-enum.rs:28:1
136    |
137 LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
138    | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
145 
146 Future breakage diagnostic:
147 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:31:1
-   --> $DIR/ub-enum.rs:31:1
+   --> $DIR/ub-enum.rs:32:1
149    |
150 LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
151    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
158 
159 Future breakage diagnostic:
160 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:46:1
-   --> $DIR/ub-enum.rs:46:1
+   --> $DIR/ub-enum.rs:47:1
162    |
163 LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
164    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
171 
172 Future breakage diagnostic:
173 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:50:1
-   --> $DIR/ub-enum.rs:50:1
+   --> $DIR/ub-enum.rs:51:1
175    |
176 LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
177    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
184 
185 Future breakage diagnostic:
186 error: any use of this value will cause an error
-   --> $DIR/ub-enum.rs:65:1
-   --> $DIR/ub-enum.rs:65:1
+   --> $DIR/ub-enum.rs:66:1
188    |
189 LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
190    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes

The actual 64bit.stderr differed from the expected 64bit.stderr.
The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/ub-enum.64bit.stderr
To only update this specific test, also pass `--test-args consts/const-eval/ub-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:28:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:28:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:32:1
   |
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:45:1
   |
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:51:1
   |
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:61:42
   |
   |
LL | const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
   |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ using uninitialized data, but this operation requires initialized memory
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:66:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:84:1
   |
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:86:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:86:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:94:1
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:94:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:99:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:101:77
   |
   |
LL | const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
   |                                                                             ^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
Future incompatibility report: Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:28:1
   |
LL | const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:32:1
   |
   |
LL | const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&1) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:47:1
   |
   |
LL | const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:51:1
   |
   |
LL | const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
Future breakage diagnostic:
error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-enum.rs:66:1
   |
   |
LL | const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(&0) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported



failures:
