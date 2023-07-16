plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0277]: `?` couldn't convert the error to `rustc_errors::ErrorGuaranteed`
    |
365 |             })?;
    |               ^ the trait `From<()>` is not implemented for `rustc_errors::ErrorGuaranteed`
    |
    |
    = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
    = help: the following other types implement trait `FromResidual<R>`:
              <std::result::Result<T, F> as FromResidual<std::ops::Yeet<E>>>
              <std::result::Result<T, F> as FromResidual<std::result::Result<Infallible, E>>>
    = note: required because of the requirements on the impl of `FromResidual<std::result::Result<Infallible, ()>>` for `std::result::Result<(), rustc_errors::ErrorGuaranteed>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_interface` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_interface` due to previous error
