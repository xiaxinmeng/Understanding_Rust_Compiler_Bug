plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unnecessary `unsafe` block
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:121:9
    |
121 |         unsafe { Self(mask_impl::Mask::from_int_unchecked(value)) }
    |         ^^^^^^ unnecessary `unsafe` block
    |
    = note: `-D unused-unsafe` implied by `-D warnings`

error: unnecessary `unsafe` block
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:148:9
    |
148 |         unsafe { self.0.test_unchecked(lane) }
    |         ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:167:9
167 |         unsafe {
167 |         unsafe {
    |         ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   --> library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:172:9
    |
172 |         unsafe { intrinsics::simd_gather(or, ptrs, enable.to_int()) }
    |         ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
   --> library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:272:9
272 |         unsafe {
272 |         unsafe {
    |         ^^^^^^ unnecessary `unsafe` block

error: unnecessary `unsafe` block
  --> library/core/src/../../portable-simd/crates/core_simd/src/round.rs:64:17
4  | / macro_rules! implement {
5  | |     {
5  | |     {
6  | |         $type:ty, $int_type:ty
7  | |     } => {
...  |
64 | |                 unsafe { intrinsics::simd_cast(self) }
   | |                 ^^^^^^ unnecessary `unsafe` block
74 | |     }
75 | | }
   | |_- in this expansion of `implement!`
76 | 
76 | 
77 |   implement! { f32, i32 }


error: unnecessary `unsafe` block
  --> library/core/src/../../portable-simd/crates/core_simd/src/round.rs:64:17
4  | / macro_rules! implement {
5  | |     {
5  | |     {
6  | |         $type:ty, $int_type:ty
7  | |     } => {
...  |
64 | |                 unsafe { intrinsics::simd_cast(self) }
   | |                 ^^^^^^ unnecessary `unsafe` block
74 | |     }
75 | | }
   | |_- in this expansion of `implement!`
...
...
78 |   implement! { f64, i64 }

error: could not compile `core` due to 7 previous errors
Build completed unsuccessfully in 0:04:48
