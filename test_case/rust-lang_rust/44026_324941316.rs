
[00:53:35] ---- [rustdoc] rustdoc/doc-masked.rs stdout ----
[00:53:35] 	
[00:53:35] error: htmldocck failed!
[00:53:35] status: exit code: 1
[00:53:35] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-masked.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/doc-masked.rs"
[00:53:35] stdout:
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] ------------------------------------------
[00:53:35] stderr:
[00:53:35] ------------------------------------------
[00:53:35] 18: @!has check failed
[00:53:35] 	`XPATH PATTERN` did not match
[00:53:35] 	// @!has - '//*[@class="impl"]//code' 'impl Copy for LocalStruct'
[00:53:35] 
[00:53:35] Encountered 1 errors
[00:53:35] 
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] thread '[rustdoc] rustdoc/doc-masked.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2435:8
[00:53:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:35] 
[00:53:35] ---- [rustdoc] rustdoc/extern-impl.rs stdout ----
[00:53:35] 	
[00:53:35] error: htmldocck failed!
[00:53:35] status: exit code: 1
[00:53:35] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-impl.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/extern-impl.rs"
[00:53:35] stdout:
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] ------------------------------------------
[00:53:35] stderr:
[00:53:35] ------------------------------------------
[00:53:35] 32: @has check failed
[00:53:35] 	`XPATH PATTERN` did not match
[00:53:35] 	// @has - '//code' 'impl Bar for fn()'
[00:53:35] 34: @has check failed
[00:53:35] 	`XPATH PATTERN` did not match
[00:53:35] 	// @has - '//code' 'impl Bar for extern "C" fn()'
[00:53:35] 36: @has check failed
[00:53:35] 	`XPATH PATTERN` did not match
[00:53:35] 	// @has - '//code' 'impl Bar for extern "system" fn()'
[00:53:35] 
[00:53:35] Encountered 3 errors
[00:53:35] 
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] thread '[rustdoc] rustdoc/extern-impl.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2435:8
[00:53:35] 
[00:53:35] ---- [rustdoc] rustdoc/inline_cross/issue-33113.rs stdout ----
[00:53:35] 	
[00:53:35] error: htmldocck failed!
[00:53:35] status: exit code: 1
[00:53:35] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/issue-33113.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/inline_cross/issue-33113.rs"
[00:53:35] stdout:
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] ------------------------------------------
[00:53:35] stderr:
[00:53:35] ------------------------------------------
[00:53:35] 18: @has check failed
[00:53:35] 	`XPATH PATTERN` did not match
[00:53:35] 	// @has - '//code' "for &'a char"
[00:53:35] 
[00:53:35] Encountered 1 errors
[00:53:35] 
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] thread '[rustdoc] rustdoc/inline_cross/issue-33113.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2435:8
[00:53:35] 
[00:53:35] ---- [rustdoc] rustdoc/issue-29503.rs stdout ----
[00:53:35] 	
[00:53:35] error: htmldocck failed!
[00:53:35] status: exit code: 1
[00:53:35] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/issue-29503.rs"
[00:53:35] stdout:
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] ------------------------------------------
[00:53:35] stderr:
[00:53:35] ------------------------------------------
[00:53:35] 18: @has check failed
[00:53:35] 	`XPATH PATTERN` did not match
[00:53:35] 	// @has - "//ul[@id='implementors-list']/li" "impl<T> MyTrait for T where T: Debug"
[00:53:35] 
[00:53:35] Encountered 1 errors
[00:53:35] 
[00:53:35] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:322:21
[00:53:35] ------------------------------------------
[00:53:35] 
[00:53:35] thread '[rustdoc] rustdoc/issue-29503.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2435:8
[00:53:35] 
[00:53:35] 
[00:53:35] failures:
[00:53:35]     [rustdoc] rustdoc/doc-masked.rs
[00:53:35]     [rustdoc] rustdoc/extern-impl.rs
[00:53:35]     [rustdoc] rustdoc/inline_cross/issue-33113.rs
[00:53:35]     [rustdoc] rustdoc/issue-29503.rs
[00:53:35] 
[00:53:35] test result: FAILED. 159 passed; 4 failed; 1 ignored; 0 measured; 0 filtered out
