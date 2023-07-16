plain
[00:44:39] 
[00:44:39] ---- [ui (nll)] ui/generator/yield-while-iterating.rs stdout ----
[00:44:39]  diff of stderr:
[00:44:39] 
[00:44:39] 6 LL |             yield();
[00:44:39] 7    |             ------- possible yield occurs here
[00:44:39] 8 
[00:44:39] - error[E0597]: borrowed value does not live long enough
[00:44:39] -   --> $DIR/yield-while-iterating.rs:50:17
[00:44:39] -    |
[00:44:39] - LL |       let mut b = || {
[00:44:39] -    |  _________________^
[00:44:39] - LL | |         for p in &mut x {
[00:44:39] - LL | |             yield p;
[00:44:39] - LL | |         }
[00:44:39] - LL | |     };
[00:44:39] -    | |     |
[00:44:39] -    | |     |
[00:44:39] -    | |_____temporary value only lives until here
[00:44:39] -    |       temporary value does not live long enough
[00:44:39] - 
[00:44:39] 23 error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
[00:44:39] 24   --> $DIR/yield-while-iterating.rs:67:20
[00:44:39] 
[00:44:39] 
[00:44:39] 35 LL |       b.resume();
[00:44:39] 36    |       - borrow later used here
[00:44:39] 37 
[00:44:39] - error[E0597]: borrowed value does not live long enough
[00:44:39] -   --> $DIR/yield-while-iterating.rs:62:17
[00:44:39] -    |
[00:44:39] - LL |       let mut b = || {
[00:44:39] -    |  _________________^
[00:44:39] - LL | |         for p in &mut x {
[00:44:39] - LL | |             yield p;
[00:44:39] - LL | |         }
[00:44:39] - LL | |     };
[00:44:39] -    | |     |
[00:44:39] -    | |     |
[00:44:39] -    | |_____temporary value only lives until here
[00:44:39] -    |       temporary value does not live long enough
[00:44:39] + error: aborting due to 2 previous errors
[00:44:39] - error: aborting due to 4 previous errors
[00:44:39] - 
[00:44:39] - Some errors occurred: E0502, E0597, E0626.
[00:44:39] + Some errors occurred: E0502, E0626.
[00:44:39] + Some errors occurred: E0502, E0626.
[00:44:39] 55 For more information about an error, try `rustc --explain E0502`.
[00:44:39] 56 
[00:44:39] 
[00:44:39] 
[00:44:39] The actual stderr differed from the expected stderr.
[00:44:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.nll.stderr
[00:44:39] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'generator/yield-while-iterating.rs'
[00:44:39] 
[00:44:39] error: 1 errors occurred comparing output.
[00:44:39] status: exit code: 101
[00:44:39] status: exit code: 101
[00:44:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-while-iterating.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.stage2-x86_64-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:39] ------------------------------------------
[00:44:39] 
[00:44:39] ------------------------------------------
[00:44:39] stderr:
[00:44:39] stderr:
[00:44:39] ------------------------------------------
[00:44:39] {"message":"borrow may still be in use when generator yields","code":{"code":"E0626","explanation":"\nThis error occurs because a borrow in a generator persists across a\nyield point.\n\n