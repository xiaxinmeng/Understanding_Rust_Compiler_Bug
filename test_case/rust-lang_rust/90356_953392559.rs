plain
 Documenting core v0.0.0 (/checkout/library/core)
error: functions cannot be const-stable if they are unstable
   --> library/core/src/num/saturating.rs:634:13
    |
634 | /             pub const fn reverse_bits(self) -> Self {
635 | |                 Saturating(self.0.reverse_bits())
    | |_____________^
...
...
785 |   saturating_int_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
    |
    |
    = note: `#[deny(rustc::incompatible_stability)]` on by default
    = note: this error originates in the macro `saturating_int_impl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: functions cannot be const-stable if they are unstable
    --> library/core/src/num/uint_macros.rs:2214:9
     |
2214 | /         pub const fn wrapping_next_power_of_two(self) -> Self {
2214 | /         pub const fn wrapping_next_power_of_two(self) -> Self {
2215 | |             self.one_less_than_next_power_of_two().wrapping_add(1)
     | |_________^
     |
    ::: library/core/src/num/mod.rs:248:5
     |
     |
248  | /     uint_impl! { u8, u8, i8, 8, 255, 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
249  | |     "[0x12]", "", "" }
     |
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: functions cannot be const-stable if they are unstable
error: functions cannot be const-stable if they are unstable
    --> library/core/src/num/uint_macros.rs:2214:9
     |
2214 | /         pub const fn wrapping_next_power_of_two(self) -> Self {
2215 | |             self.one_less_than_next_power_of_two().wrapping_add(1)
     | |_________^
     |
    ::: library/core/src/num/mod.rs:797:5
     |
     |
797  | /     uint_impl! { u16, u16, i16, 16, 65535, 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
798  | |     "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
     |
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: functions cannot be const-stable if they are unstable
error: functions cannot be const-stable if they are unstable
    --> library/core/src/num/uint_macros.rs:2214:9
     |
2214 | /         pub const fn wrapping_next_power_of_two(self) -> Self {
2215 | |             self.one_less_than_next_power_of_two().wrapping_add(1)
     | |_________^
     |
    ::: library/core/src/num/mod.rs:804:5
     |
     |
804  | /     uint_impl! { u32, u32, i32, 32, 4294967295, 8, "0x10000b3", "0xb301", "0x12345678",
805  | |     "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
     |
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: functions cannot be const-stable if they are unstable
error: functions cannot be const-stable if they are unstable
    --> library/core/src/num/uint_macros.rs:2214:9
     |
2214 | /         pub const fn wrapping_next_power_of_two(self) -> Self {
2215 | |             self.one_less_than_next_power_of_two().wrapping_add(1)
     | |_________^
     |
    ::: library/core/src/num/mod.rs:811:5
     |
     |
811  | /     uint_impl! { u64, u64, i64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
812  | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
813  | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
814  | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
815  | |     "", ""}
     |
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: functions cannot be const-stable if they are unstable
error: functions cannot be const-stable if they are unstable
    --> library/core/src/num/uint_macros.rs:2214:9
     |
2214 | /         pub const fn wrapping_next_power_of_two(self) -> Self {
2215 | |             self.one_less_than_next_power_of_two().wrapping_add(1)
     | |_________^
     |
    ::: library/core/src/num/mod.rs:820:5
     |
     |
820  | /     uint_impl! { u128, u128, i128, 128, 340282366920938463463374607431768211455, 16,
821  | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
822  | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
823  | |     "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
826  | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
827  | |      "", ""}
     | |____________- in this macro invocation
     |
     |
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: functions cannot be const-stable if they are unstable
    --> library/core/src/num/uint_macros.rs:2214:9
     |
2214 | /         pub const fn wrapping_next_power_of_two(self) -> Self {
2215 | |             self.one_less_than_next_power_of_two().wrapping_add(1)
     | |_________^
     |
    ::: library/core/src/num/mod.rs:851:5
     |
     |
851  | /     uint_impl! { usize, u64, isize, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
852  | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
853  | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
854  | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
855  | |     usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
     |
     = note: this error originates in the macro `uint_impl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: functions cannot be const-stable if they are unstable
---

error: functions cannot be const-stable if they are unstable
   --> library/core/src/intrinsics.rs:799:5
    |
799 |     pub fn size_of<T>() -> usize;

error: functions cannot be const-stable if they are unstable
   --> library/core/src/intrinsics.rs:810:5
    |
    |
810 |     pub fn min_align_of<T>() -> usize;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1152:5
     |
     |
1152 |     pub fn needs_drop<T>() -> bool;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1527:5
     |
     |
1527 |     pub fn ctpop<T: Copy>(x: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1564:5
     |
     |
1564 |     pub fn ctlz<T: Copy>(x: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1583:5
     |
     |
1583 |     pub fn ctlz_nonzero<T: Copy>(x: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1620:5
     |
     |
1620 |     pub fn cttz<T: Copy>(x: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1639:5
     |
     |
1639 |     pub fn cttz_nonzero<T: Copy>(x: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1652:5
     |
     |
1652 |     pub fn bswap<T: Copy>(x: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1665:5
     |
     |
1665 |     pub fn bitreverse<T: Copy>(x: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1678:5
     |
     |
1678 |     pub fn add_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1691:5
     |
     |
1691 |     pub fn sub_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1704:5
     |
     |
1704 |     pub fn mul_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1719:5
     |
     |
1719 |     pub fn unchecked_div<T: Copy>(x: T, y: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1727:5
     |
     |
1727 |     pub fn unchecked_rem<T: Copy>(x: T, y: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1736:5
     |
     |
1736 |     pub fn unchecked_shl<T: Copy>(x: T, y: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1744:5
     |
     |
1744 |     pub fn unchecked_shr<T: Copy>(x: T, y: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1778:5
     |
     |
1778 |     pub fn rotate_left<T: Copy>(x: T, y: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1791:5
     |
     |
1791 |     pub fn rotate_right<T: Copy>(x: T, y: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1804:5
     |
     |
1804 |     pub fn wrapping_add<T: Copy>(a: T, b: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1816:5
     |
     |
1816 |     pub fn wrapping_sub<T: Copy>(a: T, b: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1828:5
     |
     |
1828 |     pub fn wrapping_mul<T: Copy>(a: T, b: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1841:5
     |
     |
1841 |     pub fn saturating_add<T: Copy>(a: T, b: T) -> T;

error: functions cannot be const-stable if they are unstable
    --> library/core/src/intrinsics.rs:1853:5
     |
     |
1853 |     pub fn saturating_sub<T: Copy>(a: T, b: T) -> T;

error: functions cannot be const-stable if they are unstable
  --> library/core/src/../../stdarch/crates/core_arch/src/wasm32/simd128.rs:58:17
   |
   |
58 | /                 const fn v128(self) -> v128 {
59 | |                     unsafe { mem::transmute(self) }
   | |_________________^
...
...
66 | / conversions! {
67 | |     (as_u8x16 = simd::u8x16)
68 | |     (as_u16x8 = simd::u16x8)
69 | |     (as_u32x4 = simd::u32x4)
...  |
76 | |     (as_f64x2 = simd::f64x2)
   | |_- in this macro invocation
   |
   = note: this error originates in the macro `conversions` (in Nightly builds, run with -Z macro-backtrace for more info)


error: could not document `core`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.58.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps -Zsymbol-mangling-version=legacy -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.58.0-nightly
  (4dc9b9e7c
  2021-10-27)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.58.0" "--index-page" "/checkout/src/doc/index.md"


Build completed unsuccessfully in 0:35:17
