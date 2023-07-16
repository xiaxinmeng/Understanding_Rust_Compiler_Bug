\n\nIn here, `x` is created before `y` and therefore has a greater lifetime. Always\nkeep in mind that values id as immutable (Mir)
[00:41:17] 60    |
[00:41:17] 60    |
[00:41:17] 61 LL |         match map.get() {
[00:41:17] 
[00:41:17] 75 LL | |     }
[00:41:17] 76 LL | | }
[00:41:17] - 
[00:41:17] - 
[00:41:17] - error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Mir)
[00:41:17] -    |
[00:41:17] -    |
[00:41:17] - LL |         match map.get() {
[00:41:17] -    |               --- immutable borrow occurs here
[00:41:17] - LL |             Some(v) => {
[00:41:17] - LL |                 map.set(String::new()); // Both AST and MIR error here
[00:41:17] -    |                 ^^^ mutable borrow occurs here
[00:41:17] - ...
[00:41:17] - LL |                 return v;
[00:41:17] -    |                        - borrow later used here
[00:41:17] 91 error: aborting due to 6 previous errors
[00:41:17] 92 
[00:41:17] 
[00:41:17] 
[00:41:17] 
[00:41:17] The actual stderr differed from the expected stderr.
[00:41:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default/get_default.stderr
[00:41:17] To update references, rerun the tests and pass the `--bless` flag
[00:41:17] To only update this specific test, also pass `--test-args nll/get_default.rs`
[00:41:17] error: 1 errors occurred comparing output.
[00:41:17] status: exit code: 101
[00:41:17] status: exit code: 101
[00:41:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/get_default.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default/auxiliary" "-A" "unused"
[00:41:17] ------------------------------------------
[00:41:17] 
[00:41:17] ------------------------------------------
[00:41:17] stderr:
[00:41:17] stderr:
[00:41:17] ------------------------------------------
[00:41:17] {"message":"cannot borrow `*map` as mutable because it is also borrowed as immutable (Ast)","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n