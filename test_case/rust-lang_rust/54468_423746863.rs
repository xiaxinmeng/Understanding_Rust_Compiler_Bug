plain
[00:46:24] ....................................................................................................
[00:46:27] ....................................................................................i...............
[00:46:30] ....................................................................................................
[00:46:33] .......................................i.i..ii......................................................
[00:46:37] ...................................................................................F................
[00:46:43] ....................................................................................................
[00:46:46] ....................................................................................................
[00:46:48] ....................................................................................................
[00:46:52] ................................................................................................i...
---
[00:51:35] ............................................i.......................................................
[00:51:35] .....
[00:51:35] failures:
[00:51:35] 
[00:51:35] ---- [ui] ui/nll/polonius-smoke-test.rs stdout ----
[00:51:35] normalized stderr:
[00:51:35] error[E0597]: `x` does not live long enough
[00:51:35]   --> $DIR/polonius-smoke-test.rs:7:5
[00:51:35]    |
[00:51:35] LL |     &x //~ ERROR
[00:51:35]    |     ^^ borrowed value does not live long enough
[00:51:35] LL | }
[00:51:35]    | - `x` dropped here while still borrowed
[00:51:35]    |
[00:51:35]    = note: borrowed value must be valid for the static lifetime...
[00:51:35] 
[00:51:35] error[E0503]: cannot use `x` because it was mutably borrowed
[00:51:35]   --> $DIR/polonius-smoke-test.rs:13:13
[00:51:35]    |
[00:51:35] LL |     let y = &mut x;
[00:51:35]    |             ------ borrow of `x` occurs here
[00:51:35] LL |     let z = x; //~ ERROR
[00:51:35]    |             ^ use of borrowed `x`
[00:51:35] LL |     let w = y;
[00:51:35] 
[00:51:35] error[E0505]: cannot move out of `x` because it is borrowed
[00:51:35] error[E0505]: cannot move out of `x` because it is borrowed
[00:51:35]   --> $DIR/polonius-smoke-test.rs:19:13
[00:51:35]    |
[00:51:35] LL |     let y = &mut *x;
[00:51:35]    |             ------- borrow of `*x` occurs here
[00:51:35] LL |     let z = x; //~ ERROR
[00:51:35]    |             ^ move out of `x` occurs here
[00:51:35] LL |     y
[00:51:35] 
[00:51:35] error: aborting due to 3 previous errors
[00:51:35] 
[00:51:35] Some errors occurred: E0503, E0505, E0597.
[00:51:35] Some errors occurred: E0503, E0505, E0597.
[00:51:35] For more information about an error, try `rustc --explain E0503`.
[00:51:35] 
[00:51:35] 
[00:51:35] 
[00:51:35] The actual stderr differed from the expected stderr.
[00:51:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius-smoke-test/polonius-smoke-test.stderr
[00:51:35] To update references, rerun the tests and pass the `--bless` flag
[00:51:35] To only update this specific test, also pass `--test-args nll/polonius-smoke-test.rs`
[00:51:35] error: 1 errors occurred comparing output.
[00:51:35] status: exit code: 1
[00:51:35] status: exit code: 1
[00:51:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/polonius-smoke-test.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius-smoke-test/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Zpolonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius-smoke-test/auxiliary" "-A" "unused"
[00:51:35] ------------------------------------------
[00:51:35] 
[00:51:35] ------------------------------------------
[00:51:35] stderr:
[00:51:35] stderr:
[00:51:35] ------------------------------------------
[00:51:35] {"message":"`x` does not live long enough","code":{"code":"E0597","explanation":"\nThis error occurs because a borrow was made inside a variable which has a\ngreater lifetime than the borrowed one.\n\nExample of erroneous code:\n\n