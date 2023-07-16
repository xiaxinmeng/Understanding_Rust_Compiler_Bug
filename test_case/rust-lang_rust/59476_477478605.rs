plain
travis_time:end:052f5e9a:start=1553750927572300497,finish=1553751065547230830,duration=137974930333
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
[01:23:08] 
[01:23:08] running 9 tests
[01:23:08] iiiiiiiii
[01:23:08] 
[01:23:08]  finished in 0.168
[01:23:08] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:25] 
[01:23:25] running 120 tests
[01:23:55] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:24:00] .i......iii.i.....ii
[01:24:00] 
[01:24:00]  finished in 34.859
[01:24:00] travis_fold:end:test_debuginfo

---
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:48:55]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:49:06] error[E0308]: mismatched types
[01:49:06]    --> <::alloc::macros::vec macros>:3:1
[01:49:06]     |
[01:49:06] 1   | / ( $ elem : expr ; $ n : expr ) => (
[01:49:06] 2   | | $ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (
[01:49:06] 3   | | < [ _ ] > :: into_vec ( box [ $ ( $ x ) , * ] ) ) ; ( $ ( $ x : expr , ) * )
[01:49:06]     | | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:49:06]     | | expected struct `smallvec::SmallVec`, found struct `std::vec::Vec`
[01:49:06]     | | expected struct `smallvec::SmallVec`, found struct `std::vec::Vec`
[01:49:06] 4   | | => ( vec ! [ $ ( $ x ) , * ] )
[01:49:06]     | 
[01:49:06]    ::: src/libsyntax/tokenstream.rs:577:52
[01:49:06]     |
[01:49:06]     |
[01:49:06] 577 |               let eq_res = TokenStream::from_streams(vec![test_fst, test_snd]);
[01:49:06]     |
[01:49:06]     |
[01:49:06]     = note: expected type `smallvec::SmallVec<[tokenstream::TokenStream; 2]>`
[01:49:06]                found type `std::vec::Vec<tokenstream::TokenStream>`
[01:49:07] error: aborting due to previous error
[01:49:07] 
[01:49:07] For more information about this error, try `rustc --explain E0308`.
[01:49:07] error: Could not compile `syntax`.
[01:49:07] error: Could not compile `syntax`.
[01:49:07] 
[01:49:07] To learn more, run the command again with --verbose.
[01:49:07] 
[01:49:07] 
[01:49:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:49:07] 
[01:49:07] 
[01:49:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:49:07] Build completed unsuccessfully in 0:38:41
[01:49:07] Build completed unsuccessfully in 0:38:41
[01:49:07] make: *** [check] Error 1
[01:49:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:327d7648
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 28 07:20:23 UTC 2019
