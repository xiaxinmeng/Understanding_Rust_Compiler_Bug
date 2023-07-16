plain
Documenting stage2 std (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error[E0046]: not all trait items implemented, missing: `bitor`
    |
    |
123 |               impl BitOr for $Ty {
    |               ^^^^^^^^^^^^^^^^^^ missing `bitor` in implementation
...
180 | / nonzero_integers! {
181 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
182 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
183 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
192 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
    | |_- in this macro invocation
    | 
   ::: library/core/src/ops/bit.rs:240:5
    |
    |
240 |       fn bitor(self, rhs: Rhs) -> Self::Output;
    |       ----------------------------------------- `bitor` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0046]: not all trait items implemented, missing: `bitor`
    |
    |
134 |               impl BitOr<$Int> for $Ty {
    |               ^^^^^^^^^^^^^^^^^^^^^^^^ missing `bitor` in implementation
...
180 | / nonzero_integers! {
181 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
182 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
183 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
192 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
    | |_- in this macro invocation
    | 
   ::: library/core/src/ops/bit.rs:240:5
    |
    |
240 |       fn bitor(self, rhs: Rhs) -> Self::Output;
    |       ----------------------------------------- `bitor` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0046]: not all trait items implemented, missing: `bitor`
    |
    |
146 |               impl BitOr<$Ty> for $Int {
    |               ^^^^^^^^^^^^^^^^^^^^^^^^ missing `bitor` in implementation
...
180 | / nonzero_integers! {
181 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
182 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
183 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
...   |
192 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
    | |_- in this macro invocation
    | 
   ::: library/core/src/ops/bit.rs:240:5
    |
    |
240 |       fn bitor(self, rhs: Rhs) -> Self::Output;
    |       ----------------------------------------- `bitor` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `from_str`
   --> library/core/src/num/mod.rs:198:9
   --> library/core/src/num/mod.rs:198:9
    |
198 |           impl FromStr for $t {
    |           ^^^^^^^^^^^^^^^^^^^ missing `from_str` in implementation
...
210 | / from_str_radix_nzint_impl! { NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
211 | | NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize }
    | |_____________________________________________________________________- in this macro invocation
   ::: library/core/src/str/mod.rs:105:5
    |
    |
105 |       fn from_str(s: &str) -> Result<Self, Self::Err>;
    |       ------------------------------------------------ `from_str` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `from_str`
    --> library/core/src/num/mod.rs:5121:9
    --> library/core/src/num/mod.rs:5121:9
     |
5121 |         impl FromStr for $t {
     |         ^^^^^^^^^^^^^^^^^^^ missing `from_str` in implementation
...
5129 | from_str_radix_int_impl! { isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128 }
     | 
    ::: library/core/src/str/mod.rs:105:5
     |
     |
105  |     fn from_str(s: &str) -> Result<Self, Self::Err>;
     |     ------------------------------------------------ `from_str` from trait
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0046]: not all trait items implemented, missing: `max_value`, `from_u32`, `checked_mul`, `checked_sub`, `checked_add`
     |
5176 |     fn max_value() -> Self;
5176 |     fn max_value() -> Self;
     |     ----------------------- `max_value` from trait
5177 |     fn from_u32(u: u32) -> Self;
     |     ---------------------------- `from_u32` from trait
5178 |     fn checked_mul(&self, other: u32) -> Option<Self>;
     |     -------------------------------------------------- `checked_mul` from trait
5179 |     fn checked_sub(&self, other: u32) -> Option<Self>;
     |     -------------------------------------------------- `checked_sub` from trait
5180 |     fn checked_add(&self, other: u32) -> Option<Self>;
     |     -------------------------------------------------- `checked_add` from trait
...
5184 |     ($($t:ty)*) => ($(impl FromStrRadixHelper for $t {
     |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `max_value`, `from_u32`, `checked_mul`, `checked_sub`, `checked_add` in implementation
...
5205 | doit! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
     |
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0046]: not all trait items implemented, missing: `full_mul`, `full_mul_add`, `full_div_rem`
  --> library/core/src/num/bignum.rs:48:13
   |
33 |       fn full_mul(self, other: Self, carry: Self) -> (Self /* carry */, Self);
   |       ------------------------------------------------------------------------ `full_mul` from trait
...
37 |       fn full_mul_add(self, other: Self, other2: Self, carry: Self) -> (Self /* carry */, Self);
   |       ------------------------------------------------------------------------------------------ `full_mul_add` from trait
...
41 | /     fn full_div_rem(self, other: Self, borrow: Self)
42 | |     -> (Self /* quotient */, Self /* remainder */);
   | |___________________________________________________- `full_div_rem` from trait
...
48 |               impl FullOps for $ty {
   |               ^^^^^^^^^^^^^^^^^^^^ missing `full_mul`, `full_mul_add`, `full_div_rem` in implementation
...
88 | / impl_full_ops! {
89 | |     u8:  add(intrinsics::u8_add_with_overflow),  mul/div(u16);
90 | |     u16: add(intrinsics::u16_add_with_overflow), mul/div(u32);
91 | |     u32: add(intrinsics::u32_add_with_overflow), mul/div(u64);
92 | |     // See RFC #521 for enabling this.
93 | |     // u64: add(intrinsics::u64_add_with_overflow), mul/div(u128);
   | |_- in this macro invocation
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0046]: not all trait items implemented, missing: `from_str`
   --> library/core/src/num/dec2flt/mod.rs:104:9
    |
104 |         impl FromStr for $t {
    |         ^^^^^^^^^^^^^^^^^^^ missing `from_str` in implementation
...
162 | from_str_float_impl!(f32);
    | 
   ::: library/core/src/str/mod.rs:105:5
    |
    |
105 |     fn from_str(s: &str) -> Result<Self, Self::Err>;
    |     ------------------------------------------------ `from_str` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `from_str`
error[E0046]: not all trait items implemented, missing: `from_str`
   --> library/core/src/num/dec2flt/mod.rs:104:9
    |
104 |         impl FromStr for $t {
    |         ^^^^^^^^^^^^^^^^^^^ missing `from_str` in implementation
...
163 | from_str_float_impl!(f64);
    | 
   ::: library/core/src/str/mod.rs:105:5
    |
    |
105 |     fn from_str(s: &str) -> Result<Self, Self::Err>;
    |     ------------------------------------------------ `from_str` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0046]: not all trait items implemented, missing: `NAN`, `ZERO`, `Bits`, `to_bits`, `from_bits`, `classify`, `integer_decode`, `unpack`, `from_int`, `short_fast_pow10`, `CEIL_LOG5_OF_MAX_SIG`, `MAX_NORMAL_DIGITS`, `INF_CUTOFF`, `ZERO_CUTOFF`, `EXP_BITS`, `SIG_BITS`, `EXPLICIT_SIG_BITS`, `MAX_EXP`, `MIN_EXP`, `MAX_EXP_INT`, `MAX_ENCODED_EXP`, `MIN_EXP_INT`, `MAX_SIG`, `MIN_SIG`
   --> library/core/src/num/dec2flt/rawfp.rs:145:1
    |
51  |     const NAN: Self;
    |     ---------------- `NAN` from trait
52  |     const ZERO: Self;
    |     ----------------- `ZERO` from trait
...
55  |     type Bits: Add<Output = Self::Bits> + From<u8> + TryFrom<u64>;
    |     -------------------------------------------------------------- `Bits` from trait
...
58  |     fn to_bits(self) -> Self::Bits;
    |     ------------------------------- `to_bits` from trait
...
61  |     fn from_bits(v: Self::Bits) -> Self;
    |     ------------------------------------ `from_bits` from trait
...
64  |     fn classify(self) -> FpCategory;
    |     -------------------------------- `classify` from trait
...
67  |     fn integer_decode(self) -> (u64, i16, i8);
    |     ------------------------------------------ `integer_decode` from trait
...
70  |     fn unpack(self) -> Unpacked;
    |     ---------------------------- `unpack` from trait
...
74  |     fn from_int(x: u64) -> Self;
    |     ---------------------------- `from_int` from trait
...
78  |     fn short_fast_pow10(e: usize) -> Self;
    |     -------------------------------------- `short_fast_pow10` from trait
...
82  |     const CEIL_LOG5_OF_MAX_SIG: i16;
    |     -------------------------------- `CEIL_LOG5_OF_MAX_SIG` from trait
86  |     const MAX_NORMAL_DIGITS: usize;
86  |     const MAX_NORMAL_DIGITS: usize;
    |     ------------------------------- `MAX_NORMAL_DIGITS` from trait
...
90  |     const INF_CUTOFF: i64;
    |     ---------------------- `INF_CUTOFF` from trait
...
94  |     const ZERO_CUTOFF: i64;
    |     ----------------------- `ZERO_CUTOFF` from trait
...
97  |     const EXP_BITS: u8;
    |     ------------------- `EXP_BITS` from trait
...
100 |     const SIG_BITS: u8;
    |     ------------------- `SIG_BITS` from trait
...
103 |     const EXPLICIT_SIG_BITS: u8;
    |     ---------------------------- `EXPLICIT_SIG_BITS` from trait
106 |     const MAX_EXP: i16;
106 |     const MAX_EXP: i16;
    |     ------------------- `MAX_EXP` from trait
109 |     const MIN_EXP: i16;
109 |     const MIN_EXP: i16;
    |     ------------------- `MIN_EXP` from trait
112 |     const MAX_EXP_INT: i16;
112 |     const MAX_EXP_INT: i16;
    |     ----------------------- `MAX_EXP_INT` from trait
...
115 |     const MAX_ENCODED_EXP: i16;
    |     --------------------------- `MAX_ENCODED_EXP` from trait
118 |     const MIN_EXP_INT: i16;
118 |     const MIN_EXP_INT: i16;
    |     ----------------------- `MIN_EXP_INT` from trait
...
121 |     const MAX_SIG: u64;
    |     ------------------- `MAX_SIG` from trait
124 |     const MIN_SIG: u64;
124 |     const MIN_SIG: u64;
    |     ------------------- `MIN_SIG` from trait
...
145 | impl RawFloat for f32 {
    | ^^^^^^^^^^^^^^^^^^^^^ missing `NAN`, `ZERO`, `Bits`, `to_bits`, `from_bits`, `classify`, `integer_decode`, `unpack`, `from_int`, `short_fast_pow10`, `CEIL_LOG5_OF_MAX_SIG`, `MAX_NORMAL_DIGITS`, `INF_CUTOFF`, `ZERO_CUTOFF`, `EXP_BITS`, `SIG_BITS`, `EXPLICIT_SIG_BITS`, `MAX_EXP`, `MIN_EXP`, `MAX_EXP_INT`, `MAX_ENCODED_EXP`, `MIN_EXP_INT`, `MAX_SIG`, `MIN_SIG` in implementation

error[E0046]: not all trait items implemented, missing: `NAN`, `ZERO`, `Bits`, `to_bits`, `from_bits`, `classify`, `integer_decode`, `unpack`, `from_int`, `short_fast_pow10`, `CEIL_LOG5_OF_MAX_SIG`, `MAX_NORMAL_DIGITS`, `INF_CUTOFF`, `ZERO_CUTOFF`, `EXP_BITS`, `SIG_BITS`, `EXPLICIT_SIG_BITS`, `MAX_EXP`, `MIN_EXP`, `MAX_EXP_INT`, `MAX_ENCODED_EXP`, `MIN_EXP_INT`, `MAX_SIG`, `MIN_SIG`
   --> library/core/src/num/dec2flt/rawfp.rs:194:1
    |
51  |     const NAN: Self;
    |     ---------------- `NAN` from trait
52  |     const ZERO: Self;
    |     ----------------- `ZERO` from trait
...
55  |     type Bits: Add<Output = Self::Bits> + From<u8> + TryFrom<u64>;
    |     -------------------------------------------------------------- `Bits` from trait
...
58  |     fn to_bits(self) -> Self::Bits;
    |     ------------------------------- `to_bits` from trait
...
61  |     fn from_bits(v: Self::Bits) -> Self;
    |     ------------------------------------ `from_bits` from trait
...
64  |     fn classify(self) -> FpCategory;
    |     -------------------------------- `classify` from trait
...
67  |     fn integer_decode(self) -> (u64, i16, i8);
    |     ------------------------------------------ `integer_decode` from trait
...
70  |     fn unpack(self) -> Unpacked;
    |     ---------------------------- `unpack` from trait
...
74  |     fn from_int(x: u64) -> Self;
    |     ---------------------------- `from_int` from trait
...
78  |     fn short_fast_pow10(e: usize) -> Self;
    |     -------------------------------------- `short_fast_pow10` from trait
...
82  |     const CEIL_LOG5_OF_MAX_SIG: i16;
    |     -------------------------------- `CEIL_LOG5_OF_MAX_SIG` from trait
86  |     const MAX_NORMAL_DIGITS: usize;
86  |     const MAX_NORMAL_DIGITS: usize;
    |     ------------------------------- `MAX_NORMAL_DIGITS` from trait
...
90  |     const INF_CUTOFF: i64;
    |     ---------------------- `INF_CUTOFF` from trait
...
94  |     const ZERO_CUTOFF: i64;
    |     ----------------------- `ZERO_CUTOFF` from trait
...
97  |     const EXP_BITS: u8;
    |     ------------------- `EXP_BITS` from trait
...
100 |     const SIG_BITS: u8;
    |     ------------------- `SIG_BITS` from trait
...
103 |     const EXPLICIT_SIG_BITS: u8;
    |     ---------------------------- `EXPLICIT_SIG_BITS` from trait
106 |     const MAX_EXP: i16;
106 |     const MAX_EXP: i16;
    |     ------------------- `MAX_EXP` from trait
109 |     const MIN_EXP: i16;
109 |     const MIN_EXP: i16;
    |     ------------------- `MIN_EXP` from trait
112 |     const MAX_EXP_INT: i16;
112 |     const MAX_EXP_INT: i16;
    |     ----------------------- `MAX_EXP_INT` from trait
...
115 |     const MAX_ENCODED_EXP: i16;
    |     --------------------------- `MAX_ENCODED_EXP` from trait
118 |     const MIN_EXP_INT: i16;
118 |     const MIN_EXP_INT: i16;
    |     ----------------------- `MIN_EXP_INT` from trait
...
121 |     const MAX_SIG: u64;
    |     ------------------- `MAX_SIG` from trait
124 |     const MIN_SIG: u64;
124 |     const MIN_SIG: u64;
    |     ------------------- `MIN_SIG` from trait
...
194 | impl RawFloat for f64 {
    | ^^^^^^^^^^^^^^^^^^^^^ missing `NAN`, `ZERO`, `Bits`, `to_bits`, `from_bits`, `classify`, `integer_decode`, `unpack`, `from_int`, `short_fast_pow10`, `CEIL_LOG5_OF_MAX_SIG`, `MAX_NORMAL_DIGITS`, `INF_CUTOFF`, `ZERO_CUTOFF`, `EXP_BITS`, `SIG_BITS`, `EXPLICIT_SIG_BITS`, `MAX_EXP`, `MIN_EXP`, `MAX_EXP_INT`, `MAX_ENCODED_EXP`, `MIN_EXP_INT`, `MAX_SIG`, `MIN_SIG` in implementation
error[E0046]: not all trait items implemented, missing: `shl`
   --> library/core/src/num/wrapping.rs:63:9
    |
    |
63  |         impl Shl<$f> for Wrapping<$t> {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `shl` in implementation
...
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    | 
   ::: library/core/src/ops/bit.rs:435:5
    |
    |
435 |     fn shl(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shl` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shr`
   --> library/core/src/num/wrapping.rs:84:9
   --> library/core/src/num/wrapping.rs:84:9
    |
84  |         impl Shr<$f> for Wrapping<$t> {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `shr` in implementation
...
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    | 
   ::: library/core/src/ops/bit.rs:553:5
    |
    |
553 |     fn shr(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shr` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shl`
   --> library/core/src/internal_macros.rs:30:9
   --> library/core/src/internal_macros.rs:30:9
    |
30  |         impl<'a> $imp<$u> for &'a $t {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `shl` in implementation
   ::: library/core/src/ops/bit.rs:435:5
    |
    |
435 |     fn shl(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shl` from trait
   ::: library/core/src/num/wrapping.rs:125:1
    |
    |
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shl`
error[E0046]: not all trait items implemented, missing: `shl`
   --> library/core/src/internal_macros.rs:40:9
    |
40  |         impl $imp<&$u> for $t {
    |         ^^^^^^^^^^^^^^^^^^^^^ missing `shl` in implementation
   ::: library/core/src/ops/bit.rs:435:5
    |
    |
435 |     fn shl(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shl` from trait
   ::: library/core/src/num/wrapping.rs:125:1
    |
    |
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shl`
error[E0046]: not all trait items implemented, missing: `shl`
   --> library/core/src/internal_macros.rs:50:9
    |
50  |         impl $imp<&$u> for &$t {
    |         ^^^^^^^^^^^^^^^^^^^^^^ missing `shl` in implementation
   ::: library/core/src/ops/bit.rs:435:5
    |
    |
435 |     fn shl(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shl` from trait
   ::: library/core/src/num/wrapping.rs:125:1
    |
    |
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shr`
error[E0046]: not all trait items implemented, missing: `shr`
   --> library/core/src/internal_macros.rs:30:9
    |
30  |         impl<'a> $imp<$u> for &'a $t {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `shr` in implementation
   ::: library/core/src/ops/bit.rs:553:5
    |
    |
553 |     fn shr(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shr` from trait
   ::: library/core/src/num/wrapping.rs:125:1
    |
    |
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shr`
error[E0046]: not all trait items implemented, missing: `shr`
   --> library/core/src/internal_macros.rs:40:9
    |
40  |         impl $imp<&$u> for $t {
    |         ^^^^^^^^^^^^^^^^^^^^^ missing `shr` in implementation
   ::: library/core/src/ops/bit.rs:553:5
    |
    |
553 |     fn shr(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shr` from trait
   ::: library/core/src/num/wrapping.rs:125:1
    |
    |
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shr`
error[E0046]: not all trait items implemented, missing: `shr`
   --> library/core/src/internal_macros.rs:50:9
    |
50  |         impl $imp<&$u> for &$t {
    |         ^^^^^^^^^^^^^^^^^^^^^^ missing `shr` in implementation
   ::: library/core/src/ops/bit.rs:553:5
    |
    |
553 |     fn shr(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shr` from trait
   ::: library/core/src/num/wrapping.rs:125:1
    |
    |
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shl`
error[E0046]: not all trait items implemented, missing: `shl`
   --> library/core/src/num/wrapping.rs:63:9
    |
63  |         impl Shl<$f> for Wrapping<$t> {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `shl` in implementation
...
125 | sh_impl_all! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }
    | 
   ::: library/core/src/ops/bit.rs:435:5
    |
    |
435 |     fn shl(self, rhs: Rhs) -> Self::Output;
    |     --------------------------------------- `shl` from trait
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `shr`
   --> library/core/src/num/wrapping.rs:84:9
   --> library/core/src/num/wrapping.rs:84:9
    |
84  |         impl Shr<$f> for Wrapping<$t> {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `shr` in implementation
...
