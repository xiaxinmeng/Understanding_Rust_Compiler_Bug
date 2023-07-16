plain
[00:44:12] failures:
[00:44:12] 
[00:44:12] ---- [ui (nll)] ui/generator/borrowing.rs stdout ----
[00:44:12]  
[00:44:12] error: ui test compiled successfully!
[00:44:12] status: exit code: 0
[00:44:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/borrowing.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/borrowing.stage2-x86_64-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/borrowing.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:12] ------------------------------------------
[00:44:12] 
[00:44:12] ------------------------------------------
[00:44:12] stderr:
---
[00:44:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:44:12] 
[00:44:12] ---- [ui (nll)] ui/generator/dropck.rs stdout ----
[00:44:12]  
[00:44:12] error: ui test compiled successfully!
[00:44:12] status: exit code: 0
[00:44:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/dropck.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/dropck.stage2-x86_64-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/dropck.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:12] ------------------------------------------
[00:44:12] 
[00:44:12] ------------------------------------------
[00:44:12] stderr:
---
[00:44:12] 
[00:44:12] ---- [ui (nll)] ui/generator/yield-while-iterating.rs stdout ----
[00:44:12]  diff of stderr:
[00:44:12] 
[00:44:12] 6 LL |             yield();
[00:44:12] 7    |             ------- possible yield occurs here
[00:44:12] 8 
[00:44:12] - error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
[00:44:12] -   --> $DIR/yield-while-iterating.rs:67:20
[00:44:12] -    |
[00:44:12] - LL |       let mut b = || {
[00:44:12] -    |  _________________-
[00:44:12] - LL | |         for p in &mut x {
[00:44:12] - LL | |             yield p;
[00:44:12] - LL | |         }
[00:44:12] - LL | |     };
[00:44:12] -    | |_____- mutable borrow occurs here
[00:44:12] - LL |       println!("{}", x[0]); //~ ERROR
[00:44:12] -    |                      ^ immutable borrow occurs here
[00:44:12] - LL |       b.resume();
[00:44:12] -    |       - borrow later used here
[00:44:12] + error: aborting due to previous error
[00:44:12] - error: aborting due to 2 previous errors
[00:44:12] - 
[00:44:12] - Some errors occurred: E0502, E0626.
[00:44:12] - For more information about an error, try `rustc --explain E0502`.
[00:44:12] - For more information about an error, try `rustc --explain E0502`.
[00:44:12] + For more information about this error, try `rustc --explain E0626`.
[00:44:12] 28 
[00:44:12] 
[00:44:12] 
[00:44:12] The actual stderr differed from the expected stderr.
[00:44:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.nll.stderr
[00:44:12] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'generator/yield-while-iterating.rs'
[00:44:12] 
[00:44:12] error: 1 errors occurred comparing output.
[00:44:12] status: exit code: 101
[00:44:12] status: exit code: 101
[00:44:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-while-iterating.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.stage2-x86_64-unknown-linux-gnu" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:44:12] ------------------------------------------
[00:44:12] 
[00:44:12] ------------------------------------------
[00:44:12] stderr:
[00:44:12] stderr:
[00:44:12] ------------------------------------------
[00:44:12] {"message":"borrow may still be in use when generator yields","code":{"code":"E0626","explanation":"\nThis error occurs because a borrow in a generator persists across a\nyield point.\n\n