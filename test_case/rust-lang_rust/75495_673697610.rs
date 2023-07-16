plain
   Compiling ra_ide_db v0.1.0 (/checkout/src/tools/rust-analyzer/crates/ra_ide_db)
   Compiling ra_ssr v0.1.0 (/checkout/src/tools/rust-analyzer/crates/ra_ssr)
   Compiling ra_assists v0.1.0 (/checkout/src/tools/rust-analyzer/crates/ra_assists)
   Compiling ra_ide v0.1.0 (/checkout/src/tools/rust-analyzer/crates/ra_ide)
error: internal compiler error: src/librustc_trait_selection/traits/codegen/mod.rs:75:17: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@crates/ra_ide/src/expand_macro.rs:86:27: 88:10 token_iter:&mut std::iter::Peekable<std::iter::FilterMap<std::iter::Map<std::iter::Successors<ra_syntax::WalkEvent<ra_syntax::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>>, [closure@rowan::cursor::SyntaxNode::preorder_with_tokens::{{closure}}#0 0:ra_syntax::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>]>, [closure@rowan::api::SyntaxNode::<L>::preorder_with_tokens::{{closure}}#0]>, [closure@crates/ra_ide/src/expand_macro.rs:73:21: 79:10]>>] as std::ops::FnMut<(fn(ra_syntax::SyntaxKind) -> bool, bool)>>), Binder(<[closure@crates/ra_ide/src/expand_macro.rs:86:27: 88:10 token_iter:&mut std::iter::Peekable<std::iter::FilterMap<impl std::iter::Iterator, [closure@crates/ra_ide/src/expand_macro.rs:73:21: 79:10]>>] as std::ops::FnMut<(fn(ra_syntax::SyntaxKind) -> bool, bool)>>), Sorts(ExpectedFound { expected: impl std::iter::Iterator, found: std::iter::Map<std::iter::Successors<ra_syntax::WalkEvent<ra_syntax::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>>, [closure@rowan::cursor::SyntaxNode::preorder_with_tokens::{{closure}}#0 0:ra_syntax::NodeOrToken<rowan::cursor::SyntaxNode, rowan::cursor::SyntaxToken>]>, [closure@rowan::api::SyntaxNode::<L>::preorder_with_tokens::{{closure}}#0]> }))` selecting `Binder(<[closure@crates/ra_ide/src/expand_macro.rs:86:27: 88:10 token_iter:&mut std::iter::Peekable<std::iter::FilterMap<impl std::iter::Iterator, [closure@crates/ra_ide/src/expand_macro.rs:73:21: 79:10]>>] as std::ops::FnMut<(fn(ra_syntax::SyntaxKind) -> bool, bool)>>)` during codegen
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:915:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0-nightly (ad4a8ac00 2020-08-13) running on x86_64-unknown-linux-gnu
note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C linker=clang -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib

note: some of the compiler flags provided by cargo are hidden


error: aborting due to previous error

error: could not compile `ra_ide`.
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-analyzer/crates/rust-analyzer/Cargo.toml" "--message-format" "json-render-diagnostics"
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rust-analyzer/crates/rust-analyzer/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'rust-analyzer always builds', src/bootstrap/dist.rs:1402:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:33:39
== clock drift check ==
  local time: Thu Aug 13 20:35:50 UTC 2020
  local time: Thu Aug 13 20:35:50 UTC 2020
  network time: Thu, 13 Aug 2020 20:35:50 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (2744) (python)
