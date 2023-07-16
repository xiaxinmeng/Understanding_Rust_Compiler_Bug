plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0599]: no method named `try_find_layout_root` found for struct `QueryJobId` in the current scope
   --> compiler/rustc_query_system/src/query/mod.rs:130:46
    |
130 |             if let Some((info, depth)) = job.try_find_layout_root(map) {
    |                                              ^^^^^^^^^^^^^^^^^^^^ method not found in `QueryJobId`
   ::: compiler/rustc_query_system/src/query/job.rs:39:1
    |
    |
39  | pub struct QueryJobId(pub NonZeroU64);
    | --------------------- method `try_find_layout_root` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_query_system` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_query_system` due to previous error
