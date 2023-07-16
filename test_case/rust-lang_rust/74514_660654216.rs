
error[E0369]: binary operation `==` cannot be applied to type `impl AsRef<OsStr>`
    --> src/bootstrap/builder.rs:1474:9
     |
1474 |         assert_ne!(key, "RUSTFLAGS");
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |         |
     |         impl AsRef<OsStr>
     |         &str
     |
     = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
