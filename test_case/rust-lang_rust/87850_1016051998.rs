text
error[E0391]: cycle detected when computing when `manifest::CargoDeb` has a significant destructor
   --> src/manifest.rs:906:1
    |
906 | struct CargoDeb {
    | ^^^^^^^^^^^^^^^
    |
    = note: ...which immediately requires computing when `manifest::CargoDeb` has a significant destructor again
    = note: cycle used when computing whether `manifest::Cargo` has a significant drop
