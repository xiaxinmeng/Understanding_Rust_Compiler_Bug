plain
    |
694 |                 outlived_fr_name,
    |                 ^^^^^^^^^^^^^^^^
    |                 |
    |                 expected `&RegionName`, found struct `RegionName`
    |                 help: consider borrowing here: `outlived_fr_name: &outlived_fr_name`
error[E0308]: mismatched types
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:701:17
    |
701 |                 outlived_fr_name,
701 |                 outlived_fr_name,
    |                 ^^^^^^^^^^^^^^^^
    |                 |
    |                 expected `&RegionName`, found struct `RegionName`
    |                 help: consider borrowing here: `outlived_fr_name: &outlived_fr_name`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_borrowck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to 2 previous errors
