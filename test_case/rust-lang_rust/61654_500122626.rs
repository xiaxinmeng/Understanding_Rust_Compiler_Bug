plain
travis_time:end:2160f6f0:start=1559993486844391452,finish=1559993575553843713,duration=88709452261
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
[01:05:53] 
[01:05:53] running 144 tests
[01:05:55] i..iii.....iii...iiii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:05:57] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:57] 
[01:05:57]  finished in 4.488
[01:05:57] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:59] 
[01:05:59] running 9 tests
[01:05:59] iiiiiiiii
[01:05:59] 
[01:05:59]  finished in 0.146
[01:05:59] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:15] 
[01:06:15] running 122 tests
[01:06:39] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:44] .i.i......iii.i.....ii
[01:06:44] 
[01:06:44]  finished in 29.251
[01:06:44] travis_fold:end:test_debuginfo

---
[01:30:12] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:30:13]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:30:22] error[E0529]: expected an array or slice, found `std::vec::Vec<tokenstream::TokenTree>`
[01:30:22]    --> src/libsyntax/parse/mod.rs:437:25
[01:30:22] 437 | /                         [
[01:30:22] 437 | /                         [
[01:30:22] 438 | |                             TokenTree::Delimited(_, first_delim, first_tts),
[01:30:22] 439 | |                             TokenTree::Token(Token { kind: token::FatArrow, .. }),
[01:30:22] 440 | |                             TokenTree::Delimited(_, second_delim, second_tts),
[01:30:22] 441 | |                         ]
[01:30:22]     | |_________________________^ pattern cannot match with input type `std::vec::Vec<tokenstream::TokenTree>`
[01:30:22] 
[01:30:22] error[E0529]: expected an array or slice, found `std::vec::Vec<tokenstream::TokenTree>`
[01:30:22]    --> src/libsyntax/parse/mod.rs:445:33
[01:30:22] 445 | /                                 [
[01:30:22] 445 | /                                 [
[01:30:22] 446 | |                                     TokenTree::Token(Token { kind: token::Dollar, .. }),
[01:30:22] 447 | |                                     TokenTree::Token(Token { kind: token::Ident(name, false), .. }),
[01:30:22] 448 | |                                 ]
[01:30:22]     | |_________________________________^ pattern cannot match with input type `std::vec::Vec<tokenstream::TokenTree>`
[01:30:22] 
[01:30:22] error[E0277]: can't compare `&parse::token::DelimToken` with `parse::token::DelimToken`
[01:30:22]    --> src/libsyntax/parse/mod.rs:449:48
[01:30:22]     |
[01:30:22] 449 |                                 if first_delim == token::Paren && name.as_str() == "a" => {},
[01:30:22]     |                                                ^^ no implementation for `&parse::token::DelimToken == parse::token::DelimToken`
[01:30:22]     |
[01:30:22]     = help: the trait `std::cmp::PartialEq<parse::token::DelimToken>` is not implemented for `&parse::token::DelimToken`
[01:30:22] 
[01:30:22] error[E0529]: expected an array or slice, found `std::vec::Vec<tokenstream::TokenTree>`
[01:30:22]    --> src/libsyntax/parse/mod.rs:454:33
[01:30:22] 454 | /                                 [
[01:30:22] 454 | /                                 [
[01:30:22] 455 | |                                     TokenTree::Token(Token { kind: token::Dollar, .. }),
[01:30:22] 456 | |                                     TokenTree::Token(Token { kind: token::Ident(name, false), .. }),
[01:30:22] 457 | |                                 ]
[01:30:22]     | |_________________________________^ pattern cannot match with input type `std::vec::Vec<tokenstream::TokenTree>`
[01:30:22] 
[01:30:22] error[E0277]: can't compare `&parse::token::DelimToken` with `parse::token::DelimToken`
[01:30:22]    --> src/libsyntax/parse/mod.rs:458:49
[01:30:22]     |
[01:30:22] 458 |                                 if second_delim == token::Paren && name.as_str() == "a" => {},
[01:30:22]     |                                                 ^^ no implementation for `&parse::token::DelimToken == parse::token::DelimToken`
[01:30:22]     |
[01:30:22]     = help: the trait `std::cmp::PartialEq<parse::token::DelimToken>` is not implemented for `&parse::token::DelimToken`
[01:30:25] error: aborting due to 5 previous errors
[01:30:25] 
[01:30:25] Some errors have detailed explanations: E0277, E0529.
[01:30:25] For more information about an error, try `rustc --explain E0277`.
[01:30:25] For more information about an error, try `rustc --explain E0277`.
[01:30:25] error: Could not compile `syntax`.
[01:30:25] 
[01:30:25] To learn more, run the command again with --verbose.
[01:30:25] 
[01:30:25] 
[01:30:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:30:25] 
[01:30:25] 
[01:30:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:25] Build completed unsuccessfully in 1:26:04
---
travis_time:end:2ab8230d:start=1559999012461053660,finish=1559999012466611425,duration=5557765
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08004c60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a1ba430
travis_time:start:1a1ba430
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c54a2a2
$ dmesg | grep -i kill
