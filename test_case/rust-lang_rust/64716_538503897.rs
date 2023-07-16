plain
2019-10-04T18:05:08.0822653Z warning: the 'passes' flag is considered deprecated
2019-10-04T18:05:08.0823106Z   |
2019-10-04T18:05:08.0823465Z   = warning: please see https://github.com/rust-lang/rust/issues/44136
2019-10-04T18:05:08.0823608Z 
2019-10-04T18:05:08.9627381Z error[E0275]: overflow evaluating the requirement `alloc::raw_vec::RawVec<(syntax::ast::UseTree, syntax::ast::node_id_inner::NodeId)>: std::marker::Sync`
2019-10-04T18:05:08.9628838Z   |
2019-10-04T18:05:08.9630067Z   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
2019-10-04T18:05:08.9631194Z   = note: required because it appears within the type `std::vec::Vec<(syntax::ast::UseTree, syntax::ast::node_id_inner::NodeId)>`
2019-10-04T18:05:08.9631948Z   = note: required because it appears within the type `syntax::ast::UseTreeKind`
2019-10-04T18:05:08.9632683Z   = note: required because it appears within the type `syntax::ast::UseTree`
2019-10-04T18:05:08.9633648Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::UseTree>`
2019-10-04T18:05:08.9635371Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::UseTree>`
2019-10-04T18:05:08.9636201Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::UseTree>`
2019-10-04T18:05:08.9636935Z   = note: required because it appears within the type `syntax::ast::ItemKind`
2019-10-04T18:05:08.9637680Z   = note: required because it appears within the type `syntax::ast::Item`
2019-10-04T18:05:08.9638471Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2019-10-04T18:05:08.9639230Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
2019-10-04T18:05:08.9646931Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2019-10-04T18:05:08.9648903Z   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
2019-10-04T18:05:08.9651875Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<syntax::parse::token::Nonterminal>`
2019-10-04T18:05:08.9652340Z   = note: required because it appears within the type `syntax::parse::token::TokenKind`
2019-10-04T18:05:08.9652954Z   = note: required because it appears within the type `syntax::parse::token::Token`
2019-10-04T18:05:08.9653415Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2019-10-04T18:05:08.9660098Z   = note: required because it appears within the type `(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)`
2019-10-04T18:05:08.9660855Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2019-10-04T18:05:08.9661321Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2019-10-04T18:05:08.9661792Z   = note: required because it appears within the type `std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2019-10-04T18:05:08.9662315Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>`
2019-10-04T18:05:08.9680701Z   = note: required because it appears within the type `std::option::Option<std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>>`
2019-10-04T18:05:08.9681378Z   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
2019-10-04T18:05:08.9681781Z   = note: required because it appears within the type `syntax::ast::Mac`
2019-10-04T18:05:08.9682680Z   = note: required because it appears within the type `syntax::ast::PatKind`
2019-10-04T18:05:08.9683110Z   = note: required because it appears within the type `syntax::ast::Pat`
2019-10-04T18:05:08.9683601Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Pat>`
2019-10-04T18:05:08.9684542Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
2019-10-04T18:05:08.9684961Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Pat>`
2019-10-04T18:05:08.9685368Z   = note: required because it appears within the type `syntax::ast::ExprKind`
2019-10-04T18:05:08.9685745Z   = note: required because it appears within the type `syntax::ast::Expr`
2019-10-04T18:05:08.9686217Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Expr>`
2019-10-04T18:05:08.9686655Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
2019-10-04T18:05:08.9687073Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
2019-10-04T18:05:08.9687486Z   = note: required because it appears within the type `syntax::ast::AnonConst`
2019-10-04T18:05:08.9687911Z   = note: required because it appears within the type `syntax::ast::TyKind`
2019-10-04T18:05:08.9688311Z   = note: required because it appears within the type `syntax::ast::Ty`
2019-10-04T18:05:08.9688948Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
2019-10-04T18:05:08.9689459Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
2019-10-04T18:05:08.9689899Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
2019-10-04T18:05:08.9690334Z   = note: required because it appears within the type `syntax::ast::GenericArg`
2019-10-04T18:05:08.9691034Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArg>`
2019-10-04T18:05:08.9691481Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
2019-10-04T18:05:08.9692084Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
2019-10-04T18:05:08.9692488Z   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
2019-10-04T18:05:08.9693038Z   = note: required because it appears within the type `syntax::ast::GenericArgs`
2019-10-04T18:05:08.9694021Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
2019-10-04T18:05:08.9694501Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
2019-10-04T18:05:08.9694945Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
2019-10-04T18:05:08.9695410Z   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
2019-10-04T18:05:08.9695813Z   = note: required because it appears within the type `syntax::ast::PathSegment`
2019-10-04T18:05:08.9696296Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathSegment>`
2019-10-04T18:05:08.9696727Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
2019-10-04T18:05:08.9697157Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
2019-10-04T18:05:08.9697566Z   = note: required because it appears within the type `syntax::ast::Path`
2019-10-04T18:05:08.9698051Z   = note: required because it appears within the type `syntax::ast::AttrItem`
2019-10-04T18:05:08.9698628Z   = note: required because it appears within the type `syntax::ast::Attribute`
2019-10-04T18:05:08.9699113Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
2019-10-04T18:05:08.9700675Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
2019-10-04T18:05:08.9701284Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
2019-10-04T18:05:08.9981417Z error: aborting due to previous error
2019-10-04T18:05:08.9981769Z 
2019-10-04T18:05:08.9982090Z For more information about this error, try `rustc --explain E0275`.
2019-10-04T18:05:09.0131539Z [RUSTC-TIMING] rustc_resolve test:false 23.483
2019-10-04T18:05:09.0131539Z [RUSTC-TIMING] rustc_resolve test:false 23.483
2019-10-04T18:05:09.0205655Z [RUSTC-TIMING] rustc_plugin test:false 0.996
2019-10-04T18:05:09.0394789Z [RUSTC-TIMING] rustc_save_analysis test:false 12.003
2019-10-04T18:05:09.0590772Z error: Could not document `rustc_plugin`.
2019-10-04T18:05:09.0590929Z 
2019-10-04T18:05:09.0591067Z Caused by:
2019-10-04T18:05:09.0592281Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_plugin src/librustc_plugin/deprecated/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_plugin_impl=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-e58340f963559826.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-10-04T18:05:09.0774602Z [RUSTC-TIMING] rustc_codegen_ssa test:false 12.063
2019-10-04T18:05:09.7373662Z warning: `[no_mangle]` cannot be resolved, ignoring it...
2019-10-04T18:05:09.7374726Z   --> src/librustc_codegen_ssa/traits/declare.rs:33:44
2019-10-04T18:05:09.7375030Z    |
---
2019-10-04T18:05:09.7415772Z 
2019-10-04T18:05:23.8557856Z error: build failed
2019-10-04T18:05:23.8580844Z 
2019-10-04T18:05:23.8581051Z 
2019-10-04T18:05:23.8586996Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "serialize" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "graphviz" "-p" "rustc_typeck" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "rustc_plugin" "-p" "syntax_pos" "-p" "rustc_index" "-p" "rustc_driver" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_codegen_ssa" "-p" "rustc_llvm" "-p" "rustc" "-p" "rustc_errors" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_data_structures" "-p" "fmt_macros" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "syntax" "-p" "rustc_codegen_llvm" "-p" "rustc_target" "-p" "rustc_plugin_impl" "-p" "build_helper" "-p" "rustc_lint" "-p" "rustc_codegen_utils" "-p" "rustc_lexer" "-p" "arena" "-p" "rustc_metadata" "-p" "syntax_ext"
2019-10-04T18:05:23.8587940Z 
2019-10-04T18:05:23.8588005Z 
2019-10-04T18:05:23.8602616Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-10-04T18:05:23.8602823Z Build completed unsuccessfully in 1:54:56
2019-10-04T18:05:23.8602823Z Build completed unsuccessfully in 1:54:56
2019-10-04T18:05:23.8668655Z == clock drift check ==
2019-10-04T18:05:23.8712283Z   local time: Fri Oct  4 18:05:23 UTC 2019
2019-10-04T18:05:24.1238609Z   network time: Fri, 04 Oct 2019 18:05:24 GMT
2019-10-04T18:05:24.1239287Z == end clock drift check ==
2019-10-04T18:05:25.8482089Z ##[error]Bash exited with code '1'.
2019-10-04T18:05:25.8565109Z ##[section]Starting: Upload CPU usage statistics
2019-10-04T18:05:25.8579401Z ==============================================================================
2019-10-04T18:05:25.8579533Z Task         : Bash
2019-10-04T18:05:25.8579612Z Description  : Run a Bash script on macOS, Linux, or Windows
