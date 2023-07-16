plain
travis_time:end:03f1a5dc:start=1552121008554060787,finish=1552121082514661400,duration=73960600613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:59:57]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:00:03] error[E0308]: mismatched types
[01:00:03]   --> src/librustdoc/passes/collect_intra_doc_links.rs:82:49
[01:00:03]    |
[01:00:03] 82 |                                     .with_scope(id,
[01:00:03]    |                                                 ^^ expected struct `syntax::ast::NodeId`, found struct `rustc::hir::HirId`
[01:00:03]    = note: expected type `syntax::ast::NodeId`
[01:00:03]               found type `rustc::hir::HirId`
[01:00:03] 
[01:00:03] error[E0308]: mismatched types
[01:00:03] error[E0308]: mismatched types
[01:00:03]    --> src/librustdoc/passes/collect_intra_doc_links.rs:146:45
[01:00:03]     |
[01:00:03] 146 |                                 .with_scope(id,
[01:00:03]     |                                             ^^ expected struct `syntax::ast::NodeId`, found struct `rustc::hir::HirId`
[01:00:03]     = note: expected type `syntax::ast::NodeId`
[01:00:03]                found type `rustc::hir::HirId`
[01:00:03] 
[01:00:04] error[E0308]: mismatched types
[01:00:04] error[E0308]: mismatched types
[01:00:04]    --> src/librustdoc/visit_ast.rs:275:39
[01:00:04]     |
[01:00:04] 275 |                 if cx.tcx.hir().attrs(node).lists("doc").has_word("hidden") {
[01:00:04]     |                                       ^^^^ expected struct `syntax::ast::NodeId`, found struct `rustc::hir::HirId`
[01:00:04]     = note: expected type `syntax::ast::NodeId`
[01:00:04]                found type `rustc::hir::HirId`
[01:00:04] 
[01:00:04] error: aborting due to 3 previous errors
---
travis_time:end:097e7ef8:start=1552124696918302332,finish=1552124696923750692,duration=5448360
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2aca16b6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1568c58c
travis_time:start:1568c58c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers
