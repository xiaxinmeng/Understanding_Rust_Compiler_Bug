plain
    |
135 |         CString,
    |         ^^^^^^^

error[E0432]: unresolved import `super::kw::MacroRules`
    --> compiler/rustc_span/src/symbol.rs:1674:13
     |
1674 |     pub use super::kw::MacroRules as macro_rules;
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `MacroRules` in `symbol::kw`

error[E0425]: cannot find value `rust_2015_preview` in module `sym`
   |
   |
59 |             Edition::Edition2015 => sym::rust_2015_preview,
   |                                          ^^^^^^^^^^^^^^^^^ not found in `sym`

error[E0425]: cannot find value `rust_2018_preview` in module `sym`
   |
   |
60 |             Edition::Edition2018 => sym::rust_2018_preview,
   |                                          ^^^^^^^^^^^^^^^^^ not found in `sym`

error[E0425]: cannot find value `rust_2021_preview` in module `sym`
   |
   |
61 |             Edition::Edition2021 => sym::rust_2021_preview,
   |                                          ^^^^^^^^^^^^^^^^^ not found in `sym`

error[E0531]: cannot find unit struct, unit variant or constant `include` in module `sym`
    |
    |
149 |                     ExpnKind::Macro { kind: MacroKind::Bang, name: sym::include, proc_macro: _ }
    |                                                                         ^^^^^^^ not found in `sym`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
195 |                 dollar_crate_name: kw::DollarCrate,
    |                                        ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
350 |                         dollar_crate_name: kw::DollarCrate,
    |                                                ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
370 |                         dollar_crate_name: kw::DollarCrate,
    |                                                ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
386 |                 dollar_crate_name: kw::DollarCrate,
    |                                        ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    |
    |
409 |                 .take_while(|scdata| scdata.dollar_crate_name == kw::DollarCrate)
    |                                                                      ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `PathRoot` in module `kw`
    |
    |
863 |             ExpnKind::Root => kw::PathRoot.to_string(),
    |                                   ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
     |
     |
1155 |             dollar_crate_name: kw::Empty,
     |                                    ^^^^^ not found in `kw`
help: consider importing one of these items
     |
27   | use core::num::IntErrorKind::Empty;
     |
     |
27   | use crate::hygiene::field::Empty;
27   | use std::num::IntErrorKind::Empty;
     |
27   | use std::sync::mpsc::TryRecvError::Empty;
     |
     |
       and 1 other candidate

error[E0425]: cannot find value `DollarCrate` in module `kw`
     |
     |
1173 |     ctxt_data.dollar_crate_name = kw::DollarCrate;
     |                                       ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
     |
     |
1182 |         assert_eq!(dummy.dollar_crate_name, kw::Empty);
     |                                                 ^^^^^ not found in `kw`
help: consider importing one of these items
     |
27   | use core::num::IntErrorKind::Empty;
     |
     |
27   | use crate::hygiene::field::Empty;
27   | use std::num::IntErrorKind::Empty;
     |
27   | use std::sync::mpsc::TryRecvError::Empty;
     |
     |
       and 1 other candidate

error[E0425]: cannot find value `Empty` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1360:36
     |
1360 |         Ident::with_dummy_span(kw::Empty)
     |                                    ^^^^^ not found in `kw`
help: consider importing one of these items
     |
5    | use core::num::IntErrorKind::Empty;
     |
     |
5    | use crate::hygiene::event::field::Empty;
5    | use crate::source_map::io::error::error::num::IntErrorKind::Empty;
     |
     |
5    | use crate::source_map::io::sys::rand::mem::fmt::str::pattern::StrSearcherImpl::Empty;
       and 4 other candidates


error[E0425]: cannot find value `DollarCrate` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1474:38
     |
