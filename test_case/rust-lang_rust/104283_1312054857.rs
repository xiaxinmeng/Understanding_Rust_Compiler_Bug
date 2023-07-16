plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0405]: cannot find trait `TryFrom` in this scope
   --> library/core/src/num/nonzero.rs:199:18
    |
197 | / macro_rules! try_from_str_radix_nzint_impl {
198 | |     ($($t:ty)*) => {$(
199 | |         impl<'a> TryFrom<&'a str> for $t {
200 | |             type Error = ParseIntError;
...   |
208 | |     )*}
209 | | }
209 | | }
    | |_- in this expansion of `try_from_str_radix_nzint_impl!`
210 |
211 | / try_from_str_radix_nzint_impl! { NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroU128 NonZeroUsize
212 | | NonZeroI8 NonZeroI16 NonZeroI32 NonZeroI64 NonZeroI128 NonZeroIsize }
    |
help: consider importing this trait
    |
3   | use crate::convert::TryFrom;
