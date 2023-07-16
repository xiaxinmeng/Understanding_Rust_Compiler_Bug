plain
2020-01-20T13:34:36.1695915Z  Documenting rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-01-20T13:34:37.5280588Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-01-20T13:34:37.5654506Z  Documenting rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-01-20T13:34:38.5651766Z [RUSTC-TIMING] fmt_macros test:false 4.363
2020-01-20T13:34:38.6754173Z error[E0275]: overflow evaluating the requirement `alloc::raw_vec::RawVec<(syntax::ast::UseTree, syntax::node_id::NodeId)>: std::marker::Sync`
2020-01-20T13:34:38.6754538Z   |
2020-01-20T13:34:38.6754852Z   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
2020-01-20T13:34:38.6755227Z   = note: required because it appears within the type `std::vec::Vec<(syntax::ast::UseTree, syntax::node_id::NodeId)>`
2020-01-20T13:34:38.6755771Z   = note: required because it appears within the type `syntax::ast::UseTreeKind`
2020-01-20T13:34:38.6756186Z   = note: required because it appears within the type `syntax::ast::UseTree`
2020-01-20T13:34:38.6756593Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::UseTree>`
2020-01-20T13:34:38.6756991Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::UseTree>`
2020-01-20T13:34:38.6757338Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::UseTree>`
2020-01-20T13:34:38.6758020Z   = note: required because it appears within the type `syntax::ast::Item`
2020-01-20T13:34:38.6758416Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2020-01-20T13:34:38.6758793Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
2020-01-20T13:34:38.6759137Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-01-20T13:34:38.6759137Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-01-20T13:34:38.6759490Z   = note: required because it appears within the type `syntax::token::Nonterminal`
2020-01-20T13:34:38.6759887Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<syntax::token::Nonterminal>`
2020-01-20T13:34:38.6760592Z   = note: required because it appears within the type `syntax::token::Token`
2020-01-20T13:34:38.6760934Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-20T13:34:38.6760934Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-20T13:34:38.6761324Z   = note: required because it appears within the type `(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)`
2020-01-20T13:34:38.6761778Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-20T13:34:38.6762410Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-20T13:34:38.6762897Z   = note: required because it appears within the type `std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-20T13:34:38.6763536Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>`
2020-01-20T13:34:38.6764265Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-20T13:34:38.6764265Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-20T13:34:38.6764744Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::MacArgs>`
2020-01-20T13:34:38.6765173Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::MacArgs>`
2020-01-20T13:34:38.6765533Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::MacArgs>`
2020-01-20T13:34:38.6766214Z   = note: required because it appears within the type `syntax::ast::PatKind`
2020-01-20T13:34:38.6766553Z   = note: required because it appears within the type `syntax::ast::Pat`
2020-01-20T13:34:38.6766943Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Pat>`
2020-01-20T13:34:38.6767299Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
---
2020-01-20T13:34:38.6770818Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
2020-01-20T13:34:38.6771201Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
2020-01-20T13:34:38.6771565Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
2020-01-20T13:34:38.6771895Z   = note: required because it appears within the type `syntax::ast::GenericArg`
2020-01-20T13:34:38.6772413Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArg>`
2020-01-20T13:34:38.6772808Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
2020-01-20T13:34:38.6773161Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
2020-01-20T13:34:38.6773768Z   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
2020-01-20T13:34:38.6774754Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
2020-01-20T13:34:38.6775166Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
2020-01-20T13:34:38.6775536Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
2020-01-20T13:34:38.6775944Z   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
---
2020-01-20T13:34:41.9257129Z 
2020-01-20T13:34:42.3461280Z error: Could not document `rustc_ast_pretty`.
2020-01-20T13:34:42.3464125Z 
2020-01-20T13:34:42.3464394Z Caused by:
2020-01-20T13:34:42.3471612Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_ast_pretty src/librustc_ast_pretty/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-efdbb6b3ab05009c.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3eb98e4d59d2a8d4.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-5cb044b23674d736.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5fea0154b0a39f6b.rmeta --document-private-items` (exit code: 1)
2020-01-20T13:34:42.3533525Z [RUSTC-TIMING] rustc_feature test:false 8.179
2020-01-20T13:34:42.7341295Z [RUSTC-TIMING] rustc_hir test:false 4.020
2020-01-20T13:34:46.2281505Z warning: `[Body::parameters]` cannot be resolved, ignoring it.
2020-01-20T13:34:46.2282726Z     --> src/librustc_hir/hir.rs:2067:69
---
2020-01-20T13:34:46.2324667Z 
2020-01-20T13:34:46.7966737Z error: build failed
2020-01-20T13:34:46.7989373Z 
2020-01-20T13:34:46.7989463Z 
2020-01-20T13:34:46.7993185Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_llvm" "-p" "rustc_interface" "-p" "rustc_data_structures" "-p" "rustc_target" "-p" "rustc_passes" "-p" "serialize" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "syntax" "-p" "rustc_error_codes" "-p" "rustc_errors" "-p" "fmt_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_mir_build" "-p" "rustc_span" "-p" "graphviz" "-p" "rustc_feature" "-p" "rustc_codegen_ssa" "-p" "build_helper" "-p" "rustc_save_analysis" "-p" "rustc_ast_pretty" "-p" "arena" "-p" "rustc_session" "-p" "rustc_privacy" "-p" "rustc_index" "-p" "rustc_builtin_macros" "-p" "rustc_ty" "-p" "rustc_plugin_impl" "-p" "rustc" "-p" "rustc_expand" "-p" "rustc_apfloat" "-p" "rustc_codegen_utils" "-p" "rustc_hir" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "rustc_fs_util" "-p" "rustc_incremental" "-p" "rustc_driver" "-p" "rustc_macros" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_ast_passes" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_typeck" "-p" "rustc_traits"
2020-01-20T13:34:46.7993795Z 
2020-01-20T13:34:46.7993830Z 
2020-01-20T13:34:46.7998921Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-01-20T13:34:46.7999029Z Build completed unsuccessfully in 1:22:43
2020-01-20T13:34:46.7999029Z Build completed unsuccessfully in 1:22:43
2020-01-20T13:34:46.8046120Z == clock drift check ==
2020-01-20T13:34:46.8064796Z   local time: Mon Jan 20 13:34:46 UTC 2020
2020-01-20T13:34:47.6791189Z   network time: Mon, 20 Jan 2020 13:34:47 GMT
2020-01-20T13:34:47.6791747Z == end clock drift check ==
2020-01-20T13:34:50.0034270Z 
2020-01-20T13:34:50.0115594Z ##[error]Bash exited with code '1'.
2020-01-20T13:34:50.0149232Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-20T13:34:50.0151110Z ==============================================================================
2020-01-20T13:34:50.0151185Z Task         : Get sources
2020-01-20T13:34:50.0151269Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
