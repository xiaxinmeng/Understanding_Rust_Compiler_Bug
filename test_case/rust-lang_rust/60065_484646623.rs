plain
travis_time:end:24491c34:start=1555608305233132455,finish=1555608407798142737,duration=102565010282
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
[01:13:41] 
[01:13:41] running 9 tests
[01:13:41] iiiiiiiii
[01:13:41] 
[01:13:41]  finished in 0.151
[01:13:41] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:57] 
[01:13:57] running 121 tests
[01:14:23] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:27] i.i......iii.i.....ii
[01:14:27] 
[01:14:27]  finished in 29.862
[01:14:27] travis_fold:end:test_debuginfo

---
[01:41:38] ---- html/markdown.rs - html::markdown (line 8) stdout ----
[01:41:38] error[E0061]: this function takes 5 parameters but 4 parameters were supplied
[01:41:38]   --> html/markdown.rs:16:26
[01:41:38]    |
[01:41:38] 11 | let html = format!("{}", Markdown(s, &[], RefCell::new(&mut id_map), ErrorCodes::Yes));
[01:41:38] 
[01:41:38] error: aborting due to previous error
[01:41:38] 
[01:41:38] For more information about this error, try `rustc --explain E0061`.
---
[01:41:38] 
[01:41:38] error: test failed, to rerun pass '--doc'
[01:41:38] 
[01:41:38] 
[01:41:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:41:38] 
[01:41:38] 
[01:41:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:41:38] Build completed unsuccessfully in 0:39:41
[01:41:38] Build completed unsuccessfully in 0:39:41
[01:41:38] make: *** [check] Error 1
[01:41:38] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f4d3658
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr 18 19:08:36 UTC 2019
---
travis_time:end:081672a8:start=1555614517892252536,finish=1555614517896998151,duration=4745615
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1116a866
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:067311c6
travis_time:start:067311c6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:34d806ec
$ dmesg | grep -i kill
