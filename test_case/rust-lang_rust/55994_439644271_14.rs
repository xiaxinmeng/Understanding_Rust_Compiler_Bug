\n\nThis syntax specifies that we want the X type from MyTrait, as made concrete in\nMyStruct. The reason that we cannot simply use `MyStruct::X` is that MyStruct\
[00:51:08] 19    |
[00:51:08] - LL |             Sub;
[00:51:08] + LL | type Test = Add +
[00:51:08] 21    |             ^^^ non-auto additional trait
[00:51:08] 22 
[00:51:08] 23 error[E0191]: the value of the associated type `Output` (from the trait `std::ops::Add`) must be specified
[00:51:08] 
[00:51:08] The actual stderr differed from the expected stderr.
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/issue-22560.stderr
[00:51:08] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/issue-22560.stderr
[00:51:08] To update references, rerun the tests and pass the `--bless` flag
[00:51:08] To only update this specific test, also pass `--test-args issues/issue-22560.rs`
[00:51:08] error: 1 errors occurred comparing output.
[00:51:08] status: exit code: 1
[00:51:08] status: exit code: 1
[00:51:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22560.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22560/auxiliary" "-A" "unused"
[00:51:08] ------------------------------------------
[00:51:08] 
[00:51:08] ------------------------------------------
[00:51:08] stderr:
[00:51:08] stderr:
[00:51:08] ------------------------------------------
[00:51:08] {"message":"the type parameter `RHS` must be explicitly specified","code":{"code":"E0393","explanation":"\nA ty`Self` reference, type parameters must be specified on object types","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0393]: the type parameter `RHS` must be explicitly specified\n  --> /checkout/src/test/ui/issues/issue-22560.rs:15:13\n   |\nLL | type Test = Add +\n   |             ^^^ missing reference to `RHS`\n   |\n   = note: because of the default `Self` reference, type parameters must be specified on object types\n\n"}
[00:51:08] {"message":"the type parameter `RHS` must be explicitly specified","code":{"code":"E0393","explanation":"\nA type parameter which references `Self` in its default value was not specified.\nExample of erroneous code:\n\n