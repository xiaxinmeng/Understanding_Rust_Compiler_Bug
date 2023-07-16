plain
[01:30:33] 
[01:30:33] 5    |               -------
[01:30:33] 6    |               |
[01:30:33] 7    |               immutable borrow occurs here
[01:30:33] -    |               immutable borrow used here, in later iteration of loop
[01:30:33] +    |               immutable borrow later used here
[01:30:33] 9 LL |         let cap = vector.capacity();
[01:30:33] 10 LL |         vector.extend(repeat(0));      //~ ERROR cannot borrow
[01:30:33] 
[01:30:33] 17    |               -------
[01:30:33] 18    |               |
[01:30:33] 19    |               immutable borrow occurs here
[01:30:33] 19    |               immutable borrow occurs here
[01:30:33] -    |               immutable borrow used here, in later iteration of loop
[01:30:33] 21 ...
[01:30:33] 21 ...
[01:30:33] 22 LL |         vector[1] = 5;   //~ ERROR cannot borrow
[01:30:33] 
[01:30:33] 
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] The actual stderr differed from the expected stderr.
[01:30:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-for-loop-head-linkage.nll/borrowck-for-loop-head-linkage.nll.stderr
[01:30:33] To update references, rerun the tests and pass the `--bless` flag
[01:30:33] To only update this specific test, also pass `--test-args borrowck/borrowck-for-loop-head-linkage.rs`
[01:30:33] error: 1 errors occurred comparing output.
[01:30:33] status: exit code: 1
[01:30:33] status: exit code: 1
[01:30:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-for-loop-head-linkage.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-for-loop-head-linkage.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-for-loop-head-linkage.nll/auxiliary" "-A" "unused"
[01:30:33] ------------------------------------------
[01:30:33] 
[01:30:33] ------------------------------------------
[01:30:33] stderr:
[01:30:33] stderr:
[01:30:33] ------------------------------------------
[01:30:33] {"message":"cannot borrow `vector` as mutable because it is also borrowed as immutable","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n