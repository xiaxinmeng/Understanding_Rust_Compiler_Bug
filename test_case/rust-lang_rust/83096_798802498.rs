plain
    Checking chalk-solve v0.55.0
[RUSTC-TIMING] rustc_errors test:false 1.097
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
 Documenting rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0275]: overflow evaluating the requirement `std::boxed::Box<rustc_ast::Local>: std::marker::Send`
  |
  = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`rustc_session`)
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::Local>`
  = note: required because it appears within the type `rustc_ast::StmtKind`
  = note: required because it appears within the type `rustc_ast::Stmt`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<rustc_ast::Stmt>`
  = note: required because it appears within the type `alloc::raw_vec::RawVec<rustc_ast::Stmt>`
  = note: required because it appears within the type `std::vec::Vec<rustc_ast::Stmt>`
  = note: required because it appears within the type `rustc_ast::Block`
  = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<rustc_ast::Block>`
  = note: required because it appears within the type `std::boxed::Box<rustc_ast::Block>`
  = note: required because it appears within the type `rustc_ast::ptr::P<rustc_ast::Block>`
  = note: required because it appears within the type `std::option::Option<rustc_ast::ptr::P<rustc_ast::Block>>`
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
error: could not document `rustc_session`
error: could not document `rustc_session`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_session compiler/rustc_session/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-2f4ad6ed99e8eeca.rmeta --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libgetopts-a25ab4e91ddfcdce.rmeta --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-25a116dd079bd38c.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-8b6de646b17f7698.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1bcd359aa30f6700.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f614fa518bdbe09c.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-be31542a4bd19f99.rmeta --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-94dd2387e0d3e1ef.rmeta --extern rustc_lint_defs=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lint_defs-973b98ea4e610604.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/release/deps/librustc_macros-e1634dc0d7ab121f.so --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-b9a24f71da7b43c7.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-d61d80cbdc5cb0d4.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-925ae0a0d65cf69a.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-27629d44cb623deb.rmeta -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.52.0-nightly
  (247d0ff60
  2021-03-13)' --document-private-items --enable-index-page -Zunstable-options -Znormalize-docs` (exit code: 1)
[RUSTC-TIMING] rustc_hir test:false 2.578
[RUSTC-TIMING] rustc_session test:false 1.415
[RUSTC-TIMING] chalk_solve test:false 2.911
error: build failed
error: build failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--no-deps" "-p" "rustc_symbol_mangling" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_query_impl" "-p" "rustc_mir" "-p" "rustc_builtin_macros" "-p" "rustc_infer" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_middle" "-p" "rustc_codegen_llvm" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_ast_passes" "-p" "rustc_mir_build" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "coverage_test_macros" "-p" "rustc_errors" "-p" "rustc_attr" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_ast" "-p" "rustc_plugin_impl" "-p" "rustc_ty_utils" "-p" "rustc_metadata" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_target" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_error_codes" "-p" "rustc_interface" "-p" "rustc_lint_defs" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_expand" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_ssa" "-p" "rustc_apfloat" "-p" "rustc_lexer" "-p" "rustc_lint" "-p" "rustc_llvm" "-p" "rustc_macros" "-p" "rustc_typeck" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_query_system" "-p" "rustc_fs_util" "-p" "rustc_incremental" "-p" "rustc_span" "-p" "rustc_trait_selection"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata
Build completed unsuccessfully in 0:08:03
