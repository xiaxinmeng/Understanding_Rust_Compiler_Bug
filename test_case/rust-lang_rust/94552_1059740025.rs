plain
---- [ui (nll)] ui/nll/lint-no-err.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/lint-no-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/lint-no-err.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/lint-no-err.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0502]: cannot borrow `conflict` as mutable because it is also borrowed as immutable
   |
LL |     let prev = conflict.get();
LL |     let prev = conflict.get();
   |                -------------- immutable borrow occurs here
LL |     conflict.insert(*prev + *x);
   |     |               |
   |     |               immutable borrow later used here
   |     |               immutable borrow later used here
   |     mutable borrow occurs here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.

---
    [ui (nll)] ui/nll/lint-no-err.rs

test result: FAILED. 12492 passed; 1 failed; 166 ignored; 0 measured; 0 filtered out; finished in 105.20s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
