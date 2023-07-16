plain
travis_time:end:062e369b:start=1558487821529995359,finish=1558487911221335975,duration=89691340616
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
[01:22:12] 
[01:22:12] running 143 tests
[01:22:14] i..iii.....iii..iiii.....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:22:16] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:22:16] 
[01:22:16]  finished in 4.776
[01:22:16] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:18] 
[01:22:18] running 9 tests
[01:22:18] iiiiiiiii
[01:22:18] 
[01:22:18]  finished in 0.154
[01:22:18] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:35] 
[01:22:35] running 122 tests
[01:23:00] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:23:06] .i.i......iii.i.....ii
[01:23:06] 
[01:23:06]  finished in 31.326
[01:23:06] travis_fold:end:test_debuginfo

---
[01:38:35]    --> src/libfmt_macros/lib.rs:630:22
[01:38:35]     |
[01:38:35] 229 | /     pub fn new(
[01:38:35] 230 | |         s: &'a str,
[01:38:35] 231 | |         style: Option<usize>,
[01:38:35] 232 | |         skips: Vec<usize>,
[01:38:35] 245 | |         }
[01:38:35] 246 | |     }
[01:38:35]     | |_____- defined here
[01:38:35] ...
[01:38:35] ...
[01:38:35] 630 |           let parser = Parser::new(fmt, None, vec![], false, None);
[01:38:35] 
[01:38:35] error[E0061]: this function takes 4 parameters but 5 parameters were supplied
[01:38:35]    --> src/libfmt_macros/lib.rs:646:21
[01:38:35]     |
[01:38:35]     |
[01:38:35] 229 | /     pub fn new(
[01:38:35] 230 | |         s: &'a str,
[01:38:35] 231 | |         style: Option<usize>,
[01:38:35] 232 | |         skips: Vec<usize>,
[01:38:35] 245 | |         }
[01:38:35] 246 | |     }
[01:38:35]     | |_____- defined here
[01:38:35] ...
[01:38:35] ...
[01:38:35] 646 |           let mut p = Parser::new(s, None, vec![], false, None);
[01:38:35] 
[01:38:35] error: aborting due to 2 previous errors
[01:38:35] 
[01:38:35] For more information about this error, try `rustc --explain E0061`.
[01:38:35] For more information about this error, try `rustc --explain E0061`.
[01:38:35] error: Could not compile `fmt_macros`.
[01:38:35] 
[01:38:35] To learn more, run the command again with --verbose.
[01:38:35] 
[01:38:35] 
[01:38:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "fmt_macros" "--" "--quiet"
[01:38:35] 
[01:38:35] 
[01:38:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:35] Build completed unsuccessfully in 0:28:29
[01:38:35] Build completed unsuccessfully in 0:28:29
[01:38:35] make: *** [check] Error 1
[01:38:35] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2117c470
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 22 02:57:15 UTC 2019
