plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0240d400
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:21:50]     Checking tempfile v3.0.5
[01:21:50]  Documenting rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[01:21:50]     Checking parking_lot v0.7.1
[01:21:51]     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[01:21:51] error[E0275]: overflow evaluating the requirement `(syn::generics::TypeParamBound, syn::token::Add): std::marker::Unpin`
[01:21:51]   |
[01:21:51]   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
[01:21:51]   = note: required because it appears within the type `*const (syn::generics::TypeParamBound, syn::token::Add)`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<(syn::generics::TypeParamBound, syn::token::Add)>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syn::generics::TypeParamBound, syn::token::Add)>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<(syn::generics::TypeParamBound, syn::token::Add)>`
[01:21:51]   = note: required because it appears within the type `syn::punctuated::Punctuated<syn::generics::TypeParamBound, syn::token::Add>`
[01:21:51]   = note: required because it appears within the type `syn::path::Constraint`
[01:21:51]   = note: required because it appears within the type `syn::path::GenericArgument`
[01:21:51]   = note: required because it appears within the type `(syn::path::GenericArgument, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `*const (syn::path::GenericArgument, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<(syn::path::GenericArgument, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syn::path::GenericArgument, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<(syn::path::GenericArgument, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `syn::punctuated::Punctuated<syn::path::GenericArgument, syn::token::Comma>`
[01:21:51]   = note: required because it appears within the type `syn::path::AngleBracketedGenericArguments`
[01:21:51]   = note: required because it appears within the type `syn::path::PathArguments`
[01:21:51]   = note: required because it appears within the type `syn::path::PathSegment`
[01:21:51]   = note: required because it appears within the type `(syn::path::PathSegment, syn::token::Colon2)`
[01:21:51]   = note: required because it appears within the type `*const (syn::path::PathSegment, syn::token::Colon2)`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<(syn::path::PathSegment, syn::token::Colon2)>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syn::path::PathSegment, syn::token::Colon2)>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<(syn::path::PathSegment, syn::token::Colon2)>`
[01:21:51]   = note: required because it appears within the type `syn::punctuated::Punctuated<syn::path::PathSegment, syn::token::Colon2>`
[01:21:51]   = note: required because it appears within the type `syn::path::Path`
[01:21:51]   = note: required because it appears within the type `syn::attr::Attribute`
[01:21:51]   = note: required because it appears within the type `*const syn::attr::Attribute`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<syn::attr::Attribute>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<syn::attr::Attribute>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<syn::attr::Attribute>`
[01:21:51]   = note: required because it appears within the type `syn::generics::LifetimeDef`
[01:21:51]   = note: required because it appears within the type `(syn::generics::LifetimeDef, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `*const (syn::generics::LifetimeDef, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<(syn::generics::LifetimeDef, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syn::generics::LifetimeDef, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<(syn::generics::LifetimeDef, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `syn::punctuated::Punctuated<syn::generics::LifetimeDef, syn::token::Comma>`
[01:21:51]   = note: required because it appears within the type `syn::generics::BoundLifetimes`
[01:21:51]   = note: required because it appears within the type `std::option::Option<syn::generics::BoundLifetimes>`
[01:21:51]   = note: required because it appears within the type `syn::ty::TypeBareFn`
[01:21:51]   = note: required because it appears within the type `syn::ty::Type`
[01:21:51]   = note: required because it appears within the type `syn::expr::GenericMethodArgument`
[01:21:51]   = note: required because it appears within the type `(syn::expr::GenericMethodArgument, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `*const (syn::expr::GenericMethodArgument, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<(syn::expr::GenericMethodArgument, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syn::expr::GenericMethodArgument, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<(syn::expr::GenericMethodArgument, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `syn::punctuated::Punctuated<syn::expr::GenericMethodArgument, syn::token::Comma>`
[01:21:51]   = note: required because it appears within the type `syn::expr::MethodTurbofish`
[01:21:51]   = note: required because it appears within the type `std::option::Option<syn::expr::MethodTurbofish>`
[01:21:51]   = note: required because it appears within the type `syn::expr::ExprMethodCall`
[01:21:51]   = note: required because it appears within the type `syn::expr::Expr`
[01:21:51]   = note: required because it appears within the type `(syn::expr::Expr, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `*const (syn::expr::Expr, syn::token::Comma)`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<(syn::expr::Expr, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<(syn::expr::Expr, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<(syn::expr::Expr, syn::token::Comma)>`
[01:21:51]   = note: required because it appears within the type `syn::punctuated::Punctuated<syn::expr::Expr, syn::token::Comma>`
[01:21:51]   = note: required because it appears within the type `query::QueryModifier`
[01:21:51]   = note: required because it appears within the type `*const query::QueryModifier`
[01:21:51]   = note: required because it appears within the type `std::ptr::Unique<query::QueryModifier>`
[01:21:51]   = note: required because it appears within the type `alloc::raw_vec::RawVec<query::QueryModifier>`
[01:21:51]   = note: required because it appears within the type `std::vec::Vec<query::QueryModifier>`
[01:21:51]   = note: required because it appears within the type `query::List<query::QueryModifier>`
[01:21:51]   = note: required because it appears within the type `query::Query`
[01:21:51] error: aborting due to previous error
[01:21:51] 
[01:21:51] For more information about this error, try `rustc --explain E0275`.
[01:21:51] error: Could not document `rustc_macros`.
[01:21:51] error: Could not document `rustc_macros`.
[01:21:51] 
[01:21:51] Caused by:
[01:21:51]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name rustc_macros src/librustc_macros/src/lib.rs --color always -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libitertools-19fc87ee0f6fa1f2.rmeta --extern proc_macro2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libproc_macro2-0c67e46c5c2a44eb.rmeta --extern quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libquote-e43b075d44cdae48.rmeta --extern syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libsyn-c87c3069fba1eb26.rmeta --extern synstructure=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/libsynstructure-a98d8643b06622c5.rmeta` (exit code: 1)
[01:21:55] error: build failed
[01:21:55] 
[01:21:55] 
[01:21:55] 
[01:21:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_apfloat" "-p" "rustc_traits" "-p" "rustc_lint" "-p" "rustc_allocator" "-p" "rustc_codegen_llvm" "-p" "syntax_pos" "-p" "syntax_ext" "-p" "rustc_macros" "-p" "rustc_mir" "-p" "rustc_metadata" "-p" "rustc_llvm" "-p" "rustc_typeck" "-p" "rustc_data_structures" "-p" "build_helper" "-p" "rustc_passes" "-p" "rustc_interface" "-p" "rustc_codegen_ssa" "-p" "rustc_plugin" "-p" "serialize" "-p" "rustc_target" "-p" "rustc_fs_util" "-p" "rustc_codegen_utils" "-p" "fmt_macros" "-p" "arena" "-p" "rustc_resolve" "-p" "rustc_save_analysis" "-p" "graphviz" "-p" "rustc_errors" "-p" "syntax" "-p" "rustc_incremental" "-p" "rustc_driver" "-p" "rustc_borrowck" "-p" "rustc" "-p" "rustc_privacy"
[01:21:55] 
[01:21:55] 
[01:21:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:21:55] Build completed unsuccessfully in 1:15:21
---
travis_time:end:0b7ff732:start=1560036292088115841,finish=1560036292107883889,duration=19768048
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26f7df89
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09f06807
travis_time:start:09f06807
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:357fbebf
$ dmesg | grep -i kill
