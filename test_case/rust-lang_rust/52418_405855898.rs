plain
[00:43:39] diff of stderr:
[00:43:39] 
[00:43:39] 2   --> $DIR/issue-20261.rs:14:11
[00:43:39] 3    |
[00:43:39] 4 LL |     for (ref i,) in [].iter() {
[00:43:39] -    |         -------- consider giving `__next` a type
[00:43:39] +    |                     --------- the element type for this iterator is not specified
[00:43:39] 6 LL |         i.clone();
[00:43:39] 7    |           ^^^^^ cannot infer type for `_`
[00:43:39] 
[00:43:39] 
[00:43:39] The actual stderr differed from the expected stderr.
[00:43:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20261/issue-20261.stderr
[00:43:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20261/issue-20261.stderr
[00:43:39] To update references, rerun the tests and pass the `--bless` flag
[00:43:39] To only update this specific test, also pass `--test-args issue-20261.rs`
[00:43:39] error: 1 errors occurred comparing output.
[00:43:39] status: exit code: 101
[00:43:39] status: exit code: 101
[00:43:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-20261.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20261/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-20261/auxiliary" "-A" "unused"
[00:43:39] ------------------------------------------
[00:43:39] 
[00:43:39] ------------------------------------------
[00:43:39] stderr:
[00:43:39] stderr:
[00:43:39] ------------------------------------------
[00:43:39] {"message":"type annotations needed","code":{"code":"E0282","explanation":"\nThis error indicates that type inference did not result in one unique possible\ntype, and extra information is required. In most cases this can be provided\nby adding a type annotation. Sometimes you need to specify a generic type\nparameter manually.\n\nA common example is the `collect` method on `Iterator`. It has a generic type\nparameter with a `FromIterator` bound, which for a `char` iterator is\nimplemented by `Vec` and `String` among others. Consider the following snippet\nthat reverses the characters of a string:\n\n