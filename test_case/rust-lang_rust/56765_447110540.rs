plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:01ffb2fa
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:28:11] 
[01:28:11]  Documenting rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[01:28:13]  Documenting rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[01:28:14]     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[01:28:14] error[E0275]: overflow evaluating the requirement `std::boxed::Box<std::vec::Vec<syntax::ast::Attribute>>: std::marker::Sync`
[01:28:14]   |
[01:28:14]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:28:14]   = note: required because it appears within the type `std::option::Option<std::boxed::Box<std::vec::Vec<syntax::ast::Attribute>>>`
[01:28:14]   = note: required because it appears within the type `syntax::ThinVec<syntax::ast::Attribute>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::FieldPat`
[01:28:14]   = note: required because it appears within the type `syntax::source_map::Spanned<syntax::ast::FieldPat>`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::source_map::Spanned<syntax::ast::FieldPat>>`
[01:28:14]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::source_map::Spanned<syntax::ast::FieldPat>>`
[01:28:14]   = note: required because it appears within the type `std::vec::Vec<syntax::source_map::Spanned<syntax::ast::FieldPat>>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::PatKind`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Pat`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Pat>`
[01:28:14]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Pat>`
[01:28:14]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Pat>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Local`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Local>`
[01:28:14]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Local>`
[01:28:14]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Local>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::StmtKind`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Stmt`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Stmt>`
[01:28:14]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Stmt>`
[01:28:14]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Stmt>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Block`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Block>`
[01:28:14]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Block>`
[01:28:14]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Block>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::ExprKind`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Expr`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Expr>`
[01:28:14]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
[01:28:14]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::AnonConst`
[01:28:14]   = note: required because it appears within the type `syntax::ast::TyKind`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Ty`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Ty>`
[01:28:14]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
[01:28:14]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::GenericArg`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArg>`
[01:28:14]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
[01:28:14]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
[01:28:14]   = note: required because it appears within the type `syntax::ast::GenericArgs`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArgs>`
[01:28:14]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
[01:28:14]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
[01:28:14]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:28:14]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:28:14]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Path`
[01:28:14]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:28:14]   = note: required because it appears within the type `[syntax::ast::Attribute]`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<[syntax::ast::Attribute]>`
[01:28:14]   = note: required because it appears within the type `std::boxed::Box<[syntax::ast::Attribute]>`
[01:28:14]   = note: required because it appears within the type `syntax::ptr::P<[syntax::ast::Attribute]>`
[01:28:14]   = note: required because it appears within the type `rustc::hir::Crate`
[01:28:14]   = note: required because it appears within the type `rustc::hir::map::Forest`
[01:28:14]   = note: required because it appears within the type `&'tcx rustc::hir::map::Forest`
[01:28:14]   = note: required because it appears within the type `rustc::hir::map::Map<'tcx>`
[01:28:14]   = note: required because it appears within the type `rustc::ty::context::GlobalCtxt<'tcx>`
[01:28:14]   = note: required because of the requirements on the impl of `std::marker::Send` for `&'a rustc::ty::context::GlobalCtxt<'tcx>`
[01:28:14]   = note: required because it appears within the type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[01:28:14]   = note: required because it appears within the type `rvalue_promotion::CheckCrateVisitor<'a, 'tcx>`
[01:28:14] error: Could not document `rustc_passes`.
[01:28:14] 
[01:28:14] Caused by:
[01:28:14] Caused by:
[01:28:14]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_passes src/librustc_passes/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-ed63fd56fdbcbbe5.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-76d63ba10f00ac07.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-1695096f7fd7e5f4.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-fb8e2149b42fa290.rmeta --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-70c54d99947a5d4d.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-ec65f188dcafee87.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-2fe1af80a43a0355.rmeta --document-private-items` (exit code: 1)
[01:28:19] warning: `[]` cannot be resolved, ignoring it...
[01:28:19]     --> src/librustc_mir/hair/pattern/_match.rs:1289:49
[01:28:19]      |
[01:28:19] 1289 | /// For instance, a tuple pattern (_, 42, Some([])) has the arity of 3.
---
[01:28:20] 
[01:28:20] error: build failed
[01:28:20] 
[01:28:20] 
[01:28:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc" "-p" "rustc_target" "-p" "syntax_ext" "-p" "rustc_platform_intrinsics" "-p" "rustc_data_structures" "-p" "rustc_driver" "-p" "syntax" "-p" "rustc_metadata" "-p" "rustc_allocator" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_errors" "-p" "rustc_codegen_llvm" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_codegen_ssa" "-p" "arena" "-p" "rustc_fs_util" "-p" "rustc_incremental" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_save_analysis" "-p" "fmt_macros" "-p" "rustc_lint" "-p" "rustc_plugin" "-p" "rustc_borrowck" "-p" "syntax_pos" "-p" "serialize" "-p" "build_helper" "-p" "graphviz" "-p" "rustc_codegen_utils" "-p" "rustc_mir"
[01:28:20] 
[01:28:20] 
[01:28:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:28:20] Build completed unsuccessfully in 1:21:24
---
travis_time:end:0267d5d2:start=1544732969702375083,finish=1544732969715113064,duration=12737981
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0068ffe1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08088520
travis_time:start:08088520
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a57f994
$ dmesg | grep -i kill
