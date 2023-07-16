plain
    Checking idna v0.2.0
error[E0599]: no variant or associated item named `CopyNonOverlapping` found for enum `StatementKind` in the current scope
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:219:24
    |
219 |         StatementKind::CopyNonOverlapping(box rustc_middle::mir::CopyNonOverlapping { dst, src, count }) => {

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_utils` due to previous error
warning: build failed, waiting for other jobs to finish...
