plain
   Compiling flycheck v0.0.0 (/checkout/src/tools/rust-analyzer/crates/flycheck)
   Compiling proc-macro-test v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-test)
   Compiling mbe v0.0.0 (/checkout/src/tools/rust-analyzer/crates/mbe)
   Compiling base-db v0.0.0 (/checkout/src/tools/rust-analyzer/crates/base-db)
error: value assigned to `file_id` is never read
   --> crates/base-db/src/fixture.rs:248:13
248 |             file_id.0 += 1;
    |             ^^^^^^^^^^^^^^
    |
    |
    = note: `-D unused-assignments` implied by `-D warnings`
    = help: maybe it is overwritten before being read?
error: could not compile `base-db` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:26:19
