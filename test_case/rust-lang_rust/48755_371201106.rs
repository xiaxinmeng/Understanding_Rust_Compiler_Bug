
[01:13:02] ---- [rustdoc] rustdoc/fn-sidebar.rs stdout ----
[01:13:02] 	
[01:13:02] error: htmldocck failed!
[01:13:02] status: exit code: 1
[01:13:02] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/fn-sidebar.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/fn-sidebar.rs"
[01:13:02] stdout:
[01:13:02] ------------------------------------------
[01:13:02] 
[01:13:02] ------------------------------------------
[01:13:02] stderr:
[01:13:02] ------------------------------------------
[01:13:02] 17: @has check failed
[01:13:02] 	File does not exist 'foo/constant.bar.html'
[01:13:02] 	// @has foo/constant.bar.html
[01:13:02] 18: @has check failed
[01:13:02] 	File does not exist 'foo/constant.bar.html'
[01:13:02] 	// @has - '//*[@class="sidebar-elems"]' ''
[01:13:02] 
[01:13:02] Encountered 2 errors
[01:13:02] 
[01:13:02] ------------------------------------------
[01:13:02] 
[01:13:02] thread '[rustdoc] rustdoc/fn-sidebar.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2893:9
[01:13:02] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:13:02] 
[01:13:02] 
[01:13:02] failures:
[01:13:02]     [rustdoc] rustdoc/fn-sidebar.rs
