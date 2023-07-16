plain
[01:20:27] 1 error[E0507]: cannot move out of borrowed content
[01:20:27] -   --> $DIR/access-mode-in-closures.rs:19:15
[01:20:27] +   --> $DIR/access-mode-in-closures.rs:18:15
[01:20:27] 3    |
[01:20:27] - LL |         match *s { sty(v) => v } //~ ERROR cannot move out
[01:20:27] -    |               ^^       - data moved here
[01:20:27] + LL |         match *s { S(v) => v } //~ ERROR cannot move out
[01:20:27] +    |               ^^     - data moved here
[01:20:27] 7    |               cannot move out of borrowed content
[01:20:27] 7    |               cannot move out of borrowed content
[01:20:27] 8    |               help: consider removing the `*`: `s`
[01:20:27] 9    |
[01:20:27] 9    |
[01:20:27] 10 note: move occurs because `v` has type `std::vec::Vec<isize>`, which does not implement the `Copy` trait
[01:20:27] +   --> $DIR/access-mode-in-closures.rs:18:22
[01:20:27] 12    |
[01:20:27] 12    |
[01:20:27] - LL |         match *s { sty(v) => v } //~ ERROR cannot move out
[01:20:27] -    |                        ^
[01:20:27] + LL |         match *s { S(v) => v } //~ ERROR cannot move out
[01:20:27] 15 
[01:20:27] 16 error: aborting due to previous error
[01:20:27] 17 
[01:20:27] 
[01:20:27] 
[01:20:27] 
[01:20:27] The actual stderr differed from the expected stderr.
[01:20:27] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/access-mode-in-closures.nll/access-mode-in-closures.nll.stderr
[01:20:27] To update references, rerun the tests and pass the `--bless` flag
[01:20:27] To only update this specific test, also pass `--test-args access-mode-in-closures.rs`
[01:20:27] error: 1 errors occurred comparing output.
[01:20:27] status: exit code: 1
[01:20:27] status: exit code: 1
[01:20:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/access-mode-in-closures.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/access-mode-in-closures.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/access-mode-in-closures.nll/auxiliary" "-A" "unused"
[01:20:27] ------------------------------------------
[01:20:27] 
[01:20:27] ------------------------------------------
[01:20:27] stderr:
[01:20:27] stderr:
[01:20:27] ------------------------------------------
[01:20:27] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n