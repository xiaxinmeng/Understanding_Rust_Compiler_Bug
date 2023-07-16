plain
   Compiling addr2line v0.14.0
error: unused attribute
    --> library/std/src/path.rs:1077:39
     |
1077 | #[cfg_attr(not(any(bootstrap, test)), rustc_diagnostic_item = "PathBuf")]
     |
     |
     = note: `-D unused-attributes` implied by `-D warnings`
error: unused attribute
    --> library/std/src/path.rs:1732:39
     |
     |
1732 | #[cfg_attr(not(any(bootstrap, test)), rustc_diagnostic_item = "Path")]

error: aborting due to 2 previous errors

error: could not compile `std`
