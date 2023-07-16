plain
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0599]: no method named `try_collect_active_jobs` found for reference `&'b CTX` in the current scope
   --> compiler/rustc_query_system/src/query/plumbing.rs:184:33
    |
184 | ...                   tcx.try_collect_active_jobs().unwrap(),
    |                           ^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&'b CTX`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_query_system` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:02:05
