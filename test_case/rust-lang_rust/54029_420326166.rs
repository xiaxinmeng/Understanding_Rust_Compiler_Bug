plain
[01:23:03]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
[01:23:06] error[E0308]: mismatched types
[01:23:06]   --> libsyntax/util/parser_testing.rs:24:5
[01:23:06]    |
[01:23:06] 22 |   pub fn string_to_stream(source_str: String) -> TokenStream {
[01:23:06]    |                                                  ----------- expected `tokenstream::TokenStream` because of return type
[01:23:06] 23 |       let ps = ParseSess::new(FilePathMapping::empty());
[01:23:06] 24 | /     source_file_to_stream(&ps, ps.source_map()
[01:23:06] 25 | |                              .new_source_file(PathBuf::from("bogofile").into(), source_str), None)
[01:23:06]    | |__________________________________________________________________________________________________^ expected struct `tokenstream::TokenStream`, found tuple
[01:23:06]    |
[01:23:06]    = note: expected type `tokenstream::TokenStream`
[01:23:06]               found type `(tokenstream::TokenStream, std::vec::Vec<(parse::token::DelimToken, syntax_pos::Span)>)`
[01:23:12] error: aborting due to previous error
[01:23:12] 
[01:23:12] For more information about this error, try `rustc --explain E0308`.
[01:23:12] error: Could not compile `syntax`.
[01:23:12] error: Could not compile `syntax`.
[01:23:12] 
[01:23:12] To learn more, run the command again with --verbose.
[01:23:12] 
[01:23:12] 
[01:23:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:23:12] 
[01:23:12] 
[01:23:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:12] Build completed unsuccessfully in 0:36:08
[01:23:12] Build completed unsuccessfully in 0:36:08
[01:23:12] make: *** [check] Error 1
[01:23:12] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0be93662
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2e38b27d:start=1536681654103572013,finish=1536681654187055293,duration=83483280
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:021e9642
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:085329b4
$ dmesg | grep -i kill
