plain
travis_time:end:054a3ae8:start=1544736706037518611,finish=1544736765606403193,duration=59568884582
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:09] 
[00:55:09] running 121 tests
[00:55:12] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:55:13] i..ii.i.....iiii.....
[00:55:13] 
[00:55:13]  finished in 3.386
[00:55:13] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:27] 
[00:55:27] running 119 tests
[00:55:50] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i. 100/119
[00:55:54] i......iii.i.....ii
[00:55:54] 
[00:55:54]  finished in 26.839
[00:55:54] travis_fold:end:test_debuginfo

---
[01:20:07]     Finished release [optimized] target(s) in 38.77s
[01:20:07]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-5f554cec32a0a6e3
[01:20:07] 
[01:20:07] running 41 tests
[01:20:07] ..............................F.F........
[01:20:07] 
[01:20:07] ---- test::tests::make_test_fake_main stdout ----
[01:20:07] thread 'test::tests::make_test_fake_main' panicked at 'assertion failed: `(left == right)`
[01:20:07] thread 'test::tests::make_test_fake_main' panicked at 'assertion failed: `(left == right)`
[01:20:07]   left: `("#![allow(unused)]\n//Ceci n\'est pas une `fn main`\nfn main() {\nassert_eq!(2+2, 4);\n}", 2)`,
[01:20:07]  right: `("#![allow(unused)]\nfn main() {\n//Ceci n\'est pas une `fn main`\nassert_eq!(2+2, 4);\n}", 2)`', src/librustdoc/test.rs:1085:9
[01:20:07] ---- test::tests::make_test_issues_21299_33731 stdout ----
[01:20:07] thread 'test::tests::make_test_issues_21299_33731' panicked at 'assertion failed: `(left == right)`
[01:20:07] thread 'test::tests::make_test_issues_21299_33731' panicked at 'assertion failed: `(left == right)`
[01:20:07]   left: `("#![allow(unused)]\n// fn main\nfn main() {\nassert_eq!(2+2, 4);\n}", 2)`,
[01:20:07]  right: `("#![allow(unused)]\nfn main() {\n// fn main\nassert_eq!(2+2, 4);\n}", 2)`', src/librustdoc/test.rs:1134:9
[01:20:07] 
[01:20:07] 
[01:20:07] failures:
[01:20:07]     test::tests::make_test_fake_main
[01:20:07]     test::tests::make_test_fake_main
[01:20:07]     test::tests::make_test_issues_21299_33731
[01:20:07] 
[01:20:07] test result: FAILED. 39 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:20:07] 
[01:20:07] error: test failed, to rerun pass '--lib'
[01:20:07] 
[01:20:07] 
[01:20:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:20:07] 
[01:20:07] 
[01:20:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:07] Build completed unsuccessfully in 0:35:21
[01:20:07] Build completed unsuccessfully in 0:35:21
[01:20:07] Makefile:58: recipe for target 'check' failed
[01:20:07] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0089f355
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 13 22:53:00 UTC 2018
