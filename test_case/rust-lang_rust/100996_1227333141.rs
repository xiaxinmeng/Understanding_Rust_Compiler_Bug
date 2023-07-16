plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0308]: mismatched types
   --> library/core/src/num/f32.rs:925:28
    |
925 |                     panic!("const-eval error: cannot use f32::to_bits on a NaN")
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/num/f32.rs:928:28
    |
    |
928 |                     panic!("const-eval error: cannot use f32::to_bits on a subnormal number")
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/num/f32.rs:1013:28
     |
     |
1013 |                     panic!("const-eval error: cannot use f32::from_bits on a subnormal number")
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/num/f32.rs:1016:28
     |
     |
1016 |                     panic!("const-eval error: cannot use f32::from_bits on NaN")
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/num/f64.rs:918:28
    |
    |
918 |                     panic!("const-eval error: cannot use f64::to_bits on a NaN")
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/num/f64.rs:921:28
    |
    |
921 |                     panic!("const-eval error: cannot use f64::to_bits on a subnormal number")
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/num/f64.rs:1011:28
     |
     |
1011 |                     panic!("const-eval error: cannot use f64::from_bits on a subnormal number")
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/num/f64.rs:1014:28
     |
     |
1014 |                     panic!("const-eval error: cannot use f64::from_bits on NaN")
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/num/bignum.rs:415:27
     |
     |
70   | / macro_rules! define_bignum {
71   | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
72   | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
...    |
...    |
415  | |                 write!(f, "{:#x}", self.base[sz - 1])?;
     | |                           ^^^^^^^ expected struct `Arguments`, found `&[_; 0]`
422  | |     };
423  | | }
     | |_- in this expansion of `define_bignum!`
...
...
428  |   define_bignum!(Big32x40: type=Digit32, n=40);
     |
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |           $dst.write_fmt($crate::format_args!($($arg)*))
     |
     = note: expected struct `Arguments<'_>`
     = note: expected struct `Arguments<'_>`
             found reference `&[_; 0]`
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/num/bignum.rs:417:31
     |
     |
70   | / macro_rules! define_bignum {
71   | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
72   | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
...    |
...    |
417  | |                     write!(f, "_{:01$x}", v, digitlen)?;
     | |                               ^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
422  | |     };
423  | | }
     | |_- in this expansion of `define_bignum!`
...
...
428  |   define_bignum!(Big32x40: type=Digit32, n=40);
     |
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |           $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/num/bignum.rs:415:27
     |
     |
70   | / macro_rules! define_bignum {
71   | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
72   | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
...    |
...    |
415  | |                 write!(f, "{:#x}", self.base[sz - 1])?;
     | |                           ^^^^^^^ expected struct `Arguments`, found `&[_; 0]`
422  | |     };
423  | | }
     | |_- in this expansion of `define_bignum!`
...
...
433  |       define_bignum!(Big8x3: type=u8, n=3);
     |
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |           $dst.write_fmt($crate::format_args!($($arg)*))
     |
     = note: expected struct `Arguments<'_>`
     = note: expected struct `Arguments<'_>`
             found reference `&[_; 0]`
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/num/bignum.rs:417:31
     |
     |
70   | / macro_rules! define_bignum {
71   | |     ($name:ident: type=$ty:ty, n=$n:expr) => {
72   | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
...    |
...    |
417  | |                     write!(f, "_{:01$x}", v, digitlen)?;
     | |                               ^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
422  | |     };
423  | | }
     | |_- in this expansion of `define_bignum!`
...
...
433  |       define_bignum!(Big8x3: type=u8, n=3);
     |
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |           $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/num/mod.rs:1058:9
     |
     |
1058 |         "from_str_radix_int: must lie in the range `[2, 36]` - found {}",
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/mem/maybe_uninit.rs:1147:43
     |
     |
1147 |         assert_eq!(this.len(), src.len(), "destination and source slices have different lengths");
     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/macros/mod.rs:58:85
     |
     |
58   | ...ind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
     |
note: tuple variant defined here
    --> library/core/src/option.rs:526:5
     |
     |
526  |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
     |     ^^^^

error[E0308]: mismatched types
    --> library/core/src/mem/valid_align.rs:67:19
     |
67   |         write!(f, "{:?} (1 << {:?})", self.as_nonzero(), self.log2())
     |                   ^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/mem/mod.rs:1057:47
     |
     |
1057 |     assert!(size_of::<T>() >= size_of::<U>(), "cannot transmute_copy if U is larger than T");
     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/ptr/const_ptr.rs:1293:20
     |
     |
1293 |             panic!("align_offset: align is not a power-of-two");
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/ptr/const_ptr.rs:1337:20
     |
     |
