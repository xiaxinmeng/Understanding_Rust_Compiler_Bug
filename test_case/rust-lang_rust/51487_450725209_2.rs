\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/resolve-self-in-impl.rs","byte_start":306,"byte_end":310,"line_start":14,"line_end":14,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"impl Tr for Self {} //~ ERROR cycle detected","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires processing `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.","byte_start":353,"byte_end":357,"line_start":15,"line_end":15,"column_start":15,"column_end":19,"is_primary":true,"text":[{"text":"impl Tr for S<Self> {} //~ ERROR cycle detected","highlight_start":15,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which again requires processing `<impl at /checkout/src/test/ui/resolve/resolve-self-in-impl.rs:15:1: 15:23>`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing ``","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/resolve/resolve-self-in-impl.rs","byte_start":0,"byte_end":530,"line_start":1,"line_end":20,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"#![feature(associated_type_defaults)]","highlight_start":1,"highlight_end":38},{"text":"","highlight_start":1,"highlight_end":1},{"text":"struct S<T = u8>(T);","highlight_start":1,"highlight_end":21},{"text":"trait Tr<T = u8> {","highlight_start":1,"highlight_end":19},{"text":"    type A = ();","highlight_start":1,"highlight_end":17},{"text":"}","highlight_start":1,"highlight_end":2},{"text":"","highlight_start":1,"highlight_end":1},{"text":"impl Tr<Self> for S {} // OK","highlight_start":1,"highlight_end":29},{"text":"impl<T: Tr<Self>> Tr<T> for S {} // OK","highlight_start":1,"highlight_end":39},{"text":"impl Tr for S where Self: Copy {} // OK","highlight_start":1,"highlight_end":40},{"text":"impl Tr for S where S<Self>: Copy {} // OK","highlight_start":1,"highlight_end":43},{"text":"impl Tr for S where Self::A: Copy {pi16(x: u16x8, y: u16x8) -> u16x8;
[01:03:32] + 
[01:03:32] + 
[01:03:32] + error[E0442]: intrinsic return value has wrong type: found `u16`, expected `i16`
[01:03:32] +   --> $DIR/simd-intrinsic-declaration-type.rs:33:9
[01:03:32] +    |
[01:03:32] + LL |         fn x86_mm_adds_epi16(x: u16x8, y: u16x8) -> u16x8;
[01:03:32] + 
[01:03:32] + 
[01:03:32] + error[E0442]: intrinsic argument 1 has wrong type: found `i16`, expected `u16`
[01:03:32] +   --> $DIR/simd-intrinsic-declaration-type.rs:37:9
[01:03:32] +    |
[01:03:32] + LL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
[01:03:32] + 
[01:03:32] + 
[01:03:32] + error[E0442]: intrinsic argument 2 has wrong type: found `i16`, expected `u16`
[01:03:32] +   --> $DIR/simd-intrinsic-declaration-type.rs:37:9
[01:03:32] +    |
[01:03:32] + LL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
[01:03:32] + 
[01:03:32] + 
[01:03:32] + error[E0442]: intrinsic return value has wrong type: found `i16`, expected `u16`
[01:03:32] +   --> $DIR/simd-intrinsic-declaration-type.rs:37:9
[01:03:32] +    |
[01:03:32] + LL |         fn x86_mm_adds_epu16(x: i16x8, y: i16x8) -> i16x8;
[01:03:32] 72 
[01:03:32] 73 error: aborting due to 12 previous errors
[01:03:32] 74 
[01:03:32] 
[01:03:32] 
[01:03:32] 
[01:03:32] The actual stderrnsics)]\n\n#[repr(simd)]\nstruct i8x16(i8, i8, i8, i8, i8, i8, i8, i8,\n             i8, i8, i8, i8, i8, i8, i8, i8);\n#[repr(simd)]\nstruct i32x4(i32, i32, i32, i32);\n#[repr(simd)]\nstruct i64x2(i64, i64);\n\nextern \"platform-intrinsic\" {\n    fn x86_mm_adds_epi16(x: i8x16, y: i32x4) -> i64x2;\n    // error: intrinsic arguments/return value have wrong type\n}\n