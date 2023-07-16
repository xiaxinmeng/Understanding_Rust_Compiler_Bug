plain
   Compiling tracing-attributes v0.1.13
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
   Compiling chalk-derive v0.55.0
   Compiling chalk-ir v0.55.0
error[E0282]: type annotations needed for `(_, _)`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/src/de.rs:646:52
    |
646 |     let let_values = vars.clone().zip(fields).map(|(var, field)| {
    |                                                    ^^^^^^^^^^^^ consider giving this closure parameter the explicit type `(_, _)`, with the type parameters specified
    |
    = note: type must be known at this point
   Compiling tracing v0.1.25
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
