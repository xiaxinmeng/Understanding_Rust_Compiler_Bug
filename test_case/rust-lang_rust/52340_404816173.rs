plain
[01:19:33] travis_fold:end:stage0-linkchecker

[01:19:33] travis_time:end:stage0-linkchecker:start=1531484156982743671,finish=1531484159599167196,duration=2616423525

[01:19:36] std/io/struct.Error.html:190: broken link - std/io/struct.NulError.html
[01:19:37] std/sync/struct.Arc.html:399: broken link - std/sync/struct.CString.html
[01:19:37] std/sync/struct.Arc.html:401: broken link - std/sync/struct.OsString.html
[01:19:39] std/boxed/struct.Box.html:297: broken link - std/boxed/struct.CString.html
[01:19:39] std/boxed/struct.Box.html:300: broken link - std/boxed/struct.CString.html
[01:19:39] std/boxed/struct.Box.html:304: broken link - std/boxed/struct.OsString.html
[01:19:42] std/rc/struct.Rc.html:289: broken link - std/rc/struct.CString.html
[01:19:42] std/rc/struct.Rc.html:291: broken link - std/rc/struct.OsString.html
[01:19:42] std/vec/struct.Vec.html:1931: broken link - std/vec/struct.CString.html
[01:19:42] std/vec/struct.Vec.html:1932: broken link - std/vec/struct.CString.html
[01:19:54] thread 'main' panicked at 'found some broken links', tools/linkchecker/main.rs:49:9
[01:19:54] 
[01:19:54] 
[01:19:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:19:54] expected success, got: exit code: 101
[01:19:54] expected success, got: exit code: 101
[01:19:54] 
[01:19:54] 
[01:19:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:54] Build completed unsuccessfully in 0:32:39
[01:19:54] make: *** [check] Error 1
[01:19:54] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:063e39b7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
