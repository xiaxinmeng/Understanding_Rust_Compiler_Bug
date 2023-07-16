rust
error[E0119]: conflicting implementations of trait `std::convert::From<SiteError>` for type `SiteError`:
   --> src/main.rs:183:1
    |
183 | impl<E: failure::Fail> From<E> for SiteError {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: conflicting implementation in crate `core`:
            - impl<T> std::convert::From<T> for T;
