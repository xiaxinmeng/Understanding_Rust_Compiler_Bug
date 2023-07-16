plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.16s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
514 |   #[target_feature(enable = "simd128")]
...
540 | / pub const fn i8x16(
541 | |     a0: i8,
542 | |     a1: i8,
542 | |     a1: i8,
543 | |     a2: i8,
...   |
556 | |     a15: i8,
557 | | ) -> v128 {
    | |_________- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
583 |   #[target_feature(enable = "simd128")]
...
...
587 | / pub const fn u8x16(
588 | |     a0: u8,
589 | |     a1: u8,
590 | |     a2: u8,
603 | |     a15: u8,
604 | | ) -> v128 {
604 | | ) -> v128 {
    | |_________- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
616 | #[target_feature(enable = "simd128")]
...
...
634 | pub const fn i16x8(a0: i16, a1: i16, a2: i16, a3: i16, a4: i16, a5: i16, a6: i16, a7: i16) -> v128 {
    | -------------------------------------------------------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
648 | #[target_feature(enable = "simd128")]
...
...
652 | pub const fn u16x8(a0: u16, a1: u16, a2: u16, a3: u16, a4: u16, a5: u16, a6: u16, a7: u16) -> v128 {
    | -------------------------------------------------------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
663 | #[target_feature(enable = "simd128")]
...
...
668 | pub const fn i32x4(a0: i32, a1: i32, a2: i32, a3: i32) -> v128 {
    | -------------------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
677 | #[target_feature(enable = "simd128")]
...
...
681 | pub const fn u32x4(a0: u32, a1: u32, a2: u32, a3: u32) -> v128 {
    | -------------------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
690 | #[target_feature(enable = "simd128")]
...
...
695 | pub const fn i64x2(a0: i64, a1: i64) -> v128 {
    | -------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
704 | #[target_feature(enable = "simd128")]
...
...
708 | pub const fn u64x2(a0: u64, a1: u64) -> v128 {
    | -------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
717 | #[target_feature(enable = "simd128")]
...
...
722 | pub const fn f32x4(a0: f32, a1: f32, a2: f32, a3: f32) -> v128 {
    | -------------------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
731 | #[target_feature(enable = "simd128")]
...
...
736 | pub const fn f64x2(a0: f64, a1: f64) -> v128 {
    | -------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
776 |   #[target_feature(enable = "simd128")]
...
...
779 | / pub fn i8x16_shuffle<
780 | |     const I0: usize,
781 | |     const I1: usize,
782 | |     const I2: usize,
798 | |     b: v128,
799 | | ) -> v128 {
799 | | ) -> v128 {
    | |_________- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
871 |   #[target_feature(enable = "simd128")]
...
...
874 | / pub fn i16x8_shuffle<
875 | |     const I0: usize,
876 | |     const I1: usize,
877 | |     const I2: usize,
885 | |     b: v128,
886 | | ) -> v128 {
886 | | ) -> v128 {
    | |_________- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
928 |   #[target_feature(enable = "simd128")]
...
...
931 | / pub fn i32x4_shuffle<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
932 | |     a: v128,
933 | |     b: v128,
934 | | ) -> v128 {
    | |_________- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
961 | #[target_feature(enable = "simd128")]
...
...
964 | pub fn i64x2_shuffle<const I0: usize, const I1: usize>(a: v128, b: v128) -> v128 {
    | -------------------------------------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
    |
    |
986 | #[target_feature(enable = "simd128")]
...
...
989 | pub fn i8x16_extract_lane<const N: usize>(a: v128) -> i8 {
    | -------------------------------------------------------- not an `unsafe` function
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
    = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1000 | #[target_feature(enable = "simd128")]
...
...
1003 | pub fn u8x16_extract_lane<const N: usize>(a: v128) -> u8 {
     | -------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1014 | #[target_feature(enable = "simd128")]
...
...
1017 | pub fn i8x16_replace_lane<const N: usize>(a: v128, val: i8) -> v128 {
     | ------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1028 | #[target_feature(enable = "simd128")]
...
...
1031 | pub fn u8x16_replace_lane<const N: usize>(a: v128, val: u8) -> v128 {
     | ------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1042 | #[target_feature(enable = "simd128")]
...
...
1045 | pub fn i16x8_extract_lane<const N: usize>(a: v128) -> i16 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1056 | #[target_feature(enable = "simd128")]
...
...
1059 | pub fn u16x8_extract_lane<const N: usize>(a: v128) -> u16 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1070 | #[target_feature(enable = "simd128")]
...
...
1073 | pub fn i16x8_replace_lane<const N: usize>(a: v128, val: i16) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1084 | #[target_feature(enable = "simd128")]
...
...
1087 | pub fn u16x8_replace_lane<const N: usize>(a: v128, val: u16) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1098 | #[target_feature(enable = "simd128")]
...
...
1101 | pub fn i32x4_extract_lane<const N: usize>(a: v128) -> i32 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1111 | #[target_feature(enable = "simd128")]
...
...
1114 | pub fn u32x4_extract_lane<const N: usize>(a: v128) -> u32 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1124 | #[target_feature(enable = "simd128")]
...
...
1127 | pub fn i32x4_replace_lane<const N: usize>(a: v128, val: i32) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1137 | #[target_feature(enable = "simd128")]
...
...
1140 | pub fn u32x4_replace_lane<const N: usize>(a: v128, val: u32) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1150 | #[target_feature(enable = "simd128")]
...
...
1153 | pub fn i64x2_extract_lane<const N: usize>(a: v128) -> i64 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1163 | #[target_feature(enable = "simd128")]
...
...
1166 | pub fn u64x2_extract_lane<const N: usize>(a: v128) -> u64 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1176 | #[target_feature(enable = "simd128")]
...
...
1179 | pub fn i64x2_replace_lane<const N: usize>(a: v128, val: i64) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1189 | #[target_feature(enable = "simd128")]
...
...
1192 | pub fn u64x2_replace_lane<const N: usize>(a: v128, val: u64) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1202 | #[target_feature(enable = "simd128")]
...
...
1205 | pub fn f32x4_extract_lane<const N: usize>(a: v128) -> f32 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1216 | #[target_feature(enable = "simd128")]
...
...
1219 | pub fn f32x4_replace_lane<const N: usize>(a: v128, val: f32) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1230 | #[target_feature(enable = "simd128")]
...
...
1233 | pub fn f64x2_extract_lane<const N: usize>(a: v128) -> f64 {
     | --------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1244 | #[target_feature(enable = "simd128")]
...
...
1247 | pub fn f64x2_replace_lane<const N: usize>(a: v128, val: f64) -> v128 {
     | -------------------------------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1259 | #[target_feature(enable = "simd128")]
...
...
1262 | pub fn i8x16_swizzle(a: v128, s: v128) -> v128 {
     | ---------------------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1274 | #[target_feature(enable = "simd128")]
...
...
1277 | pub fn i8x16_splat(a: i8) -> v128 {
     | --------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1286 | #[target_feature(enable = "simd128")]
...
...
1289 | pub fn u8x16_splat(a: u8) -> v128 {
     | --------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1298 | #[target_feature(enable = "simd128")]
...
...
1301 | pub fn i16x8_splat(a: i16) -> v128 {
     | ---------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
     |
     |
1310 | #[target_feature(enable = "simd128")]
...
...
1313 | pub fn u16x8_splat(a: u16) -> v128 {
     | ---------------------------------- not an `unsafe` function
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = note: see issue #69098 <https://github.com/rust-lang/rust/issues/69098> for more information
     = help: add `#![feature(target_feature_11)]` to the crate attributes to enable

error[E0658]: `#[target_feature(..)]` can only be applied to `unsafe` functions
