plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:07f80f94
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:45:06]  Documenting rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[01:45:10]  Documenting rustc_incremental v0.0.0 (/checkout/src/librustc_incremental)
[01:45:20]  Documenting rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[01:45:20]  Documenting rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[01:45:23] error[E0275]: overflow evaluating the requirement `syntax::ast::FieldPat: std::marker::Sync`
[01:45:23]   |
[01:45:23]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:45:23]   = note: required because it appears within the type `syntax::source_map::Spanned<syntax::ast::FieldPat>`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::source_map::Spanned<syntax::ast::FieldPat>>`
[01:45:23]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::source_map::Spanned<syntax::ast::FieldPat>>`
[01:45:23]   = note: required because it appears within the type `std::vec::Vec<syntax::source_map::Spanned<syntax::ast::FieldPat>>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::PatKind`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Pat`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Pat>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Pat>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Local`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Local>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Local>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Local>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::StmtKind`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Stmt`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Stmt>`
[01:45:23]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Stmt>`
[01:45:23]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Stmt>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Block`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Block>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Block>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Block>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::ExprKind`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Expr`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Expr>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::AnonConst`
[01:45:23]   = note: required because it appears within the type `syntax::ast::TyKind`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Ty`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Ty>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::GenericArg`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArg>`
[01:45:23]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
[01:45:23]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
[01:45:23]   = note: required because it appears within the type `syntax::ast::GenericArgs`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArgs>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
[01:45:23]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:45:23]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:45:23]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Path`
[01:45:23]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:45:23]   = note: required because it appears within the type `[syntax::ast::Attribute]`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<[syntax::ast::Attribute]>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<[syntax::ast::Attribute]>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<[syntax::ast::Attribute]>`
[01:45:23]   = note: required because it appears within the type `rustc::hir::GenericParam`
[01:45:23]   = note: required because it appears within the type `[rustc::hir::GenericParam]`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<[rustc::hir::GenericParam]>`
[01:45:23]   = note: required because it appears within the type `std::boxed::Box<[rustc::hir::GenericParam]>`
[01:45:23]   = note: required because it appears within the type `syntax::ptr::P<[rustc::hir::GenericParam]>`
[01:45:23]   = note: required because it appears within the type `rustc::hir::Generics`
[01:45:23]   = note: required because it appears within the type `&'tcx rustc::hir::Generics`
[01:45:23]   = note: required because it appears within the type `std::option::Option<&'tcx rustc::hir::Generics>`
[01:45:23]   = note: required because it appears within the type `rustc::lint::LateContext<'a, 'tcx>`
[01:45:23]   = note: required because of the requirements on the impl of `std::marker::Send` for `&'a rustc::lint::LateContext<'a, 'tcx>`
[01:45:23]   = note: required because it appears within the type `types::ImproperCTypesVisitor<'a, 'tcx>`
[01:45:23] error: Could not document `rustc_lint`.
[01:45:23] 
[01:45:23] Caused by:
[01:45:23] Caused by:
[01:45:23]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_lint src/librustc_lint/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-9c5c77c34a03c5e4.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-55bb2e998705ec79.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/st[01:45:25] warning: `[xxx]` cannot be resolved, ignoring it...
[01:45:25]    |
[01:45:25] 55 |     /// #[xxx] pub async/const/extern "Abi" fn foo()
[01:45:25]    |           ^^^ cannot be resolved, ignoring
[01:45:25]    |
---
[01:45:48] 
[01:45:49] error: build failed
[01:45:49] 
[01:45:49] 
[01:45:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_borrowck" "-p" "arena" "-p" "syntax_ext" "-p" "rustc_platform_intrinsics" "-p" "rustc_codegen_ssa" "-p" "rustc" "-p" "serialize" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_apfloat" "-p" "syntax_pos" "-p" "graphviz" "-p" "rustc_plugin" "-p" "rustc_target" "-p" "fmt_macros" "-p" "rustc_save_analysis" "-p" "rustc_traits" "-p" "rustc_driver" "-p" "rustc_errors" "-p" "rustc_data_structures" "-p" "rustc_metadata" "-p" "rustc_incremental" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_lint" "-p" "rustc_typeck" "-p" "rustc_allocator" "-p" "rustc_privacy" "-p" "syntax" "-p" "rustc_fs_util" "-p" "rustc_mir" "-p" "rustc_codegen_utils" "-p" "build_helper"
[01:45:49] 
[01:45:49] 
[01:45:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:45:49] Build completed unsuccessfully in 1:17:49
