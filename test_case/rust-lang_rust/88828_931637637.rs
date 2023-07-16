
warning: dropping unsupported crate type `dylib` for target `x86_64-unknown-redox`

error[E0609]: no field `sa_sigaction` on type `sigaction`
    |
    |
337 |             action.sa_sigaction = libc::SIG_DFL;
    |
    |
    = note: available fields are: `sa_handler`, `sa_flags`, `sa_restorer`, `sa_mask`
For more information about this error, try `rustc --explain E0609`.
[RUSTC-TIMING] std test:false 2.497
warning: `std` (lib) generated 1 warning
