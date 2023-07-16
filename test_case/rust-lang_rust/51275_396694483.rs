plain
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:29] 
[00:50:29] running 1502 tests
[00:50:34] ...............................................F............................................i.......
[00:50:40] ......................................................i................F............................
[00:50:48] ....................................................................................................
[00:50:52] ....................................................................................................
[00:50:55] ....................................................................................................
[00:51:00] ....................................................................................................
[00:51:00] ....................................................................................................
[00:51:05] ....................................................................................................
[00:51:10] ....................................................................................................
[00:51:16] ....................................................................................................
[00:51:21] ....i..............................................................................i................
[00:51:27] ....................................................................................................
[00:51:32] ....................................................................................................
[00:51:37] ....................................................................................................
[00:51:43] .................i.....................F............................................................
[00:51:44] failures:
[00:51:44] 
[00:51:44] ---- [ui (nll)] ui/borrowck/mut-borrow-of-mut-ref.rs stdout ----
[00:51:44] diff of stderr:
[00:51:44] diff of stderr:
[00:51:44] 
[00:51:44] 1 error[E0596]: cannot borrow immutable item `b` as mutable
[00:51:44] 3    |
[00:51:44] 3    |
[00:51:44] + LL | fn f(b: &mut i32) {
[00:51:44] +    |      - help: consider changing this to be mutable: `mut b`
[00:51:44] 4 LL |     g(&mut b) //~ ERROR cannot borrow
[00:51:44] 5    |       ^^^^^^ cannot borrow as mutable
[00:51:44] 
[00:51:44] 
[00:51:44] The actual stderr differed from the expected stderr.
[00:51:44] The actual stderr differed from the expected stderr.
[00:51:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-of-mut-ref.nll/mut-borrow-of-mut-ref.nll.stderr
[00:51:44] To update references, rerun the tests and pass the `--bless` flag
[00:51:44] To only update this specific test, also pass `--test-args borrowck/mut-borrow-of-mut-ref.rs`
[00:51:44] error: 1 errors occurred comparing output.
[00:51:44] status: exit code: 101
[00:51:44] status: exit code: 101
[00:51:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-of-mut-ref.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linuxd","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable item `b` as mutable\n  --> /checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs:18:7\n   |\nLL | fn f(b: &mut i32) {\n   |      - help: consider changing this to be mutable: `mut b`\nLL |     g(&mut b) //~ ERROR cannot borrow\n   |       ^^^^^^ cannot borrow as mutable\n\n"}
[00:51:44] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:51:44] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:51:44] ------------------------------------------
[00:51:44] 
[00:51:44] thread '[ui (nll)] ui/borrowck/mut-borrow-of-mut-ref.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:51:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:44] 
[00:51:44] ---- [ui (nll)] ui/did_you_mean/issue-31424.rs stdout ----
[00:51:44] diff of stderr:
[00:51:44] 
[00:51:44] 8   --> $DIR/issue-31424.rs:23:9
[00:51:44] 9    |
[00:51:44] 10 LL |     fn bar(self: &mut Self) {
[00:51:44] -    |            --------------- help: consider changing this to be mutable: `mut self`
[00:51:44] +    |            ---- help: consider changing this to be mutable: `mut self`
[00:51:44] 12 LL |         (&mut self).bar(); //~ ERROR cannot borrow
[00:51:44] 13    |         ^^^^^^^^^^^ cannot borrow as mutable
[00:51:44] 
[00:51:44] 
[00:51:44] The actual stderr differed from the expected stderr.
[00:51:44] The actual stderr differed from the expected stderr.
[00:51:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-31424.nll/issue-31424.nll.stderr
[00:51:44] To update references, rerun the tests and pass the `--bless` flag
[00:51:44] To only update this specific test, also pass `--test-args did_you_mean/issue-31424.rs`
[00:51:44] error: 1 errors occurred comparing output.
[00:51:44] status: exit code: 101
[00:51:44] status: exit code: 101
[00:51:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-31424.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-31424.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-31424.nll/auxiliary" "-A" "unused"
[00:51:44] ------------------------------------------
[00:51:44] 
[00:51:44] ------------------------------------------
[00:51:44] stderr:
[00:51:44] stderr:
[00:51:44] ------------------------------------------
[00:51:44] {"message":"cannot borrow immutable item `self` as mutable","code":{"code":"E0596","explanation":"\nThis error occurs because you tried to mutably borrow a non-mutable variable.\n\nExample of erroneous code:\n\n