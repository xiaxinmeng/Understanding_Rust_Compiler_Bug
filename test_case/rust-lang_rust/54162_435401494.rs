
[01:20:09] ---- [rustdoc] rustdoc/assoc-consts.rs stdout ----
[01:20:09] 
[01:20:09] error: htmldocck failed!
[01:20:09] status: exit code: 1
[01:20:09] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
[01:20:09] stdout:
[01:20:09] ------------------------------------------
[01:20:09] 
[01:20:09] ------------------------------------------
[01:20:09] stderr:
[01:20:09] ------------------------------------------
[01:20:09] 102: @has check failed
[01:20:09] 	`XPATH PATTERN` did not match
[01:20:09] 	    // @has - '//*[@class="docblock"]' "Docs for QUX_DEFAULT12 in trait."
[01:20:09] 
[01:20:09] Encountered 1 errors
[01:20:09] 
[01:20:09] ------------------------------------------
[01:20:09] 
[01:20:09] thread '[rustdoc] rustdoc/assoc-consts.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:20:09] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:20:09] 
[01:20:09] 
[01:20:09] failures:
[01:20:09]     [rustdoc] rustdoc/assoc-consts.rs
