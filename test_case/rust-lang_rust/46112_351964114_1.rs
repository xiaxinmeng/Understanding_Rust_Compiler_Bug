rust
error[E0308]: mismatched types
   --> src/bus/mod.rs:339:37
    |
339 |                     .and_then(|len| T::from_bytes(&self.buffer[0..len]))
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `serde::export::Option`, found enum `serde::export::Result`
    |
    = note: expected type `serde::export::Option<_>`
               found type `serde::export::Result<C, services::ServiceError>`
