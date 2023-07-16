plain
2019-10-03T12:01:32.4661621Z warning: the 'passes' flag is considered deprecated
2019-10-03T12:01:32.4662016Z   |
2019-10-03T12:01:32.4662385Z   = warning: please see https://github.com/rust-lang/rust/issues/44136
2019-10-03T12:01:32.4662486Z 
2019-10-03T12:01:33.3690675Z error[E0275]: overflow evaluating the requirement `alloc::raw_vec::RawVec<(syntax::ast::UseTree, syntax::ast::node_id_inner::NodeId)>: std::marker::Sync`
2019-10-03T12:01:33.3691462Z   |
2019-10-03T12:01:33.3691964Z   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
2019-10-03T12:01:33.3692470Z   = note: required because it appears within the type `std::vec::Vec<(syntax::ast::UseTree, syntax::ast::node_id_inner::NodeId)>`
2019-10-03T12:01:33.3693138Z   = note: required because it appears within the type `syntax::ast::UseTreeKind`
2019-10-03T12:01:33.3693518Z   = note: required because it appears within the type `syntax::ast::UseTree`
2019-10-03T12:01:33.3693988Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::UseTree>`
2019-10-03T12:01:33.3694653Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::UseTree>`
2019-10-03T12:01:33.3695045Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::UseTree>`
2019-10-03T12:01:33.3695527Z   = note: required because it appears within the type `syntax::ast::ItemKind`
2019-10-03T12:01:33.3696196Z   = note: required because it appears within the type `syntax::ast::Item`
2019-10-03T12:01:33.3696647Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2019-10-03T12:01:33.3697065Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
2019-10-03T12:01:33.3697467Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2019-10-03T12:01:33.3697874Z   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
2019-10-03T12:01:33.3698345Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<syntax::parse::token::Nonterminal>`
2019-10-03T12:01:33.3698762Z   = note: required because it appears within the type `syntax::parse::token::TokenKind`
2019-10-03T12:01:33.3699161Z   = note: required because it appears within the type `syntax::parse::token::Token`
2019-10-03T12:01:33.3699988Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2019-10-03T12:01:33.3700470Z   = note: required because it appears within the type `(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)`
2019-10-03T12:01:33.3700986Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2019-10-03T12:01:33.3701479Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2019-10-03T12:01:33.3701962Z   = note: required because it appears within the type `std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2019-10-03T12:01:33.3702625Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>`
2019-10-03T12:01:33.3703230Z   = note: required because it appears within the type `std::option::Option<std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>>`
2019-10-03T12:01:33.3703661Z   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
2019-10-03T12:01:33.3704070Z   = note: required because it appears within the type `syntax::ast::Mac`
2019-10-03T12:01:33.3712783Z   = note: required because it appears within the type `syntax::ast::PatKind`
2019-10-03T12:01:33.3721794Z   = note: required because it appears within the type `syntax::ast::Pat`
2019-10-03T12:01:33.3729774Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Pat>`
2019-10-03T12:01:33.3740713Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
2019-10-03T12:01:33.3741175Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Pat>`
2019-10-03T12:01:33.3741835Z   = note: required because it appears within the type `syntax::ast::ExprKind`
2019-10-03T12:01:33.3742233Z   = note: required because it appears within the type `syntax::ast::Expr`
2019-10-03T12:01:33.3742663Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Expr>`
2019-10-03T12:01:33.3743102Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
2019-10-03T12:01:33.3743519Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
2019-10-03T12:01:33.3743899Z   = note: required because it appears within the type `syntax::ast::AnonConst`
2019-10-03T12:01:33.3744301Z   = note: required because it appears within the type `syntax::ast::TyKind`
2019-10-03T12:01:33.3744682Z   = note: required because it appears within the type `syntax::ast::Ty`
2019-10-03T12:01:33.3745212Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
2019-10-03T12:01:33.3745623Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
2019-10-03T12:01:33.3745999Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
2019-10-03T12:01:33.3746391Z   = note: required because it appears within the type `syntax::ast::GenericArg`
2019-10-03T12:01:33.3746835Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArg>`
2019-10-03T12:01:33.3747245Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
2019-10-03T12:01:33.3747765Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
2019-10-03T12:01:33.3748278Z   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
2019-10-03T12:01:33.3748717Z   = note: required because it appears within the type `syntax::ast::GenericArgs`
2019-10-03T12:01:33.3749266Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
2019-10-03T12:01:33.3750653Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
2019-10-03T12:01:33.3751119Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
2019-10-03T12:01:33.3751579Z   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
2019-10-03T12:01:33.3751993Z   = note: required because it appears within the type `syntax::ast::PathSegment`
2019-10-03T12:01:33.3752455Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathSegment>`
2019-10-03T12:01:33.3753172Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
2019-10-03T12:01:33.3753587Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
2019-10-03T12:01:33.3753987Z   = note: required because it appears within the type `syntax::ast::Path`
2019-10-03T12:01:33.3754491Z   = note: required because it appears within the type `syntax::ast::AttrItem`
2019-10-03T12:01:33.3755089Z   = note: required because it appears within the type `syntax::ast::Attribute`
2019-10-03T12:01:33.3755522Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
2019-10-03T12:01:33.3755951Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
2019-10-03T12:01:33.3756336Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
2019-10-03T12:01:33.3977163Z error: aborting due to previous error
2019-10-03T12:01:33.3983789Z 
2019-10-03T12:01:33.3992283Z For more information about this error, try `rustc --explain E0275`.
2019-10-03T12:01:34.8941584Z warning: `[E0502]` cannot be resolved, ignoring it...
---
2019-10-03T12:01:56.4918691Z [RUSTC-TIMING] rustc_plugin test:false 29.946
2019-10-03T12:02:01.5924569Z error: Could not document `rustc_plugin`.
2019-10-03T12:02:01.5924954Z 
2019-10-03T12:02:01.5925066Z Caused by:
2019-10-03T12:02:01.5926265Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_plugin src/librustc_plugin/deprecated/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --color always -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_plugin_impl=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_plugin_impl-65720c0d80ea00a8.rmeta --document-private-items --passes strip-hidden` (exit code: 1)
2019-10-03T12:02:06.9648275Z [RUSTC-TIMING] rustc_codegen_ssa test:false 46.170
2019-10-03T12:02:06.9822278Z error: build failed
2019-10-03T12:02:06.9853954Z 
2019-10-03T12:02:06.9856170Z 
2019-10-03T12:02:06.9856170Z 
2019-10-03T12:02:06.9858460Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_errors" "-p" "rustc_privacy" "-p" "rustc_save_analysis" "-p" "rustc_plugin" "-p" "rustc_codegen_utils" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "build_helper" "-p" "syntax_ext" "-p" "rustc_lexer" "-p" "rustc_traits" "-p" "rustc_llvm" "-p" "rustc_codegen_ssa" "-p" "rustc_lint" "-p" "rustc_metadata" "-p" "rustc_mir" "-p" "rustc_target" "-p" "arena" "-p" "rustc" "-p" "fmt_macros" "-p" "rustc_driver" "-p" "syntax_pos" "-p" "rustc_data_structures" "-p" "rustc_macros" "-p" "syntax" "-p" "rustc_plugin_impl" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_codegen_llvm" "-p" "serialize" "-p" "graphviz"
2019-10-03T12:02:06.9859585Z 
2019-10-03T12:02:06.9859829Z 
2019-10-03T12:02:06.9868982Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-10-03T12:02:06.9869791Z Build completed unsuccessfully in 1:54:57
2019-10-03T12:02:06.9869791Z Build completed unsuccessfully in 1:54:57
2019-10-03T12:02:06.9931269Z == clock drift check ==
2019-10-03T12:02:06.9951448Z   local time: Thu Oct  3 12:02:06 UTC 2019
2019-10-03T12:02:07.3595199Z   network time: Thu, 03 Oct 2019 12:02:07 GMT
2019-10-03T12:02:07.3596120Z == end clock drift check ==
2019-10-03T12:02:10.8796187Z ##[error]Bash exited with code '1'.
2019-10-03T12:02:10.8844629Z ##[section]Starting: Upload CPU usage statistics
2019-10-03T12:02:10.8863114Z ==============================================================================
2019-10-03T12:02:10.8863233Z Task         : Bash
2019-10-03T12:02:10.8863334Z Description  : Run a Bash script on macOS, Linux, or Windows
