
[00:56:43] ---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----
[00:56:43] 	
[00:56:43] error: htmldocck failed!
[00:56:43] status: exit code: 1
[00:56:43] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/issue-19190-3.rs"
[00:56:43] stdout:
[00:56:43] ------------------------------------------
[00:56:43] 
[00:56:43] ------------------------------------------
[00:56:43] stderr:
[00:56:43] ------------------------------------------
[00:56:43] 20: @has check failed
[00:56:43] 	`XPATH PATTERN` did not match
[00:56:43] 	// @has - '//*[@id="method.count_ones"]' 'fn count_ones(self) -> u32'
[00:56:43] 
[00:56:43] Encountered 1 errors
[00:56:43] 
[00:56:43] ------------------------------------------
[00:56:43] 
[00:56:43] failures:
[00:56:43]     [rustdoc] rustdoc/issue-19190-2.rs
[00:56:43]     [rustdoc] rustdoc/issue-19190-3.rs
