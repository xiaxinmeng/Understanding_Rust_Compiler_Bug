plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
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

error[E0223]: ambiguous associated type
error[E0223]: ambiguous associated type
   --> library/alloc/src/collections/mod.rs:118:13
    |
118 |             TryReserveError::AllocError { layout, non_exhaustive: () } => {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<TryReserveError as Trait>::AllocError`

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

error[E0223]: ambiguous associated type
   --> library/alloc/src/raw_vec.rs:245:34
    |
    |
245 |             Err(_) => return Err(TryReserveError::AllocError { layout, non_exhaustive: () }),
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<TryReserveError as Trait>::AllocError`

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

error[E0223]: ambiguous associated type
    --> library/alloc/src/sync.rs:1218:26
     |
     |
1218 |             .map_err(|_| TryReserveError::AllocError { layout, non_exhaustive: () })
     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<TryReserveError as Trait>::AllocError`
Some errors have detailed explanations: E0223, E0277, E0308, E0433.
For more information about an error, try `rustc --explain E0223`.
error: could not compile `alloc` due to 7 previous errors
warning: build failed, waiting for other jobs to finish...
