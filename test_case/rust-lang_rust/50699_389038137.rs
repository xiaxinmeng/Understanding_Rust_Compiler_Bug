plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:01f4f8bb
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:07:06]    Compiling scopeguard v0.3.3
[00:07:06]    Compiling libc v0.2.40
[00:07:06]    Compiling memoffset v0.2.1
[00:07:06]    Compiling lazy_static v1.0.0
[00:07:06]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:07:06]    Compiling smallvec v0.6.0
[00:07:06]    Compiling bitflags v1.0.1
[00:07:06]    Compiling either v1.5.0
[00:07:07]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:07:44]    Compiling crossbeam-deque v0.2.0
[00:07:44]    Compiling parking_lot v0.5.5
[00:07:45]    Compiling rls-data v0.15.0
[00:07:52]    Compiling flate2 v1.0.1
[00:07:53]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:08:02]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:08:05]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:08:06]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:08:10]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[00:37:14]    Compiling libc v0.2.40
[00:37:14]    Compiling memoffset v0.2.1
[00:37:14]    Compiling lazy_static v1.0.0
[00:37:14]    Compiling scopeguard v0.3.3
[00:37:14]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:37:14]    Compiling stable_deref_trait v1.0.0
[00:37:14]    Compiling bitflags v1.0.1
[00:37:15]    Compiling either v1.5.0
[00:37:15]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[00:37:54]    Compiling parking_lot v0.5.5
[00:37:54]    Compiling crossbeam-deque v0.2.0
[00:37:55]    Compiling rls-data v0.15.0
[00:38:02]    Compiling flate2 v1.0.1
[00:38:08]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[00:38:11]    Compiling rustc_data_structures v0.0.0 (file:///checkout/src/librustc_data_structures)
[00:38:13]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[00:38:13]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:38:18]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
---
[01:05:19]    Compiling crossbeam-epoch v0.3.1
[01:05:20]    Compiling crossbeam-deque v0.2.0
[01:05:21]    Compiling rand v0.3.22
[01:05:21]    Compiling parking_lot_core v0.2.14
[01:05:21]    Compiling rustc-rayon-core v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[01:05:21]    Compiling parking_lot v0.5.5
[01:05:21]    Compiling parking_lot v0.5.5
[01:05:22]    Compiling rustc-rayon v0.1.0 (https://github.com/Zoxc/rayon.git?branch=rustc#1e976c1f)
[01:05:27]    Compiling arena v0.0.0 (file:///checkout/src/libarena)
[01:05:27]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[01:05:28]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[01:05:30]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
---
[01:08:00] warning: [plumbing] cannot be resolved, ignoring it...
[01:08:00] 
[01:08:00] warning: [Garbage] cannot be resolved, ignoring it...
[01:08:00] 
[01:08:02] error[E0275]: overflow evaluating the requirement `syntax::ast::Local: std::marker::Send`
[01:08:02]   |
[01:08:02]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Local>`
[01:08:02]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Local>`
[01:08:02]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Local>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::StmtKind`
[01:08:02]   = note: required because it appears within the type `syntax::ast::Stmt`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Stmt>`
[01:08:02]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Stmt>`
[01:08:02]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Stmt>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::Block`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Block>`
[01:08:02]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Block>`
[01:08:02]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Block>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::ExprKind`
[01:08:02]   = note: required because it appears within the type `syntax::ast::Expr`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Expr>`
[01:08:02]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
[01:08:02]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::TyKind`
[01:08:02]   = note: required because it appears within the type `syntax::ast::Ty`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Ty>`
[01:08:02]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
[01:08:02]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ptr::P<syntax::ast::Ty>>`
[01:08:02]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ptr::P<syntax::ast::Ty>>`
[01:08:02]   = note: required because it appears within the type `std::vec::Vec<syntax::ptr::P<syntax::ast::Ty>>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::AngleBracketedParameterData`
[01:08:02]   = note: required because it appears within the type `syntax::ast::PathParameters`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathParameters>`
[01:08:02]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::PathParameters>`
[01:08:02]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::PathParameters>`
[01:08:02]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::PathParameters>>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:08:02]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:08:02]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::Path`
[01:08:02]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
[01:08:02]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
[01:08:02]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
[01:08:02]   = note: required because it appears within the type `syntax::ast::Item`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Item>`
[01:08:02]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
[01:08:02]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
[01:08:02]   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
[01:08:02]   = note: required because it appears within the type `(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)>`
[01:08:02]   = note: required because it appears within the type `syntax::parse::token::Token`
[01:08:02]   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
[01:08:02]   = note: required because it appears within the type `syntax::tokenstream::TokenStreamKind`
[01:08:02]   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
[01:08:02]   = note: required because it appears within the type `rustc::hir::MacroDef`
[01:08:02]   = note: required because it appears within the type `[rustc::hir::MacroDef]`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<[rustc::hir::MacroDef]>`
[01:08:02]   = note: required because it appears within the type `std::boxed::Box<[rustc::hir::MacroDef]>`
[01:08:02]   = note: required because it appears within the type `syntax::ptr::P<[rustc::hir::MacroDef]>`
[01:08:02]   = note: required because it appears within the type `rustc::hir::Crate`
[01:08:02]   = note: required because it appears within the type `rustc::hir::map::Forest`
[01:08:02]   = note: required because it appears within the type `&'tcx rustc::hir::map::Forest`
[01:08:02]   = note: required because it appears within the type `rustc::hir::map::Map<'tcx>`
[01:08:02]   = note: required because it appears within the type `rustc::ty::context::GlobalCtxt<'tcx>`
[01:08:02]   = note: required because of the requirements on the impl of `std::marker::Send` for `&'a rustc::ty::context::GlobalCtxt<'tcx>`
[01:08:02]   = note: required because it appears within the type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[01:08:02]   = note: required because it appears within the type `lowering::ClauseDumper<'a, 'tcx>`
[01:08:02] error: Could not document `rustc_traits`.
[01:08:02] 
[01:08:02] Caused by:
[01:08:02] Caused by:
[01:08:02]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_traits librustc_traits/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-b551c083e49559b3.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-4373576cdda5aaa8.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3c4fd57c7e1eed1c.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-41300bfca61e2b5f.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-add3d5640fff9efd.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-37572f331f4e0a66.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-562b296673ccdabd.rmeta --document-private-items` (exit code: 101)
[01:08:08] warning: [Debug] cannot be resolved, ignoring it...
[01:08:08] 
[01:08:12] warning: [Predicate] cannot be resolved, ignoring it...
[01:08:12] 
[01:08:12] 
[01:08:12] warning: [repr] cannot be resolved, ignoring it...
[01:08:12] 
[01:08:24] error: build failed
[01:08:24] 
[01:08:24] 
[01:08:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "syntax" "-p" "graphviz" "-p" "rustc_lint" "-p" "rustc_target" "-p" "fmt_macros" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_privacy" "-p" "rustc_trans_utils" "-p" "rustc_traits" "-p" "rustc_mir" "-p" "rustc_allocator" "-p" "syntax_ext" "-p" "rustc_platform_intrinsics" "-p" "rustc_incremental" "-p" "rustc_borrowck" "-p" "rustc" "-p" "rustc_save_analysis" "-p" "rustc_plugin" "-p" "rustc_driver" "-p" "serialize" "-p" "rustc_errors" "-p" "rustc_passes" "-p" "rustc_metadata" "-p" "rustc_resolve" "-p" "arena" "-p" "syntax_pos" "-p" "rustc_typeck" "-p" "proc_macro"
[01:08:24] 
[01:08:24] 
[01:08:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:08:24] Build completed unsuccessfully in 1:02:37
[01:08:24] Build completed unsuccessfully in 1:02:37
221928 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
210188 ./src/llvm/test
207100 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc-2s7kkg2ctqvfe
207096 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/rustc-2s7kkg2ctqvfe/s-f11r4p192f-18nmjzm-u7ip35n8lkzt
190172 ./obj/build/x86_64-unknown-linux-gnu/compiler-doc
189104 ./obj/build/bootstrap/debug/deps
185432 ./obj/build/cache
185428 ./obj/build/cache/2018-04-24
---
141404 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
137132 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
137128 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release
124220 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b
124216 ./obj/build/bootstrap/debug/incremental/bootstrap-182x3aewwy26b/s-f11pfwazbd-1izkbiu-34xtuyxu3kvz1
116016 ./obj/build/x86_64-unknown-linux-gnu/native
115916 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
103112 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
103108 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
103108 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
102820 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
100148 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
97904 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
90572 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-3e7jdtz0mpyk6
90568 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-3e7jdtz0mpyk6/s-f11r2wbkx7-2x7dn8-2w5hr60m9n1wn
87816 ./obj/build/x86_64-unknown-linux-gnu/doc/core
83216 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
83212 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
83208 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
