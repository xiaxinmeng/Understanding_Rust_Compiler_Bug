plain
[00:45:42] ......................................................................i.............................
[00:45:46] ....................................................................................................
[00:45:52] ....................................................................................................
[00:45:59] ...................................................................................................i
[00:46:02] .................iiiiiiiii...................................................
[00:46:02] 
[00:46:02] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:52] ......................................................................i.............................
[00:46:57] ....................................................................................................
[00:47:02] ....................................................................................................
[00:47:08] ...................................................................................................i
[00:47:11] .................iiiiiiiii...................................................
[00:47:11] 
[00:47:11]  finished in 68.618
[00:47:11] travis_fold:end:test_ui_nll

---
[01:24:19] travis_fold:end:stage0-linkchecker

[01:24:19] travis_time:end:stage0-linkchecker:start=1527689629492990992,finish=1527689632262114331,duration=2769123339

[01:24:27] std/cell/struct.Cell.html:484: broken link fragment `#method.sort_by_key` pointing to `std/cell/struct.Cell.html`
[01:24:27] std/cell/struct.Cell.html:551: broken link fragment `#method.make_ascii_uppercase` pointing to `std/cell/struct.Cell.html`
[01:24:27] std/cell/struct.Cell.html:556: broken link fragment `#method.make_ascii_lowercase` pointing to `std/cell/struct.Cell.html`
[01:24:36] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:24:36] 
[01:24:36] 
[01:24:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:24:36] expected success, got: exit code: 101
[01:24:36] expected success, got: exit code: 101
[01:24:36] 
[01:24:36] 
[01:24:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:36] Build completed unsuccessfully in 0:41:17
[01:24:36] make: *** [check] Error 1
[01:24:36] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14487498
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151260 ./.git/modules
151256 ./.git/modules/src
149116 ./src/llvm-emscripten/test
123156 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123152 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1ionthtlz-1dg93zr-1r74c9muuiiqu
121292 ./obj/build/x86_64-unknown-linu788 ./src/llvm/lib
65412 ./src/llvm-emscripten/test/CodeGen
63888 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
63728 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools
