plain
[01:46:55] 
[01:46:55] ---- [ui (nll)] ui/issues/issue-26619.rs stdout ----
[01:46:55] diff of stderr:
[01:46:55] 
[01:46:55] - error[E0597]: borrowed value does not live long enough
[01:46:55] + error[E0515]: cannot return value referencing function parameter
[01:46:55] +   --> $DIR/issue-26619.rs:7:76
[01:46:55] 3    |
[01:46:55] 3    |
[01:46:55] 4 LL |         for s in vec!["1|2".to_string()].into_iter().filter_map(|ref line| self.make_entry(line)) {
[01:46:55] -    |                                                                  ^^^^^^^^                      -- temporary value needs to live until here
[01:46:55] -    |                                                                  |                             |
[01:46:55] -    |                                                                  |                             temporary value dropped here while still borrowed
[01:46:55] -    |                                                                  temporary value does not live long enough
[01:46:55] +    |                                                                  --------  ^^^^^^^^^^^^^^^^^^^^^ returns a value referencing data owned by the current function
[01:46:55] +    |                                                                  function parameter borrowed here
[01:46:55] 9 
[01:46:55] 10 error: aborting due to previous error
[01:46:55] 11 
[01:46:55] 11 
[01:46:55] 
[01:46:55] - For more information about this error, try `rustc --explain E0597`.
[01:46:55] + For more information about this error, try `rustc --explain E0515`.
[01:46:55] 13 
[01:46:55] 
[01:46:55] 
[01:46:55] The actual stderr differed from the expected stderr.
[01:46:55] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26619.nll/issue-26619.nll.stderr
[01:46:55] To update references, rerun the tests and pass the `--bless` flag
[01:46:55] To only update this specific test, also pass `--test-args issues/issue-26619.rs`
[01:46:55] error: 1 errors occurred comparing output.
[01:46:55] status: exit code: 1
[01:46:55] status: exit code: 1
[01:46:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26619.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26619.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26619.nll/auxiliary" "-A" "unused"
[01:46:55] ------------------------------------------
[01:46:55] 
[01:46:55] ------------------------------------------
[01:46:55] stderr:
[01:46:55] stderr:
[01:46:55] ------------------------------------------
[01:46:55] {"message":"cannot return value referencing function parameter","code":{"code":"E0515","explanation":"\nCannot return value that references local variable\n\nLocal variables, function parameters and temporaries are all dropped before the\nend of the function body. So a reference to them cannot be returned.\n\n