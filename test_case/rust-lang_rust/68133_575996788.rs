plain
2020-01-19T11:46:44.5456792Z    |
2020-01-19T11:46:44.5457535Z    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
2020-01-19T11:46:44.5457864Z 
2020-01-19T11:46:45.0830312Z  Documenting rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-01-19T11:46:45.5955424Z error[E0275]: overflow evaluating the requirement `alloc::raw_vec::RawVec<(syntax::ast::UseTree, syntax::node_id::NodeId)>: std::marker::Sync`
2020-01-19T11:46:45.5955878Z   |
2020-01-19T11:46:45.5956636Z   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
2020-01-19T11:46:45.5957119Z   = note: required because it appears within the type `std::vec::Vec<(syntax::ast::UseTree, syntax::node_id::NodeId)>`
2020-01-19T11:46:45.5957538Z   = note: required because it appears within the type `syntax::ast::UseTreeKind`
2020-01-19T11:46:45.5958252Z   = note: required because it appears within the type `syntax::ast::UseTree`
2020-01-19T11:46:45.5958731Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::UseTree>`
2020-01-19T11:46:45.5959165Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::UseTree>`
2020-01-19T11:46:45.5959645Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::UseTree>`
2020-01-19T11:46:45.5960558Z   = note: required because it appears within the type `syntax::ast::Item`
2020-01-19T11:46:45.5961142Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2020-01-19T11:46:45.5961601Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
2020-01-19T11:46:45.5962031Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-01-19T11:46:45.5962031Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-01-19T11:46:45.5962435Z   = note: required because it appears within the type `syntax::token::Nonterminal`
2020-01-19T11:46:45.5962940Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<syntax::token::Nonterminal>`
2020-01-19T11:46:45.5964461Z   = note: required because it appears within the type `syntax::token::Token`
2020-01-19T11:46:45.5964881Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-19T11:46:45.5964881Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-19T11:46:45.5965534Z   = note: required because it appears within the type `(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)`
2020-01-19T11:46:45.5966305Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-19T11:46:45.5966871Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-19T11:46:45.5967393Z   = note: required because it appears within the type `std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-19T11:46:45.5967941Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>`
2020-01-19T11:46:45.5968954Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-19T11:46:45.5968954Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-19T11:46:45.5969393Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::MacArgs>`
2020-01-19T11:46:45.5970229Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::MacArgs>`
2020-01-19T11:46:45.5970675Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::MacArgs>`
2020-01-19T11:46:45.5971736Z   = note: required because it appears within the type `syntax::ast::PatKind`
2020-01-19T11:46:45.5972183Z   = note: required because it appears within the type `syntax::ast::Pat`
2020-01-19T11:46:45.5974179Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Pat>`
2020-01-19T11:46:45.5975008Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
---
2020-01-19T11:46:45.5980466Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
2020-01-19T11:46:45.5980900Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
2020-01-19T11:46:45.5981319Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
2020-01-19T11:46:45.5981818Z   = note: required because it appears within the type `syntax::ast::GenericArg`
2020-01-19T11:46:45.5982280Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArg>`
2020-01-19T11:46:45.5982743Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
2020-01-19T11:46:45.5983312Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
2020-01-19T11:46:45.5983725Z   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
2020-01-19T11:46:45.5984800Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
2020-01-19T11:46:45.5985400Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
2020-01-19T11:46:45.5986476Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
2020-01-19T11:46:45.5986929Z   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
---
2020-01-19T11:47:59.2936878Z  Documenting rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-01-19T11:47:59.3288953Z error: Could not document `rustc_ast_pretty`.
2020-01-19T11:47:59.3292969Z 
2020-01-19T11:47:59.3293374Z Caused by:
2020-01-19T11:47:59.3302051Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_ast_pretty src/librustc_ast_pretty/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6e008a44e139dcf.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-55d9268df61feac7.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-ce294e742384616d.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-c9b9cc5458288484.rmeta --document-private-items` (exit code: 1)
2020-01-19T11:48:14.3941867Z warning: `[link_reborrowed_region]` cannot be resolved, ignoring it.
2020-01-19T11:48:14.3943283Z     --> src/librustc_typeck/check/regionck.rs:1204:45
2020-01-19T11:48:14.3943956Z      |
2020-01-19T11:48:14.3944862Z 1204 |     /// of the borrow that's provided. See [link_reborrowed_region] for some
---
2020-01-19T11:48:34.0808993Z 
2020-01-19T11:48:35.7480826Z error: build failed
2020-01-19T11:48:35.7511676Z 
2020-01-19T11:48:35.7512171Z 
2020-01-19T11:48:35.7518809Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_hir" "-p" "rustc_expand" "-p" "rustc_data_structures" "-p" "graphviz" "-p" "rustc_attr" "-p" "rustc_codegen_utils" "-p" "rustc_llvm" "-p" "rustc_index" "-p" "rustc_ty" "-p" "rustc_codegen_ssa" "-p" "rustc_driver" "-p" "rustc_span" "-p" "rustc_resolve" "-p" "rustc_feature" "-p" "rustc_parse" "-p" "rustc_ast_passes" "-p" "rustc_privacy" "-p" "rustc_macros" "-p" "rustc_lint" "-p" "rustc_target" "-p" "serialize" "-p" "rustc_errors" "-p" "rustc_fs_util" "-p" "syntax" "-p" "rustc_lexer" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_passes" "-p" "rustc_mir" "-p" "rustc_builtin_macros" "-p" "rustc_metadata" "-p" "arena" "-p" "rustc_apfloat" "-p" "build_helper" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_ast_pretty" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_incremental" "-p" "fmt_macros" "-p" "rustc_ast_lowering" "-p" "rustc_save_analysis" "-p" "rustc_mir_build" "-p" "rustc" "-p" "rustc_session"
2020-01-19T11:48:35.7520336Z 
2020-01-19T11:48:35.7520492Z 
2020-01-19T11:48:35.7532884Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-01-19T11:48:35.7533325Z Build completed unsuccessfully in 1:48:58
2020-01-19T11:48:35.7533325Z Build completed unsuccessfully in 1:48:58
2020-01-19T11:48:35.7592204Z == clock drift check ==
2020-01-19T11:48:35.7614237Z   local time: Sun Jan 19 11:48:35 UTC 2020
2020-01-19T11:48:35.9956233Z   network time: Sun, 19 Jan 2020 11:48:35 GMT
2020-01-19T11:48:35.9956956Z == end clock drift check ==
2020-01-19T11:48:37.2953875Z 
2020-01-19T11:48:37.3054889Z ##[error]Bash exited with code '1'.
2020-01-19T11:48:37.3098941Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-19T11:48:37.3101868Z ==============================================================================
2020-01-19T11:48:37.3101966Z Task         : Get sources
2020-01-19T11:48:37.3102073Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
