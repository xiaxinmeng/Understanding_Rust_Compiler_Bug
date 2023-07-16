plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:1a579f8c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:47]    Compiling libc v0.2.40
[00:06:47]    Compiling scopeguard v0.3.3
[00:06:47]    Compiling memoffset v0.2.1
[00:06:47]    Compiling lazy_static v1.0.0
[00:06:47]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:06:47]    Compiling smallvec v0.6.0
[00:06:47]    Compiling either v1.5.0
[00:06:48]    Compiling bitflags v1.0.1
[00:06:48]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:07:21]    Compiling parking_lot v0.5.5
[00:07:24]    Compiling rls-data v0.15.0
[00:07:27]    Compiling flate2 v1.0.1
[00:07:28]    Compiling backtrace v0.3.6
[00:07:30]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:07:40]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:07:41]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:07:45]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:09:17]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
---
[00:33:56]    Compiling memoffset v0.2.1
[00:33:56]    Compiling lazy_static v1.0.0
[00:33:56]    Compiling scopeguard v0.3.3
[00:33:56]    Compiling libc v0.2.40
[00:33:57]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:33:57]    Compiling stable_deref_trait v1.0.0
[00:33:57]    Compiling either v1.5.0
[00:33:57]    Compiling bitflags v1.0.1
[00:33:57]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:34:32]    Compiling crossbeam-deque v0.2.0
[00:34:33]    Compiling parking_lot v0.5.5
[00:34:39]    Compiling flate2 v1.0.1
[00:34:40]    Compiling backtrace v0.3.6
[00:34:46]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:34:50]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:34:51]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:34:54]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:36:21]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
---
[00:58:45]    Compiling crossbeam-deque v0.2.0
[00:58:46]    Compiling rustc_target v0.0.0 (file:///checkout/src/librustc_target)
[00:58:46]    Compiling rand v0.3.22
[00:58:46]    Compiling parking_lot_core v0.2.14
[00:58:46]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:58:46]    Compiling parking_lot v0.5.5
[00:58:47]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:58:51]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:58:52]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:58:53]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:58:54]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
---
[01:01:38] warning: [plumbing] cannot be resolved, ignoring it...
[01:01:38] 
[01:01:38] warning: [Garbage] cannot be resolved, ignoring it...
[01:01:38] 
[01:01:39] error[E0275]: overflow evaluating the requirement `syntax::ast::Local: std::marker::Send`
[01:01:39]   |
[01:01:39]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Local>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Local>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Local>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::StmtKind`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Stmt`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Stmt>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Stmt>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Stmt>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Block`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Block>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Block>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Block>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::ExprKind`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Expr`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Expr>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::TyKind`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Ty`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ptr::P<syntax::ast::Ty>>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ptr::P<syntax::ast::Ty>>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ptr::P<syntax::ast::Ty>>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::AngleBracketedParameterData`
[01:01:39]   = note: required because it appears within the type `syntax::ast::PathParameters`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathParameters>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::PathParameters>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::PathParameters>`
[01:01:39]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::PathParameters>>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Path`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Item`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Item>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
[01:01:39]   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
[01:01:39]   = note: required because it appears within the type `(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)>`
[01:01:39]   = note: required because it appears within the type `syntax::parse::token::Token`
[01:01:39]   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
[01:01:39]   = note: required because it appears within the type `syntax::tokenstream::TokenStreamKind`
[01:01:39]   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
[01:01:39]   = note: required because it appears within the type `rustc::hir::MacroDef`
[01:01:39]   = note: required because it appears within the type `[rustc::hir::MacroDef]`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<[rustc::hir::MacroDef]>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<[rustc::hir::MacroDef]>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<[rustc::hir::MacroDef]>`
[01:01:39]   = note: required because it appears within the type `rustc::hir::Crate`
[01:01:39]   = note: required because it appears within the type `rustc::hir::map::Forest`
[01:01:39]   = note: required because it appears within the type `&'tcx rustc::hir::map::Forest`
[01:01:39]   = note: required because it appears within the type `rustc::hir::map::Map<'tcx>`
[01:01:39]   = note: required because it appears within the type `rustc::ty::context::GlobalCtxt<'tcx>`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `&'l rustc::ty::context::GlobalCtxt<'tcx>`
[01:01:39]   = note: required because it appears within the type `rustc::ty::TyCtxt<'l, 'tcx, 'tcx>`
[01:01:39]   = note: required because it appears within the type `SaveContext<'l, 'tcx>`
[01:01:39] error: Could not document `rustc_save_analysis`.
[01:01:39] 
[01:01:39] Caused by:
[01:01:39] Caused by:
[01:01:39]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_save_analysis librustc_save_analysis/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern rustc_typeck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-03806b9ef78741f1.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_serialize-a2e08b7a31666e46.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-f69a5661037830d0.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-ffb073d2b07e917d.rmeta --extern rls_data=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librls_data-a5ae99825960aea2.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-ac09ff383da42fc8.rmeta --extern rls_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librls_span-8d480067108d4365.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5d915ffc05448c39.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-80438f736d0204c7.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-d3237d8bd265046d.rmeta --document-private-items` (exit code: 101)
[01:01:39] warning: build failed, waiting for other jobs to finish...
[01:01:39] error[E0275]: overflow evaluating the requirement `syntax::ast::Local: std::marker::Send`
[01:01:39]   |
[01:01:39]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Local>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Local>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Local>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::StmtKind`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Stmt`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Stmt>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Stmt>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Stmt>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Block`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Block>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Block>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Block>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::ExprKind`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Expr`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Expr>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::TyKind`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Ty`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ptr::P<syntax::ast::Ty>>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ptr::P<syntax::ast::Ty>>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ptr::P<syntax::ast::Ty>>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::AngleBracketedParameterData`
[01:01:39]   = note: required because it appears within the type `syntax::ast::PathParameters`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathParameters>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::PathParameters>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::PathParameters>`
[01:01:39]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::PathParameters>>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Path`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
[01:01:39]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
[01:01:39]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
[01:01:39]   = note: required because it appears within the type `syntax::ast::Item`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Item>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
[01:01:39]   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
[01:01:39]   = note: required because it appears within the type `(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)>`
[01:01:39]   = note: required because it appears within the type `syntax::parse::token::Token`
[01:01:39]   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
[01:01:39]   = note: required because it appears within the type `syntax::tokenstream::TokenStreamKind`
[01:01:39]   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
[01:01:39]   = note: required because it appears within the type `rustc::hir::MacroDef`
[01:01:39]   = note: required because it appears within the type `[rustc::hir::MacroDef]`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<[rustc::hir::MacroDef]>`
[01:01:39]   = note: required because it appears within the type `std::boxed::Box<[rustc::hir::MacroDef]>`
[01:01:39]   = note: required because it appears within the type `syntax::ptr::P<[rustc::hir::MacroDef]>`
[01:01:39]   = note: required because it appears within the type `rustc::hir::Crate`
[01:01:39]   = note: required because it appears within the type `rustc::hir::map::Forest`
[01:01:39]   = note: required because it appears within the type `&'tcx rustc::hir::map::Forest`
[01:01:39]   = note: required because it appears within the type `rustc::hir::map::Map<'tcx>`
[01:01:39]   = note: required because it appears within the type `rustc::ty::context::GlobalCtxt<'tcx>`
[01:01:39]   = note: required because of the requirements on the impl of `std::marker::Send` for `&'a rustc::ty::context::GlobalCtxt<'tcx>`
[01:01:39]   = note: required because it appears within the type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[01:01:39]   = note: required because it appears within the type `PubRestrictedVisitor<'a, 'tcx>`
[01:01:39] error: Could not document `rustc_privacy`.
[01:01:39] 
[01:01:39] Caused by:
[01:01:39] Caused by:
[01:01:39]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_privacy librustc_privacy/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5d915ffc05448c39.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-ffb073d2b07e917d.rmeta --extern rustc_typeck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-03806b9ef78741f1.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-80438f736d0204c7.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-d3237d8bd265046d.rmeta --document-private-items` (exit code: 101)
[01:01:43] warning: [] cannot be resolved, ignoring it...
[01:01:43] 
[01:01:43] warning: [First] cannot be resolved, ignoring it...
[01:01:43] 
---
[01:01:55] 
[01:01:57] error: build failed
[01:01:57] 
[01:01:57] 
[01:01:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_target" "-p" "syntax" "-p" "rustc_errors" "-p" "serialize" "-p" "rustc_driver" "-p" "arena" "-p" "rustc_save_analysis" "-p" "rustc_allocator" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_apfloat" "-p" "rustc_platform_intrinsics" "-p" "proc_macro" "-p" "fmt_macros" "-p" "rustc_trans_utils" "-p" "syntax_pos" "-p" "rustc_borrowck" "-p" "rustc_lint" "-p" "graphviz" "-p" "syntax_ext" "-p" "rustc_data_structures" "-p" "rustc" "-p" "rustc_incremental" "-p" "rustc_metadata" "-p" "rustc_plugin" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_mir"
[01:01:57] 
[01:01:57] 
[01:01:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:01:57] Build completed unsuccessfully in 0:56:22
