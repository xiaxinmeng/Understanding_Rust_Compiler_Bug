
   Compiling xsv v0.10.3 (file:///C:/projects/rust/build/ct/xsv)
error[E0283]: type annotations required: cannot resolve `f64: std::ops::AddAssign<_>`
   --> src\cmd\stats.rs:470:32
    |
470 |                         *float += from_bytes(sample).unwrap();
    |                                ^^
