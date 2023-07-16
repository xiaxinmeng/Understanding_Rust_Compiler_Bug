plain
travis_time:end:013ec130:start=1554365829450513934,finish=1554365905777002284,duration=76326488350
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:08] 
[01:21:08] running 9 tests
[01:21:08] iiiiiiiii
[01:21:08] 
[01:21:08]  finished in 0.153
[01:21:08] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:24] 
[01:21:24] running 121 tests
[01:21:50] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:21:54] i.i......iii.i.....ii
[01:21:54] 
[01:21:54]  finished in 30.587
[01:21:54] travis_fold:end:test_debuginfo

---
[01:46:41]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:1949:46
[01:46:44]      |
[01:46:44] 1949 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:1997:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 1997 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2007:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2007 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2017:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2017 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2027:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2027 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2037:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2037 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2047:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2047 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2057:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2057 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2067:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2067 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2077:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2077 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2089:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2089 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2135:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2135 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:44] error[E0433]: failed to resolve: use of undeclared type or module `FilePathMapping`
[01:46:44]     --> src/libsyntax/parse/lexer/mod.rs:2150:46
[01:46:44]      |
[01:46:44]      |
[01:46:44] 2150 |             let sm = Lrc::new(SourceMap::new(FilePathMapping::empty()));
[01:46:44]      |                                              ^^^^^^^^^^^^^^^ use of undeclared type or module `FilePathMapping`
[01:46:53] error: aborting due to 13 previous errors
[01:46:53] 
[01:46:53] For more information about this error, try `rustc --explain E0433`.
[01:46:53] error: Could not compile `syntax`.
[01:46:53] error: Could not compile `syntax`.
[01:46:53] 
[01:46:53] To learn more, run the command again with --verbose.
[01:46:53] 
[01:46:53] 
[01:46:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:46:53] 
[01:46:53] 
[01:46:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:46:53] Build completed unsuccessfully in 0:37:35
[01:46:53] Build completed unsuccessfully in 0:37:35
[01:46:53] make: *** [check] Error 1
[01:46:53] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:077eb17c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr  4 10:05:28 UTC 2019
---
travis_time:end:187e2b1c:start=1554372329838296970,finish=1554372329910896610,duration=72599640
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1123798f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f76a322
$ dmesg | grep -i kill
