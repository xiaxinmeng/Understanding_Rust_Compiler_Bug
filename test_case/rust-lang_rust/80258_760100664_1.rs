
error[E0282]: type annotations needed
   --> library/core/src/fmt/num.rs:234:45
    |
209 | / macro_rules! impl_Display {
210 | |     ($($t:ident),* as $u:ident via $conv_fn:ident named $name:ident) => {
211 | |         fn $name(mut n: $u, is_nonnegative: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
212 | |             // 2^128 is about 3*10^38, so 39 gives an extra byte of space
...   |
234 | |                     let d1 = (rem / 100) << 1;
    | |                                             ^ cannot infer type for type `{integer}`
...   |
290 | |     };
291 | | }
    | |_- in this expansion of `impl_Display!`
...
462 | /     impl_Display!(
463 | |         i8, u8, i16, u16, i32, u32, i64, u64, usize, isize
464 | |             as u64 via to_u64 named fmt_u64
465 | |     );
    | |______- in this macro invocation

error: aborting due to previous error
