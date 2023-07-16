plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:09354054
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:08:36]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[01:08:41] error[E0308]: mismatched types
[01:08:41]    --> src/librustdoc/passes/collect_intra_doc_links.rs:428:38
[01:08:41]     |
[01:08:41] 428 |     let path = ast::Path { segments: vec![segment], span: DUMMY_SP };
[01:08:41]     |                                      ^^^^^^^^^^^^^ expected struct `smallvec::SmallVec`, found struct `std::vec::Vec`
[01:08:41]     |
[01:08:41]     = note: expected type `smallvec::SmallVec<[syntax::ast::PathSegment; 1]>`
[01:08:41]                found type `std::vec::Vec<syntax::ast::PathSegment>`
[01:08:41] 
[01:08:41] error: aborting due to previous error
[01:08:41] 
[01:08:41] For more information about this error, try `rustc --explain E0308`.
---
travis_time:end:0204d9d0:start=1547540278496175761,finish=1547540278514191173,duration=18015412
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:046481e8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0252efa6
travis_time:start:0252efa6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02ddfc64
$ dmesg | grep -i kill
