plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.66
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0545]: `issue` must be a non-zero numeric string or "none"
   |
   |
55 |     #[unstable(feature = "ptr_const_cast", issue = "")]
   |                                                    |
   |                                                    cannot parse integer from empty string


error[E0545]: `issue` must be a non-zero numeric string or "none"
   |
   |
56 |     #[rustc_const_unstable(feature = "ptr_const_cast", issue = "")]
   |                                                                |
   |                                                                cannot parse integer from empty string


error[E0545]: `issue` must be a non-zero numeric string or "none"
   |
   |
58 |     #[unstable(feature = "ptr_const_cast", issue = "")]
   |                                                    |
   |                                                    cannot parse integer from empty string


error[E0545]: `issue` must be a non-zero numeric string or "none"
   |
   |
59 |     #[rustc_const_unstable(feature = "ptr_const_cast", issue = "")]
   |                                                                |
   |                                                                cannot parse integer from empty string

For more information about this error, try `rustc --explain E0545`.
