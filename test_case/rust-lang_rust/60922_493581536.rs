plain
[00:59:57] 26 
[00:59:57] 
[00:59:57] 
[00:59:57] The actual stderr differed from the expected stderr.
[00:59:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/invalid-const-arg-for-type-param.stderr
[00:59:57] To update references, rerun the tests and pass the `--bless` flag
[00:59:57] To only update this specific test, also pass `--test-args const-generics/invalid-const-arg-for-type-param.rs`
[00:59:57] error: 1 errors occurred comparing output.
[00:59:57] status: exit code: 1
[00:59:57] status: exit code: 1
[00:59:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/auxiliary" "-A" "unused"
[00:59:57] ------------------------------------------
[00:59:57] 
[00:59:57] ------------------------------------------
[00:59:57] stderr:
[00:59:57] stderr:
[00:59:57] ------------------------------------------
[00:59:57] {"message":"wrong number of const arguments: expected 0, found 1","code":{"code":"E0107","explanation":"\nThis error means that an incorrect number of generic arguments were provided:\n\n