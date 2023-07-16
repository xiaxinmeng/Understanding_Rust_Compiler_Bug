
[01:27:14] ---- [rustdoc] rustdoc/synthetic_auto/no-redundancy.rs stdout ----
[01:27:14] 	
[01:27:14] error: htmldocck failed!
[01:27:14] status: exit code: 1
[01:27:14] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/synthetic_auto/no-redundancy.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/synthetic_auto/no-redundancy.rs"
[01:27:14] stdout:
[01:27:14] ------------------------------------------
[01:27:14] 
[01:27:14] ------------------------------------------
[01:27:14] stderr:
[01:27:14] ------------------------------------------
[01:27:14] 22: @has check failed
[01:27:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:478:22
[01:27:14] 	`XPATH PATTERN` did not match
[01:27:14] 	// @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]/*/code' "impl<T> Send for Outer<T> where T: Copy + Send"
[01:27:14] 
[01:27:14] Encountered 1 errors
[01:27:14] 
[01:27:14] ------------------------------------------
[01:27:14] 
[01:27:14] thread '[rustdoc] rustdoc/synthetic_auto/no-redundancy.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[01:27:14] 
[01:27:14] 
[01:27:14] failures:
[01:27:14]     [rustdoc] rustdoc/synthetic_auto/no-redundancy.rs
