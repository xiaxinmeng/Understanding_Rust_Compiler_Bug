compile_fail,E0621\nfnin a constant
[01:00:57] -    |                ^^^^^^^^^^^^^ reserved keyword
[01:00:57] - 
[01:00:57] - error[E0516]: `typeof` is a reserved keyword but unimplemented
[01:00:57] 21    |
[01:00:57] 21    |
[01:00:57] 22 LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
[01:00:57] 23    |      ^^^^^^^^^ reserved keyword
[01:00:57] + 
[01:00:57] + 
[01:00:57] + error[E0516]: `typeof` is a reserved keyword but unimplemented
[01:00:57] +   --> $DIR/issue-42060.rs:13:16
[01:00:57] +    |
[01:00:57] + LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
[01:00:57] +    |                ^^^^^^^^^^^^^ reserved keyword
[01:00:57] 25 error: aborting due to 4 previous errors
[01:00:57] 26 
[01:00:57] 
[01:00:57] 
[01:00:57] 
[01:00:57] The actual stderr differed from the expected stderr.
[01:00:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/issue-42060.stderr
[01:00:57] To update references, rerun the tests and pass the `--bless` flag
[01:00:57] To only update this specific test, also pass `--test-args issues/issue-42060.rs`
[01:00:57] error: 1 errors occurred comparing output.
[01:00:57] status: exit code: 1
[01:00:57] status: exit code: 1
[01:00:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42060.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/a" "-Crpath" "-O" "-Zunstable-             ^^^^^ non-constant value\n\n"}
[01:00:57] {"message":"attempt to use a non-constant value in a constant","code":{"code":"E0435","explanation":"\nA non-constant value was used in a constant expression.\n\nErroneous code example:\n\n