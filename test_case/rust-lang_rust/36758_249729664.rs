
error: environment variable `CFG_VERSION` not defined
  --> src/librustc_incremental/persist/file_format.rs:38:37
   |
38 | const RUSTC_VERSION: &'static str = env!("CFG_VERSION");
   |                                     ^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

Build failed, waiting for other jobs to finish...
error: Could not compile `rustc_incremental`.
