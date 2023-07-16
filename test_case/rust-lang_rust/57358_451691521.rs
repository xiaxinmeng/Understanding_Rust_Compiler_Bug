plain
travis_time:end:01237960:start=1546718688161057070,finish=1546718689148061788,duration=987004718
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:05] 
[01:11:05] running 118 tests
[01:11:30] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:11:35] ......iii.i.....ii
[01:11:35] 
[01:11:35]  finished in 30.024
[01:11:35] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:59] 
[01:16:59] running 288 tests
[01:18:05] ..........................i............FF......F.FF......................F........F................. 100/288
[01:19:00] .....................................i...................................F.......................... 200/288
 to get path ".//h1"
[01:19:50] ------------------------------------------
[01:19:50] stderr:
[01:19:50] ------------------------------------------
[01:19:50] Traceback (most recent call last):
[01:19:50] Traceback (most recent call last):
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 455, in <module>
[01:19:50]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 448, in check
[01:19:50]     check_command(c, cache)
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 406, in check_command
[01:19:50]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 348, in check_tree_text
[01:19:50]     value = flatten(e)
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 249, in flatten
[01:19:50]     return ''.join(acc)
[01:19:50] UnicodeDecodeError: 'ascii' codec can't decode byte 0xe2 in position 0: ordinal not in range(128)
[01:19:50] ------------------------------------------
[01:19:50] 
[01:19:50] thread '[rustdoc] rustdoc/inline_cross/renamed-via-module.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:19:50] 
[01:19:50] 
[01:19:50] ---- [rustdoc] rustdoc/issue-42760.rs stdout ----
[01:19:50] 
[01:19:50] error: htmldocck failed!
[01:19:50] status: exit code: 1
[01:19:50] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-42760" "/checkout/src/test/rustdoc/issue-42760.rs"
[01:19:50] ------------------------------------------
[01:19:50] ------------------------------------------
[01:19:50] Failed to get path "./--------------------------
[01:19:50] ------------------------------------------
[01:19:50] Traceback (most recent call last):
[01:19:50] Traceback (most recent call last):
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 455, in <module>
[01:19:50]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 448, in check
[01:19:50]     check_command(c, cache)
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 406, in check_command
[01:19:50]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 348, in check_tree_text
[01:19:50]     value = flatten(e)
[01:19:50]   File "/checkout/src/etc/htmldocck.py", line 249, in flatten
[01:19:50]     return ''.join(acc)
[01:19:50] UnicodeDecodeError: 'ascii' codec can't decode byte 0xe2 in position 0: ordinal not in range(128)
[01:19:50] ------------------------------------------
[01:19:50] 
[01:19:50] thread '[rustdoc] rustdoc/redirect.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:19:50] 
[01:19:50] 
[01:19:50] ---- [rustdoc] rustdoc/titles.rs stdout ----
[01:19:50] 
[01:19:50] error: htmldocck failed!
[01:19:50] status: exit code: 1
[01:19:50] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/titles" "/checkout/src/test/rustdoc/titles.rs"
[01:19:50] ------------------------------------------
[01:19:50] ------------------------------------------
[01:19:50] Failed to get path ".//h1"
[01:19:50] ------------------------------------------
[01:19:50] stderr:
[01:19debug/bootstrap test
[01:19:50] Build completed unsuccessfully in 0:20:34
[01:19:50] Build completed unsuccessfully in 0:20:34
[01:19:50] make: *** [check] Error 1
[01:19:50] Makefile:48: recipe for target 'check' failed
travis_time:end:1e562f23:start=1546723490192242950,finish=1546723490343266287,duration=151023337
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
---
travis_time:end:16931d86:start=1546723491545690922,finish=1546723491550322404,duration=4631482
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03071950
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travi
