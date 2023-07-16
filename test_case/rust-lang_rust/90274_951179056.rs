plain
Updating files:  98% (30537/31041)
Updating files:  99% (30731/31041)
Updating files: 100% (31041/31041)
Updating files: 100% (31041/31041), done.
Switched to a new branch 'try'
Branch 'try' set up to track remote branch 'try' from 'origin'.
[command]/usr/local/bin/git log -1 --format='%H'
'5149eb5f1733b5aeadfdd0a2e8a18c34cc023b71'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
   Compiling tera v1.10.0
[RUSTC-TIMING] rustdoc_json_types test:false 4.575
[RUSTC-TIMING] tracing_subscriber test:false 15.049
   Compiling rustdoc v0.0.0 (/Users/runner/work/rust/rust/src/librustdoc)
error[E0432]: unresolved import `rustc_middle::ty::WithConstness`
 --> src/librustdoc/clean/blanket_impl.rs:6:37
  |
6 | use rustc_middle::ty::{ToPredicate, WithConstness};
  |                                     ^^^^^^^^^^^^^ no `WithConstness` in `ty`
For more information about this error, try `rustc --explain E0432`.
[RUSTC-TIMING] rustdoc test:false 6.018
error: could not compile `rustdoc` due to previous error
warning: build failed, waiting for other jobs to finish...
