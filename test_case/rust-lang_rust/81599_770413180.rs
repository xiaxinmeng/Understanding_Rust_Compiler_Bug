plain
   Compiling libc v0.2.79
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0405]: cannot find trait `TrustedLen` in this scope
    |
    |
186 | unsafe impl<I> TrustedLen for Fuse<I> where I: TrustedLen {}
    |
help: consider importing this trait
    |
1   | use crate::iter::TrustedLen;
1   | use crate::iter::TrustedLen;
    |

error[E0405]: cannot find trait `TrustedLen` in this scope
    |
    |
186 | unsafe impl<I> TrustedLen for Fuse<I> where I: TrustedLen {}
    |
help: consider importing this trait
    |
1   | use crate::iter::TrustedLen;
1   | use crate::iter::TrustedLen;
    |

error: an `#[unstable]` annotation here has no effect
    |
    |
185 | #[unstable(feature = "trusted_len", issue = "37572")]
    |
    |
    = note: `#[deny(rustc::ineffective_unstable_trait_impl)]` on by default

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0405`.
