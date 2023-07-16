plain
2020-01-19T09:50:05.2564511Z  Documenting rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-01-19T09:50:05.2885635Z [RUSTC-TIMING] rustc_feature test:false 9.784
2020-01-19T09:50:05.2886075Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-01-19T09:50:05.3147467Z  Documenting rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-01-19T09:50:07.4288542Z error[E0275]: overflow evaluating the requirement `alloc::raw_vec::RawVec<(syntax::ast::UseTree, syntax::node_id::NodeId)>: std::marker::Sync`
2020-01-19T09:50:07.4289742Z   |
2020-01-19T09:50:07.4290432Z   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
2020-01-19T09:50:07.4291138Z   = note: required because it appears within the type `std::vec::Vec<(syntax::ast::UseTree, syntax::node_id::NodeId)>`
2020-01-19T09:50:07.4291809Z   = note: required because it appears within the type `syntax::ast::UseTreeKind`
2020-01-19T09:50:07.4292501Z   = note: required because it appears within the type `syntax::ast::UseTree`
2020-01-19T09:50:07.4293303Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::UseTree>`
2020-01-19T09:50:07.4294034Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::UseTree>`
2020-01-19T09:50:07.4294877Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::UseTree>`
2020-01-19T09:50:07.4296757Z   = note: required because it appears within the type `syntax::ast::Item`
2020-01-19T09:50:07.4297541Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2020-01-19T09:50:07.4298507Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
2020-01-19T09:50:07.4299334Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-01-19T09:50:07.4299334Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-01-19T09:50:07.4300057Z   = note: required because it appears within the type `syntax::token::Nonterminal`
2020-01-19T09:50:07.4301274Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<syntax::token::Nonterminal>`
2020-01-19T09:50:07.4303162Z   = note: required because it appears within the type `syntax::token::Token`
2020-01-19T09:50:07.4304641Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-19T09:50:07.4304641Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-19T09:50:07.4305376Z   = note: required because it appears within the type `(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)`
2020-01-19T09:50:07.4306145Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-19T09:50:07.4307091Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-19T09:50:07.4307854Z   = note: required because it appears within the type `std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-19T09:50:07.4308664Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>`
2020-01-19T09:50:07.4310029Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-19T09:50:07.4310029Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-19T09:50:07.4311590Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::MacArgs>`
2020-01-19T09:50:07.4312457Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::MacArgs>`
2020-01-19T09:50:07.4313126Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::MacArgs>`
2020-01-19T09:50:07.4314229Z   = note: required because it appears within the type `syntax::ast::PatKind`
2020-01-19T09:50:07.4314770Z   = note: required because it appears within the type `syntax::ast::Pat`
2020-01-19T09:50:07.4315349Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Pat>`
2020-01-19T09:50:07.4315923Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
---
2020-01-19T09:50:07.4322265Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
2020-01-19T09:50:07.4322816Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
2020-01-19T09:50:07.4323371Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
2020-01-19T09:50:07.4323942Z   = note: required because it appears within the type `syntax::ast::GenericArg`
2020-01-19T09:50:07.4324708Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArg>`
2020-01-19T09:50:07.4325325Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
2020-01-19T09:50:07.4325917Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
2020-01-19T09:50:07.4326837Z   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
2020-01-19T09:50:07.4327968Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
2020-01-19T09:50:07.4328536Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
2020-01-19T09:50:07.4329077Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
2020-01-19T09:50:07.4329746Z   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
---
2020-01-19T09:50:12.5868264Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-01-19T09:50:12.6184010Z error: Could not document `rustc_ast_pretty`.
2020-01-19T09:50:12.6190625Z 
2020-01-19T09:50:12.6190787Z Caused by:
2020-01-19T09:50:12.6197739Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_ast_pretty src/librustc_ast_pretty/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-5ff1ff6f0eda1e40.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-f9e34e071a4bbb4e.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-f1f7e817b31b7500.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-24253dc2b90f700b.rmeta --document-private-items` (exit code: 1)
2020-01-19T09:50:12.6281333Z [RUSTC-TIMING] rustc_hir test:false 5.119
2020-01-19T09:50:14.5137349Z warning: `[Body::parameters]` cannot be resolved, ignoring it.
2020-01-19T09:50:14.5137704Z     --> src/librustc_hir/hir.rs:2067:69
2020-01-19T09:50:14.5137987Z      |
---
2020-01-19T09:50:15.2564617Z [RUSTC-TIMING] rustc_parse test:false 2.663
2020-01-19T09:50:15.2742352Z error: build failed
2020-01-19T09:50:15.2770516Z 
2020-01-19T09:50:15.2770595Z 
2020-01-19T09:50:15.2773155Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_typeck" "-p" "rustc_ast_lowering" "-p" "rustc_error_codes" "-p" "rustc_expand" "-p" "rustc_traits" "-p" "rustc_lexer" "-p" "rustc_ty" "-p" "rustc_feature" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_metadata" "-p" "rustc_macros" "-p" "rustc_incremental" "-p" "rustc_passes" "-p" "build_helper" "-p" "rustc" "-p" "rustc_fs_util" "-p" "rustc_span" "-p" "syntax" "-p" "rustc_lint" "-p" "rustc_llvm" "-p" "rustc_mir" "-p" "rustc_save_analysis" "-p" "rustc_driver" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_interface" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_utils" "-p" "rustc_apfloat" "-p" "rustc_ast_pretty" "-p" "rustc_session" "-p" "fmt_macros" "-p" "rustc_data_structures" "-p" "rustc_target" "-p" "serialize" "-p" "rustc_ast_passes" "-p" "rustc_hir" "-p" "rustc_errors" "-p" "rustc_parse" "-p" "rustc_index" "-p" "rustc_mir_build" "-p" "arena" "-p" "rustc_attr" "-p" "graphviz" "-p" "rustc_codegen_ssa"
2020-01-19T09:50:15.2777870Z 
2020-01-19T09:50:15.2777908Z 
2020-01-19T09:50:15.2786155Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-01-19T09:50:15.2786303Z Build completed unsuccessfully in 1:35:26
2020-01-19T09:50:15.2786303Z Build completed unsuccessfully in 1:35:26
2020-01-19T09:50:15.2841574Z == clock drift check ==
2020-01-19T09:50:15.2860778Z   local time: Sun Jan 19 09:50:15 UTC 2020
2020-01-19T09:50:15.7450266Z   network time: Sun, 19 Jan 2020 09:50:15 GMT
2020-01-19T09:50:15.7451287Z == end clock drift check ==
2020-01-19T09:50:18.0889943Z 
2020-01-19T09:50:18.0983825Z ##[error]Bash exited with code '1'.
2020-01-19T09:50:18.1023664Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-19T09:50:18.1025629Z ==============================================================================
2020-01-19T09:50:18.1025716Z Task         : Get sources
2020-01-19T09:50:18.1025812Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
