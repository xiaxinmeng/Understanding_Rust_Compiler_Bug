plain
[00:49:30] .................................................................................................... 2200/4635
[00:49:34] ....................................i............................................................... 2300/4635
[00:49:37] .................................................................................................... 2400/4635
[00:49:41] .................................................................................................... 2500/4635
[00:49:45] ..................................................iiiiiiiii......................................... 2600/4635
[00:49:51] .................................................................................................... 2800/4635
[00:49:55] .................................................................................................... 2900/4635
[00:49:58] ................................................................................i................... 3000/4635
[00:50:01] .................................................................................................... 3100/4635
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:41] 
[01:02:41] running 111 tests
[01:02:44] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i....ii.ii.i.. 100/111
[01:02:44] ..iiii.....
[01:02:44] 
[01:02:44]  finished in 3.463
[01:02:44] travis_fold:end:test_codegen

---
[01:21:43] .................................................................................................... 400/961
[01:21:54] .....................................iiii........ii................................................. 500/961
[01:22:00] .................................................................................................... 600/961
[01:22:11] .....................................................................................iiii........... 700/961
[01:22:25] .......................................F...F........................................................ 800/961
[01:22:39] ..........iiii...............................................
[01:22:39] failures:
[01:22:39] 
[01:22:39] ---- process.rs - process::Stdio::from (line 1054) stdout ----
[01:22:39] ---- process.rs - process::Stdio::from (line 1054) stdout ----
[01:22:39] error[E0433]: failed to resolve. Use of undeclared type or module `Stdio`
[01:22:39]  --> process.rs:1059:13
[01:22:39]   |
[01:22:39] 8 |     .stdout(Stdio::piped())
[01:22:39]   |             ^^^^^ Use of undeclared type or module `Stdio`
[01:22:39] 
[01:22:39] thread 'process.rs - process::Stdio::from (line 1054)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:22:39] 
[01:22:39] ---- process.rs - process::Stdio::from (line 1114) stdout ----
[01:22:39] ---- process.rs - process::Stdio::from (line 1114) stdout ----
[01:22:39] error: expected one of `.`, `;`, `?`, or an operator, found `assert_eq`
[01:22:39]    |
[01:22:39] 12 |     .output()?
[01:22:39] 12 |     .output()?
[01:22:39]    |               - expected one of `.`, `;`, `?`, or an operator here
[01:22:39] 13 | 
[01:22:39] 14 | assert_eq!(reverse.stdout, b"!dlrow ,olleH");
[01:22:39]    | ^^^^^^^^^ unexpected token
[01:22:39] error: unused import: `std::process::Command`
[01:22:39]  --> process.rs:1116:5
[01:22:39]   |
[01:22:39] 5 | use std::process::Command;
---
[01:22:39] 1 | #![deny(warnings)]
[01:22:39]   |         ^^^^^^^^
[01:22:39]   = note: #[deny(unused_imports)] implied by #[deny(warnings)]
[01:22:39] 
[01:22:39] thread 'process.rs - process::Stdio::from (line 1114)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:22:39] 
[01:22:39] failures:
[01:22:39]     process.rs - process::Stdio::from (line 1054)
[01:22:39]     process.rs - process::Stdio::from (line 1114)
[01:22:39]     process.rs - process::Stdio::from (line 1114)
[01:22:39] 
[01:22:39] test result: FAILED. 935 passed; 2 failed; 24 ignored; 0 measured; 0 filtered out
[01:22:39] 
[01:22:39] error: test failed, to rerun pass '--doc'
[01:22:39] 
[01:22:39] 
[01:22:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:22:39] 
[01:22:39] 
[01:22:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:22:39] Build completed unsuccessfully in 0:37:51
[01:22:39] Build completed unsuccessfully in 0:37:51
[01:22:39] make: *** [check] Error 1
[01:22:39] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:031cf3db
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:37ef1f00:start=1539974290771851504,finish=1539974290777398890,duration=5547386
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00fbf3fb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|
