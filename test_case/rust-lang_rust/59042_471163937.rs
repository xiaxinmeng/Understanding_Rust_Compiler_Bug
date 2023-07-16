plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:15d26819
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:16:00]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:16:05] error[E0308]: mismatched types
[01:16:05]   --> src/librustdoc/passes/collect_intra_doc_links.rs:82:49
[01:16:05]    |
[01:16:05] 82 |                                     .with_scope(id,
[01:16:05]    |                                                 ^^ expected struct `syntax::ast::NodeId`, found struct `rustc::hir::HirId`
[01:16:05]    = note: expected type `syntax::ast::NodeId`
[01:16:05]               found type `rustc::hir::HirId`
[01:16:05] 
[01:16:05] error[E0308]: mismatched types
[01:16:05] error[E0308]: mismatched types
[01:16:05]    --> src/librustdoc/passes/collect_intra_doc_links.rs:146:45
[01:16:05]     |
[01:16:05] 146 |                                 .with_scope(id,
[01:16:05]     |                                             ^^ expected struct `syntax::ast::NodeId`, found struct `rustc::hir::HirId`
[01:16:05]     = note: expected type `syntax::ast::NodeId`
[01:16:05]                found type `rustc::hir::HirId`
[01:16:05] 
[01:16:06] error[E0308]: mismatched types
[01:16:06] error[E0308]: mismatched types
[01:16:06]    --> src/librustdoc/visit_ast.rs:275:39
[01:16:06]     |
[01:16:06] 275 |                 if cx.tcx.hir().attrs(node).lists("doc").has_word("hidden") {
[01:16:06]     |                                       ^^^^ expected struct `syntax::ast::NodeId`, found struct `rustc::hir::HirId`
[01:16:06]     = note: expected type `syntax::ast::NodeId`
[01:16:06]                found type `rustc::hir::HirId`
[01:16:06] 
[01:16:06] error: aborting due to 3 previous errors
---
travis_time:end:29603c5a:start=1552125624405458878,finish=1552125624414692049,duration=9233171
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:060a2f09
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e52fe18
travis_time:start:0e52fe18
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:057ad52b
$ dmesg | grep -i kill
