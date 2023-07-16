plain
travis_time:end:13c043e8:start=1560003836551308152,finish=1560003923701219448,duration=87149911296
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:52] 
[01:03:52] running 144 tests
[01:03:55] i..iii.....iii..iiii.....i......................i..i.................i......i.........ii.i..i..i.ii. 100/144
[01:03:57] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:03:57] 
[01:03:57]  finished in 4.571
[01:03:57] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:59] 
[01:03:59] running 9 tests
[01:03:59] iiiiiiiii
[01:03:59] 
[01:03:59]  finished in 0.150
[01:03:59] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:14] 
[01:04:14] running 122 tests
[01:04:39] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:04:44] .i.i......iii.i.....ii
[01:04:44] 
[01:04:44]  finished in 29.466
[01:04:44] travis_fold:end:test_debuginfo

---
[01:28:16]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:28:25] error[E0277]: can't compare `&syntax_pos::symbol::Symbol` with `syntax_pos::symbol::Symbol`
[01:28:25]    --> src/libsyntax/parse/mod.rs:434:37
[01:28:25]     |
[01:28:25] 434 |                 if name_macro_rules == sym::macro_rules && name_zip.as_str() == "zip" => {
[01:28:25]     |                                     ^^ no implementation for `&syntax_pos::symbol::Symbol == syntax_pos::symbol::Symbol`
[01:28:25]     |
[01:28:25]     = help: the trait `std::cmp::PartialEq<syntax_pos::symbol::Symbol>` is not implemented for `&syntax_pos::symbol::Symbol`
[01:28:25] 
[01:28:25] error[E0277]: can't compare `&parse::token::DelimToken` with `parse::token::DelimToken`
[01:28:25]    --> src/libsyntax/parse/mod.rs:442:40
[01:28:25]     |
[01:28:25] 442 |                         if macro_delim == token::Paren => {
[01:28:25]     |                                        ^^ no implementation for `&parse::token::DelimToken == parse::token::DelimToken`
[01:28:25]     |
[01:28:25]     = help: the trait `std::cmp::PartialEq<parse::token::DelimToken>` is not implemented for `&parse::token::DelimToken`
[01:28:25] 
[01:28:25] error[E0277]: can't compare `&parse::token::DelimToken` with `parse::token::DelimToken`
[01:28:25]    --> src/libsyntax/parse/mod.rs:449:48
[01:28:25]     |
[01:28:25] 449 |                                 if first_delim == token::Paren && name.as_str() == "a" => {},
[01:28:25]     |                                                ^^ no implementation for `&parse::token::DelimToken == parse::token::DelimToken`
[01:28:25]     |
[01:28:25]     = help: the trait `std::cmp::PartialEq<parse::token::DelimToken>` is not implemented for `&parse::token::DelimToken`
[01:28:25] 
[01:28:25] error[E0277]: can't compare `&parse::token::DelimToken` with `parse::token::DelimToken`
[01:28:25]    --> src/libsyntax/parse/mod.rs:458:49
[01:28:25]     |
[01:28:25] 458 |                                 if second_delim == token::Paren && name.as_str() == "a" => {},
[01:28:25]     |                                                 ^^ no implementation for `&parse::token::DelimToken == parse::token::DelimToken`
[01:28:25]     |
[01:28:25]     = help: the trait `std::cmp::PartialEq<parse::token::DelimToken>` is not implemented for `&parse::token::DelimToken`
[01:28:28] error: aborting due to 4 previous errors
[01:28:28] 
[01:28:28] For more information about this error, try `rustc --explain E0277`.
[01:28:28] error: Could not compile `syntax`.
[01:28:28] error: Could not compile `syntax`.
[01:28:28] 
[01:28:28] To learn more, run the command again with --verbose.
[01:28:28] 
[01:28:28] 
[01:28:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:28:28] 
[01:28:28] 
[01:28:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:28:28] Build completed unsuccessfully in 1:24:22
---
travis_time:end:21ee3b55:start=1560009243322195574,finish=1560009243383285447,duration=61089873
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:36d4a120
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:22bbe7d6
$ dmesg | grep -i kill
