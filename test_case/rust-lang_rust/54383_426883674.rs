plain
[00:49:17] ...............................................................................................i.... 2200/4549
[00:49:21] .................................................................................................... 2300/4549
[00:49:25] .................................................................................................... 2400/4549
[00:49:28] .................................................................................................... 2500/4549
[00:49:32] .......iiiiiiiii.................................................................................... 2600/4549
[00:49:38] .................................................................................................... 2800/4549
[00:49:42] .................................................................................................... 2900/4549
[00:49:45] ..........................i......................................................................... 3000/4549
[00:49:47] ......................................................................................i.i..ii....... 3100/4549
---
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:34] 
[00:50:34] running 2874 tests
[00:50:45] ...........F........................................................................................ 100/2874
[00:51:03] .................................................................................................... 300/2874
[00:51:13] .................................................................................................... 400/2874
[00:51:21] .................................................................................................... 500/2874
[00:51:33] .................................................................................................... 600/2874
---
[00:55:30] .................................................................................................... 2500/2874
[00:55:56] .................................................................................................... 2600/2874
[00:56:05] .................................................................................................... 2700/2874
[00:56:13] .................................................................................................... 2800/2874
56:23] To update references, rerun the tests and pass the `--bless` flag
[00:56:23] To only update this specific test, also pass `--test-args arbitrary_self_types_pointers_and_wrappers.rs`
[00:56:23] error: 1 errors occurred comparing output.
[00:56:23] status: exit code: 0
[00:56:23] status: exit code: 0
[00:56:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/arbitrary_self_types_pointers_and_wrappers.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_pointers_and_wrappers/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/arbitrary_self_types_pointers_and_wrappers/auxiliary"
[00:56:23] ------------------------------------------
[00:56:23] 
[00:56:23] ------------------------------------------
[00:56:23] stderr:
[00:56:23] stderr:
[00:56:23] ------------------------------------------
[00:56:23] {"message":"unused import: `fmt::Debug`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/arbitrary_self_types_pointers_and_wrappers.rs","byte_start":645,"byte_end":655,"line_start":16,"line_end":16,"column_start":5,"column_end":15,"is_primary":true,"text":[{"text":"    fmt::Debug,","highlight_start":5,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused/bootstrap-3ivyub3ic2113/s-f5eakr5kqq-ffhwli-8vudgacockuw
134192 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
131276 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
130808 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130804 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
