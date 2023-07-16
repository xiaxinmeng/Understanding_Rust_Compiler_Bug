plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: expected one of `.`, `?`, `{`, or an operator, found `;`
    |
    |
238 |             let layout = match Layout::array::<T>(capacity)?;
    |                          -----                              ^ expected one of `.`, `?`, `{`, or an operator
    |                          while parsing this match expression
    |                          help: try removing this `match`


error[E0433]: failed to resolve: use of undeclared type `TryReserveErrorKind`
     |
     |
1222 |             .map_err(|_| TryReserveErrorKind::AllocError { layout, non_exhaustive: () })
     |                          ^^^^^^^^^^^^^^^^^^^ use of undeclared type `TryReserveErrorKind`

error[E0433]: failed to resolve: use of undeclared type `TryReserveErrorKind`
    |
    |
105 |             Err(TryReserveErrorKind::CapacityOverflow)
    |                 ^^^^^^^^^^^^^^^^^^^ use of undeclared type `TryReserveErrorKind`

error[E0603]: function `capacity_overflow` is private
    |
59  | use crate::raw_vec::capacity_overflow;
    |                     ^^^^^^^^^^^^^^^^^ private function
    |
    |
note: the function `capacity_overflow` is defined here
    |
    |
594 | fn capacity_overflow() -> ! {

error[E0308]: mismatched types
   --> library/alloc/src/collections/mod.rs:118:13
    |
    |
93  |     CapacityOverflow,
    |     ---------------- unit variant defined here
...
117 |         match self {
    |               ---- this expression has type `TryReserveError`
118 |             TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `TryReserveError`, found enum `TryReserveErrorKind`
    |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
117 |         match self.kind {
    |               ~~~~~~~~~

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> library/alloc/src/collections/mod.rs:119:13
    |
117 |         match self {
    |               ---- this expression has type `TryReserveError`
118 |             TryReserveErrorKind::CapacityOverflow => capacity_overflow(),
119 |             TryReserveErrorKind::AllocError { layout, non_exhaustive: () } => {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `TryReserveError`, found enum `TryReserveErrorKind`
    |
help: you might have meant to use field `kind` whose type is `TryReserveErrorKind`
117 |         match self.kind {
    |               ~~~~~~~~~


error[E0277]: `?` couldn't convert the error to `TryReserveError`
     |
     |
1216 |             let layout = Layout::array::<T>(len)?;
     |                                                 ^ the trait `From<core::alloc::LayoutError>` is not implemented for `TryReserveError`
     |
     = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
     = help: the following implementations were found:
               <TryReserveError as From<TryReserveErrorKind>>
     = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, core::alloc::LayoutError>>` for `Result<*mut ArcInner<[T]>, TryReserveError>`
Some errors have detailed explanations: E0277, E0308, E0433, E0603.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `alloc` due to 7 previous errors
warning: build failed, waiting for other jobs to finish...
