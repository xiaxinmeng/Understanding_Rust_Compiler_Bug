

[01:26:30] ---- [rustdoc] rustdoc/synthetic_auto/no-redundancy.rs stdout ----

[01:26:30] 	

[01:26:30] error: htmldocck failed!

[01:26:30] status: exit code: 1

[01:26:30] command: "/usr/local/opt/python/bin/python2.7" "/Users/travis/build/rust-lang/rust/src/etc/htmldocck.py" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/rustdoc/synthetic_auto/no-redundancy.stage2-i686-apple-darwin" "/Users/travis/build/rust-lang/rust/src/test/rustdoc/synthetic_auto/no-redundancy.rs"

[01:26:30] stdout:

[01:26:30] ------------------------------------------

[01:26:30] 

[01:26:30] ------------------------------------------

[01:26:30] stderr:

[01:26:30] ------------------------------------------

[01:26:30] 22: @has check failed

[01:26:30] 	`XPATH PATTERN` did not match

[01:26:30] 	// @has - '//*[@id="synthetic-implementations-list"]/*[@class="impl"]/*/code' "impl<T> Send for Outer<T> where T: Copy + Send"

[01:26:30] 

[01:26:30] Encountered 1 errors

[01:26:30] 

[01:26:30] ------------------------------------------
