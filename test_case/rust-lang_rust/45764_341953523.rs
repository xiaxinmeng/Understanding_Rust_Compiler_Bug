
[01:00:27] failures:
[01:00:27] 
[01:00:27] ---- [rustdoc] rustdoc/playground-arg.rs stdout ----
[01:00:27] 	
[01:00:27] error: htmldocck failed!
[01:00:27] status: exit code: 1
[01:00:27] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground-arg.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/playground-arg.rs"
[01:00:27] stdout:
[01:00:27] ------------------------------------------
[01:00:27] 
[01:00:27] ------------------------------------------
[01:00:27] stderr:
[01:00:27] ------------------------------------------
[01:00:27] 24: @matches check failed
[01:00:27] 	`XPATH PATTERN` did not match
[01:00:27] 	// @matches foo/index.html '//a[@class="test-arrow"][@href="https://example.com/?code=extern%20crate%20foo%3B%0Afn%20main()%20%7B%0Ause%20foo%3A%3Adummy%3B%0Adummy()%3B%0A%7D"]' "Run"
[01:00:27] 
[01:00:27] Encountered 1 errors
[01:00:27] 
[01:00:27] ------------------------------------------
[01:00:27] 
[01:00:27] thread '[rustdoc] rustdoc/playground-arg.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2497:8
[01:00:27] 
[01:00:27] ---- [rustdoc] rustdoc/playground.rs stdout ----
[01:00:27] 	
[01:00:27] error: htmldocck failed!
[01:00:27] status: exit code: 1
[01:00:27] command: "/usr/bin/python2.7" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/playground.stage2-x86_64-unknown-linux-gnu" "/checkout/src/test/rustdoc/playground.rs"
[01:00:27] stdout:
[01:00:27] ------------------------------------------
[01:00:27] 
[01:00:27] ------------------------------------------
[01:00:27] stderr:
[01:00:27] ------------------------------------------
[01:00:27] 37: @matches check failed
[01:00:27] 	`XPATH PATTERN` did not match
[01:00:27] 	// @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=fn%20main()%20%7B%0A%20%20%20%20println!(%22Hello%2C%20world!%22)%3B%0A%7D%0A"]' "Run"
[01:00:27] 38: @matches check failed
[01:00:27] 	`XPATH PATTERN` did not match
[01:00:27] 	// @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=fn%20main()%20%7B%0Aprintln!(%22Hello%2C%20world!%22)%3B%0A%7D"]' "Run"
[01:00:27] 39: @matches check failed
[01:00:27] 	`XPATH PATTERN` did not match
[01:00:27] 	// @matches foo/index.html '//a[@class="test-arrow"][@href="https://www.example.com/?code=%23!%5Bfeature(something)%5D%0A%0Afn%20main()%20%7B%0A%20%20%20%20println!(%22Hello%2C%20world!%22)%3B%0A%7D%0A&version=nightly"]' "Run"
[01:00:27] 
[01:00:27] Encountered 3 errors
[01:00:27] 
[01:00:27] ------------------------------------------
[01:00:27] 
[01:00:27] thread '[rustdoc] rustdoc/playground.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2497:8
[01:00:27] 
[01:00:27] 
[01:00:27] failures:
[01:00:27]     [rustdoc] rustdoc/no-run-still-checks-lints.rs
[01:00:27]     [rustdoc] rustdoc/playground-arg.rs
[01:00:27]     [rustdoc] rustdoc/playground.rs
[01:00:27] 
[01:00:27] test result: FAILED. 167 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
