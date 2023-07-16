plain
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:40] 
[00:56:40] running 246 tests
[00:57:08] ...........F.........i..............................................................................
[00:57:29] ..................i.................................................................................
[00:57:36] ....................F.........................
[00:57:36] 
[00:57:36] ---- [rustdoc] rustdoc/auto-impl-primitive.rs stdout ----
[00:57:36] 
[00:57:36] 
[00:57:36] error: htmldocck failed!
[00:57:36] status: exit code: 1
[00:57:36] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/auto-impl-primitive" "/checkout/src/test/rustdoc/auto-impl-primitive.rs"
[00:57:36] ------------------------------------------
[00:57:36] 
[00:57:36] ------------------------------------------
[00:57:36] stderr:
[00:57:36] stderr:
[00:57:36] ------------------------------------------
[00:57:36] 14: @has check failed
[00:57:36]  `XPATH PATTERN` did not match
[00:57:36]  // @has 'foo/primitive.i16.html' '//h2[@id="synthetic-implementations"]' 'Auto Trait Implementation'
[00:57:36] Encountered 1 errors
[00:57:36] 
[00:57:36] ------------------------------------------
[00:57:36] 
