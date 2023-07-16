plain
   Compiling libc v0.2.88
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0711]: feature `nonzero_leading_trailing_zeros` is declared unstable, but was previously declared stable
    |
181 | / macro_rules! nonzero_leading_trailing_zeros {
181 | / macro_rules! nonzero_leading_trailing_zeros {
182 | |     ( $( $Ty: ident($Uint: ty) , $LeadingTestExpr:expr ;)+ ) => {
183 | |         $(
184 | |             impl $Ty {
...   |
199 | |                 #[rustc_const_unstable(feature = "nonzero_leading_trailing_zeros", issue = "79143")]
...   |
230 | |     }
231 | | }
231 | | }
    | |_- in this expansion of `nonzero_leading_trailing_zeros!`
232 | 
233 | / nonzero_leading_trailing_zeros! {
234 | |     NonZeroU8(u8), u8::MAX;
235 | |     NonZeroU16(u16), u16::MAX;
236 | |     NonZeroU32(u32), u32::MAX;
...   |
245 | |     NonZeroIsize(usize), -1isize;
    | |_- in this macro invocation


error[E0711]: feature `nonzero_leading_trailing_zeros` is declared unstable, but was previously declared stable
    |
181 | / macro_rules! nonzero_leading_trailing_zeros {
181 | / macro_rules! nonzero_leading_trailing_zeros {
182 | |     ( $( $Ty: ident($Uint: ty) , $LeadingTestExpr:expr ;)+ ) => {
183 | |         $(
184 | |             impl $Ty {
...   |
221 | |                 #[rustc_const_unstable(feature = "nonzero_leading_trailing_zeros", issue = "79143")]
...   |
230 | |     }
231 | | }
231 | | }
    | |_- in this expansion of `nonzero_leading_trailing_zeros!`
232 | 
233 | / nonzero_leading_trailing_zeros! {
234 | |     NonZeroU8(u8), u8::MAX;
235 | |     NonZeroU16(u16), u16::MAX;
236 | |     NonZeroU32(u32), u32::MAX;
...   |
245 | |     NonZeroIsize(usize), -1isize;
    | |_- in this macro invocation

error: aborting due to 2 previous errors