1337 |             panic!("is_aligned_to: align is not a power-of-two");
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/ptr/mut_ptr.rs:1573:20
     |
     |
1573 |             panic!("align_offset: align is not a power-of-two");
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/ptr/mut_ptr.rs:1617:20
     |
     |
1617 |             panic!("is_aligned_to: align is not a power-of-two");
     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/ops/range.rs:48:21
     |
     |
48   |         write!(fmt, "..")
     |                     ^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/ops/range.rs:93:21
     |
     |
93   |         write!(fmt, "..")?;
     |                     ^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/ops/range.rs:197:21
     |
     |
197  |         write!(fmt, "..")?;
     |                     ^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/ops/range.rs:277:21
     |
     |
277  |         write!(fmt, "..")?;
     |                     ^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/ops/range.rs:463:21
     |
     |
463  |         write!(fmt, "..=")?;
     |                     ^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/ops/range.rs:466:25
     |
     |
466  |             write!(fmt, " (exhausted)")?;
     |                         ^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/ops/range.rs:595:21
     |
     |
595  |         write!(fmt, "..=")?;
     |                     ^^^^^ expected struct `Arguments`, found `&[&str; 1]`
---
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/iter/adapters/zip.rs:574:16
    |
    |
574 |         panic!("Should only be called on TrustedRandomAccess iterators");
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/panic.rs:90:24
     |
     |
