plain
travis_time:end:22bee6e6:start=1546734024474019577,finish=1546734094647375798,duration=70173356221
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
[01:07:56] 
[01:07:56] running 118 tests
[01:08:20] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:08:24] ......iii.i.....ii
[01:08:24] 
[01:08:24]  finished in 28.618
[01:08:24] travis_fold:end:test_debuginfo

---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:37] 
[01:13:37] running 289 tests
[01:14:40] ....F.....................i..F..F.......FF......F.FF..............F.......F.......F..F..........F... 100/289
[01:15:32] ...F..................F.F.............i...............F...................F......................... 200/289
[01:16:20] ..............................................F.............F............F.......F.....F.
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/assoc-types.rs stdout ----
[01:16:20] 
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-types" "/checkout/src/test/rustdoc/assoc-types.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//*[@class="rust fn"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\xa0' in position 20: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/assoc-types.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:16:20] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/doc-assoc-item.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-assoc-item" "/checkout/src/test/rustdoc/doc-assoc-item.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//*[@class="impl"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\xa0' in position 7: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/doc-assoc-item.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/doc-spotlight.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-spotlight" "/checkout/src/test/rustdoc/doc-spotlight.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//code[@class="content"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\xa0' in position 7: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/doc-spotlight.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/empty-mod-private.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-mod-private" "/checkout/src/test/rustdoc/empty-mod-private.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//h1"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------ck (most recent call last):
[01:16:20] ------------------ck (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 253, in flatten
[01:16:20]     return ''.join(acc)
[01:16:20] UnicodeDecodeError: 'ascii' codec can't decode byte 0xe2 in position 0: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/extern-links.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/external-doc.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/external-doc" "/checkout/src/test/rustdoc/external-doc.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//h1"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 253, in flatten
[01:16:20]     return ''.join(acc)
[01:16:20] UnicodeDecodeError: 'ascii' codec can't decode byte 0xe2 in position 0: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/external-doc.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/external-cross.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/external-cross" "/checkout/src/test/rustdoc/external-cross.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//h1"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/k.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\xa0' in position 7: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/impl-parts.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/inline_cross/issue-28480.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-28480" "/checkout/src/test/rustdoc/inline_cross/issue-28480.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//a"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 253, in flatten
[01:16:20]     return ''.join(acc)
[01:16:20] UnicodeDecodeError: 'ascii' codec can't decode byte 0xe2 in position 0: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/inline_cross/issue-28480.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/inline_cross/macros.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/macros" "/checkout/src/test/rustdoc/inline_cross/macros.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//*[@class="stab unstable"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\U0001f52c' in position 0: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/inline_cross/macros.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/inline_cross/renamed-via-module.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/renamed-via-module" "/checkout/src/test/rustdoc/inline_cross/renamed-via-module.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//h1"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 253, in flatten
[01:16:20]     return ''.join(acc)
[01:16:20] UnicodeDecodeError: 'ascii' codec can't decode byte 0xe2 in position 0: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/inline_cross/renamed-via-module.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/internal.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/internal" "/checkout/src/test/rustdoc/internal.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//*[@class="stab internal"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode characters in position 0-1: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/internal.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/issue-12834.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-12834" "/checkout/src/test/rustdoc/issue-12834.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//pre"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\u2208' in position 6: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/issue-12834.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/issue-20727-2.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2" "/checkout/src/test/rustdoc/issue-20727-2.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//*[@class="rust trait"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459="rust trait"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\xa0' in position 20: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/issue-20727-4.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/issue-32374.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-32374" "/checkout/src/test/rustdoc/issue-32374.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//*[@class="stab unstable"]"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 252, in flatten
[01:16:20]     _flatten(node, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 245, in _flatten
[01:16:20]     _flatten(e, acc)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 240, in _flatten
[01:16:20]     text = node.text.decode('utf8')
[01:16:20]   File "/usr/lib/python2.7/encodings/utf_8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\U0001f52c' in position 0: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/issue-32374.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/issue-42760.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-42760" "/checkout/src/test/rustdoc/issue-42760.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//h1"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 459, in <module>
[01:16:20]     check(sys.argv[1], get_commands(sys.argv[2]))
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 452, in check
[01:16:20]     check_command(c, cache)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 410, in check_command
[01:16:20]     ret = check_tree_text(cache.get_tree(c.args[0]), pat, c.args[2], regexp)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 352, in check_tree_text
[01:16:20]     value = flatten(e)
[01:16:20]   File "/checkout/src/etc/htmldocck.py", line 253, in flatten
[01:16:20]     return ''.join(acc)
[01:16:20] UnicodeDecodeError: 'ascii' codec can't decode byte 0xe2 in position 0: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:8.py", line 16, in decode
[01:16:8.py", line 16, in decode
[01:16:20]     return codecs.utf_8_decode(input, errors, True)
[01:16:20] UnicodeEncodeError: 'ascii' codec can't encode character u'\xa0' in position 14: ordinal not in range(128)
[01:16:20] ------------------------------------------
[01:16:20] 
[01:16:20] thread '[rustdoc] rustdoc/synthetic_auto/complex.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:16:20] 
[01:16:20] 
[01:16:20] ---- [rustdoc] rustdoc/titles.rs stdout ----
[01:16:20] 
[01:16:20] error: htmldocck failed!
[01:16:20] status: exit code: 1
[01:16:20] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/titles" "/checkout/src/test/rustdoc/titles.rs"
[01:16:20] ------------------------------------------
[01:16:20] ------------------------------------------
[01:16:20] Failed to get path ".//h1"
[01:16:20] ------------------------------------------
[01:16:20] stderr:
[01:16:20] ------------------------------------------
[01:16:20] Traceback (most recent call last):
[01:16:20] Traceback (most recent call last):
---
[01:16:20] test result: FAILED. 264 passed; 23 failed; 2 ignored; 0 measured; 0 filtered out
[01:16:20] 
[01:16:20] 
[01:16:20] 
[01:16:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:20] 
[01:16:20] 
[01:16:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:20] Build completed unsuccessfully in 0:19:48
[01:16:20] Build completed unsuccessfully in 0:19:48
[01:16:20] make: *** [check] Error 1
[01:16:20] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07201320
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan  6 01:38:04 UTC 2019
