plain
[01:08:51] .............................................................................ii.....................
[01:09:48] .........................................i...............................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[01:09:57] .....................i.ii..
[01:10:50] ....................................................................................................
[01:11:18] ..iiiiiii...........................................................................................
[01:12:24] ....................................................................................................
[01:12:51] ...........................................................................
[01:12:51] test result: ok. 2956 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[01:12:51] 
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:26:05] 
[01:26:05] running 231 tests
[01:28:29] ...................i...................................F.....................................FFF.F..
[01:30:21] ...............................
[01:30:21] failures:
[01:30:21] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:30:21] 
[01:30:21] 
[01:30:21] ---- [rustdoc] rustdoc/inline-default-methods.rs stdout ----
[01:30:21]  
[01:30:21] error: htmldocck failed!
[01:30:21] status: exit code: 1
[01:30:21] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline-default-methods.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/inline-default-methods.rs"
[01:30:21] ------------------------------------------
[01:30:21] 
[01:30:21] ------------------------------------------
[01:30:21] stderr:
[01:30:21] stderr:
[01:30:21] ------------------------------------------
[01:30:21] 17: @has check failed
[01:30:21]  `XPATH PATTERN` did not match
[01:30:21]  // @has - '//*[@class="rust trait"]' 'fn bar(&self);'
[01:30:21] 18: @has check failed
[01:30:21]  `XPATH PATTERN` did not match
[01:30:21]  // @has - '//*[@class="rust trait"]' 'fn foo(&mut self) { ... }'
[01:30:21] Encountered 2 errors
[01:30:21] 
[01:30:21] ------------------------------------------
[01:30:21] 
[01:30:21] 
[01:30:21] thread '[rustdoc] rustdoc/inline-default-methods.rs' panicked at 'explicit panic---------------------------
[01:30:21] 32: @has check failed
[01:30:21]  `XPATH PATTERN` did not match
[01:30:21]      // @has - '//*[@class="rust trait"]' 'fn deref(&self) -> Self::Target;'
[01:30:21] Encountered 1 errors
[01:30:21] 
[01:30:21] ------------------------------------------
[01:30:21] 
[01:30:21] 
[01:30:21] thread '[rustdoc] rustdoc/issue-20727-3.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:30:21] 
[01:30:21] ---- [rustdoc] rustdoc/issue-20727-4.rs stdout ----
[01:30:21]  
[01:30:21] error: htmldocck failed!
[01:30:21] status: exit code: 1
[01:30:21] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/issue-20727-4.rs"
[01:30:21] ------------------------------------------
[01:30:21] 
[01:30:21] ------------------------------------------
[01:30:21] stderr:
[01:30:21] stderr:
[01:30:21] ------------------------------------------
[01:30:21] 47: @has check failed
[01:30:21]  `XPATH PATTERN` did not match
[01:30:21]      // @has - '//*[@class="rust trait"]' 'fn index_mut(&mut self, index: Idx) -> &mut Self::Output;'
[01:30:21] Encountered 1 errors
[01:30:21] 
[01:30:21] ------------------------------------------
[01:30:21] 
[01:30:21] 
[01:30:21] thread '[rustdoc] rustdoc/issue-20727-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:30:21] 
[01:30:21] ---- [rustdoc] rustdoc/issue-20727.rs stdout ----
[01:30:21]  
[01:30:21] error: htmldocck failed!