57   |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |           ---------------------------- arguments to this function are incorrect
...
85   | / pub macro unreachable_2021 {
87   | |         $crate::panicking::panic("internal error: entered unreachable code")
88   | |     ),
89   | |     ($($t:tt)+) => (
89   | |     ($($t:tt)+) => (
90   | |         $crate::panic!("internal error: entered unreachable code: {}", $crate::format_args!($($t)+))
     | |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
92   | | }
     | |_- in this expansion of `$crate::panic::unreachable_2021!` (#2)
     |
    ::: library/core/src/iter/traits/iterator.rs:3859:9
    ::: library/core/src/iter/traits/iterator.rs:3859:9
     |
3859 |           unreachable!("Always specialized");
     |           |
     |           in this macro invocation (#1)
     |           in this macro invocation (#2)
     |
---
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/panic/location.rs:195:27
     |
     |
195  |         write!(formatter, "{}:{}:{}", self.file, self.line, self.col)
     |                           ^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/panic/panic_info.rs:155:31
     |
     |
155  |             write!(formatter, "'{}', ", message)?
     |                               ^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
    --> library/core/src/panic/panic_info.rs:157:31
     |
     |
157  |             write!(formatter, "'{}', ", payload)?
     |                               ^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
    ::: library/core/src/macros/mod.rs:500:14
     |
     |
500  |         $dst.write_fmt($crate::format_args!($($arg)*))
     |
note: associated function defined here
    --> library/core/src/fmt/mod.rs:1655:12
     |
     |
1655 |     pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {

error[E0308]: mismatched types
   --> library/core/src/panicking.rs:63:28
    |
    |
63  |     panic_fmt(format_args!("internal error: entered unreachable code: {}", *x));
    |     ---------              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    |     arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/panicking.rs:72:28
    |
    |
72  |     panic_fmt(format_args!("{}", *x));
    |     ---------              ^^^^ expected struct `Arguments`, found `&[_; 0]`
    |     arguments to this function are incorrect
    |
    = note: expected struct `Arguments<'_>`
    = note: expected struct `Arguments<'_>`
            found reference `&[_; 0]`
   --> library/core/src/panicking.rs:127:14
    |
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/panicking.rs:84:12
    |
    |
84  |     panic!("index out of bounds: the len is {len} but the index is {index}")
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/panicking.rs:106:51
    |
    |
106 |     let pi = PanicInfo::internal_constructor(Some(&fmt), Location::caller(), false);
    |                                              ---- ^^^^ expected struct `Arguments`, found `&[&str; 1]`
    |                                              arguments to this enum variant are incorrect
    |
    = note: expected reference `&Arguments<'_>`
    = note: expected reference `&Arguments<'_>`
               found reference `&&[&str; 1]`
   --> library/core/src/option.rs:526:5
    |
526 |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
    |     ^^^^
    |     ^^^^

error[E0308]: mismatched types
   --> library/core/src/panicking.rs:219:13
    |
219 | /             r#"assertion failed: `(left {} right)`
220 | |   left: `{:?}`,
221 | |  right: `{:?}`: {}"#,
    | |____________________^ expected struct `Arguments`, found `&[&str; 4]`
   ::: library/core/src/panic.rs:57:9
    |
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |           ---------------------------- arguments to this function are incorrect
    |           ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/panicking.rs:225:13
    |
    |
225 | /             r#"assertion failed: `(left {} right)`
226 | |   left: `{:?}`,
227 | |  right: `{:?}`"#,
    | |________________^ expected struct `Arguments`, found `&[&str; 4]`
   ::: library/core/src/panic.rs:57:9
    |
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |           ---------------------------- arguments to this function are incorrect
    |           ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/result.rs:1819:12
     |
     |
1819 |     panic!("{msg}: {error:?}")
     |            ^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:2949:31
     |
     |
2949 |             Acquire => panic!("there is no such thing as an acquire store"),
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:2950:30
     |
     |
2950 |             AcqRel => panic!("there is no such thing as an acquire-release store"),
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:2964:31
     |
     |
2964 |             Release => panic!("there is no such thing as a release load"),
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:2965:30
     |
     |
2965 |             AcqRel => panic!("there is no such thing as an acquire-release load"),
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:3048:35
     |
     |
3048 |             (_, AcqRel) => panic!("there is no such thing as an acquire-release failure ordering"),
     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:3049:36
     |
     |
3049 |             (_, Release) => panic!("there is no such thing as a release failure ordering"),
     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:3083:35
     |
     |
3083 |             (_, AcqRel) => panic!("there is no such thing as an acquire-release failure ordering"),
     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:3084:36
     |
     |
3084 |             (_, Release) => panic!("there is no such thing as a release failure ordering"),
     |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:3308:31
     |
     |
3308 |             Relaxed => panic!("there is no such thing as a relaxed fence"),
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
    --> library/core/src/sync/atomic.rs:3391:31
     |
     |
3391 |             Relaxed => panic!("there is no such thing as a relaxed compiler fence"),
     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
    ::: library/core/src/panic.rs:57:9
     |
57   |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
     |         ---------------------------- arguments to this function are incorrect
     |         ---------------------------- arguments to this function are incorrect
     |
note: function defined here
    --> library/core/src/panicking.rs:127:14
     |
127  | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/fmt/builders.rs:782:17
    |
    |
782 | /                 "attempted to begin a new map entry \
783 | |                                     without completing the previous one"
    | |________________________________________________________________________^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |           ---------------------------- arguments to this function are incorrect
    |           ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/fmt/builders.rs:844:35
    |
    |
844 |             assert!(self.has_key, "attempted to format a map value before its key");
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/fmt/builders.rs:930:36
    |
    |
930 |             assert!(!self.has_key, "attempted to finish a map with a partial entry");
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 1]`
   ::: library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------- arguments to this function are incorrect
    |         ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/fmt/num.rs:138:33
    |
    |
130 | / macro_rules! radix {
131 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |         impl GenericRadix for $T {
133 | |             const BASE: u8 = $base;
...   |
138 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
142 | |     }
143 | | }
    | |_- in this expansion of `radix!`
144 |
144 |
145 |   radix! { Binary,    2, "0b", x @  0 ..=  1 => b'0' + x }
    |
   ::: library/core/src/panic.rs:57:9
    |
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |           ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/fmt/num.rs:138:33
    |
    |
130 | / macro_rules! radix {
131 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |         impl GenericRadix for $T {
133 | |             const BASE: u8 = $base;
...   |
138 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
142 | |     }
143 | | }
    | |_- in this expansion of `radix!`
...
...
146 |   radix! { Octal,     8, "0o", x @  0 ..=  7 => b'0' + x }
    |
   ::: library/core/src/panic.rs:57:9
    |
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |           ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/fmt/num.rs:138:33
    |
    |
130 | / macro_rules! radix {
131 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |         impl GenericRadix for $T {
133 | |             const BASE: u8 = $base;
...   |
138 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
142 | |     }
143 | | }
    | |_- in this expansion of `radix!`
...
...
147 |   radix! { LowerHex, 16, "0x", x @  0 ..=  9 => b'0' + x, x @ 10 ..= 15 => b'a' + (x - 10) }
    |
   ::: library/core/src/panic.rs:57:9
    |
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
57  |           $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |           ---------------------------- arguments to this function are incorrect
    |
note: function defined here
   --> library/core/src/panicking.rs:127:14
    |
127 | pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {

error[E0308]: mismatched types
   --> library/core/src/fmt/num.rs:138:33
    |
    |
130 | / macro_rules! radix {
131 | |     ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
132 | |         impl GenericRadix for $T {
133 | |             const BASE: u8 = $base;
...   |
138 | |                     x => panic!("number not in the range 0..={}: {}", Self::BASE - 1, x),
    | |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found `&[&str; 2]`
142 | |     }
143 | | }
    | |_- in this expansion of `radix!`
...
...
