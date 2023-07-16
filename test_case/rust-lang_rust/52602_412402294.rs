plain
[00:45:49] ....................i...............................................................................
[00:45:52] ...............................i....................................................................
[00:45:55] ....................................................................................................
[00:45:59] ....................................................................................................
 error[E0271]: type mismatch resolving `<std::result::Result<i32, i32> as std::ops::Try>::Ok == &str`
[00:46:05] 
[00:46:05] The actual stderr differed from the expected stderr.
[00:46:05] The actual stderr differed from the expected stderr.
[00:46:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/try-block-bad-type.stderr
[00:46:05] To update references, rerun the tests and pass the `--bless` flag
[00:46:05] To only update this specific test, also pass `--test-args try-block/try-block-bad-type.rs`
[00:46:05] error: 1 errors occurred comparing output.
[00:46:05] status: exit code: 1
[00:46:05] status: exit code: 1
[00:46:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-bad-type.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-bad-type/auxiliary" "-A" "unused"
[00:46:05] ------------------------------------------
[00:46:05] 
[00:46:05] ------------------------------------------
[00:46:05] stderr:
[00:46:05] stderr:
[00:46:05] ------------------------------------------
[00:46:05] {"message":"the trait bound `i32: std::convert::From<&str>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous cthe input type to types that implement Debug.\nfn some_func<T: fmt::Debug>(foo: T) {\n    println!(\"{:?}\", foo);\n}\n\nfn main() {\n    // Calling the method is still fine, as i32 implements Debug.\n    some_func(5i32);\n\n    // This would fail to compile now:\n    // struct WithoutDebug;\n    // some_func(WithoutDebug);\n}\n