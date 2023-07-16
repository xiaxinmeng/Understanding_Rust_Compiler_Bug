plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0422]: cannot find struct, variant or union type `QueryCtxt` in this scope
    |
    |
182 |                 QueryCtxt { tcx }
    |
help: consider importing this struct
    |
1   | use rustc_query_impl::QueryCtxt;
1   | use rustc_query_impl::QueryCtxt;
    |

error: unused import: `QueryContext`
   --> compiler/rustc_interface/src/util.rs:171:38
    |
171 |     use rustc_query_impl::{deadlock, QueryContext};
    |
    |
    = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0422`.
error: could not compile `rustc_interface` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_interface` due to 2 previous errors
