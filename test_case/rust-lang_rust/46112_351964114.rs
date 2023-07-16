rust
error[E0308]: mismatched types
   --> src/bus/mod.rs:350:25
    |
350 |                         Err(BusError::ServiceTimeout(elapsed))
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `std::option::Option`, found enum `std::result::Result`
    |
    = note: expected type `std::option::Option<_>`
               found type `std::result::Result<_, bus::BusError>`
