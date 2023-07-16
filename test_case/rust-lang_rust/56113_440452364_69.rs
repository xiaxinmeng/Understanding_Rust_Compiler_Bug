\nfn bar(x: &mut i32) {}\nfn foo(a: &mut i32) {\n    bar(a);\n    let ref y = a; // ok!\n}\  |     |
[00:48:22] 8 LL | 
[00:48:22] 
[00:48:22] 
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] The actual stderr differed from the expected stderr.
[00:48:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-cannot-nest-mut-self-calls/two-phase-cannot-nest-mut-self-calls.stderr
[00:48:22] To update references, rerun the tests and pass the `--bless` flag
[00:48:22] To only update this specific test, also pass `--test-args borrowck/two-phase-cannot-nest-mut-self-calls.rs`
[00:48:22] error: 1 errors occurred comparing output.
[00:48:22] status: exit code: 1
[00:48:22] status: exit code: 1
[00:48:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-cannot-nest-mut-self-calls.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-cannot-nest-mut-self-calls/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "two-phase-borrows" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-cannot-nest-mut-self-calls/auxiliary" "-A" "unused"
[00:48:22] ------------------------------------------
[00:48:22] 
[00:48:22] ------------------------------------------
[00:48:22] stderr:
[00:48:22] stderr:
[00:48:22] ------------------------------------------
[00:48:22] {"message":"cannot borrow `vec` as mutable because it is also borrowed as immutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n