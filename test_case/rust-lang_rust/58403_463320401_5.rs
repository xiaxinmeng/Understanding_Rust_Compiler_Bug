\n\nStatics are shared everywhere, and if they refer to mutable data one might\nviolate memory safety since holding multiple mutable references to shared data\nis not allowed.\n\nIf you really want global mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0017.rs","byte_start":340,"byte_end":346,"line_start":8,"line_end":8,"column_start":38,"column_end":44,"is_primary":true,"text":[{"text":"static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017","highlight_start":38,"highlight_end":44}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/error-codes/E0017.rs:8:38\n   |\nLL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017\n   |                                      ^^^^^^ statics require immutable values\n\n"}
[01:29:35] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:29:35] {"message":"Some errors occurred: E0017, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0017, E0596.\n"}
[01:29:35] 
[01:29:35] ------------------------------------------
[01:29:35] 
[01:29:35] thread '[ui (nll)] ui/error-codes/E0017.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:29:35] thread '[ui (nll)] ui/error-codes/E0017.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:29:35] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:29:35] 
[01:29:35] ---- [ui (nll)] ui/error-codes/E0388.rs stdout ----
[01:29:35] diff of stderr:
[01:29:35] 
[01:29:35] 4 LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
[01:29:35] 5    |                              ^^^^^^ constants require immutable values
[01:29:35] - error: cannot mutate statics in the initializer of another static
[01:29:35] - error: cannot mutate statics in the initializer of another static
[01:29:35] + error[E0017]: references in statics may only refer to immutable values
[01:29:35] 8   --> $DIR/E0388.rs:5:39
[01:29:35] 9    |
[01:29:35] 10 LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
[01:29:35] -    |                                       ^^^^^^
[01:29:35] +    |                                       ^^^^^^ statics require immutable values
[01:29:35] 12 
[01:29:35] 12 
[01:29:35] - error[E0017]: references in statics may only refer to immutable values
[01:29:35] + error: cannot mutate statics in the initializer of another static
[01:29:35] 14   --> $DIR/E0388.rs:5:39
[01:29:35] 15    |
[01:29:35] 16 LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
[01:29:35] -    |                                       ^^^^^^ statics require immutable values
[01:29:35] +    |                                       ^^^^^^
[01:29:35] 18 
[01:29:35] 18 
[01:29:35] 19 error[E0596]: cannot borrow immutable static item `X` as mutable
[01:29:35] 20   --> $DIR/E0388.rs:5:39
[01:29:35] 
[01:29:35] The actual stderr differed from the expected stderr.
[01:29:35] The actual stderr differed from the expected stderr.
[01:29:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388.nll/E0388.nll.stderr
[01:29:35] To update references, rerun the tests and pass the `--bless` flag
[01:29:35] To only update this specific test, also pass `--test-args error-codes/E0388.rs`
[01:29:35] error: 1 errors occurred comparing output.
[01:29:35] status: exit code: 1
[01:29:35] status: exit code: 1
[01:29:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0388.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388.nll/auxiliary" "-A" "unused"
[01:29:35] ------------------------------------------
[01:29:35] 
[01:29:35] ------------------------------------------
[01:29:35] stderr:
[01:29:35] stderr:
[01:29:35] ------------------------------------------
[01:29:35] {"message":"references in constants may only refer to immutable values","code":{"code":"E0017","explanation":"\nReferences in statics and constants may only refer to immutable values.\nErroneous code example:\n\n