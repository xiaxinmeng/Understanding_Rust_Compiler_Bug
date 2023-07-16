\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/regions/region-borrow-params-issue-29793-big.rs","byte_start":2061,"byte_end":2070,"line_start":81,"line_end":81,"column_start":26,"column_end":35,"is_primary":false,"text":[{"text":"        WrapB:orrowed value only lives until here
[00:51:24] + ...
[00:51:24] + LL | }
[00:51:24] +    | - borrowed value needs to live until here
[00:51:24] 52 
[00:51:24] 53 error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
[00:51:24] 
[00:51:24] 
[00:51:24] The actual stderr differed from the expected stderr.
[00:51:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-small/region-borrow-params-issue-29793-small.stderr
[00:51:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-small/region-borrow-params-issue-29793-small.stderr
[00:51:24] To update references, rerun the tests and pass the `--bless` flag
[00:51:24] To only update this specific test, also pass `--test-args regions/region-borrow-params-issue-29793-small.rs`
[00:51:24] error: 1 errors occurred comparing output.
[00:51:24] status: exit code: 1
[00:51:24] status: exit code: 1
[00:51:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-borrow-params-issue-29793-small.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-small/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-borrow-params-issue-29793-small/auxiliary" "-A" "unused"
[00:51:24] ------------------------------------------
[00:51:24] 
[00:51:24] ------------------------------------------
[00:51:24] stderr:
[00:51:24] stderr:
[00:51:24] ------------------------------------------
[00:51:24] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n