plain
[00:49:37] .................................................................................................... 2200/4658
[00:49:41] ....................................i............................................................... 2300/4658
[00:49:45] .................................................................................................... 2400/4658
[00:49:48] .................................................................................................... 2500/4658
[00:49:52] ...................................................iiiiiiiii........................................ 2600/4658
[00:49:58] .................................................................................................... 2800/4658
[00:50:02] .................................................................................................... 2900/4658
[00:50:05] .................................................................................i.................. 3000/4658
[00:50:08] .................................................................................................... 3100/4658
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:50] 
[01:02:50] running 111 tests
[01:02:53] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i.i.ii..............i...ii..ii.i.. 100/111
[01:02:53] ..iiii.....
[01:02:53] 
[01:02:53]  finished in 3.434
[01:02:53] travis_fold:end:test_codegen

---
[01:04:11] 25 
[01:04:11] 
[01:04:11] 
[01:04:11] The actual stderr differed from the expected stderr.
[01:04:11] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/rust-2018/suggestions-not-always-applicable/suggestions-not-always-applicable.stderr
[01:04:11] To update references, rerun the tests and pass the `--bless` flag
[01:04:11] To only update this specific test, also pass `--test-args rust-2018/suggestions-not-always-applicable.rs`
[01:04:11] error: 1 errors occurred comparing output.
[01:04:11] status: exit code: 0
[01:04:11] status: exit code: 0
[01:04:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/rust-2018/suggestions-not-always-applicable.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/rust-2018/suggestions-not-always-applicable/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/rust-2018/suggestions-not-always-applicable/auxiliary" "-A" "unused"
[01:04:11] ------------------------------------------
[01:04:11] 
[01:04:11] ------------------------------------------
[01:04:11] stderr:
