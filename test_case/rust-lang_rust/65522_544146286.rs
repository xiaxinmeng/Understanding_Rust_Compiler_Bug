rust
error[E0277]: arrays only have std trait implementations for lengths 0..=32
   --> src/lib.rs:362:5
    |
362 |     pub data: [::std::os::raw::c_char; 128usize],
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::array::LengthAtMost32` is not implemented for `[i8; 128]`
    |
    = note: required because of the requirements on the impl of `std::fmt::Debug` for `[i8; 128]`
    = note: required because of the requirements on the impl of `std::fmt::Debug` for `&[i8; 128]`
    = note: required for the cast to the object type `dyn std::fmt::Debug`
