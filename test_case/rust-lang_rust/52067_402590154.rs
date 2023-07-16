plain
[00:40:21] ....................................................................................................
[00:40:25] ....................................................................................................
[00:40:30] ....................................................................................................
[00:40:34] ....................................................................................................
[00:40:40] .............................................F......................................................
[00:40:45] ....................................................................................................
[00:40:52] .............................i........................................F.............................
[00:41:02] ....................................................................................................
[00:41:07] ....................................................................................................
[00:41:15] .........................................................i..........................................
[00:41:17] ..........................................
---
[00:41:17] diff of stderr:
[00:41:17] 
[00:41:17] 2   --> $DIR/issue-47184.rs:14:44
[00:41:17] 3    |
[00:41:17] 4 LL |     let _vec: Vec<&'static String> = vec![&String::new()];
[00:41:17] -    |                                            ^^^^^^^^^^^^^ temporary value does not live long enough
[00:41:17] - LL |     //~^ ERROR borrowed value does not live long enough [E0597]
[00:41:17] - LL | }
[00:41:17] -    | - temporary value only lives until here
[00:41:17] +    |                                            ^^^^^^^^^^^^^ - temporary value only lives until here
[00:41:17] +    |                                            |
[00:41:17] +    |                                            temporary value does not live long enough
[00:41:17] 9    |
[00:41:17] 10    = note: borrowed value must be valid for the static lifetime...
[00:41:17] 
[00:41:17] 
[00:41:17] The actual stderr differed from the expected stderr.
[00:41:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47184/issue-47184.stderr
[00:41:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47184/issue-47184.stderr
[00:41:17] To update references, rerun the tests and pass the `--bless` flag
[00:41:17] To only update this specific test, also pass `--test-args issue-47184.rs`
[00:41:17] error: 1 errors occurred comparing output.
[00:41:17] status: exit code: 101
[00:41:17] status: exit code: 101
[00:41:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-47184.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47184/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-47184/auxiliary" "-A" "unused"
[00:41:17] ------------------------------------------
[00:41:17] 
[00:41:17] ------------------------------------------
[00:41:17] stderr:
[00:41:17] stderr:
[00:41:17] ------------------------------------------
[00:41:17] {"message":"borrowed value does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n