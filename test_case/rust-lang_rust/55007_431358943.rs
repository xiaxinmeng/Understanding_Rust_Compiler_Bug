plain
[00:56:31] ---- [ui] ui/issues/issue-39175.rs stdout ----
[00:56:31] diff of stderr:
[00:56:31] 
[00:56:31] 3    |
[00:56:31] 4 LL |     Command::new("echo").arg("hello").exec();
[00:56:31] -    |
[00:56:31] -    = help: items from traits can only be used if the trait is in scope
[00:56:31] - help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:56:31] -    |
[00:56:31] -    |
[00:56:31] - LL | use std::os::unix::process::CommandExt;
[00:56:31] 12 
[00:56:31] 13 error: aborting due to previous error
[00:56:31] 14 
[00:56:31] 
[00:56:31] 
[00:56:31] 
[00:56:31] The actual stderr differed from the expected stderr.
[00:56:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/issue-39175.stderr
[00:56:31] To update references, rerun the tests and pass the `--bless` flag
[00:56:31] To only update this specific test, also pass `--test-args issues/issue-39175.rs`
[00:56:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:56:31] error: 1 errors occurred comparing output.
[00:56:31] status: exit code: 1
[00:56:31] status: exit code: 1
[00:56:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39175.rs" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39175/auxiliary" "-A" "unused"
[00:56:31] ------------------------------------------
[00:56:31] 
[00:56:31] ------------------------------------------
[00:56:31] stderr:
[00:56:31] stderr:
[00:56:31] ------------------------------------------
[00:56:31] {"message":"no method named `exec` found for type `&mut std::process::Command` in the current scope","code":{"code":"E0599","explanation":"\nThis error occurs when a method is used on a type which doesn't implement it:\n\nErroneous code example:\n\n