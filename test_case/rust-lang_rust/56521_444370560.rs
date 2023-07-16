plain
[01:06:08] 
[01:06:08] failures:
[01:06:08] 
[01:06:08] ---- collections/btree/map.rs - collections::btree::map::BTreeMap (line 69) stdout ----
[01:06:08] error[E0425]: cannot find value `book_reviews` in this scope
[01:06:08]   --> collections/btree/map.rs:101:33
[01:06:08]    |
[01:06:08] 35 | println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
[01:06:08]    |                                 ^^^^^^^^^^^^ did you mean `movie_reviews`?
[01:06:08] thread 'collections/btree/map.rs - collections::btree::map::BTreeMap (line 69)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:323:13
[01:06:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:06:08] 
[01:06:08] 
---
[01:06:08] 
[01:06:08] error: test failed, to rerun pass '--doc'
[01:06:08] 
[01:06:08] 
[01:06:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--"
[01:06:08] 
[01:06:08] 
[01:06:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:08] Build completed unsuccessfully in 0:21:10
[01:06:08] Build completed unsuccessfully in 0:21:10
[01:06:08] Makefile:58: recipe for target 'check' failed
[01:06:08] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a22981c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec  5 06:07:19 UTC 2018
---
travis_time:end:22905f98:start=1543990041946817262,finish=1543990041956087973,duration=9270711
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0de04eba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b490950
travis_time:start:0b490950
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0767ed06
$ dmesg | grep -i kill
