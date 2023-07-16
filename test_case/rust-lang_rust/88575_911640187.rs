plain
 finished in 0.242 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 146 tests
FFFFFF....F.FFF................F........................................F.F....F...............F.F.. 100/146
.......F.....FFF...FF....F.......FF...........
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_another_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 5 previous errors



------------------------------------------


---- [incremental] incremental/change_symbol_export_status.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `change_symbol_export_status-mod2` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "change_symbol_export_status-mod2", cfg = "rpass2")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-fn_calls_changed_method` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_changed_method", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_another_method", cfg="cfail2")]

error: aborting due to 5 previous errors



------------------------------------------


---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_another_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 5 previous errors



------------------------------------------


---- [incremental] incremental/change_add_field/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-call_fn_with_type_in_body` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-call_fn_with_type_in_body", cfg="cfail2")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_another_method", cfg="cfail2")]

error: aborting due to 4 previous errors



------------------------------------------


---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_free_fn` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_free_fn", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 5 previous errors



------------------------------------------


---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_another_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 5 previous errors



------------------------------------------


---- [incremental] incremental/commandline-args.rs stdout ----

error in revision `rpass3`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `commandline_args` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="commandline_args", cfg="rpass3")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_another_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 5 previous errors



------------------------------------------


---- [incremental] incremental/change_implementation_cross_crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_implementation_cross_crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `main` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "main", cfg = "rpass2")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/issue-35593.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-35593.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/issue-35593.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `issue_35593` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="issue_35593", cfg="rpass2")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/issue-38222.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-38222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/issue-38222.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/a" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `issue_38222` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "issue_38222", cfg = "rpass2")]


error: CGU-reuse for `issue_38222-mod1` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "issue_38222-mod1", cfg = "rpass2")]

error: aborting due to 2 previous errors



------------------------------------------


---- [incremental] incremental/hygiene/load_cached_hygiene.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hygiene/load_cached_hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/load_cached_hygiene.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `load_cached_hygiene-call_unchanged_function` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="load_cached_hygiene-call_unchanged_function", cfg="rpass2")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/krate-inherent.rs stdout ----
---
   |         ^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

error: CGU-reuse for `spans_in_type_debuginfo-structs` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="spans_in_type_debuginfo-structs", cfg="rpass2")]


error: CGU-reuse for `spans_in_type_debuginfo-enums` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="spans_in_type_debuginfo-enums", cfg="rpass2")]

error: aborting due to 2 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/spike.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spike.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/spike.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `spike-y` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="spike-y", cfg="rpass2")]


error: CGU-reuse for `spike` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="spike", cfg="rpass2")]

error: aborting due to 2 previous errors



------------------------------------------


---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove-private-item-cross-crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `main` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="main", cfg="rpass2")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/thinlto/cgu_invalidated_via_import.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_via_import.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/cgu_invalidated_via_import.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `cgu_invalidated_via_import-bar` is `No` but should be `PreLto`
   |
   |
LL | / #![rustc_expected_cgu_reuse(module="cgu_invalidated_via_import-bar",
LL | |                             cfg="cfail2",
LL | |                             kind="pre-lto")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/thinlto/cgu_keeps_identical_fn.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_keeps_identical_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn/cgu_keeps_identical_fn.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `cgu_keeps_identical_fn-bar` is `No` but should be `PostLto`
   |
   |
LL | / #![rustc_expected_cgu_reuse(module="cgu_keeps_identical_fn-bar",
LL | |                             cfg="cfail2",
LL | |                             kind="post-lto")]

error: aborting due to previous error



------------------------------------------


---- [incremental] incremental/thinlto/independent_cgus_dont_affect_each_other.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/independent_cgus_dont_affect_each_other.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/independent_cgus_dont_affect_each_other.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: CGU-reuse for `independent_cgus_dont_affect_each_other-bar` is `No` but should be `PreLto`
   |
   |
LL | / #![rustc_expected_cgu_reuse(module="independent_cgus_dont_affect_each_other-bar",
LL | |                             cfg="cfail2",
LL | |                             kind="pre-lto")]


error: CGU-reuse for `independent_cgus_dont_affect_each_other-baz` is `No` but should be `PostLto`
   |
   |
LL | / #![rustc_expected_cgu_reuse(module="independent_cgus_dont_affect_each_other-baz",
LL | |                             cfg="cfail2",
LL | |                             kind="post-lto")]

error: aborting due to 2 previous errors


---
test result: FAILED. 121 passed; 25 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:17:15
