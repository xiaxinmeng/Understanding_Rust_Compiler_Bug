
[00:53:03] 15: @has check failed
[00:53:03] 	`XPATH PATTERN` did not match
[00:53:03] 	// @has foo/fn.bar.html '//*[@class="tooltip compile_fail"]/span' "This code doesn't compile so be extra careful!"
[00:53:03] 16: @has check failed
[00:53:03] 	`XPATH PATTERN` did not match
[00:53:03] 	// @has foo/fn.bar.html '//*[@class="tooltip ignore"]/span' "Be careful when using this code, it's not being tested!"
[00:53:03] 
[00:53:03] Encountered 2 errors
[00:53:03] 
[00:53:03] ------------------------------------------
[00:53:03] 
[00:53:03] thread '[rustdoc] rustdoc/codeblock-title.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2498:8
