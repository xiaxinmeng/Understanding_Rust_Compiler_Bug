plain
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
error[E0308]: mismatched types
   --> compiler/rustc_macros/src/newtype.rs:150:31
    |
150 |             derive_paths.push(quote!(Ord));
    |                               ^^^^^^^^^^^ expected struct `syn::Path`, found struct `TokenStream2`
    = note: this error originates in the macro `quote` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0308]: mismatched types
   --> compiler/rustc_macros/src/newtype.rs:151:31
   --> compiler/rustc_macros/src/newtype.rs:151:31
    |
151 |             derive_paths.push(quote!(PartialOrd));
    |                               ^^^^^^^^^^^^^^^^^^ expected struct `syn::Path`, found struct `TokenStream2`
    = note: this error originates in the macro `quote` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_macros` due to 2 previous errors
