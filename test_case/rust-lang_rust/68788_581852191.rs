plain
2020-02-04T10:52:14.6170579Z     |
2020-02-04T10:52:14.6170970Z     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-02-04T10:52:14.6173798Z 
2020-02-04T10:52:14.9409925Z [RUSTC-TIMING] rustc_builtin_macros test:false 4.245
2020-02-04T10:52:22.7219184Z error[E0275]: overflow evaluating the requirement `std::ptr::Unique<syntax::ast::Path>: std::marker::Sync`
2020-02-04T10:52:22.7220687Z   |
2020-02-04T10:52:22.7221476Z   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
2020-02-04T10:52:22.7222259Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Path>`
2020-02-04T10:52:22.7223814Z   = note: required because it appears within the type `syntax::ast::VisibilityKind`
2020-02-04T10:52:22.7224735Z   = note: required because it appears within the type `rustc_span::source_map::Spanned<syntax::ast::VisibilityKind>`
2020-02-04T10:52:22.7225491Z   = note: required because it appears within the type `syntax::ast::Item`
2020-02-04T10:52:22.7226285Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2020-02-04T10:52:22.7226285Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2020-02-04T10:52:22.7227055Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
2020-02-04T10:52:22.7227813Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-02-04T10:52:22.7228527Z   = note: required because it appears within the type `syntax::token::Nonterminal`
2020-02-04T10:52:22.7229317Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<syntax::token::Nonterminal>`
2020-02-04T10:52:22.7232252Z   = note: required because it appears within the type `syntax::token::Token`
2020-02-04T10:52:22.7233186Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-02-04T10:52:22.7233186Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-02-04T10:52:22.7234054Z   = note: required because it appears within the type `(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)`
2020-02-04T10:52:22.7234966Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-02-04T10:52:22.7235836Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-02-04T10:52:22.7237229Z   = note: required because it appears within the type `std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-02-04T10:52:22.7238205Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>`
2020-02-04T10:52:22.7239468Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-02-04T10:52:22.7239468Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-02-04T10:52:22.7240116Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::MacArgs>`
2020-02-04T10:52:22.7240947Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::MacArgs>`
2020-02-04T10:52:22.7241570Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::MacArgs>`
2020-02-04T10:52:22.7242832Z   = note: required because it appears within the type `syntax::ast::PatKind`
2020-02-04T10:52:22.7243468Z   = note: required because it appears within the type `syntax::ast::Pat`
2020-02-04T10:52:22.7244142Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Pat>`
2020-02-04T10:52:22.7244795Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
---
2020-02-04T10:52:22.7251290Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
2020-02-04T10:52:22.7251928Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
2020-02-04T10:52:22.7252563Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
2020-02-04T10:52:22.7271626Z   = note: required because it appears within the type `syntax::ast::GenericArg`
2020-02-04T10:52:22.7272390Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArg>`
2020-02-04T10:52:22.7273028Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
2020-02-04T10:52:22.7273655Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
2020-02-04T10:52:22.7278079Z   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
2020-02-04T10:52:22.7279703Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
2020-02-04T10:52:22.7280390Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
2020-02-04T10:52:22.7281205Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
2020-02-04T10:52:22.7281945Z   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
---
2020-02-04T10:52:26.9643152Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2020-02-04T10:52:26.9947051Z error: Could not document `rustc_ast_lowering`.
2020-02-04T10:52:26.9948028Z 
2020-02-04T10:52:26.9948869Z Caused by:
2020-02-04T10:52:26.9954646Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_ast_lowering src/librustc_ast_lowering/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-4a57237387b77b3e.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-da6c3e1b18511131.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-dc4da9d9bdcfcd3e.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-aed95ce3c2cbc0b5.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-bc6a7be3780a271e.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-c185f352ba334eed.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-e414e969ff2a637d.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-ed182a2a16af924b.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-4906c325a2df8ef8.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-6f02fa7aae2cc6b0.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-cc09ca1932427855.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-a920d54ce271da24.rmeta --document-private-items` (exit code: 1)
2020-02-04T10:52:27.5696387Z [RUSTC-TIMING] rustc_codegen_utils test:false 0.598
2020-02-04T10:52:49.9170331Z warning: `[E0502]` cannot be resolved, ignoring it.
2020-02-04T10:52:49.9171663Z    --> src/librustc_mir/borrow_check/diagnostics/conflict_errors.rs:572:37
2020-02-04T10:52:49.9172359Z     |
---
2020-02-04T10:52:50.6576870Z 
2020-02-04T10:52:51.4864352Z error: build failed
2020-02-04T10:52:51.4895276Z 
2020-02-04T10:52:51.4896638Z 
2020-02-04T10:52:51.4899576Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_index" "-p" "rustc_plugin_impl" "-p" "rustc_feature" "-p" "rustc_typeck" "-p" "rustc_incremental" "-p" "arena" "-p" "rustc_lint" "-p" "rustc_mir" "-p" "graphviz" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_apfloat" "-p" "serialize" "-p" "rustc_target" "-p" "rustc_data_structures" "-p" "rustc_ty" "-p" "rustc_codegen_ssa" "-p" "rustc_errors" "-p" "fmt_macros" "-p" "rustc_attr" "-p" "rustc_mir_build" "-p" "rustc_metadata" "-p" "rustc_ast_lowering" "-p" "rustc_driver" "-p" "rustc_hir" "-p" "rustc_fs_util" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "syntax" "-p" "rustc_ast_pretty" "-p" "rustc_error_codes" "-p" "rustc" "-p" "rustc_codegen_utils" "-p" "rustc_save_analysis" "-p" "rustc_macros" "-p" "rustc_expand" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_session" "-p" "rustc_llvm" "-p" "build_helper" "-p" "rustc_span" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_traits"
2020-02-04T10:52:51.4900292Z 
2020-02-04T10:52:51.4900333Z 
2020-02-04T10:52:51.4909949Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-02-04T10:52:51.4910330Z Build completed unsuccessfully in 1:42:02
2020-02-04T10:52:51.4910330Z Build completed unsuccessfully in 1:42:02
2020-02-04T10:52:51.4960097Z == clock drift check ==
2020-02-04T10:52:51.4974872Z   local time: Tue Feb  4 10:52:51 UTC 2020
2020-02-04T10:52:51.9776491Z   network time: Tue, 04 Feb 2020 10:52:51 GMT
2020-02-04T10:52:51.9777348Z == end clock drift check ==
2020-02-04T10:52:53.2296710Z 
2020-02-04T10:52:53.2405483Z ##[error]Bash exited with code '1'.
2020-02-04T10:52:53.2451280Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-04T10:52:53.2453491Z ==============================================================================
2020-02-04T10:52:53.2453609Z Task         : Get sources
2020-02-04T10:52:53.2453715Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
