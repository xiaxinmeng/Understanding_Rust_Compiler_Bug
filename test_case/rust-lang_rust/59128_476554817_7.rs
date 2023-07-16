text
[01:51:12] 97 
[01:51:12] 98 
[01:51:12] 98 
[01:51:12] 
[01:51:12] 
[01:51:12] The actual stderr differed from the expected stderr.
[01:51:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/invalid-syntax.stderr
[01:51:12] To update references, rerun the tests and pass the `--bless` flag
[01:51:12] To only update this specific test, also pass `--test-args invalid-syntax.rs`
[01:51:12] error: 1 errors occurred comparing output.
[01:51:12] status: exit code: 0
[01:51:12] status: exit code: 0
[01:51:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/a" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/auxiliary"
[01:51:12] ------------------------------------------
[01:51:12] 
[01:51:12] ------------------------------------------
[01:51:12] stderr:
[01:51:12] stderr:
[01:51:12] ------------------------------------------
[01:51:12] warning: could not parse code block as Rust code
[01:51:12]   --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:3:5
[01:51:12]    |
[01:51:12] LL |   /// 