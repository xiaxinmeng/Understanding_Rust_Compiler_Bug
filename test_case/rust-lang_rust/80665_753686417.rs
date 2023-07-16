plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: unnecessary `unsafe` block
    --> compiler/rustc_query_impl/src/plumbing.rs:415:17
     |
205  |  /   macro_rules! define_queries {
206  |  |       (<$tcx:tt>
207  |  |        $($(#[$attr:meta])*
208  |  |           [$($modifiers:tt)*] fn $name:ident($K:ty) -> $V:ty,)*) => {
...     |
216  | /|           define_queries_struct! {
217  | ||               tcx: $tcx,
218  | ||               input: ($(([$($modifiers)*] [$($attr)*] [$name]))*)
219  | ||           }
     | ||___________- in this macro invocation (#3)
275  |  |       }
276  |  |   }
276  |  |   }
     |  |___- in this expansion of `define_queries!` (#2)
...
338  | /    macro_rules! define_queries_struct {
339  | |        (tcx: $tcx:tt,
340  | |         input: ($(([$($modifiers:tt)*] [$($attr:tt)*] [$name:ident]))*)) => {
341  | |            pub struct Queries<$tcx> {
...    |
415  | |                    unsafe { rustc_query_system::query::deadlock(tcx, registry) }
     | |                    ^^^^^^ unnecessary `unsafe` block
536  | |        };
537  | |    }
537  | |    }
     | |____- in this expansion of `define_queries_struct!` (#3)
    ::: compiler/rustc_query_impl/src/lib.rs:90:1
     |
     |
90   |      rustc_query_append! { [define_queries!][<'tcx>] }
     |      ------------------------------------------------- in this macro invocation (#1)
     | 
    ::: /checkout/compiler/rustc_middle/src/query/mod.rs:12:1
     |
12   |    / rustc_queries! {
13   |    |     Other {
14   |    |         query trigger_delay_span_bug(key: DefId) -> () {
15   |    |             desc { "trigger a delay span bug" }
1640 |    |     }
1641 |    | }
     |    | -
     |    | |
     |    | |
     |    |_in this expansion of `rustc_query_append!` (#1)
     |      in this macro invocation (#2)
     |
     = note: `-D unused-unsafe` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_query_impl`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_feature" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_attr" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_infer" "-p" "rustc_data_structures" "-p" "rustc_span" "-p" "rustc_metadata" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_ast_pretty" "-p" "rustc_errors" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_mir_build" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_ast_lowering" "-p" "rustc_symbol_mangling" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_serialize" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:26
