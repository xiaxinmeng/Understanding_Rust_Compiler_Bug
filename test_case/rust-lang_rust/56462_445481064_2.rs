\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/infinite/infinite-vec-type-recursion.rs","byte_start":480,"byte_end":481,"line_start":11,"line_end":11,"columROR cycle detected
[00:47:57] 5    |                              ^^^^^^^^^^
[00:47:57] 6    |
[00:47:57] 6    |
[00:47:57] - note: ...which requires processing `DefaultFoo`...
[00:47:57] + note: ...which requires finding type of DefaultFoo...
[00:47:57] 9    |
[00:47:57] 9    |
[00:47:57] 10 LL | type DefaultFoo = Foo;
[00:47:57] 11    |                   ^^^
[00:47:57] 11    |                   ^^^
[00:47:57] -    = note: ...which again requires processing `Foo::T`, completing the cycle
[00:47:57] +    = note: ...which again requires finding type of Foo::T, completing the cycle
[00:47:57] 14 error: aborting due to previous error
[00:47:57] 15 
[00:47:57] 
[00:47:57] 
[00:47:57] 
[00:47:57] The actual stderr differed from the expected stderr.
[00:47:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/issue-34373.stderr
[00:47:57] To update references, rerun the tests and pass the `--bless` flag
[00:47:57] To only update this specific test, also pass `--test-args issues/issue-34373.rs`
[00:47:57] error: 1 errors occurred comparing output.
[00:47:57] status: exit code: 1
[00:47:57] status: exit code: 1
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34373.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"cycle detected when finding type of Foo::T","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n