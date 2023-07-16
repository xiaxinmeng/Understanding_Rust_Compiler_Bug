plain
 Documenting rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
 Documenting rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
 Documenting rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
[RUSTC-TIMING] rustc_attr test:false 0.514
error[E0275]: overflow evaluating the requirement `std::ptr::Unique<rustc_ast::Param>: std::marker::Send`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`rustc_parse`)
  = note: required because it appears within the type `alloc::raw_vec::RawVec<rustc_ast::Param>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::Param>`
  = note: required because it appears within the type `rustc_ast::FnDecl`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<rustc_ast::FnDecl>`
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
  = note: required because it appears within the type `rustc_ast::token::TokenKind`
  = note: required because it appears within the type `rustc_ast::token::Token`
  = note: required because it appears within the type `rustc_ast::tokenstream::TokenTree`
  = note: required because it appears within the type `(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)>`
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
  = note: required because it appears within the type `rustc_ast::TyKind`
  = note: required because it appears within the type `rustc_ast::Ty`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::Ty>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::Ty>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::Ty>`
  = note: required because it appears within the type `rustc_ast::GenericArg`
  = note: required because it appears within the type `rustc_ast::AngleBracketedArg`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::AngleBracketedArg>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<rustc_ast::AngleBracketedArg>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::AngleBracketedArg>`
  = note: required because it appears within the type `rustc_ast::AngleBracketedArgs`
  = note: required because it appears within the type `rustc_ast::GenericArgs`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::GenericArgs>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::GenericArgs>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::GenericArgs>`
  = note: required because it appears within the type `std::option::Option<rustc_ast::ptr::P<rustc_ast::GenericArgs>>`
  = note: required because it appears within the type `rustc_ast::PathSegment`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::PathSegment>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<rustc_ast::PathSegment>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::PathSegment>`
  = note: required because it appears within the type `rustc_ast::Path`
  = note: required because it appears within the type `rustc_ast::AttrItem`
  = note: required because it appears within the type `rustc_ast::AttrKind`
  = note: required because it appears within the type `rustc_ast::Attribute`
  = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<rustc_ast::Attribute>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<rustc_ast::Attribute>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::Attribute>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
error: could not document `rustc_parse`
error: could not document `rustc_parse`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_parse compiler/rustc_parse/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-2f4ad6ed99e8eeca.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-8b6de646b17f7698.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-ad30460d38ef7103.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1bcd359aa30f6700.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f614fa518bdbe09c.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-be31542a4bd19f99.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-63fdfb72cdb44d2f.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-5b92629ecdb6b4b2.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-d61d80cbdc5cb0d4.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-9fb44e28278df6b0.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-27629d44cb623deb.rmeta --extern unicode_normalization=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_normalization-8e79760e98364c96.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.52.0-nightly
  (9b3d7d5d8
  2021-03-14)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
[RUSTC-TIMING] rustc_query_system test:false 0.955
[RUSTC-TIMING] chalk_solve test:false 3.097
[RUSTC-TIMING] rustc_ast_lowering test:false 1.411
[RUSTC-TIMING] rustc_parse test:false 2.073
[RUSTC-TIMING] rustc_parse test:false 2.073
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_plugin_impl" "-p" "rustc_trait_selection" "-p" "rustc_lint_defs" "-p" "rustc_incremental" "-p" "rustc_query_impl" "-p" "rustc_ast_pretty" "-p" "rustc_attr" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_llvm" "-p" "rustc_ast_passes" "-p" "rustc_session" "-p" "rustc_save_analysis" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_span" "-p" "rustc_infer" "-p" "rustc_query_system" "-p" "rustc_driver" "-p" "rustc_data_structures" "-p" "rustc_hir_pretty" "-p" "rustc_serialize" "-p" "rustc_arena" "-p" "rustc_metadata" "-p" "rustc_type_ir" "-p" "rustc_mir" "-p" "rustc_index" "-p" "rustc_codegen_ssa" "-p" "rustc_expand" "-p" "rustc_parse_format" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_parse" "-p" "rustc_traits" "-p" "rustc_target" "-p" "rustc_lexer" "-p" "rustc_builtin_macros" "-p" "rustc_fs_util" "-p" "rustc_symbol_mangling" "-p" "rustc_graphviz" "-p" "rustc_resolve" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_ty_utils" "-p" "coverage_test_macros" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:08:05
