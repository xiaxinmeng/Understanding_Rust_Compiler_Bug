plain
travis_time:start:07eefcc6
$ git fetch origin +refs/pull/55798/merge:
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    94% |██████████████████████████████▏ | 4.8MB 4.7MB/s eta 0:00:01
    94% |██████████████████████████████▎ | 4.8MB 4.7MB/s eta 0:00:01
    94% |██████████████████████████████▎ | 4.8MB 3.2MB/s eta 0:00:01
    94% |██████████████████████████████▍ | 4.9MB 4.7MB/s eta 0:00:01
    95% |██████████████████████████████▍ | 4.9B/s eta 0:00:01████████████████████████████▊| 5.1MB 79.6MB/s eta 0:00:01
    99% |████████████████████████████████| 5.1MB 81.4MB/s eta 0:00:01
    99% |████████████████████████████████| 5.1MB 81.4MB/s eta 0:00:01MB 4.7MB/s eta 0:00:01
    95% |██████████████████████████████▌ | 4.9MB 4.7MB/s eta 0:00:01
    95% |██████████████████████████████▌ | 4.9MB 4.7MB/s eta 0:00:01
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:36] 
[00:51:36] running 121 tests
[00:51:39] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:39] i..ii.i.....iiii.....
[00:51:39] 
[00:51:39]  finished in 3.337
[00:51:39] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:54] 
[00:51:54] running 119 tests
[00:52:16] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:52:20] i......iii.i.....ii
[00:52:20] 
[00:52:20]  finished in 26.483
[00:52:20] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:04] 
[00:57:04] running 281 tests
[00:58:09] .FF...........F........i................F.F.......................F..........F...................... 100/281
[00:59:03] .............F.......F..F........i....F................F............................................ 200/281
[00:59:47] ......................F..........................................................
[00:59:47] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
[00:59:47] 
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'assoc_consts/trait.Foo.html': multiple elements on top level
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/assoc-consts.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/assoc-types.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] 18: @has check failed
[00:59:47]  `XPATH PATTERN` did not match
[00:59:47]      // @has - '//*[@id="Output.t"]//code' 'type Output: ?Sized'
[00:59:47] 20: @has check failed
[00:59:47]  `XPATH PATTERN` did not match
[00:59:47]      // @has - '//*[@id="index.v"]//code' 'fn index'
[00:59:47] Encountered 2 errors
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/assoc-types.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/const-doc.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-doc" "/checkout/src/test/rustdoc/const-doc.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'const_doc/struct.ContentType.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/const-doc.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/extern-default-method.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-default-method" "/checkout/src/test/rustdoc/extern-default-method.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 419, in check_command
[00:59:47]     found = get_tree_count(cache.get_tree(c.args[0]), c.args[1])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'extern_default_method/struct.Struct.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/extern-default-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/extern-impl-trait.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl-trait" "/checkout/src/test/rustdoc/extern-impl-trait.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'foo/struct.X.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/extern-impl-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/assoc-items.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'foo/struct.MyStruct.html': multiple elements on top level
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/inline_cross/assoc-items.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/inline_cross/trait-vis.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/trait-vis" "/checkout/src/test/rustdoc/inline_cross/trait-vis.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'trait_vis/struct.SomeStruct.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/inline_cross/trait-vis.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3" "/checkout/src/test/rustdoc/issue-19190-3.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'issue_19190_3/struct.Foo.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/issue-19190-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/issue-21092.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21092" "/checkout/src/test/rustdoc/issue-21092.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'issue_21092/struct.Bar.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/issue-21092.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/issue-21801.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-21801" "/checkout/src/test/rustdoc/issue-21801.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'issue_21801/struct.Foo.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/issue-21801.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/issue-28478.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28478" "/checkout/src/test/rustdoc/issue-28478.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'issue_28478/trait.Bar.html': pop from empty stack
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/issue-28478.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/issue-33302.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302" "/checkout/src/test/rustdoc/issue-33302.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'issue_33302/trait.T.html': multiple elements on top level
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/issue-33302.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
[00:59:47] 
[00:59:47] ---- [rustdoc] rustdoc/primitive-generic-impl.rs stdout ----
[00:59:47] 
[00:59:47] error: htmldocck failed!
[00:59:47] status: exit code: 1
[00:59:47] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-generic-impl" "/checkout/src/test/rustdoc/primitive-generic-impl.rs"
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] ------------------------------------------
[00:59:47] stderr:
[00:59:47] stderr:
[00:59:47] ------------------------------------------
[00:59:47] Traceback (most recent call last):
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 461, in <module>
[00:59:47]     check(sys.argv[1], get_commands(sys.argv[2]))
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 454, in check
[00:59:47]     check_command(c, cache)
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 403, in check_command
[00:59:47]     tree = cache.get_tree(c.args[0])
[00:59:47]   File "/checkout/src/etc/htmldocck.py", line 311, in get_tree
[00:59:47]     raise RuntimeError('Cannot parse an HTML file {!r}: {}'.format(path, e))
[00:59:47] RuntimeError: Cannot parse an HTML file 'foo/primitive.i32.html': multiple elements on top level
[00:59:47] ------------------------------------------
[00:59:47] 
[00:59:47] thread '[rustdoc] rustdoc/primitive-generic-impl.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[00:59:47] 
---
[00:59:47] test result: FAILED. 266 passed; 13 failed; 2 ignored; 0 measured; 0 filtered out
[00:59:47] 
[00:59:47] 
[00:59:47] 
[00:59:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:47] 
[00:59:47] 
[00:59:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:47] Build completed unsuccessfully in 0:18:19
[00:59:47] Build completed unsuccessfully in 0:18:19
[00:59:47] Makefile:58: recipe for target 'check' failed
[00:59:47] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:021e7f08
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Dec 16 21:59:38 UTC 2018
---
travis_time:end:03c3c384:start=1544997579544352599,finish=1544997579549191734,duration=4839135
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0be0d370
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15d70410
travis_time:start:15d70410
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16d9a4ce
$ dmesg | grep -i kill
