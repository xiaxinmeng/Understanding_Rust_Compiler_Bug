plain
[00:48:02] ....................................................................................................
[00:48:06] ...................................................................................................i
[00:48:08] ....................................................................................................
[00:48:12] ....................................................................................................
[00:48:14] ................................................iiiiiiiii...........................................
[00:48:20] ....................................................................................................
[00:48:24] ....................................................................................................
[00:48:26] .............................i......................................................................
[00:48:29] ...............................................................................i.i..ii..............
---
[01:14:25] 
[01:14:25] To learn more, run the command again with --verbose.
[01:14:25] 
[01:14:25] 
[01:14:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:14:25] 
[01:14:25] 
[01:14:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:25] Build completed unsuccessfully in 0:30:30
[01:14:25] Build completed unsuccessfully in 0:30:30
[01:14:25] make: *** [check] Error 1
[01:14:25] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b24645b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0785a1e0:start=1534617783578860227,finish=1534617783664993812,duration=86133585
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:014d11fa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11b57994
$ dmesg | grep -i kill
