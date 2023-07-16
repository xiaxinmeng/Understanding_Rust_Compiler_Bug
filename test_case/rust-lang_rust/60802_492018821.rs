plain
travis_time:end:0128d6de:start=1557782604023451807,finish=1557782604857405369,duration=833953562
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:23] 
[01:23:23] running 143 tests
[01:23:26] i..iii.....iii...iiii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/143
[01:23:28] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:23:28] 
[01:23:28]  finished in 4.553
[01:23:28] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:30] 
[01:23:30] running 9 tests
[01:23:30] iiiiiiiii
[01:23:30] 
[01:23:30]  finished in 0.152
[01:23:30] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:23:45] 
[01:23:45] running 122 tests
[01:24:10] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:24:15] .i.i......iii.i.....ii
[01:24:15] 
[01:24:15]  finished in 29.532
[01:24:15] travis_fold:end:test_debuginfo

---
[01:45:30]     Finished release [optimized] target(s) in 1m 03s
[01:45:30]      Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/rustdoc-673b6a025d1a02b5
[01:45:30] 
[01:45:30] running 42 tests
[01:45:30] ............F.............................
[01:45:30] 
[01:45:30] ---- html::markdown::tests::test_header stdout ----
[01:45:30] thread 'html::markdown::tests::test_header' panicked at 'assertion failed: `(left == right)`
[01:45:30] thread 'html::markdown::tests::test_header' panicked at 'assertion failed: `(left == right)`
[01:45:30]   left: `"<h4 id=\"foo--bar----qux\" class=\"section-header\"><a href=\"#foo--bar----qux\"><strong>Foo?</strong> &amp; *bar?!*  <em><code>baz</code></em> ❤ #qux</a></h4>"`,
[01:45:30]  right: `"<h4 id=\"foo--bar--baz--qux\" class=\"section-header\"><a href=\"#foo--bar--baz--qux\"><strong>Foo?</strong> &amp; *bar?!*  <em><code>baz</code></em> ❤ #qux</a></h4>"`: original: #### **Foo?** & \*bar?!*  _`baz`_ ❤ #qux', src/librustdoc/html/markdown.rs:1100:13
[01:45:30] 
[01:45:30] 
[01:45:30] failures:
[01:45:30]     html::markdown::tests::test_header
[01:45:30]     html::markdown::tests::test_header
[01:45:30] 
[01:45:30] test result: FAILED. 41 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:45:30] 
[01:45:30] error: test failed, to rerun pass '--lib'
[01:45:30] 
[01:45:30] 
[01:45:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"
[01:45:30] 
[01:45:30] 
[01:45:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:30] Build completed unsuccessfully in 0:33:51
[01:45:30] Build completed unsuccessfully in 0:33:51
[01:45:30] make: *** [check] Error 1
[01:45:30] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0890cc6c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon May 13 23:09:04 UTC 2019
---
travis_time:end:0cd9132a:start=1557788946786115900,finish=1557788946790922809,duration=4806909
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b7f74aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09e63e20
travis_time:start:09e63e20
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:018e5630
$ dmesg | grep -i kill
