plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:14c874ad
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:24:40] 
[01:24:41]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[01:24:41]  Documenting rustc v0.0.0 (/checkout/src/librustc)
[01:24:41]  Documenting syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[01:24:43] error[E0275]: overflow evaluating the requirement `std::vec::Vec<syntax::ptr::P<syntax::ast::Expr>>: std::marker::Sync`
[01:24:43]   |
[01:24:43]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:24:43]   = note: required because it appears within the type `syntax::ast::ExprKind`
[01:24:43]   = note: required because it appears within the type `syntax::ast::Expr`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Expr>`
[01:24:43]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Expr>`
[01:24:43]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Expr>`
[01:24:43]   = note: required because it appears within the type `syntax::ast::AnonConst`
[01:24:43]   = note: required because it appears within the type `syntax::ast::TyKind`
[01:24:43]   = note: required because it appears within the type `syntax::ast::Ty`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Ty>`
[01:24:43]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Ty>`
[01:24:43]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Ty>`
[01:24:43]   = note: required because it appears within the type `syntax::ast::GenericArg`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArg>`
[01:24:43]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::GenericArg>`
[01:24:43]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::GenericArg>`
[01:24:43]   = note: required because it appears within the type `syntax::ast::AngleBracketedArgs`
[01:24:43]   = note: required because it appears within the type `syntax::ast::GenericArgs`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::GenericArgs>`
[01:24:43]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
[01:24:43]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
[01:24:43]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
[01:24:43]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:24:43]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:24:43]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:24:43]   = note: required because it appears within the type `syntax::ast::Path`
[01:24:43]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Attribute>`
[01:24:43]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
[01:24:43]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
[01:24:43]   = note: required because it appears within the type `syntax::ast::Item`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::ast::Item>`
[01:24:43]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
[01:24:43]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
[01:24:43]   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
[01:24:43]   = note: required because it appears within the type `(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)>`
[01:24:43]   = note: required because it appears within the type `syntax::parse::token::Token`
[01:24:43]   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
[01:24:43]   = note: required because it appears within the type `syntax::tokenstream::TokenStreamKind`
[01:24:43]   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<syntax::tokenstream::TokenStream>`
[01:24:43]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::tokenstream::TokenStream>`
[01:24:43]   = note: required because it appears within the type `std::vec::Vec<syntax::tokenstream::TokenStream>`
[01:24:43]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<std::vec::Vec<syntax::tokenstream::TokenStream>>`
[01:24:43]   = note: required because it appears within the type `syntax::util::RcVec<syntax::tokenstream::TokenStream>`
[01:24:43] error: Could not document `syntax_ext`.
[01:24:43] 
[01:24:43] Caused by:
[01:24:43] Caused by:
[01:24:43]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name syntax_ext src/libsyntax_ext/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-8239fefad5bc759d.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-aeb8a81c811b0b35.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-b765a9dc216fc88d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-31246a3b2649fa63.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-a0a7c26140788693.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-d6753d7706de69ac.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5aa2face1b26e88c.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-cf4d5923473a267b.rmeta --document-private-items` (exit code: 1)
[01:25:34] warning: `[xxx]` cannot be resolved, ignoring it...
[01:25:34]   --> src/librustc/hir/intravisit.rs:55:11
[01:25:34]    |
[01:25:34] 55 |     /// #[xxx] pub async/const/extern "Abi" fn foo()
---
[01:25:34] 
[01:25:38] error: build failed
[01:25:38] 
[01:25:38] 
[01:25:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_llvm" "-p" "rustc_incremental" "-p" "syntax" "-p" "fmt_macros" "-p" "rustc_mir" "-p" "serialize" "-p" "syntax_pos" "-p" "rustc_allocator" "-p" "rustc_resolve" "-p" "rustc_driver" "-p" "rustc_traits" "-p" "rustc_errors" "-p" "rustc_fs_util" "-p" "rustc_borrowck" "-p" "graphviz" "-p" "rustc" "-p" "rustc_apfloat" "-p" "rustc_typeck" "-p" "syntax_ext" "-p" "rustc_metadata" "-p" "rustc_codegen_llvm" "-p" "build_helper" "-p" "rustc_target" "-p" "rustc_passes" "-p" "rustc_save_analysis" "-p" "rustc_plugin" "-p" "rustc_platform_intrinsics" "-p" "rustc_lint" "-p" "rustc_codegen_ssa" "-p" "rustc_codegen_utils" "-p" "rustc_data_structures" "-p" "rustc_privacy" "-p" "arena"
[01:25:38] 
[01:25:38] 
[01:25:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:25:38] Build completed unsuccessfully in 1:20:52
---
travis_time:end:132ea85e:start=1544703039804753742,finish=1544703039816286796,duration=11533054
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09ec63d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:024c2187
travis_time:start:024c2187
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e66e69e
$ dmesg | grep -i kill
