plain
[01:40:36] +    |              --
[01:40:36] +    |              |
[01:40:36] +    |              borrow of `a` occurs here
[01:40:36] +    |              borrow later used here
[01:40:36] 6 LL |         for j in a {
[01:40:36] 7    |                  ^ move out of `a` occurs here
[01:40:36] 
[01:40:36] 9 error[E0382]: use of moved value: `a`
[01:40:36] 10   --> $DIR/borrow-for-loop-head.rs:4:18
[01:40:36] 11    |
[01:40:36] 11    |
[01:40:36] + LL |     let a = vec![1, 2, 3];
[01:40:36] +    |         - move occurs because `a` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
[01:40:36] + LL |     for i in &a {
[01:40:36] 12 LL |         for j in a {
[01:40:36] -    |                  ^ value moved here in previous iteration of loop
[01:40:36] -    |
[01:40:36] -    = note: move occurs because `a` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
[01:40:36] - help: consider borrowing this to avoid moving it into the for loop
[01:40:36] -    |
[01:40:36] - LL |         for j in &a {
[01:40:36] -    |                  ^^
[01:40:36] +    |                  ^ value moved here, in previous iteration of loop
[01:40:36] 21 error: aborting due to 2 previous errors
[01:40:36] 22 
[01:40:36] 
[01:40:36] 
[01:40:36] 
[01:40:36] The actual stderr differed from the expected stderr.
[01:40:36] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head.nll/borrow-for-loop-head.nll.stderr
[01:40:36] To update references, rerun the tests and pass the `--bless` flag
[01:40:36] To only update this specific test, also pass `--test-args suggestions/borrow-for-loop-head.rs`
[01:40:36] error: 1 errors occurred comparing output.
[01:40:36] status: exit code: 1
[01:40:36] status: exit code: 1
[01:40:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/borrow-for-loop-head.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head.nll/auxiliary" "-A" "unused"
[01:40:36] ------------------------------------------
[01:40:36] 
[01:40:36] ------------------------------------------
[01:40:36] stderr:
[01:40:36] stderr:
[01:40:36] ------------------------------------------
[01:40:36] {"message":"cannot move out of `a` because it is borrowed","code":{"code":"E0505","explanation":"\nA value was moved out while it was still borrowed.\n\nErroneous code example:\n\n