plain
[00:44:04] 
[00:44:04] warning: `[atomic::compiler_fence]` cannot be resolved, ignoring it...
[00:44:04]    --> libstd/sync/mod.rs:101:24
[00:44:04]     |
[00:44:04] 101 | //! [compiler fences]: atomic::compiler_fence
[00:44:04]     |
[00:44:04]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:44:04] 
[00:44:04] warning: `[atomic::fence]` cannot be resolved, ignoring it...
[00:44:04] warning: `[atomic::fence]` cannot be resolved, ignoring it...
[00:44:04]    --> libstd/sync/mod.rs:104:22
[00:44:04]     |
[00:44:04] 104 | //! [memory fences]: atomic::fence
[00:44:04]     |
[00:44:04]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:44:04] 
[00:44:04] 
[00:44:04] warning: `[mpsc]` cannot be resolved, ignoring it...
[00:44:04]    --> libstd/sync/mod.rs:133:17
[00:44:04]     |
[00:44:04] 133 | //! [channels]: mpsc
[00:44:04]     |
[00:44:04]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:44:04] 
[00:44:04] warning: `[Mutex]` cannot be resolved, ignoring it...
[00:44:04] warning: `[Mutex]` cannot be resolved, ignoring it...
[00:44:04]    --> libstd/sync/mod.rs:122:6
[00:44:04]     |
[00:44:04] 122 | //! [`Mutex`] can involve blocking until another thread releases it.
[00:44:04]     |
[00:44:04]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:44:04] 
[00:44:04] warning: `[RwLock]` cannot be resolved, ignoring it...
[00:44:04] warning: `[RwLock]` cannot be resolved, ignoring it...
[00:44:04]    --> libstd/sync/mod.rs:123:10
[00:44:04]     |
[00:44:04] 123 | //! For [`RwLock`], while! any number of readers may acquire it without
[00:44:04]     |
[00:44:04]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:44:04] 
[00:44:04] warning: `[Weak::upgrade]` cannot be resolved, ignoring it...
---
[00:53:16] ....................................................................................................
[00:53:19] ...............................................................i....................................
[00:53:22] ....................................................................................................
[00:53:25] ....................................................................................................
[00:53:28] ............iiiiiiiii...............................................................................
[00:53:33] ....................................................................................................
[00:53:37] ................................................................................................i...
[00:53:39] ....................................................................................................
[00:53:42] ........................................................i.i..ii.....................................
---
[01:23:23] travis_fold:end:stage0-linkchecker

[01:23:23] travis_time:end:stage0-linkchecker:start=1538074295393550818,finish=1538074297751386313,duration=2357835495

[01:25:29] std/sync/index.html:65: broken link - std/sync/atomic::compiler_fence
[01:25:29] std/sync/index.html:79: broken link - std/sync/atomic::fence
[01:25:29] std/sync/index.html:100: directory link - std/sync/mpsc
[01:26:15] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:26:15] 
[01:26:15] 
[01:26:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:26:15] expected success, got: exit code: 101
[01:26:15] expected success, got: exit code: 101
[01:26:15] 
[01:26:15] 
[01:26:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:26:15] Build completed unsuccessfully in 0:41:25
[01:26:15] Makefile:58: recipe for target 'check' failed
[01:26:15] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02446757
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
