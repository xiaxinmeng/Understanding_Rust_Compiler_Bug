plain
[01:24:59] 
[01:24:59] ---- [ui (nll)] ui/augmented-assignments.rs stdout ----
[01:24:59] diff of stderr:
[01:24:59] 
[01:24:59] 9 LL | |     //~^ value used here after move
[01:24:59] 10 LL | |     +=
[01:24:59] 11 LL | |     x;  //~ value moved here
[01:24:59] +    | |     ^
[01:24:59] 13    | |     |
[01:24:59] 14    | |_____move out of `x` occurs here
[01:24:59] 15    |       borrow later used here
[01:24:59] 15    |       borrow later used here
[01:24:59] 
[01:24:59] 
[01:24:59] The actual stderr differed from the expected stderr.
[01:24:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments.nll/augmented-assignments.nll.stderr
[01:24:59] To update references, rerun the tests and pass the `--bless` flag
[01:24:59] To only update this specific test, also pass `--test-args augmented-assignments.rs`
[01:24:59] error: 1 errors occurred comparing output.
[01:24:59] status: exit code: 1
[01:24:59] status: exit code: 1
[01:24:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/augmented-assignments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/augmented-assignments.nll/auxiliary" "-A" "unused"
[01:24:59] ------------------------------------------
[01:24:59] 
[01:24:59] ------------------------------------------
[01:24:59] stderr:
[01:24:59] stderr:
[01:24:59] ------------------------------------------
[01:24:59] {"message":"cannot move out of `x` because it is borrowed","code":{"code":"E0505","explanation":"\nA value was moved out while it was still borrowed.\n\nErroneous code example:\n\n