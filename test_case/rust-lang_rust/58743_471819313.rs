plain
[01:03:56] 
[01:03:56] 3    |
[01:03:56] 4 LL |       let x: i32 = {
[01:03:56] 5    |  __________________^
[01:03:56] - LL | |         //~^ ERROR mismatched types
[01:03:56] - LL | |         foo(); //~ HELP consider removing this semicolon
[01:03:56] + LL | |
[01:03:56] + LL | |         foo();
[01:03:56] 8    | |              - help: consider removing this semicolon
[01:03:56] 9 LL | |     };
[01:03:56] 10    | |_____^ expected i32, found ()
[01:03:56] 
[01:03:56] The actual stderr differed from the expected stderr.
[01:03:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expression-remove-semicolon/block-expression-remove-semicolon.stderr
[01:03:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expression-remove-semicolon/block-expression-remove-semicolon.stderr
[01:03:56] To update references, rerun the tests and pass the `--bless` flag
[01:03:56] To only update this specific test, also pass `--test-args block-expression-remove-semicolon.rs`
[01:03:56] error: 1 errors occurred comparing output.
[01:03:56] status: exit code: 1
[01:03:56] status: exit code: 1
[01:03:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-expression-remove-semicolon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expression-remove-semicolon/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-x86_64/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-expression-remove-semicolon/auxiliary" "-A" "unused"
[01:03:56] ------------------------------------------
[01:03:56] 
[01:03:56] ------------------------------------------
[01:03:56] stderr:
[01:03:56] stderr:
[01:03:56] ------------------------------------------
[01:03:56] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n