
---- [ui] src/test/ui-fulldeps/mod_dir_path_canonicalized.rs stdout ----

error: test run failed!
status: exit status: 101
command: "/home/njn/dev/rust1/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/mod_dir_path_canonicalized/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'WorkerLocal can only be used on the thread pool it was created on', /home/njn/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-core-0.4.1/src/worker_local.rs:49:17
stack backtrace:
   0: std::panicking::begin_panic::<&str>
   1: rustc_ast::attr::mk_attr_from_item
   2: <rustc_parse::parser::Parser>::collect_tokens_trailing_token::<rustc_ast::ast::Attribute, <rustc_parse::parser::Parser>::collect_tokens_no_attrs<rustc_ast::ast::Attribute, <rustc_parse::parser::Parser>::parse_attribute::{closure#0}>::{closure#0}>
   3: <rustc_parse::parser::Parser>::parse_inner_attributes
   4: <rustc_parse::parser::Parser>::parse_mod
   5: <rustc_parse::parser::Parser>::parse_crate_mod
   6: mod_dir_path_canonicalized::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
------------------------------------------

failures:
    [ui] src/test/ui-fulldeps/mod_dir_path_canonicalized.rs
