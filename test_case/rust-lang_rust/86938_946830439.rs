plain
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking cfg-if v0.1.10
    Checking adler v0.2.3
    Checking rustc-demangle v0.1.21
error[E0433]: failed to resolve: use of undeclared type `TryReserveErrorKind`
    |
    |
245 |             Err(_) => return Err(TryReserveErrorKind::AllocError { layout, non_exhaustive: () }),
    |                                  ^^^^^^^^^^^^^^^^^^^ use of undeclared type `TryReserveErrorKind`

error[E0433]: failed to resolve: use of undeclared type `TryReserveErrorKind`
     |
     |
1218 |             .map_err(|_| TryReserveErrorKind::AllocError { layout, non_exhaustive: () })
     |                          ^^^^^^^^^^^^^^^^^^^ use of undeclared type `TryReserveErrorKind`

error[E0433]: failed to resolve: use of undeclared type `TryReserveErrorKind`
    |
    |
105 |             Err(TryReserveErrorKind::CapacityOverflow)
    |                 ^^^^^^^^^^^^^^^^^^^ use of undeclared type `TryReserveErrorKind`
error[E0308]: mismatched types
   --> library/alloc/src/collections/mod.rs:117:13
    |
92  |     CapacityOverflow,
92  |     CapacityOverflow,
    |     ---------------- unit variant defined here
...
116 |         match self {
    |               ---- this expression has type `TryReserveError`
117 |             TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `TryReserveError`, found enum `TryReserveErrorKind`
    |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
116 |         match self.kind {
    |               ~~~~~~~~~

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/alloc/src/collections/mod.rs:118:13
    |
116 |         match self {
    |               ---- this expression has type `TryReserveError`
117 |             TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
118 |             TryReserveErrorKind::AllocError { layout, non_exhaustive: () } => {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `TryReserveError`, found enum `TryReserveErrorKind`
    |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
116 |         match self.kind {
    |               ~~~~~~~~~


error[E0277]: the trait bound `TryReserveError: From<core::alloc::LayoutError>` is not satisfied
    |
    |
237 |         let layout = Layout::array::<T>(capacity)?;
    |                                                  ^ the trait `From<core::alloc::LayoutError>` is not implemented for `TryReserveError`
    = help: the following implementations were found:
    = help: the following implementations were found:
              <TryReserveError as From<TryReserveErrorKind>>
    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, core::alloc::LayoutError>>` for `Result<RawVec<T, A>, TryReserveError>`
note: required by `from_residual`
    |
    |
339 |     fn from_residual(residual: R) -> Self;


error[E0277]: the trait bound `TryReserveError: From<core::alloc::LayoutError>` is not satisfied
     |
     |
1212 |             let layout = Layout::array::<T>(len)?;
     |                                                 ^ the trait `From<core::alloc::LayoutError>` is not implemented for `TryReserveError`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <TryReserveError as From<TryReserveErrorKind>>
     = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, core::alloc::LayoutError>>` for `Result<*mut ArcInner<[T]>, TryReserveError>`
note: required by `from_residual`
     |
     |
339  |     fn from_residual(residual: R) -> Self;

Some errors have detailed explanations: E0277, E0308, E0433.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `alloc` due to 7 previous errors