1474 |         } else if self.symbol == kw::DollarCrate {
     |                                      ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1551:21
     |
1551 |         self == kw::Empty
     |                     ^^^^^ not found in `kw`
help: consider importing one of these items
     |
5    | use core::num::IntErrorKind::Empty;
     |
     |
5    | use crate::hygiene::event::field::Empty;
5    | use crate::source_map::io::error::error::num::IntErrorKind::Empty;
     |
     |
5    | use crate::source_map::io::sys::rand::mem::fmt::str::pattern::StrSearcherImpl::Empty;
       and 4 other candidates


error[E0425]: cannot find value `Underscore` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1692:21
     |
1692 |         self <= kw::Underscore
     |                     ^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `As` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1696:21
     |
1696 |         self >= kw::As && self <= kw::While
     |                     ^^ not found in `kw`

error[E0425]: cannot find value `While` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1696:39
     |
1696 |         self >= kw::As && self <= kw::While
     |                                       ^^^^^ not found in `kw`

error[E0425]: cannot find value `Async` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1700:22
     |
1700 |         (self >= kw::Async && self <= kw::Dyn) && edition() >= Edition::Edition2018
     |                      ^^^^^ not found in `kw`
help: consider importing this unit variant
     |
5    | use crate::DesugaringKind::Async;
     |
     |

error[E0425]: cannot find value `Dyn` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1700:43
     |
1700 |         (self >= kw::Async && self <= kw::Dyn) && edition() >= Edition::Edition2018
     |                                           ^^^ not found in `kw`

error[E0425]: cannot find value `Abstract` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1704:21
     |
1704 |         self >= kw::Abstract && self <= kw::Yield
     |                     ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Yield` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1704:45
     |
1704 |         self >= kw::Abstract && self <= kw::Yield
     |                                             ^^^^^ not found in `kw`

error[E0425]: cannot find value `Try` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1708:21
     |
1708 |         self == kw::Try && edition() >= Edition::Edition2018
     |                     ^^^ not found in `kw`

error[E0425]: cannot find value `Super` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1721:21
     |
1721 |         self == kw::Super
     |                     ^^^^^ not found in `kw`

error[E0425]: cannot find value `SelfLower` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1722:28
     |
1722 |             || self == kw::SelfLower
     |                            ^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `SelfUpper` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1723:28
     |
1723 |             || self == kw::SelfUpper
     |                            ^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `Crate` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1724:28
     |
1724 |             || self == kw::Crate
     |                            ^^^^^ not found in `kw`

error[E0425]: cannot find value `PathRoot` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1725:28
     |
1725 |             || self == kw::PathRoot
     |                            ^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `DollarCrate` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1726:28
     |
1726 |             || self == kw::DollarCrate
     |                            ^^^^^^^^^^^ not found in `kw`

error[E0425]: cannot find value `True` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1731:21
     |
1731 |         self == kw::True || self == kw::False
     |                     ^^^^ not found in `kw`

error[E0425]: cannot find value `False` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1731:41
     |
1731 |         self == kw::True || self == kw::False
     |                                         ^^^^^ not found in `kw`

error[E0425]: cannot find value `Empty` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1736:21
     |
1736 |         self != kw::Empty && self != kw::Underscore && !self.is_path_segment_keyword()
     |                     ^^^^^ not found in `kw`
help: consider importing one of these items
     |
5    | use core::num::IntErrorKind::Empty;
     |
     |
5    | use crate::hygiene::event::field::Empty;
5    | use crate::source_map::io::error::error::num::IntErrorKind::Empty;
     |
     |
5    | use crate::source_map::io::sys::rand::mem::fmt::str::pattern::StrSearcherImpl::Empty;
       and 4 other candidates


error[E0425]: cannot find value `Underscore` in module `kw`
    --> compiler/rustc_span/src/symbol.rs:1736:42
     |
1736 |         self != kw::Empty && self != kw::Underscore && !self.is_path_segment_keyword()
     |                                          ^^^^^^^^^^ not found in `kw`
error: aborting due to 36 previous errors

Some errors have detailed explanations: E0425, E0432, E0531.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_span`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_session" "-p" "rustc_macros" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_plugin_impl" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_ty_utils" "-p" "rustc_target" "-p" "rustc_error_codes" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:36
