plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:2c3d591c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:08:07]    Compiling libc v0.2.40
[00:08:07]    Compiling memoffset v0.2.1
[00:08:07]    Compiling scopeguard v0.3.3
[00:08:07]    Compiling lazy_static v1.0.0
[00:08:07]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:08:08]    Compiling stable_deref_trait v1.0.0
[00:08:08]    Compiling either v1.5.0
[00:08:08]    Compiling bitflags v1.0.1
[00:08:08]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:09:02]    Compiling crossbeam-deque v0.2.0
[00:09:02]    Compiling parking_lot v0.5.5
[00:09:02]    Compiling flate2 v1.0.1
[00:09:02]    Compiling backtrace v0.3.6
[00:09:12]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:09:20]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:09:20]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:09:25]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:11:31]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
---
[00:43:02]    Compiling libc v0.2.40
[00:43:02]    Compiling lazy_static v1.0.0
[00:43:02]    Compiling memoffset v0.2.1
[00:43:02]    Compiling scopeguard v0.3.3
[00:43:02]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:43:02]    Compiling smallvec v0.6.0
[00:43:02]    Compiling bitflags v1.0.1
[00:43:03]    Compiling either v1.5.0
[00:43:03]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:43:53]    Compiling crossbeam-deque v0.2.0
[00:43:53]    Compiling parking_lot v0.5.5
[00:43:54]    Compiling flate2 v1.0.1
[00:43:58]    Compiling backtrace v0.3.6
[00:44:06]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:44:17]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:44:17]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:44:22]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:46:23]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
---
[01:17:27]    Compiling crossbeam-epoch v0.3.1
[01:17:28]    Compiling crossbeam-deque v0.2.0
[01:17:29]    Compiling rand v0.3.22
[01:17:29]    Compiling parking_lot_core v0.2.14
[01:17:29]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[01:17:30]    Compiling parking_lot v0.5.5
[01:17:30]    Compiling parking_lot v0.5.5
[01:17:30]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[01:17:37]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[01:17:37]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[01:17:39]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[01:17:41]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
---
[01:21:34] warning: [plumbing] cannot be resolved, ignoring it...
[01:21:34] 
[01:21:35] warning: [Garbage] cannot be resolved, ignoring it...
[01:21:35] 
[01:21:36] error[E0275]: overflow evaluating the requirement `syntax::ast::Local: std::marker::Send`
[01:21:36]   |
[01:21:36]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Local>`
[01:21:36]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Local>`
[01:21:36]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Local>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::StmtKind`
[01:21:36]   = note: required because it appears within the type `syntax::ast::Stmt`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Stmt>`
[01:21:36]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Stmt>`
[01:21:36]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Stmt>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::Block`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Block>`
[01:21:36]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Block>`
[01:21:36]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Block>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::ExprKind`
[01:21:36]   = note: required because it appears within the type `syntax::ast::Expr`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Expr>`
[01:21:36]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
[01:21:36]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::TyKind`
[01:21:36]   = note: required because it appears within the type `syntax::ast::Ty`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
[01:21:36]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
[01:21:36]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ptr::P<syntax::ast::Ty>>`
[01:21:36]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ptr::P<syntax::ast::Ty>>`
[01:21:36]   = note: required because it appears within the type `std::vec::Vec<syntax::ptr::P<syntax::ast::Ty>>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::AngleBracketedParameterData`
[01:21:36]   = note: required because it appears within the type `syntax::ast::PathParameters`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathParameters>`
[01:21:36]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::PathParameters>`
[01:21:36]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::PathParameters>`
[01:21:36]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::PathParameters>>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:21:36]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:21:36]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::Path`
[01:21:36]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
[01:21:36]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
[01:21:36]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
[01:21:36]   = note: required because it appears within the type `syntax::ast::Item`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Item>`
[01:21:36]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
[01:21:36]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
[01:21:36]   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
[01:21:36]   = note: required because it appears within the type `(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)>`
[01:21:36]   = note: required because it appears within the type `syntax::parse::token::Token`
[01:21:36]   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
[01:21:36]   = note: required because it appears within the type `syntax::tokenstream::TokenStreamKind`
[01:21:36]   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
[01:21:36]   = note: required because it appears within the type `rustc::hir::MacroDef`
[01:21:36]   = note: required because it appears within the type `[rustc::hir::MacroDef]`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<[rustc::hir::MacroDef]>`
[01:21:36]   = note: required because it appears within the type `std::boxed::Box<[rustc::hir::MacroDef]>`
[01:21:36]   = note: required because it appears within the type `syntax::ptr::P<[rustc::hir::MacroDef]>`
[01:21:36]   = note: required because it appears within the type `rustc::hir::Crate`
[01:21:36]   = note: required because it appears within the type `rustc::hir::map::Forest`
[01:21:36]   = note: required because it appears within the type `&'tcx rustc::hir::map::Forest`
[01:21:36]   = note: required because it appears within the type `rustc::hir::map::Map<'tcx>`
[01:21:36]   = note: required because it appears within the type `rustc::ty::context::GlobalCtxt<'tcx>`
[01:21:36]   = note: required because of the requirements on the impl of `std::marker::Send` for `&'a rustc::ty::context::GlobalCtxt<'tcx>`
[01:21:36]   = note: required because it appears within the type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[01:21:36]   = note: required because it appears within the type `PubRestrictedVisitor<'a, 'tcx>`
[01:21:36] error: Could not document `rustc_privacy`.
[01:21:36] 
[01:21:36] Caused by:
[01:21:36] Caused by:
[01:21:36]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_privacy librustc_privacy/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-9021c14e05e70f12.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-eda773497e9aedef.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-31c8a6589d9c645f.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-5b650cc88ddeec46.rmeta --extern rustc_typeck=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_typeck-1a1314e8b8985b26.rmeta --document-private-items` (exit code: 101)
[01:21:40] warning: [] cannot be resolved, ignoring it...
[01:21:40] 
[01:21:40] warning: [First] cannot be resolved, ignoring it...
[01:21:40] 
---
[01:22:00] 
[01:22:01] error: build failed
[01:22:01] 
[01:22:01] 
[01:22:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "syntax_ext" "-p" "rustc_apfloat" "-p" "rustc_lint" "-p" "rustc_data_structures" "-p" "rustc_allocator" "-p" "rustc_borrowck" "-p" "syntax_pos" "-p" "proc_macro" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_driver" "-p" "rustc_mir" "-p" "fmt_macros" "-p" "syntax" "-p" "rustc_typeck" "-p" "rustc_metadata" "-p" "rustc_save_analysis" "-p" "serialize" "-p" "rustc_plugin" "-p" "rustc_trans_utils" "-p" "graphviz" "-p" "arena" "-p" "rustc_traits" "-p" "rustc_platform_intrinsics"
[01:22:01] 
[01:22:01] 
[01:22:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:22:01] Build completed unsuccessfully in 1:15:30
