plain
 Documenting rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
[RUSTC-TIMING] rustc_ast_passes test:false 0.964
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
 Documenting rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0275]: overflow evaluating the requirement `std::ptr::Unique<rustc_ast::FnDecl>: std::marker::Send`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`rustc_expand`)
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::FnDecl>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::FnDecl>`
  = note: required because it appears within the type `rustc_ast::FnSig`
  = note: required because it appears within the type `rustc_ast::FnKind`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<rustc_ast::FnKind>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::FnKind>`
  = note: required because it appears within the type `rustc_ast::ItemKind`
  = note: required because it appears within the type `rustc_ast::Item`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<rustc_ast::Item>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::Item>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::Item>`
  = note: required because it appears within the type `rustc_ast::token::Nonterminal`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<rustc_ast::token::Nonterminal>`
  = note: required because it appears within the type `rustc_ast::token::Token`
  = note: required because it appears within the type `rustc_ast::tokenstream::TokenTree`
  = note: required because it appears within the type `rustc_ast::tokenstream::TokenTree`
  = note: required because it appears within the type `(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)>`
  = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)>`
  = note: required because it appears within the type `std::vec::Vec<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)>`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<std::vec::Vec<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)>>`
  = note: required because it appears within the type `rustc_ast::tokenstream::TokenStream`
  = note: required because it appears within the type `rustc_ast::MacArgs`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::MacArgs>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::MacArgs>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::MacArgs>`
  = note: required because it appears within the type `rustc_ast::MacCall`
  = note: required because it appears within the type `rustc_ast::PatKind`
  = note: required because it appears within the type `rustc_ast::Pat`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::Pat>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::Pat>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::Pat>`
  = note: required because it appears within the type `rustc_ast::ExprKind`
  = note: required because it appears within the type `rustc_ast::Expr`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::Expr>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::Expr>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::Expr>`
  = note: required because it appears within the type `rustc_ast::AnonConst`
  = note: required because it appears within the type `rustc_ast::Ty`
  = note: required because it appears within the type `rustc_ast::Ty`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::Ty>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::Ty>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::Ty>`
  = note: required because it appears within the type `rustc_ast::GenericArg`
  = note: required because it appears within the type `rustc_ast::AngleBracketedArg`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::AngleBracketedArg>`
  = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<rustc_ast::AngleBracketedArg>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::AngleBracketedArg>`
  = note: required because it appears within the type `rustc_ast::AngleBracketedArgs`
  = note: required because it appears within the type `rustc_ast::GenericArgs`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::GenericArgs>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::GenericArgs>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::GenericArgs>`
  = note: required because it appears within the type `std::option::Option<rustc_ast::ptr::P<rustc_ast::GenericArgs>>`
  = note: required because it appears within the type `rustc_ast::PathSegment`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::PathSegment>`
  = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<rustc_ast::PathSegment>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::PathSegment>`
  = note: required because it appears within the type `rustc_ast::AttrItem`
  = note: required because it appears within the type `rustc_ast::AttrKind`
  = note: required because it appears within the type `rustc_ast::Attribute`
  = note: required because it appears within the type `rustc_ast::Attribute`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::Attribute>`
  = note: required because it appears within the type `smallvec::alloc::raw_vec::RawVec<rustc_ast::Attribute>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::Attribute>`
For more information about this error, try `rustc --explain E0275`.
error: could not document `rustc_expand`

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_expand compiler/rustc_expand/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-d0c4e2f0b670f279.rmeta --extern rustc_ast_passes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_passes-fdf2fb28f2fbf9d1.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-cbfa67232ad12f8e.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-ab518251e1696a09.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-2ff7bac5981ac3f8.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-18e1958b45c5311b.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-338d988ff6d6e324.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-83cfe981e8157502.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-683977ca32549746.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-7516fa25664181cf.so --extern rustc_parse=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_parse-ae9ab03526cf8160.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-db6f8e606bc96ed6.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-33ac5274a9de91ff.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2e9df1247654b5d2.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-066b78a4dc093f91.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-6d4418b4aaab503a.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.56.0-nightly
  (a83f4ae8c
  2021-08-06)' --document-private-items '-Arustdoc::private-intra-doc-links' --enable-index-page -Zunstable-options -Znormalize-docs --show-type-layout --cfg=parallel_compiler` (exit status: 1)
[RUSTC-TIMING] rustc_expand test:false 1.291
[RUSTC-TIMING] rustc_middle test:false 18.177
error: build failed



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-Zskip-rustdoc-fingerprint" "--no-deps" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "-p" "rustc_builtin_macros" "-p" "rustc_hir_pretty" "-p" "rustc_feature" "-p" "rustc_traits" "-p" "rustc_errors" "-p" "rustc_index" "-p" "rustc_infer" "-p" "rustc_lint_defs" "-p" "rustc_target" "-p" "rustc_incremental" "-p" "rustc_driver" "-p" "rustc_serialize" "-p" "rustc_query_impl" "-p" "rustc_ast" "-p" "rustc_typeck" "-p" "rustc_trait_selection" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_data_structures" "-p" "rustc_resolve" "-p" "rustc_interface" "-p" "rustc_parse_format" "-p" "rustc_expand" "-p" "rustc_session" "-p" "rustc_graphviz" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_query_system" "-p" "rustc_fs_util" "-p" "rustc_span" "-p" "rustc_error_codes" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_metadata" "-p" "rustc_ast_lowering" "-p" "rustc_lint" "-p" "rustc_mir_build" "-p" "rustc_symbol_mangling" "-p" "rustc_lexer" "-p" "rustc_type_ir" "-p" "rustc_ast_pretty" "-p" "rustc_arena" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_passes"


Build completed unsuccessfully in 0:08:17
