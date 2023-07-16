plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:30] 
[00:44:30] running 1507 tests
[00:44:34] ...............................................F.............................................i......
[00:44:45] ....................................................................................................
[00:44:48] ....................................................................................................
[00:44:52] ....................................................................................................
[00:44:55] ....................................................................................................
---
[00:45:46] normalized stderr:
[00:45:46] error[E0507]: cannot move out of borrowed content
[00:45:46]   --> $DIR/issue-51415.rs:16:46
[00:45:46]    |
[00:45:46] LL |     let opt = a.iter().enumerate().find(|(_, &s)| {
[00:45:46]    |                                              ^-
[00:45:46]    |                                              ||
[00:45:46]    |                                              |hint: to prevent move, use `ref s` or `ref mut s`
[00:45:46]    |                                              cannot move out of borrowed content
[00:45:46] error: aborting due to previous error
[00:45:46] 
[00:45:46] For more information about this error, try `rustc --explain E0507`.
[00:45:46] 
[00:45:46] 
[00:45:46] 
[00:45:46] 
[00:45:46] The actual stderr differed from the expected stderr.
[00:45:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-51415/issue-51415.stderr
[00:45:46] To update references, rerun the tests and pass the `--bless` flag
[00:45:46] To only update this specific test, also pass `--test-args borrowck/issue-51415.rs`
[00:45:46] error: 1 errors occurred comparing output.
[00:45:46] status: exit code: 101
[00:45:46] status: exit code: 101
[00:45:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-51415.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-51415/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-51415/auxiliary" "-A" "unused"
[00:45:46] ------------------------------------------
[00:45:46] 
[00:45:46] ------------------------------------------
[00:45:46] stderr:
[00:45:46] stderr:
[00:45:46] ------------------------------------------
[00:45:46] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n