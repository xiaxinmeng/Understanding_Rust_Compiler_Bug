plain
travis_time:end:072bb578:start=1549370804865059063,finish=1549370808410466260,duration=3545407197
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:03] 
[01:14:03] running 119 tests
[01:14:29] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:14:33] i......iii.i.....ii
[01:14:33] 
[01:14:33]  finished in 29.991
[01:14:33] travis_fold:end:test_debuginfo

---
[01:38:26]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:38:29] error[E0308]: mismatched types
[01:38:29]   --> src/libsyntax/util/parser_testing.rs:14:5
[01:38:29]    |
[01:38:29] 12 |   pub fn string_to_stream(source_str: String) -> TokenStream {
[01:38:29]    |                                                  ----------- expected `tokenstream::TokenStream` because of return type
[01:38:29] 13 |       let ps = ParseSess::new(FilePathMapping::empty());
[01:38:29] 14 | /     source_file_to_stream(&ps, ps.source_map()
[01:38:29] 15 | |                              .new_source_file(PathBuf::from("bogofile").into(), source_str), None)
[01:38:29]    | |__________________________________________________________________________________________________^ expected struct `tokenstream::TokenStream`, found tuple
[01:38:29]    = note: expected type `tokenstream::TokenStream`
[01:38:29]    = note: expected type `tokenstream::TokenStream`
[01:38:29]               found type `(tokenstream::TokenStream, std::vec::Vec<parse::lexer::UnmatchedBrace>)`
[01:38:35] error: aborting due to previous error
[01:38:35] 
[01:38:35] For more information about this error, try `rustc --explain E0308`.
[01:38:35] error: Could not compile `syntax`.
[01:38:35] error: Could not compile `syntax`.
[01:38:35] 
[01:38:35] To learn more, run the command again with --verbose.
[01:38:35] 
[01:38:35] 
[01:38:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:38:35] 
[01:38:35] 
[01:38:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:35] Build completed unsuccessfully in 0:36:21
[01:38:35] Build completed unsuccessfully in 0:36:21
[01:38:35] Makefile:48: recipe for target 'check' failed
[01:38:35] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1597e988
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb  5 14:25:34 UTC 2019
---
travis_time:end:1dc8e2c4:start=1549376735776739331,finish=1549376735781455408,duration=4716077
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:13f9cf78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1779d3b4
travis_time:start:1779d3b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a1d169a
$ dmesg | grep -i kill
