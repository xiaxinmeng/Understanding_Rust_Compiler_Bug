plain
........................................................................................ 2816/13613
........................................................................................ 2904/13613
........................................................................................ 2992/13613
....................................................................i................... 3080/13613
..................................i....................................F.FFFFFFF........ 3168/13613
........................................................................................ 3344/13613
..........iiiii......................................................................... 3432/13613
........................................................................................ 3520/13613
........................................................................................ 3608/13613
---
failures:

---- [ui] src/test/ui/associated-type-bounds/traits-assoc-type-macros.rs stdout ----
normalized stderr:
warning: Error finalizing incremental compilation session directory `$TEST_BUILD_DIR/associated-type-bounds/traits-assoc-type-macros/traits-assoc-type-macros.inc/traits_assoc_type_macros-32u8onpztv4vz/s-ge125wtzy1-1bwj9mz-working`: No such file or directory (os error 2)
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args associated-type-bounds/traits-assoc-type-macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/traits-assoc-type-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros/traits-assoc-type-macros.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/traits-assoc-type-macros/traits-assoc-type-macros.inc/traits_assoc_type_macros-32u8onpztv4vz/s-ge125wtzy1-1bwj9mz-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/async-await/issues/issue-64964.rs stdout ----
normalized stderr:
warning: Error finalizing incremental compilation session directory `$TEST_BUILD_DIR/async-await/issues/issue-64964/issue-64964.inc/issue_64964-ctxg4sx9uswo/s-ge125ys77n-1blmpy1-working`: No such file or directory (os error 2)
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args async-await/issues/issue-64964.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64964.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964/issue-64964.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964/issue-64964.inc/issue_64964-ctxg4sx9uswo/s-ge125ys77n-1blmpy1-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-caller-callee.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/dep-graph-caller-callee.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/dep-graph-caller-callee.inc/dep_graph_caller_callee-1v6e2wh2gam0r/s-ge1268zpbw-p4gw6y-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-struct-signature.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-struct-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/dep-graph-struct-signature.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-struct-signature/dep-graph-struct-signature.inc/dep_graph_struct_signature-2m9u927mll6du/s-ge1268zs1y-jddmm1-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/dep-graph-trait-impl-two-traits.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits/dep-graph-trait-impl-two-traits.inc/dep_graph_trait_impl_two_traits-lsg3h713y8ui/s-ge126903zk-ga8oum-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/dep-graph-assoc-type-codegen.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/dep-graph-assoc-type-codegen.inc/dep_graph_assoc_type_codegen-2b1z66e588b1j/s-ge12690330-pkk7js-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-trait-impl.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/dep-graph-trait-impl.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/dep-graph-trait-impl.inc/dep_graph_trait_impl-r12c6fy6tysf/s-ge126908yb-k3khxl-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-variance-alias.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/dep-graph-variance-alias.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/dep-graph-variance-alias.inc/dep_graph_variance_alias-5wtadqns7vzj/s-ge12690k6h-klcbdu-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-type-alias.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-type-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/dep-graph-type-alias.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-type-alias/dep-graph-type-alias.inc/dep_graph_type_alias-1f63sjcc70583/s-ge12690me2-16q8fhc-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/dep-graph-trait-impl-two-traits-same-method.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/dep-graph-trait-impl-two-traits-same-method.inc/dep_graph_trait_impl_two_traits_same_method-3tfoo84tn2bwv/s-ge126909pk-16vui3k-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/traits/vtable/issue-91807.rs stdout ----
normalized stderr:
warning: Error finalizing incremental compilation session directory `$TEST_BUILD_DIR/traits/vtable/issue-91807/issue-91807.inc/issue_91807-1xl1n26753jo8/s-ge127qpqx6-1s47kr5-working`: No such file or directory (os error 2)
warning: 1 warning emitted



---
To only update this specific test, also pass `--test-args traits/vtable/issue-91807.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/issue-91807.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/issue-91807/issue-91807.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/issue-91807" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/issue-91807/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/issue-91807/issue-91807.inc/issue_91807-1xl1n26753jo8/s-ge127qpqx6-1s47kr5-working`: No such file or directory (os error 2)
warning: 1 warning emitted
------------------------------------------


