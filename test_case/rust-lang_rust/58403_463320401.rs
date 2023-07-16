plain
[01:29:35] 
[01:29:35] ---- [ui (nll)] ui/error-codes/E0017.rs stdout ----
[01:29:35] diff of stderr:
[01:29:35] 
[01:29:35] 4 LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
[01:29:35] 5    |                              ^^^^^^ constants require immutable values
[01:29:35] - error: cannot mutate statics in the initializer of another static
[01:29:35] - error: cannot mutate statics in the initializer of another static
[01:29:35] + error[E0017]: references in statics may only refer to immutable values
[01:29:35] 8   --> $DIR/E0017.rs:5:39
[01:29:35] 9    |
[01:29:35] 10 LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
[01:29:35] -    |                                       ^^^^^^
[01:29:35] +    |                                       ^^^^^^ statics require immutable values
[01:29:35] 12 
[01:29:35] 12 
[01:29:35] - error[E0017]: references in statics may only refer to immutable values
[01:29:35] + error: cannot mutate statics in the initializer of another static
[01:29:35] 14   --> $DIR/E0017.rs:5:39
[01:29:35] 15    |
[01:29:35] 16 LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
[01:29:35] -    |                                       ^^^^^^ statics require immutable values
[01:29:35] +    |                                       ^^^^^^
[01:29:35] 18 
[01:29:35] 18 
[01:29:35] 19 error[E0596]: cannot borrow immutable static item `X` as mutable
[01:29:35] 20   --> $DIR/E0017.rs:5:39
[01:29:35] 
[01:29:35] The actual stderr differed from the expected stderr.
[01:29:35] The actual stderr differed from the expected stderr.
[01:29:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017.nll/E0017.nll.stderr
[01:29:35] To update references, rerun the tests and pass the `--bless` flag
[01:29:35] To only update this specific test, also pass `--test-args error-codes/E0017.rs`
[01:29:35] error: 1 errors occurred comparing output.
[01:29:35] status: exit code: 1
[01:29:35] status: exit code: 1
[01:29:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017.nll/auxiliary" "-A" "unused"
[01:29:35] ------------------------------------------
[01:29:35] 
[01:29:35] ------------------------------------------
[01:29:35] stderr:
[01:29:35] stderr:
[01:29:35] ------------------------------------------
[01:29:35] {"message":"references in constants may only refer to immutable values","code":{"code":"E0017","explanation":"\nReferences in statics and constants may only refer to immutable values.\nErroneous code example:\n\n