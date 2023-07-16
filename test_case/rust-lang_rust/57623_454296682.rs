plain
travis_time:end:0084e7c0:start=1547535282726058183,finish=1547535283632201523,duration=906143340
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:53:04]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:53:10] error[E0308]: mismatched types
[00:53:10]    --> src/librustdoc/passes/collect_intra_doc_links.rs:428:38
[00:53:10]     |
[00:53:10] 428 |     let path = ast::Path { segments: vec![segment], span: DUMMY_SP };
[00:53:10]     |                                      ^^^^^^^^^^^^^ expected struct `smallvec::SmallVec`, found struct `std::vec::Vec`
[00:53:10]     |
[00:53:10]     = note: expected type `smallvec::SmallVec<[syntax::ast::PathSegment; 1]>`
[00:53:10]                found type `std::vec::Vec<syntax::ast::PathSegment>`
[00:53:10] 
[00:53:10] error: aborting due to previous error
[00:53:10] 
[00:53:10] For more information about this error, try `rustc --explain E0308`.
---
[00:53:10] 
[00:53:10] 
[00:53:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:53:10] Build completed unsuccessfully in 0:49:22
[00:53:10] Makefile:18: recipe for target 'all' failed
[00:53:10] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0084590a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 15 07:48:05 UTC 2019
---
travis_time:end:00002600:start=1547538486366242390,finish=1547538486370921618,duration=4679228
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:027f6b74
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05f5a9a2
travis_time:start:05f5a9a2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynami
