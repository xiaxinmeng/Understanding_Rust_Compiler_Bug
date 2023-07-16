plain
 finished in 0.474 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 153 tests
FFFFF.....FF.F....F.................FFF.FFF.FFF.F.FFFFFFFFFFFFFFFFFFF.FFFF.F.....F.................. 100/153
...............F.F.F.....FF.F..F...F.FF..F......F....

---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2", loaded_from_disk="typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/change_add_field/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck,fn_sig,optimized_mir", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck,optimized_mir", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck,optimized_mir", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck,fn_sig,optimized_mir", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck,fn_sig,optimized_mir", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck,fn_sig,optimized_mir", cfg="cfail2")]

error: aborting due to 7 previous errors
------------------------------------------



---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck,optimized_mir", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/callee_caller_cross_crate/b.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/callee_caller_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/callee_caller_cross_crate/b/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="typeck", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] incremental/crate_hash_reorder.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/crate_hash_reorder.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/crate_hash_reorder.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/crate_hash_reorder/auxiliary"
stdout: none
--- stderr -------------------------------
error: `hir_owner(X)` should be clean but is not
   |
LL | / pub struct X {
LL | |     pub x: u32,
LL | | }
LL | | }
   | |_^

error: `hir_owner_nodes(X)` should be clean but is not
   |
LL | / pub struct X {
LL | |     pub x: u32,
LL | | }
---


---- [incremental] incremental/hashes/exported_vs_not.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/exported_vs_not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/exported_vs_not.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/exported_vs_not/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]

error: aborting due to 3 previous errors
------------------------------------------



---- [incremental] incremental/hashes/closure_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zmir-opt-level=0" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir, typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, typeck, optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2", except = "hir_owner_nodes, typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir, typeck")]

error: aborting due to 6 previous errors
------------------------------------------



---- [incremental] incremental/hashes/call_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="hir_owner_nodes,typeck", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]

error: aborting due to 9 previous errors
------------------------------------------



---- [incremental] incremental/dirty_clean.rs stdout ----

error in revision `cfail2`: /checkout/src/test/incremental/dirty_clean.rs:28: unexpected error: '28:5: 31:7: found unchecked `#[rustc_clean]` attribute'

error in revision `cfail2`: /checkout/src/test/incremental/dirty_clean.rs:45: unexpected error: '45:5: 45:50: found unchecked `#[rustc_clean]` attribute'

error in revision `cfail2`: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
    Error {
        line_num: 28,
        kind: Some(
            Error,
            Error,
        ),
        msg: "28:5: 31:7: found unchecked `#[rustc_clean]` attribute",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:5: 45:50: found unchecked `#[rustc_clean]` attribute",
]

thread '[incremental] incremental/dirty_clean.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1382:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [incremental] incremental/hashes/extern_mods.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2", except = "hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2", except = "hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg = "cfail2", except = "hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg = "cfail2")]

error: aborting due to 12 previous errors
------------------------------------------



---- [incremental] incremental/hashes/consts.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/consts.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/consts/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]

error: aborting due to 9 previous errors
------------------------------------------



---- [incremental] incremental/hashes/enum_defs.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,predicates_of,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,predicates_of,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,predicates_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,predicates_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,predicates_of,type_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,predicates_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,predicates_of")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,generics_of,predicates_of,type_of")]
---
   |
LL |     struct First;
   |            ^^^^^

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2",except="hir_owner")]

error: aborting due to previous error; 2 warnings emitted
------------------------------------------



---- [incremental] incremental/hashes/enum_constructors.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zmir-opt-level=0" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | /     #[rustc_clean(
LL | |         cfg="cfail2",
LL | |         except="fn_sig,hir_owner,hir_owner_nodes,optimized_mir,\
LL | |                 typeck"
LL | |     )]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | / #[rustc_clean(
LL | |     cfg="cfail2",
LL | |     except="hir_owner_nodes,optimized_mir,typeck"
LL | | )]
   | |__^

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | / #[rustc_clean(
LL | |     cfg="cfail2",
LL | |     except="hir_owner_nodes,optimized_mir,typeck"
LL | | )]
   | |__^

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | /     #[rustc_clean(
LL | |         cfg="cfail2",
LL | |         except="fn_sig,hir_owner,hir_owner_nodes,optimized_mir,\
LL | |                 typeck"
LL | |     )]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | /     #[rustc_clean(
LL | |         cfg="cfail2",
LL | |         except="fn_sig,hir_owner,hir_owner_nodes,optimized_mir,\
LL | |                 typeck"
LL | |     )]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]

error: aborting due to 15 previous errors
------------------------------------------



---- [incremental] incremental/hashes/inline_asm.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inline_asm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/inline_asm.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inline_asm/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir")]

error: aborting due to 6 previous errors
------------------------------------------



---- [incremental] incremental/hashes/indexing_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes,typeck", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes,typeck", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes,typeck", cfg="cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes,typeck", cfg="cfail2")]

error: aborting due to 7 previous errors
------------------------------------------



---- [incremental] incremental/hashes/loop_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/loop_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/loop_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/loop_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir, typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, optimized_mir, typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes, typeck, optimized_mir")]

error: aborting due to 8 previous errors
------------------------------------------



---- [incremental] incremental/hashes/let_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]

error: aborting due to 12 previous errors
------------------------------------------



---- [incremental] incremental/hashes/if_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,typeck")]

error: aborting due to 8 previous errors
------------------------------------------



---- [incremental] incremental/hashes/match_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,typeck")]

---

warning: function is never used: `baz`
  --> /checkout/src/test/incremental/ich_method_call_trait_scope.rs:35:8
   |
LL |     fn baz() {


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except="typeck", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="rpass2")]

error: aborting due to 2 previous errors; 3 warnings emitted
------------------------------------------



---- [incremental] incremental/hello_world.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hello_world.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world/hello_world.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hello_world/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="rpass2")]

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] incremental/ich_resolve_results.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/ich_resolve_results.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/ich_resolve_results.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/ich_resolve_results/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function is never used: `test`
   |
   |
LL | fn test<T>() { }
   |
   = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `Foo`
---

warning: struct is never constructed: `Foo`
  --> /checkout/src/test/incremental/ich_resolve_results.rs:16:16
   |
LL |     pub struct Foo(pub i64);

warning: function is never used: `in_expr`
  --> /checkout/src/test/incremental/ich_resolve_results.rs:34:8
   |
---
   |
LL |     fn in_type() {
   |        ^^^^^^^

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="rpass2")]

error: aborting due to 2 previous errors; 5 warnings emitted
------------------------------------------



---- [incremental] incremental/hygiene/load_cached_hygiene.rs stdout ----

error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/load_cached_hygiene.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
  --> /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:16:1
   |
LL | #[rustc_clean(except="hir_owner_nodes,typeck,optimized_mir,promoted_mir", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
  --> /checkout/src/test/incremental/hygiene/auxiliary/cached_hygiene.rs:32:1
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] incremental/issue-42602.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-42602.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/issue-42602.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-42602/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg="cfail2")]

error: aborting due to previous error
------------------------------------------



---- [incremental] incremental/source_loc_macros.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/source_loc_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/source_loc_macros.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/source_loc_macros/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes,optimized_mir", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="hir_owner_nodes,optimized_mir", cfg="rpass2")]

error: aborting due to 5 previous errors
------------------------------------------



---- [incremental] incremental/span_hash_stable/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/span_hash_stable/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/a" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/span_hash_stable/main/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
  --> /checkout/src/test/incremental/span_hash_stable/auxiliary/sub1.rs:1:1
   |
LL | #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
  --> /checkout/src/test/incremental/span_hash_stable/auxiliary/sub2.rs:1:1
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] incremental/spans_significant_w_debuginfo.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_debuginfo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/spans_significant_w_debuginfo.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/a" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_debuginfo/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except = "hir_owner,hir_owner_nodes,optimized_mir", cfg = "rpass2")]

error: aborting due to previous error
------------------------------------------



---- [incremental] incremental/rlib_cross_crate/b.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/rlib_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib_cross_crate/b/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL |     let x: a::Y = 'c';
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default


warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X() -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="typeck,optimized_mir", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 2 previous errors; 3 warnings emitted
------------------------------------------



---- [incremental] incremental/spans_significant_w_panic.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_significant_w_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/spans_significant_w_panic.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_significant_w_panic/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except = "hir_owner,hir_owner_nodes,optimized_mir", cfg = "rpass2")]

error: aborting due to previous error
------------------------------------------



---- [incremental] incremental/string_constant.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/string_constant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/string_constant.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/string_constant/auxiliary"
stdout: none
--- stderr -------------------------------
error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(except = "hir_owner,hir_owner_nodes,optimized_mir,promoted_mir", cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg = "cfail2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL |     #[rustc_clean(cfg = "cfail2")]

error: aborting due to 3 previous errors
------------------------------------------



---- [incremental] incremental/struct_add_field.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_add_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field/struct_add_field.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_add_field/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default


warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X(x: X) -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(embed: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="fn_sig,typeck", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="typeck", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 3 previous errors; 4 warnings emitted
------------------------------------------



---- [incremental] incremental/struct_change_field_type.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_field_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type/struct_change_field_type.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |                   ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
  --> /checkout/src/test/incremental/struct_change_field_type.rs:41:9
  --> /checkout/src/test/incremental/struct_change_field_type.rs:41:9
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: field is never read: `x`
  --> /checkout/src/test/incremental/struct_change_field_type.rs:20:5
   |
LL |     x: X
LL |     x: X
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X() -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="typeck", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="typeck", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 3 previous errors; 6 warnings emitted
------------------------------------------



---- [incremental] incremental/struct_change_nothing.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_nothing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing/struct_change_nothing.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_nothing/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |                   ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
  --> /checkout/src/test/incremental/struct_change_nothing.rs:41:9
  --> /checkout/src/test/incremental/struct_change_nothing.rs:41:9
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: field is never read: `x`
  --> /checkout/src/test/incremental/struct_change_nothing.rs:20:5
   |
LL |     x: X
LL |     x: X
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X() -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(x: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 3 previous errors; 6 warnings emitted
------------------------------------------



---- [incremental] incremental/struct_remove_field.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_remove_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field/struct_remove_field.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_remove_field/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default


warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X(x: X) -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_EmbedX` should have a snake case name
   |
   |
LL | pub fn use_EmbedX(embed: EmbedX) -> u32 {
   |        ^^^^^^^^^^ help: convert the identifier to snake case: `use_embed_x`

warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="typeck,fn_sig", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(except="typeck", cfg="rpass2")]


error: found unchecked `#[rustc_clean]` attribute
   |
   |
LL | #[rustc_clean(cfg="rpass2")]

error: aborting due to 3 previous errors; 4 warnings emitted
------------------------------------------



---- [incremental] incremental/struct_change_field_type_cross_crate/b.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/struct_change_field_type_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type_cross_crate/b/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/struct_change_field_type_cross_crate/b/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL |     let x: Y = Y { y: 'c' };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
