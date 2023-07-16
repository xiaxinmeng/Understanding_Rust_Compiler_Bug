plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.52
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0277]: the trait bound `&[T; N]: From<&mut [T]>` is not satisfied
    |
    |
202 |         <&Self>::try_from(slice).map(|r| *r)
    |         ^^^^^^^^^^^^^^^^^ the trait `From<&mut [T]>` is not implemented for `&[T; N]`
    |
note: required because of the requirements on the impl of `Into<&[T; N]>` for `&mut [T]`
    |
    |
537 | impl<T, U> Into<U> for T
    |            ^^^^^^^     ^
    = note: required because of the requirements on the impl of `TryFrom<&mut [T]>` for `&[T; N]`
note: required by `TryFrom::try_from`
    |
    |
479 |     fn try_from(value: T) -> Result<Self, Self::Error>;

error[E0308]: mismatched types
   --> library/core/src/array/mod.rs:202:9
    |
    |
201 |     fn try_from(slice: &mut [T]) -> Result<[T; N], TryFromSliceError> {
    |                                     --------------------------------- expected `result::Result<[T; N], TryFromSliceError>` because of return type
202 |         <&Self>::try_from(slice).map(|r| *r)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `TryFromSliceError`, found enum `Infallible`
    |
    = note: expected enum `result::Result<_, TryFromSliceError>`
               found enum `result::Result<_, Infallible>`

error[E0277]: the trait bound `&[T; N]: From<&mut [T]>` is not satisfied
    |
    |
202 |         <&Self>::try_from(slice).map(|r| *r)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<&mut [T]>` is not implemented for `&[T; N]`
    |
note: required because of the requirements on the impl of `Into<&[T; N]>` for `&mut [T]`
    |
    |
537 | impl<T, U> Into<U> for T
    |            ^^^^^^^     ^
    = note: required because of the requirements on the impl of `TryFrom<&mut [T]>` for `&[T; N]`
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `core` due to 3 previous errors
Build completed unsuccessfully in 0:00:07
