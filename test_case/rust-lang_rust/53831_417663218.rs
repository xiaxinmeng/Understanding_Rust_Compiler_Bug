plain
[00:46:50] ....................................................................................................
[00:46:53] ....................................................................................................
[00:46:56] ....................................................................................................
[00:47:00] ........i...........................................................................................
[00:47:04] .......................F............................................................................
[00:47:07] .i.......i..........................................................................................
[00:47:13] ....................................................................................................
[00:47:16] ....................................................................................................
[00:47:18] ....................................................................................................
[00:47:20] ....................................................................................................
---
[00:48:51] 
[00:48:51] ---- [ui] ui/consts/const-eval/const-pointer-values-in-various-types.rs stdout ----
[00:48:51] diff of stderr:
[00:48:51] 
[00:48:51] 1 error[E0080]: this constant likely exhibits undefined behavior
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:24:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:24:5
[00:48:51] 3    |
[00:48:51] 4 LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
[00:48:51] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type usize
[00:48:51] 
[00:48:51] 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:48:51] 9 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:27:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:27:5
[00:48:51] 11    |
[00:48:51] 11    |
[00:48:51] 12 LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
[00:48:51] 
[00:48:51] 17    = note: #[deny(const_err)] on by default
[00:48:51] 18 
[00:48:51] 19 error: this constant cannot be used
[00:48:51] 19 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:30:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:30:5
[00:48:51] 21    |
[00:48:51] 22 LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
[00:48:51] 
[00:48:51] 
[00:48:51] 25    |                                             a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 27 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:33:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:33:5
[00:48:51] 29    |
[00:48:51] 29    |
[00:48:51] 30 LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
[00:48:511] +   --> $DIR/const-pointer-values-in-various-types.rs:42:5
[00:48:51] 51    |
[00:48:51] 52 LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
[00:48:51] 
[00:48:51] 
[00:48:51] 55    |                                           a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 57 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:45:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:45:5
[00:48:51] 59    |
[00:48:51] 59    |
[00:48:51] 60 LL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };
[00:48:51] 
[00:48:51] 
[00:48:51] 63    |                                             a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 65 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:48:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:48:5
[00:48:51] 67    |
[00:48:51] 67    |
[00:48:51] 68 LL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };
[00:48:51] 
[00:48:51] 
[00:48:51] 71    |                                             a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 72 
[00:48:51] 73 error[E0080]: this constant likely e:48:51] 177    |
[00:48:51] 178 LL |     const STR_I32_UNION: i32 = unsafe { Nonsense { stringy: "3" }.int_32 };
[00:48:51] 
[00:48:51] 
[00:48:51] 181    |                                         a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 182 
[00:48:51] 183 error[E0080]: this constant likely exhibits undefined behavior
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:93:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:93:5
[00:48:51] 185    |
[00:48:51] 186 LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
[00:48:51] 187    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type i64
[00:48:51] 
[00:48:51] 189    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:48:51] 191 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:96:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:96:5
[00:48:51] 193    |
[00:48:51] 193    |
[00:48:51] 194 LL |     const STR_I128_UNION: i128 = unsafe { Nonsense { stringy: "3" }.int_128 };
[00:48:51] 
[00:48:51] 
[00:48:51] 197    |                                           a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 199 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:99:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:99:5
[00:48:51] 201    |
[00:48:51] 201    |
[00:48:51] 202 LL |     const STR_F32_UNION: f32 = unsafe { Nonsense { stringy: "3" }.float_32 };
[00:48:51] 
[00:48:51] 
[00:48:51] 205    |                                         a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 206 
[00:48:51] 207 error[E0080]: this constant likely exhibits undefined behavior
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:102:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:102:5
[00:48:51] 209    |
[00:48:51] 210 LL |     const STR_F64_UNION: f64 = unsafe { Nonsense { stringy: "3" }.float_64 };
[00:48:51] 211    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type f64
[00:48:51] 
[00:48:51] 213    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:48:51] 215 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:105:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:105:5
[00:48:51] 217    |
[00:48:51] 217    |
[00:48:51] 218 LL |     const STR_BOOL_UNION: bool = unsafe { Nonsense { stringy: "3" }.truthy_falsey };
[00:48:51] 
[00:48:51] 
[00:48:51] 221    |                                           a raw memory access tried to access part of a pointer value as raw bytes
[00:48:51] 223 error: this constant cannot be used
[00:48:51] -   --> $DIR/const-pointer-values-in-int-types.rs:108:5
[00:48:51] +   --> $DIR/const-pointer-values-in-various-types.rs:108:5
[00:48:51] 225    |
[00:48:51] 225    |
[00:48:51] 226 LL |     const STR_CHAR_UNION: char = unsafe { Nonsense { stringy: "3" }.character };
[00:48:51] 
[00:48:51] 
[00:48:51] The actual stderr differed from the expected stderr.
[00:48:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/const-pointer-values-in-various-types.stderr
[00:48:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/const-pointer-values-in-various-types.stderr
[00:48:51] To update references, rerun the tests and pass the `--bless` flag
[00:48:51] To only update this specific test, also pass `--test-args consts/const-eval/const-pointer-values-in-various-types.rs`
[00:48:51] error: 1 errors occurred comparing output.
[00:48:51] status: exit code: 1
[00:48:51] status: exit code: 1
[00:48:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/tered a pointer, but expected the type usize","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: this constant likely exhibits undefined behavior\n  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:24:5\n   |\nLL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type usize\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[00:48:51] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs","byte_start":566,"byte_end":600,"line_start":27,"line_end":27,"column_start":43,"column_end":77,"is_primary":false,"text":[{"text":"    const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };","highlight_start":43,"highlight_end":77}],"label":"a raw memory access tried to access part of a pointer value as raw bytes"5,"highlight_end":80}],"label":"a raw memory access tried to access part of a pointer value as raw bytes","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs","byte_start":653,"byte_end":731,"line_start":30,"line_end":30,"column_start":5,"column_end":83,"is_primary":true,"text":[{"text":"    const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };","highlight_start":5,"highlight_end":83}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:30:5\n   |\nLL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------------------------------^^^\n   |                                             |\n   |                                             a raw memory access tried to access part of a pointer value as raw bytes\n\n"}
[00:48:51] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs","byte_start":821,"byte_end":856,"line_start":33,"line_end":33,"column_start":45,"column_end":80,"is_primary":false,"text":[{"text":"    const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };","highlight_start":45,"highlight_end":80}],"label":"a raw memory ac information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs","byte_start":909,"byte_end":987,"line_start":36,"line_end":36,"column_start":5,"column_end":83,"is_primary":true,"text":[{"text":"    const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };","highlight_start":5,"highlight_end":83}],"label":"type validation failed: encountered a pointer, but expected the type u64","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0080]: this constant likely exhibits undefined behavior\n  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:36:5\n   |\nLL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected the type u64\n   |\n   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior\n\n"}
[00:48:51] {"message":"this con"byte_end":1389,"line_start":45,"line_end":45,"column_start":5,"column_end":82,"is_primary":true,"text":[{"text":"    const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };","highlight_start":5,"highlight_end":82}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:45:5\n   |\nLL |     const I32_REF_I16_UNION: i16 = unsafe { Nonsense { int_32_ref: &3 }.int_16 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------------------------^^^\n   |                                             |\n   |                                             a raw memory access tried to access part of a pointer value as raw bytes\n\n"}
[00:48:51] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs","byte_start":1479,"byte_end":1513,"line_start":48,"line_end":48,"column_start":45,"column_end":79,"is_primary":false,"text":[{"text":"    const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };","highlight_start":45,"highlight_end":79}],"label":"a raw memory access tried to access part of a pointer value as raw bytes","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs","byte_start":1439,"byte_end":1516,"line_start":48,"line_end":48,"column_start":5,"column_end":82,"is_primary":true,"text":[{"text":"    const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };","highlight_start":5,"highlight_end":82}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/const-pointer-values-in-various-types.rs:48:5\n   |\nLL |     const I32_REF_I32_UNION: i32 = unsafe { Nonsense { int_32_ref: &3 }.int_32 };\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^----------------------------------^^^\n   |                                             |\n   |                                             a raw memory access tried to access part of a pointer value as raw bytes\n\n"}
[00:48:51] {"message":"this constant likely exhibits undefined behavior","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n