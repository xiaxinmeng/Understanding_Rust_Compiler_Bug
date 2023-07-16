plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `sp` in this scope
   --> compiler/rustc_lint/src/builtin.rs:582:58
    |
582 |                 cx.tcx.sess.source_map().guess_head_span(sp),

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
