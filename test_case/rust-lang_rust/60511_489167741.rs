plain
travis_time:end:33dc25ee:start=1556896305362585116,finish=1556896307644533201,duration=2281948085
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:27:27] 
[01:27:27] running 9 tests
[01:27:27] iiiiiiiii
[01:27:27] 
[01:27:27]  finished in 0.151
[01:27:27] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:27:43] 
[01:27:43] running 121 tests
[01:28:09] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:28:13] i.i......iii.i.....ii
[01:28:13] 
[01:28:13]  finished in 30.610
[01:28:13] travis_fold:end:test_debuginfo

---
[01:50:22] travis_fold:end:stage0-linkchecker

[01:50:22] travis_time:end:stage0-linkchecker:start=1556902938686259992,finish=1556902940807284614,duration=2121024622

[01:50:25] std/boxed/struct.Box.html:362: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:362: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:388: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:388: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:420: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:430: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:439: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:449: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:458: broken link - std/boxed/trait.Error.html
[01:50:25] std/boxed/struct.Box.html:469: broken link - std/boxed/trait.Error.html
[01:50:27] std/default/trait.Default.html:100: broken link - std/default/struct.DefaultHasher.html
[01:50:27] std/borrow/enum.Cow.html:184: broken link - std/borrow/trait.Error.html
[01:50:27] std/borrow/enum.Cow.html:195: broken link - std/borrow/trait.Error.html
[01:50:27] std/convert/trait.From.html:390: broken link - std/convert/struct.SocketAddrV4.html
[01:50:27] std/convert/trait.From.html:390: broken link - std/convert/enum.SocketAddr.html
[01:50:27] std/convert/trait.From.html:391: broken link - std/convert/struct.SocketAddrV6.html
[01:50:27] std/convert/trait.From.html:391: broken link - std/convert/enum.SocketAddr.html
[01:50:27] std/convert/trait.From.html:468: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:478: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:487: broken link - std/convert/struct.OsString.html
[01:50:27] std/convert/trait.From.html:509: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:530: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:540: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:550: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:561: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:561: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:587: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:587: broken link - std/convert/trait.Error.html
[01:50:27] std/convert/trait.From.html:619: broken link - std/convert/enum.IpAddr.html
[01:50:27] std/convert/trait.From.html:619: broken link - std/convert/enum.SocketAddr.html
[01:50:27] std/convert/trait.From.html:620: broken link - std/convert/enum.SocketAddr.html
[01:50:27] std/convert/trait.From.html:620: broken link - std/convert/enum.IpAddr.html
[01:50:27] std/convert/trait.From.html:621: broken link - std/convert/enum.SocketAddr.html
[01:50:27] std/convert/trait.From.html:621: broken link - std/convert/enum.IpAddr.html
[01:50:27] std/convert/trait.From.html:622: broken link - std/convert/enum.SocketAddr.html
[01:50:27] std/convert/trait.From.html:635: broken link - std/convert/struct.Mutex.html
[01:50:27] std/convert/trait.From.html:637: broken link - std/convert/struct.RwLock.html
[01:50:30] thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:41:9
[01:50:30] 
[01:50:30] 
[01:50:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
[01:50:30] expected success, got: exit code: 101
[01:50:30] expected success, got: exit code: 101
[01:50:30] 
[01:50:30] 
[01:50:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:50:30] Build completed unsuccessfully in 0:34:45
[01:50:30] make: *** [check] Error 1
[01:50:30] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07b66e08
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May  3 17:02:29 UTC 2019
