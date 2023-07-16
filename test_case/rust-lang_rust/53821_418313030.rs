plain
[00:44:08] ....................................................................................................
[00:44:11] ....................................................................................................
[00:44:14] ....................................................................................................
[00:44:17] ........i...........................................................................................
[00:44:21] .......................F.........................F............F................................F....
[00:44:27] ............................ii..iii.................................................................
[00:44:30] ....................................................................................................
[00:44:32] ....................................................................................................
[00:44:34] ....................................................................................................
---
[00:44:52] ....................................................................................................
[00:44:55] ....................................................................................................
[00:44:58] ....................................................................................................
[00:45:01] ....................................................................................................
[00:45:04] .......................................................................F............................
[00:45:07] ......................................................................................F.............
[00:45:13] ....................................................................................................
[00:45:16] ..........................................................i.........................................
[00:45:19] ....................................................................................................
[00:45:22] ........i.i..ii.....................................................................................
---
[00:46:00] 
[00:46:00] ---- [ui] ui/consts/const-eval/const-pointer-values-in-various-types.rs stdout ----
[00:46:00] diff of stderr:
[00:46:00] 
[00:46:00] - error[E0080]: this constant likely exhibits undefined behavior
[00:46:00] + error[E0080]: it is undefined behavior to use this value
[00:46:00] 3    |
[00:46:00] 3    |
[00:46:00] 4 LL |     const I32_REF_USIZE_UNION: usize = unsafe { Nonsense { int_32_ref: &3 }.u };
[00:46:00] 6    |
[00:46:00] 6    |
[00:46:00] 7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 11    |
[00:46:00] 11    |
[00:46:00] 12 LL |     const I32_REF_U8_UNION: u8 = unsafe { Nonsense { int_32_ref: &3 }.uint_8 };
[00:46:00] 16    |
[00:46:00] 17    = note: #[deny(const_err)] on by default
[00:46:00] 18 
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 21    |
[00:46:00] 21    |
[00:46:00] 22 LL |     const I32_REF_U16_UNION: u16 = unsafe { Nonsense { int_32_ref: &3 }.uint_16 };
[00:46:00] 24    |                                             |
[00:46:00] 24    |                                             |
[00:46:00] 25    |                                             a raw memory access tried to access part of a pointer value as raw bytes
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 29    |
[00:46:00] 29    |
[00:46:00] 30 LL |     const I32_REF_U32_UNION: u32 = unsafe { Nonsense { int_32_ref: &3 }.uint_32 };
[00:46:00] 32    |                                             |
[00:46:00] 32    |                                             |
[00:46:00] 33    |                                             a raw memory access tried to access part of a pointer value as raw bytes
[00:46:00] 34 
[00:46:00] - error[E0080]: this constant likely exhibits undefined behavior
[00:46:00] + error[E0080]: it is undefined behavior to use this value
[00:46:00] 37    |
[00:46:00] 37    |
[00:46:00] 38 LL |     const I32_REF_U64_UNION: u64 = unsafe { Nonsense { int_32_ref: &3 }.uint_64 };
[00:46:00] 40    |
[00:46:00] 40    |
[00:46:00] 41    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 45    |
[00:46:00] 45    |
[00:46:00] 46 LL |     const I32_REF_U128_UNION: u128 = unsafe { Nonsense { int_32_ref: &3 }.uint_128 };
[00:46:00] 47    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempted to read undefined bytes
[00:46:00] 48 
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 51    |
[00:46:00] 51    |
[00:46:00] 52 LL |     const I32_REF_I8_UNION: i8 = unsafe { Nonsense { int_32_ref: &3 }.int_8 };
[00:46:00] 54    |                                           |
[00:46:00] 54    |                                           |
[00:46:00] 55    |        en't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 105    |
[00:46:00] 105    |
[00:46:00] 106 LL |     const I32_REF_BOOL_UNION: bool = unsafe { Nonsense { int_32_ref: &3 }.truthy_falsey };
[00:46:00] 108    |                                               |
[00:46:00] 108    |                                               |
[00:46:00] 109    |                                               a raw memory access tried to access part of a pointer value as raw bytes
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 113    |
[00:46:00] 113    |
[00:46:00] 114 LL |     const I32_REF_CHAR_UNION: char = unsafe { Nonsense { int_32_ref: &3 }.character };
[00:46:00] 116    |                                               |
[00:46:00] 116    |                                               |
[00:46:00] 117    |                                               a raw memory access tried to access part of a pointer value as raw bytes
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 121    |
[00:46:00] 121    |
[00:46:00] 122 LL |     const STR_U8_UNION: u8 = unsafe { Nonsense { stringy: "3" }.uint_8 };
[00:46:00] 
[00:46:00] 124    |                          :46:00] 173    |                                         a raw memory access tried to access part of a pointer value as raw bytes
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 177    |
[00:46:00] 177    |
[00:46:00] 178 LL |     const STR_I32_UNION: i32 = unsafe { Nonsense { stringy: "3" }.int_32 };
[00:46:00] 180    |                                         |
[00:46:00] 180    |                                         |
[00:46:00] 181    |                                         a raw memory access tried to access part of a pointer value as raw bytes
[00:46:00] 182 
[00:46:00] - error[E0080]: this constant likely exhibits undefined behavior
[00:46:00] + error[E0080]: it is undefined behavior to use this value
[00:46:00] 185    |
[00:46:00] 185    |
[00:46:00] 186 LL |     const STR_I64_UNION: i64 = unsafe { Nonsense { stringy: "3" }.int_64 };
[00:46:00] 188    |
[00:46:00] 188    |
[00:46:00] 189    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
[00:46:00] - error: this constant cannot be used
[00:46:00] - error: this constant cannot be used
[00:46:00] + error: any use of this value will cause an error
[00:46:00] 193    |
[00:46:00] 193    |
[00:46:00] 194 LL |     const STR_I128_UNION: i128 = unsafe { Nonsense { stringy: "3" }.int_128 };
[00:46:00] 
[00:46:00] 196    |     " "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-pointer-values-in-various-types/auxiliary" "-A" "unused"
[00:46:00] ------------------------------------------
[00:46:00] 
[00:46:00] ------------------------------------------
[00:46:00] stderr:
[00:46:00] stderr:
[00:46:00] ------------------------------------------
[00:46:00] {"message":"it is undefined behavior to use this value","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n