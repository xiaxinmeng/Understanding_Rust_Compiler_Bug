plain
2020-01-12T07:25:31.8864884Z      |         ^^^^^^^
2020-01-12T07:25:31.8865090Z 
2020-01-12T07:25:32.5761319Z  Documenting rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-01-12T07:25:32.6479175Z  Documenting rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-01-12T07:25:33.6773723Z error[E0275]: overflow evaluating the requirement `rustc_span::source_map::Spanned<syntax::ast::VisibilityKind>: std::marker::Sync`
2020-01-12T07:25:33.6777780Z   |
2020-01-12T07:25:33.6778607Z   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate
2020-01-12T07:25:33.6781149Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
2020-01-12T07:25:33.6781595Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
2020-01-12T07:25:33.6782690Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
2020-01-12T07:25:33.6783112Z   = note: required because it appears within the type `syntax::token::Nonterminal`
2020-01-12T07:25:33.6783112Z   = note: required because it appears within the type `syntax::token::Nonterminal`
2020-01-12T07:25:33.6785074Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<syntax::token::Nonterminal>`
2020-01-12T07:25:33.6785982Z   = note: required because it appears within the type `syntax::token::Token`
2020-01-12T07:25:33.6786422Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-12T07:25:33.6786422Z   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
2020-01-12T07:25:33.6786909Z   = note: required because it appears within the type `(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)`
2020-01-12T07:25:33.6787680Z   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-12T07:25:33.6788121Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-12T07:25:33.6788557Z   = note: required because it appears within the type `std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>`
2020-01-12T07:25:33.6789052Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<(syntax::tokenstream::TokenTree, syntax::tokenstream::IsJoint)>>`
2020-01-12T07:25:33.6789801Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-12T07:25:33.6789801Z   = note: required because it appears within the type `syntax::ast::MacArgs`
2020-01-12T07:25:33.6790409Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::MacArgs>`
2020-01-12T07:25:33.6790789Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::MacArgs>`
2020-01-12T07:25:33.6791404Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::MacArgs>`
2020-01-12T07:25:33.6792280Z   = note: required because it appears within the type `syntax::ast::PatKind`
2020-01-12T07:25:33.6792623Z   = note: required because it appears within the type `syntax::ast::Pat`
2020-01-12T07:25:33.6793019Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Pat>`
2020-01-12T07:25:33.6793405Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
---
2020-01-12T07:25:33.6797767Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
2020-01-12T07:25:33.6798231Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
2020-01-12T07:25:33.6798620Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
2020-01-12T07:25:33.6798983Z   = note: required because it appears within the type `syntax::ast::GenericArg`
2020-01-12T07:25:33.6799423Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArg>`
2020-01-12T07:25:33.6799835Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
2020-01-12T07:25:33.6800234Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
2020-01-12T07:25:33.6800619Z   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
2020-01-12T07:25:33.6801521Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
2020-01-12T07:25:33.6801982Z   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
2020-01-12T07:25:33.6802362Z   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
2020-01-12T07:25:33.6802782Z   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
---
2020-01-12T07:25:33.6804859Z   = note: required because it appears within the type `syntax::ast::Path`
2020-01-12T07:25:33.6805222Z   = note: required because it appears within the type `syntax::ast::AttrItem`
2020-01-12T07:25:33.6805573Z   = note: required because it appears within the type `syntax::ast::AttrKind`
2020-01-12T07:25:33.6805943Z   = note: required because it appears within the type `syntax::ast::Attribute`
2020-01-12T07:25:33.6806366Z   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
2020-01-12T07:25:33.6806788Z   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
2020-01-12T07:25:33.6807178Z   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
2020-01-12T07:25:33.7124515Z error: aborting due to previous error
2020-01-12T07:25:33.7125795Z 
2020-01-12T07:25:33.7131242Z For more information about this error, try `rustc --explain E0275`.
2020-01-12T07:25:37.2027495Z warning: `[E0502]` cannot be resolved, ignoring it.
---
2020-01-12T07:25:38.7053352Z  Documenting rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
2020-01-12T07:25:43.3443637Z error: Could not document `rustc_mir_build`.
2020-01-12T07:25:43.3443756Z 
2020-01-12T07:25:43.3443862Z Caused by:
2020-01-12T07:25:43.3449281Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name rustc_mir_build src/librustc_mir_build/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-fc0c7917bc55b61f.rmeta --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libitertools-825d5ef6d2b1c128.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-185debf85f2e4bf7.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f382be1c98bb80d7.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-99c80997ce5749b5.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3dc694bf49ea01f3.rmeta --extern rustc_error_codes=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_error_codes-54cd379d78008b87.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-6eff6d9f662a1321.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-ff1d703b8ce00574.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-f05721db955ab89f.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-58acebf55205ccb8.so --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-ee0c7a107bbe252c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-995c29fb63819225.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-4b731df810cdf021.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-9c5c3208e6e40f81.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-835ef2c539b5f74c.rmeta --document-private-items` (exit code: 1)
2020-01-12T07:25:43.3616562Z [RUSTC-TIMING] rustc_expand test:false 99.930
2020-01-12T07:25:43.3999492Z [RUSTC-TIMING] rustc_typeck test:false 62.678
2020-01-12T07:25:44.8952975Z error: build failed
2020-01-12T07:25:44.8981229Z 
2020-01-12T07:25:44.8981229Z 
2020-01-12T07:25:44.8983403Z 
2020-01-12T07:25:44.8989737Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "jemalloc llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_lexer" "-p" "rustc_traits" "-p" "rustc_macros" "-p" "graphviz" "-p" "rustc_fs_util" "-p" "rustc_expand" "-p" "rustc_mir_build" "-p" "rustc_metadata" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_incremental" "-p" "serialize" "-p" "rustc_driver" "-p" "rustc_errors" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc" "-p" "rustc_plugin_impl" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_llvm" "-p" "rustc_codegen_ssa" "-p" "rustc_target" "-p" "fmt_macros" "-p" "rustc_codegen_utils" "-p" "rustc_ast_passes" "-p" "build_helper" "-p" "syntax" "-p" "rustc_typeck" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_data_structures" "-p" "rustc_apfloat" "-p" "rustc_resolve" "-p" "rustc_mir" "-p" "rustc_passes" "-p" "arena" "-p" "rustc_privacy" "-p" "rustc_save_analysis" "-p" "rustc_ast_lowering"
2020-01-12T07:25:44.8996062Z 
2020-01-12T07:25:44.8997521Z 
2020-01-12T07:25:44.9017713Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2020-01-12T07:25:44.9018203Z Build completed unsuccessfully in 1:39:43
2020-01-12T07:25:44.9018203Z Build completed unsuccessfully in 1:39:43
2020-01-12T07:25:44.9078339Z == clock drift check ==
2020-01-12T07:25:44.9098249Z   local time: Sun Jan 12 07:25:44 UTC 2020
2020-01-12T07:25:45.6368288Z   network time: Sun, 12 Jan 2020 07:25:45 GMT
2020-01-12T07:25:45.6368525Z == end clock drift check ==
2020-01-12T07:25:47.2369349Z 
2020-01-12T07:25:47.2463436Z ##[error]Bash exited with code '1'.
2020-01-12T07:25:47.2510694Z ##[section]Starting: Checkout
2020-01-12T07:25:47.2512506Z ==============================================================================
2020-01-12T07:25:47.2512605Z Task         : Get sources
2020-01-12T07:25:47.2512693Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
