plain
travis_time:end:03e59b8f:start=1541106271489245479,finish=1541106348102456572,duration=76613211093
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:34] .................................................................................................... 100/4983
[00:48:37] .................................................................................................... 200/4983
[00:48:39] ...........................................................................................ii....... 300/4983
[00:48:42] .........................................................................................iii........ 400/4983
[00:48:45] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4983
[00:48:52] .................................................................................................... 700/4983
[00:48:58] ..................................................................i...........i..................... 800/4983
[00:49:01] ....................................................................................iiiii........... 900/4983
[00:49:04] .................................................................................................... 1000/4983
---
[00:49:39] .................................................................................................... 2200/4983
[00:49:43] .................................................................................................... 2300/4983
[00:49:47] .................................................................................................... 2400/4983
[00:49:50] .................................................................................................... 2500/4983
[00:49:54] ...................................................................iiiiiiiii........................ 2600/4983
[00:50:00] ..................ii................................................................................ 2800/4983
[00:50:03] .................................................................................................... 2900/4983
[00:50:07] .................................................................................................... 3000/4983
[00:50:09] .............i...................................................................................... 3100/4983
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:17] 
[01:03:17] running 115 tests
[01:03:20] i..ii...iii..iii.....i....i........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:03:20] .i....iiii.....
[01:03:20] 
[01:03:20]  finished in 3.818
[01:03:20] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:37] 
[01:03:37] running 118 tests
[01:04:02] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:04:06] ......iii.i.....ii
[01:04:06] 
[01:04:06]  finished in 29.545
[01:04:06] travis_fold:end:test_debuginfo

---
[01:30:04]   |
[01:30:04] 1 | ::abc::def::return
[01:30:04]   |             ^^^^^^ expected identifier, found keyword
[01:30:04] 
[01:30:04] ..........................F...........................................................
[01:30:04] 
[01:30:04] ---- parse::tests::out_of_line_mod stdout ----
[01:30:04] ---- parse::tests::out_of_line_mod stdout ----
[01:30:04] thread 'parse::tests::out_of_line_mod' panicked at 'called `Result::unwrap()` on an `Err` value: Diagnostic { level: Error, message: [("file not found for module `this_does_not_exist`", NoStyle)], code: Some(Error("E0583")), span: MultiSpan { primary_spans: [Span { lo: BytePos(24), hi: BytePos(43), ctxt: #0 }], span_labels: [] }, children: [SubDiagnostic { level: Help, message: [("name the file either this_does_not_exist.rs or this_does_not_exist/mod.rs inside the directory \"foo\"", NoStyle)], span: MultiSpan { primary_spans: [], span_labels: [] }, render_span: None }], suggestions: [] }', libcore/result.rs:1009:5
[01:30:04] 
[01:30:04] 
[01:30:04] failures:
[01:30:04]     parse::tests::out_of_line_mod
[01:30:04]     parse::tests::out_of_line_mod
[01:30:04] 
[01:30:04] test result: FAILED. 85 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:30:04] 
[01:30:04] error: test failed, to rerun pass '--lib'
[01:30:04] 
[01:30:04] 
[01:30:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:30:04] 
[01:30:04] 
[01:30:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:30:04] Build completed unsuccessfully in 0:45:09
[01:30:04] Build completed unsuccessfully in 0:45:09
[01:30:04] Makefile:58: recipe for target 'check' failed
[01:30:04] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:005142b7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
