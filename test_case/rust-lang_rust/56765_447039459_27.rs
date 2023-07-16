\n// Basic Example:\ntrait Trait { type Assoc45] + error[E0308]: mismatched types
[00:47:45] +   --> $DIR/associated-types-path-2.rs:51:18
[00:47:45] +    |
[00:47:45] + LL |     let _: i32 = f2(2i32);
[00:47:45] +    |                  ^^^^^^^^ expected i32, found u32
[00:47:45] + 
[00:47:45] 37 error[E0277]: the trait bound `u32: Foo` is not satisfied
[00:47:45] 39    |
[00:47:45] 
[00:47:45] 
[00:47:45] 41    |     ^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `u32`
[00:47:45] 43 error[E0308]: mismatched types
[00:47:45] -   --> $DIR/associated-types-path-2.rs:51:18
[00:47:45] +   --> $DIR/associated-types-path-2.rs:29:14
[00:47:45] 45    |
[00:47:45] 45    |
[00:47:45] - LL |     let _: i32 = f2(2i32);
[00:47:45] -    |                  ^^^^^^^^ expected i32, found u32
[00:47:45] + LL |     f1(2i32, 4i32);
[00:47:45] 48 
[00:47:45] 49 error: aborting due to 6 previous errors
[00:47:45] 50 
[00:47:45] 
[00:47:45] 
[00:47:45] 
[00:47:45] The actual stderr differed from the expected stderr.
[00:47:45] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/associated-types-path-2.stderr
[00:47:45] To update references, rerun the tests and pass the `--bless` flag
[00:47:45] To only update this specific test, also pass `--test-args associated-types/associated-types-path-2.rs`
[00:47:45] error: 1 errors occurred comparing output.
[00:47:45] status: exit code: 1
[00:47:45] status: exit code: 1
[00:47:45] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-path-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-path-2/auxiliary" "-A" "unused"
[00:47:45] ------------------------------------------
[00:47:45] 
[00:47:45] ------------------------------------------
[00:47:45] stderr:
[00:47:45] stderr:
[00:47:45] ------------------------------------------
[00:47:45] {"message":"the trait bound `u32: Foo` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n