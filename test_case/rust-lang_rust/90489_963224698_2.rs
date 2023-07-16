
   2: <rustc_metadata::creader::CrateLoader>::maybe_resolve_crate
   3: <rustc_metadata::creader::CrateLoader>::maybe_process_path_extern
             at /home/joshua/rustc2/compiler/rustc_metadata/src/creader.rs:1052:9
   4: <rustc_resolve::Resolver>::extern_prelude_get::{closure#0}
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:3293:21
   5: <core::option::Option<rustc_resolve::ExternPreludeEntry>>::and_then::<&rustc_resolve::NameBinding, <rustc_resolve::Resolver>::extern_prelude_get::{closure#0}>
             at /home/joshua/rustc2/library/core/src/option.rs:1053:24
   6: <rustc_resolve::Resolver>::extern_prelude_get
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:3283:9
   7: <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope::{closure#0}
             at /home/joshua/rustc2/compiler/rustc_resolve/src/macros.rs:850:51
   8: <rustc_resolve::Resolver>::visit_scopes::<core::result::Result<&rustc_resolve::NameBinding, rustc_resolve::Determinacy>, <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope::{closure#0}>
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:1824:50
   9: <rustc_resolve::Resolver>::early_resolve_ident_in_lexical_scope
             at /home/joshua/rustc2/compiler/rustc_resolve/src/macros.rs:685:28
  10: <rustc_resolve::Resolver>::resolve_path_with_ribs::{closure#1}
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:2330:21
  11: <rustc_resolve::Resolver>::resolve_path_with_ribs
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:2368:33
  12: <rustc_resolve::Resolver>::resolve_path
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:2193:9
  13: <rustc_resolve::Resolver>::resolve_ast_path
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:3349:15
  14: <rustc_resolve::Resolver>::resolve_str_path_error
             at /home/joshua/rustc2/compiler/rustc_resolve/src/lib.rs:3338:19
  15: <rustdoc::passes::collect_intra_doc_links::LinkCollector>::resolve_path::{closure#0}
             at /home/joshua/rustc2/src/librustdoc/passes/collect_intra_doc_links.rs:442:13
