\n\nStatics are shared everywhere, and if they refer to mutable data one might\nviolate memory safety since holding multiple mutable references to shared data\nis not allowed.\n\nIf you really want global mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0017.rs","byte_start":728,"byte_end":734,"line_start":17,"line_end":17,"column_start":38,"column_end":44,"is_primary":true,"text":[{"text":"static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017","highlight_start":38,"highlight_end":44}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/error-codes/E0017.rs:17:38\n   |\nLL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017\n   |                                      ^^^^^^ statics require immutable values\n\n"}
[00:54:37] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:54:37] {"message":"Some errors occurred: E0017, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0017, E0596.\n"}
[00:54:37] {"message":"For more information about an error, try `rustc --explain E0017`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0017`.\n"}
[00:54:37] ------------------------------------------
[00:54:37] 
[00:54:37] thread '[ui] ui/error-codes/E0017.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:54:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:37] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:37] 
[00:54:37] ---- [ui] ui/error-codes/E0388.rs stdout ----
[00:54:37] diff of stderr:
[00:54:37] 
[00:54:37] 4 LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
[00:54:37] 5    |                              ^^^^^^ constants require immutable values
[00:54:37] 6 
[00:54:37] + error: cannot mutate statics in the initializer of another static
[00:54:37] +   --> $DIR/E0388.rs:15:39
[00:54:37] +    |
[00:54:37] + LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
[00:54:37] + 
[00:54:37] + 
[00:54:37] 7 error[E0017]: references in statics may only refer to immutable values
[00:54:37] 8   --> $DIR/E0388.rs:15:39
[00:54:37] 
[00:54:37] 
[00:54:37] 22 LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017
[00:54:37] 23    |                                      ^^^^^^ statics require immutable values
[00:54:37] - error: aborting due to 4 previous errors
[00:54:37] + error: aborting due to 5 previous errors
[00:54:37] 26 
[00:54:37] 27 Some errors occurred: E0017, E0596.
[00:54:37] 27 Some errors occurred: E0017, E0596.
[00:54:37] 28 For more information about an error, try `rustc --explain E0017`.
[00:54:37] 
[00:54:37] 
[00:54:37] The actual stderr differed from the expected stderr.
[00:54:37] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/E0388.stderr
[00:54:37] To update references, rerun the tests and pass the `--bless` flag
[00:54:37] To only update this specific test, also pass `--test-args error-codes/E0388.rs`
[00:54:37] error: 1 errors occurred comparing output.
[00:54:37] status: exit code: 1
[00:54:37] status: exit code: 1
[00:54:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0388.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/auxiliary" "-A" "unused"
[00:54:37] ------------------------------------------
[00:54:37] 
[00:54:37] ------------------------------------------
[00:54:37] stderr:
[00:54:37] stderr:
[00:54:37] ------------------------------------------
[00:54:37] {"message":"references in constants may only refer to immutable values","code":{"code":"E0017","explanation":"\nReferences in statics and constants may only refer to immutable values.\nErroneous code example:\n\n