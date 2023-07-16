plain
   Compiling rustc-demangle v0.1.21
error[E0308]: mismatched types
   --> library/alloc/src/collections/mod.rs:125:9
    |
124 |     fn from(_: LayoutError) -> Self {
    |                                ---- expected `TryReserveError` because of return type
125 |         TryReserveErrorKind::CapacityOverflow
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `TryReserveError`, found enum `TryReserveErrorKind`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
