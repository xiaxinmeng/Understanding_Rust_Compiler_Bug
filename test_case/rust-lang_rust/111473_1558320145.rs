plain
........................................................................................ 13376/15036
........................................................................................ 13464/15036
........................................................................................ 13552/15036
........................................................................................ 13640/15036
.......................F.F.............................................................. 13728/15036
........................................................................................ 13904/15036
........................................................................................ 13992/15036
........................................................................................ 14080/15036
........................................................................................ 14168/15036
---
---- [ui] tests/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq/auxiliary" "-Ztrait-solver=next"
stdout: none
stderr: none

---- [ui] tests/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/alias_eq_substs_eq_not_intercrate/auxiliary" "-Ztrait-solver=next"
stdout: none
stderr: none


failures:
    [ui] tests/ui/traits/new-solver/alias_eq_dont_use_normalizes_to_if_substs_eq.rs
