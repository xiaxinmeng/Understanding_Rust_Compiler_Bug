
error: unused attribute
   --> src/libcore/convert.rs:565:1
    |
565 | / #[rustc_reservation_impl = "permitting this impl would forbid us from adding \
566 | |                             `impl<T> From<!> for T` later; see rust-lang/rust#64715 for details"]
    | |_________________________________________________________________________________________________^
    |
    = note: `-D unused-attributes` implied by `-D warnings`

error: aborting due to previous error
