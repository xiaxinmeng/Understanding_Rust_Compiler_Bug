plain
    Checking clippy_utils v0.1.54 (/checkout/src/tools/clippy/clippy_utils)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_utils/src/paths.rs:119:33
    |
119 | pub const PTR_COPY: [&str; 4] = ["core", "intrinsics", "copy"];
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 4 elements, found one with 3 elements
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_utils/src/paths.rs:120:48
    |
    |
120 | pub const PTR_COPY_NONOVERLAPPING: [&str; 4] = ["core", "intrinsics", "copy_nonoverlapping"];
    |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 4 elements, found one with 3 elements
    Checking cargo_metadata v0.12.0
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
