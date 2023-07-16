plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1450c601
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:29:11] 
[01:29:13]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[01:29:13]  Documenting rustc v0.0.0 (/checkout/src/librustc)
[01:29:13]  Documenting syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[01:29:15] error[E0275]: overflow evaluating the requirement `syntax::ast::GenericArgs: std::marker::Send`
[01:29:15]   |
[01:29:15]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:29:15]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::GenericArgs>`
[01:29:15]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::GenericArgs>`
[01:29:15]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::GenericArgs>`
[01:29:15]   = note: required because it appears within the type `std::option::Option<syntax::ptr::P<syntax::ast::GenericArgs>>`
[01:29:15]   = note: required because it appears within the type `syntax::ast::PathSegment`
[01:29:15]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::PathSegment>`
[01:29:15]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::PathSegment>`
[01:29:15]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::PathSegment>`
[01:29:15]   = note: required because it appears within the type `syntax::ast::Path`
[01:29:15]   = note: required because it appears within the type `syntax::ast::Attribute`
[01:29:15]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Attribute>`
[01:29:15]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax::ast::Attribute>`
[01:29:15]   = note: required because it appears within the type `std::vec::Vec<syntax::ast::Attribute>`
[01:29:15]   = note: required because it appears within the type `syntax::ast::Item`
[01:29:15]   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax::ast::Item>`
[01:29:15]   = note: required because it appears within the type `std::boxed::Box<syntax::ast::Item>`
[01:29:15]   = note: required because it appears within the type `syntax::ptr::P<syntax::ast::Item>`
[01:29:15]   = note: required because it appears within the type `syntax::parse::token::Nonterminal`
[01:29:15]   = note: required because it appears within the type `(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)`
[01:29:15]   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<(syntax::parse::token::Nonterminal, syntax::parse::token::LazyTokenStream)>`
[01:29:15]   = note: required because it appears within the type `syntax::parse::token::Token`
[01:29:15]   = note: required because it appears within the type `syntax::tokenstream::TokenTree`
[01:29:15]   = note: required because it appears within the type `syntax::tokenstream::TokenStreamKind`
[01:29:15]   = note: required because it appears within the type `syntax::tokenstream::TokenStream`
[01:29:15] error: Could not document `syntax_ext`.
[01:29:15] 
[01:29:15] Caused by:
[01:29:15] Caused by:
[01:29:15]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name syntax_ext src/libsyntax_ext/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-0ac2121f47dfd1cb.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-18f2d817315b6e70.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-7508cc4c80da0cd8.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-e84b2f2182f8dcdd.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-0c9d54efb5c62fc9.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-be3ea3cfc043a331.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-305073bf81d6cc22.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-a15c0fe42507987a.rmeta --document-private-items` (exit code: 1)
[01:30:41] warning: `[xxx]` cannot be resolved, ignoring it...
[01:30:41]   --> src/librustc/hir/intravisit.rs:55:11
[01:30:41]    |
[01:30:41] 55 |     /// #[xxx] pub async/const/extern "Abi" fn foo()
---
[01:30:41] 
[01:30:45] error: build failed
[01:30:45] 
[01:30:45] 
[01:30:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_privacy" "-p" "fmt_macros" "-p" "rustc_target" "-p" "syntax" "-p" "rustc_passes" "-p" "rustc_plugin" "-p" "rustc_fs_util" "-p" "rustc_metadata" "-p" "rustc_mir" "-p" "rustc_traits" "-p" "serialize" "-p" "rustc_data_structures" "-p" "rustc_allocator" "-p" "graphviz" "-p" "rustc_lint" "-p" "rustc_llvm" "-p" "arena" "-p" "syntax_ext" "-p" "rustc_borrowck" "-p" "rustc_codegen_utils" "-p" "rustc_apfloat" "-p" "rustc_driver" "-p" "rustc_platform_intrinsics" "-p" "rustc_errors" "-p" "syntax_pos" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "rustc" "-p" "build_helper" "-p" "rustc_save_analysis" "-p" "rustc_codegen_llvm" "-p" "rustc_incremental"
[01:30:45] 
[01:30:45] 
[01:30:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:30:45] Build completed unsuccessfully in 1:25:58
---
travis_time:end:2be11820:start=1543666415837312184,finish=1543666415851112590,duration=13800406
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1875819e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00767d24
travis_time:start:00767d24
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07c109ec
$ dmesg | grep -i kill
