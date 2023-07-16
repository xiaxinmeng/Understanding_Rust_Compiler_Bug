plain
[00:44:45] 
[00:44:45] ---- [ui] ui/precise_pointer_size_matching.rs stdout ----
[00:44:45] diff of stderr:
[00:44:45] 
[00:44:45] - error[E0004]: non-exhaustive patterns: `-9223372036854775808isize..=-6isize` and `21isize..=9223372036854775807isize` not covered
[00:44:45] + error[E0004]: non-exhaustive patterns: `-2147483648isize..=-6isize` and `21isize..=2147483647isize` not covered
[00:44:45] 2   --> $DIR/precise_pointer_size_matching.rs:17:11
[00:44:45] 3    |
[00:44:45] 4 LL |     match 0isize { //~ ERROR non-exhaustive patterns
[00:44:45] 
[00:44:45] -    |           ^^^^^^ patterns `-9223372036854775808isize..=-6isize` and `21isize..=9223372036854775807isize` not covered
[00:44:45] +    |           ^^^^^^ patterns `-2147483648isize..=-6isize` and `21isize..=2147483647isize` not covered
[00:44:45] 6 
[00:44:45] - error[E0004]: non-exhaustive patterns: `0usize` and `21usize..=18446744073709551615usize` not covered
[00:44:45] + error[E0004]: non-exhaustive patterns: `0usize` and `21usize..=4294967295usize` not covered
[00:44:45] 8   --> $DIR/precise_pointer_size_matching.rs:22:11
[00:44:45] 9    |
[00:44:45] 10 LL |     match 0usize { //~ ERROR non-exhaustive patterns
[00:44:45] 
[00:44:45] -    |           ^^^^^^ patterns `0usize` and `21usize..=18446744073709551615usize` not covered
[00:44:45] +    |           ^^^^^^ patterns `0usize` and `21usize..=4294967295usize` not covered
[00:44:45] 13 error: aborting due to 2 previous errors
[00:44:45] 14 
[00:44:45] 
[00:44:45] 
[00:44:45] 
[00:44:45] The actual stderr differed from the expected stderr.
[00:44:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/precise_pointer_size_matching.stderr
[00:44:45] To update references, rerun the tests and pass the `--bless` flag
[00:44:45] To only update this specific test, also pass `--test-args precise_pointer_size_matching.rs`
[00:44:45] error: 1 errors occurred comparing output.
[00:44:45] status: exit code: 1
[00:44:45] status: exit code: 1
[00:44:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/precise_pointer_size_matching.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/precise_pointer_size_matching/auxiliary" "-A" "unused"
[00:44:45] ------------------------------------------
[00:44:45] 
[00:44:45] ------------------------------------------
[00:44:45] stderr:
[00:44:45] stderr:
[00:44:45] ------------------------------------------
[00:44:45] {"message":"non-exhaustive patterns: `-2147483648isize..=-6isize` and `21isize..=2147483647isize` not covered","code":{"code":"E0004","explanation":"\nThis error indicates that the compiler cannot guarantee a matching pattern for\none or more possible inputs to a match expression. Guaranteed matches are\nrequired in order to assign values to match expressions, or alternatively,\ndetermine the flow of execution. Erroneous code example:\n\n