plain
[01:31:30] diff of stderr:
[01:31:30] 
[01:31:30] 11   --> $DIR/two-phase-nonrecv-autoref.rs:69:11
[01:31:30] 12    |
[01:31:30] 13 LL |     fn twice_ten_so<F: FnOnce(i32) -> i32>(f: Box<F>) {
[01:31:30] -    |                     - consider adding a `Copy` constraint to this type argument
[01:31:30] +    |                                            - move occurs because `f` has type `std::boxed::Box<F>`, which does not implement the `Copy` trait
[01:31:30] 15 LL |         f(f(10));
[01:31:30] 16    |         - ^ value used here after move
[01:31:30] 
[01:31:30] 18    |         value moved here
[01:31:30] -    |
[01:31:30] -    |
[01:31:30] -    = note: move occurs because `f` has type `std::boxed::Box<F>`, which does not implement the `Copy` trait
[01:31:30] 21 
[01:31:30] 22 error[E0499]: cannot borrow `*f` as mutable more than once at a time
[01:31:30] 
[01:31:30] 
[01:31:30] 31 error[E0382]: use of moved value: `f`
[01:31:30] 33    |
[01:31:30] 33    |
[01:31:30] + LL |     fn twice_ten_oo(f: Box<FnOnce(i32) -> i32>) {
[01:31:30] +    |                     - move occurs because `f` has type `std::boxed::Box<dyn std::ops::FnOnce(i32) -> i32>`, which does not implement the `Copy` trait
[01:31:30] 34 LL |         f(f(10));
[01:31:30] 35    |         - ^ value used here after move
[01:31:30] 
[01:31:30] 37    |         value moved here
[01:31:30] -    |
[01:31:30] -    |
[01:31:30] -    = note: move occurs because `f` has type `std::boxed::Box<dyn std::ops::FnOnce(i32) -> i32>`, which does not implement the `Copy` trait
[01:31:30] 40 
[01:31:30] 41 error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
[01:31:30] 
[01:31:30] 
[01:31:30] The actual stderr differed from the expected stderr.
[01:31:30] The actual stderr differed from the expected stderr.
[01:31:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.ast.nll/two-phase-nonrecv-autoref.ast.nll.stderr
[01:31:30] To update references, rerun the tests and pass the `--bless` flag
[01:31:30] To only update this specific test, also pass `--test-args borrowck/two-phase-nonrecv-autoref.rs`
[01:31:30] 
[01:31:30] error in revision `ast`: 1 errors occurred comparing output.
[01:31:30] status: exit code: 1
[01:31:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.ast.nll/auxiliary" "-A" "unused"
[01:31:30] ------------------------------------------
[01:31:30] 
[01:31:30] ------------------------------------------
[01:31:30] stderr:
[01:31:30] stderr:
[01:31:30] ------------------------------------------
[01:31:30] {"message":"cannot borrow `*f` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n