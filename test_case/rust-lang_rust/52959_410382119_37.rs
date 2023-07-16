\n\nrop(x);
[00:43:19] +    |          - borrow used here in later iteration of loop
[00:43:19] 12 
[00:43:19] 13 error: aborting due to previous error
[00:43:19] 14 
[00:43:19] 14 
[00:43:19] 
[00:43:19] 
[00:43:19] The actual stderr differed from the expected stderr.
[00:43:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-referent-issue-38899/borrowed-referent-issue-38899.stderr
[00:43:19] To update references, rerun the tests and pass the `--bless` flag
[00:43:19] To only update this specific test, also pass `--test-args nll/borrowed-referent-issue-38899.rs`
[00:43:19] error: 1 errors occurred comparing output.
[00:43:19] status: exit code: 1
[00:43:19] status: exit code: 1
[00:43:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/borrowed-referent-issue-38899.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-referent-issue-38899/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-referent-issue-38899/auxiliary" "-A" "unused"
[00:43:19] ------------------------------------------
[00:43:19] 
[00:43:19] ------------------------------------------
[00:43:19] stderr:
[00:43:19] stderr:
[00:43:19] ------------------------------------------
[00:43:19] {"message":"cannot borrow `*block.current` as immutable because it is also borrowed as mutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n