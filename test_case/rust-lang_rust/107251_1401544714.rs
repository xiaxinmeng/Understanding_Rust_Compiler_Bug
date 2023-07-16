plain
---- [ui] tests/ui/drop/drop_order.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop_order" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/drop_order/a"
-- if --
1
2
3
---
4
5
6
------------------------------------------
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: self.0.into_inner().into_iter().enumerate().all(|(idx, item)|\n        idx + 1 == item.try_into().unwrap())', fake-test-src-base/drop/drop_order.rs:242:9
------------------------------------------


---- [ui] tests/ui/mir/mir_let_chains_drop_order.rs stdout ----
---- [ui] tests/ui/mir/mir_let_chains_drop_order.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `[3, 2, 8, 7, 1]`,
 right: `[8, 7, 1, 3, 2]`', fake-test-src-base/mir/mir_let_chains_drop_order.rs:60:9
------------------------------------------


---- [ui] tests/ui/nll/issue-54556-niconii.rs stdout ----
---- [ui] tests/ui/nll/issue-54556-niconii.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/nll/issue-54556-niconii.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-54556-niconii" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-54556-niconii/auxiliary"
stdout: none
stderr: none


failures:
    [ui] tests/ui/drop/drop_order.rs
