\n\nStatics are shared everywhere, and if they refer to mutable data one might\nviolate memory safety since holding multiple mutable references to shared data\nis not allowed.\n\nIf you really want global mutable state, try using `static mut` or a global\n`UnsafeCell`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0017.rs","byte_start":807,"byte_end":813,"line_start":18,"line_end":18,"column_start":38,"column_end":44,"is_primary":true,"text":[{"text":"static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017","highlight_start":38,"highlight_end":44}],"label":"statics require immutable values","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0017]: references in statics may only refer to immutable values\n  --> /checkout/src/test/ui/error-codes/E0017.rs:18:38\n   |\nLL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017\n   |                                      ^^^^^^ statics require immutable values\n\n"}
[00:51:54] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:51:54] {"message":"Some errors occurred: E0017, E0596.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0017, E0596.\n"}
[00:51:54] {"message":"For more information about an error, try `rustc --explain E0017`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0017`.\n"}
[00:51:54] ------------------------------------------
[00:51:54] 
[00:51:54] thread '[ui (nll)] ui/error-codes/E0017.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[00:51:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:54] 
[00:51:54] ---- [ui (nll)] ui/error-codes/E0388.rs stdout ----
[00:51:54] diff of stderr:
[00:51:54] 
[00:51:54] 4 LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
[00:51:54] 5    |                              ^^^^^^ constants require immutable values
[00:51:54] 6 
[00:51:54] + error: cannot mutate statics in the initializer of another static
[00:51:54] +   --> $DIR/E0388.rs:15:39
[00:51:54] +    |
[00:51:54] + LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
[00:51:54] + 
[00:51:54] + 
[00:51:54] 7 error[E0017]: references in statics may only refer to immutable values
[00:51:54] 8   --> $DIR/E0388.rs:15:39
[00:51:54] 
[00:51:54] 17    |                                       ^^^^^^ cannot borrow as mutable
[00:51:54] 18 
[00:51:54] 18 
[00:51:54] 19 error[E0017]: references in statics may only refer to immutable values
[00:51:54] -   --> $DIR/E0388.rs:17:38
[00:51:54] +   --> $DIR/E0388.rs:18:38
[00:51:54] 21    |
[00:51:54] 22 LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017
[00:51:54] 23    |                                      ^^^^^^ statics require immutable values
[00:51:54] 24 
[00:51:54] - error: aborting due to 4 previous errors
[00:51:54] + error: aborting due to 5 previous errors
[00:51:54] 26 
[00:51:54] 26 
[00:51:54] 27 Some errors occurred: E0017, E0596.
[00:51:54] 28 For more information about an error, try `rustc --explain E0017`.
[00:51:54] 
[00:51:54] 
[00:51:54] The actual stderr differed from the expected stderr.
[00:51:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388.nll/E0388.nll.stderr
[00:51:54] To update references, rerun the tests and pass the `--bless` flag
[00:51:54] To only update this specific test, also pass `--test-args error-codes/E0388.rs`
[00:51:54] error: 1 errors occurred comparing output.
[00:51:54] status: exit code: 1
[00:51:54] status: exit code: 1
[00:51:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0388.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388.nll/auxiliary" "-A" "unused"
[00:51:54] stdout:
[00:51:54] ----------- {"message":"cannot mutate statics in the initializer of another static","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0388.rs","byte_start":596,"byte_end":602,"line_start":15,"line_end":15,"column_start":39,"column_end":45,"is_primary":true,"text":[{"text":"static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017","highlight_start":39,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: cannot mutate statics in the initializer of another static\n  --> /checkout/src/test/ui/error-codes/E0388.rs:15:39\n   |\nLL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017\n   |                                       ^^^^^^\n\n"}
[00:51:54] {"message":"references in statics may only refer to immutable values","code":{"code":"E0017","explanation":"\nReferences in statics and constants may only refer to immutable values.\nErroneous code example:\n\n