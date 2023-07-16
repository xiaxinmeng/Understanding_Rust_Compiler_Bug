
[00:58:27] ---- [rustdoc] rustdoc/doc-spotlight.rs stdout ----
[00:58:27] 	
[00:58:27] error: htmldocck failed!
[00:58:27] status: exit code: 1
[00:58:27] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-spotlight.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/doc-spotlight.rs"
[00:58:27] stdout:
[00:58:27] ------------------------------------------
[00:58:27] 
[00:58:27] ------------------------------------------
[00:58:27] stderr:
[00:58:27] ------------------------------------------
[00:58:27] 22: @has check failed
[00:58:27] 	`XPATH PATTERN` did not match
[00:58:27] 	    // @has - '//code[@class="spotlight"]' 'impl<T: SomeTrait> SomeTrait for Wrapper<T>'
[00:58:27] 35: @has check failed
[00:58:27] 	`XPATH PATTERN` did not match
[00:58:27] 	    // @has - '//code[@class="spotlight"]' 'impl SomeTrait for SomeStruct'
[00:58:27] 36: @has check failed
[00:58:27] 	`XPATH PATTERN` did not match
[00:58:27] 	    // @has - '//code[@class="spotlight"]' 'impl<T: SomeTrait> SomeTrait for Wrapper<T>'
[00:58:27] 43: @has check failed
[00:58:27] 	`XPATH PATTERN` did not match
[00:58:27] 	// @has - '//code[@class="spotlight"]' 'impl SomeTrait for SomeStruct'
[00:58:27] 
[00:58:27] Encountered 4 errors
