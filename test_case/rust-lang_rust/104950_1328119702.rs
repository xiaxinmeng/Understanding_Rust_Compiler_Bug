plain
failures:

---- [ui] src/test/ui/typeck/hang-in-overflow.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/hang-in-overflow.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/typeck/hang-in-overflow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/typeck/hang-in-overflow/auxiliary"
stdout: none
--- stderr -------------------------------
memory allocation of 713031696 bytes failed


---- [ui] src/test/ui/traits/predicate_can_apply-hang.rs stdout ----


error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
Some tests failed in compiletest suite=ui mode=ui host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/predicate_can_apply-hang.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/traits/predicate_can_apply-hang" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/traits/predicate_can_apply-hang/auxiliary"
stdout: none
--- stderr -------------------------------
memory allocation of 713031696 bytes failed



failures:
