plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: diagnostics should only be created in `SessionDiagnostic`/`AddSubdiagnostic` impls
   --> compiler/rustc_query_impl/src/plumbing.rs:103:27
    |
103 |                 self.sess.fatal(&format!("queries overflow the depth limit!"));
    |
note: the lint level is defined here
   --> compiler/rustc_query_impl/src/lib.rs:11:9
    |
    |
11  | #![deny(rustc::diagnostic_outside_of_impl)]


error: diagnostics should be created using translatable messages
   --> compiler/rustc_query_impl/src/plumbing.rs:103:27
    |
103 |                 self.sess.fatal(&format!("queries overflow the depth limit!"));
    |
note: the lint level is defined here
   --> compiler/rustc_query_impl/src/lib.rs:10:9
    |
    |
10  | #![deny(rustc::untranslatable_diagnostic)]

error: could not compile `rustc_query_impl` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_query_impl` due to 2 previous errors
