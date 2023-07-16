plain
.................................................................................................... 9300/11542
.................................................................................................... 9400/11542
.......................................................................i......i..................... 9500/11542
.................................................................................................... 9600/11542
.................iiiiiii..iiiiii.i.................................................................. 9700/11542
.................................................................................................... 9900/11542
.................................................................................................... 10000/11542
.................................................................................................... 10100/11542
.................................................................................................... 10200/11542
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.067 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 127 tests
FFFFFF.....F....FFF....F.......F............................FF.F...F........F.F.......F..F..F.....F. 100/127
..F....F........FF.........

---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_another_method", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_changed_method` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_changed_method", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 5 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


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

error: aborting due to 5 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/change_symbol_export_status.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_symbol_export_status.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/change_symbol_export_status.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_symbol_export_status/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `change_symbol_export_status-mod2` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "change_symbol_export_status-mod2", cfg = "rpass2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_another_method", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 4 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


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

error: aborting due to 5 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/change_add_field/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-call_fn_with_type_in_body` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-call_fn_with_type_in_body", cfg="cfail2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/commandline-args.rs stdout ----

error in revision `rpass3`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=2
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `commandline_args` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="commandline_args", cfg="rpass3")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_free_fn` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_free_fn", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]

error: aborting due to 5 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


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

error: aborting due to 5 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `struct_point-fn_make_struct` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_make_struct", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_same_impl", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_read_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_read_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_write_field", cfg="cfail2")]


error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="struct_point-fn_calls_methods_in_another_impl", cfg="cfail2")]

error: aborting due to 5 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/dirty_clean.rs stdout ----

error in revision `cfail2`: incremental test compiled successfully!
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/dirty_clean.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/dirty_clean.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/dirty_clean/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any

---


---- [incremental] incremental/change_implementation_cross_crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_implementation_cross_crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_implementation_cross_crate/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `main` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "main", cfg = "rpass2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/issue-38222.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-38222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/issue-38222.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=1
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `issue_38222-mod1` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "issue_38222-mod1", cfg = "rpass2")]


error: CGU-reuse for `issue_38222` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "issue_38222", cfg = "rpass2")]

error: aborting due to 2 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/issue-35593.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-35593.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/issue-35593.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `issue_35593` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="issue_35593", cfg="rpass2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/issue-39828/issue-39828.rs stdout ----

error in revision `rpass2`: auxiliary build of "/checkout/src/test/incremental/issue-39828/auxiliary/generic.rs" failed to compile: 
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-39828/auxiliary/generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/issue-39828.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-39828/issue-39828/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type dylib
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `generic-fallback` is `No` but should be at least `PreLto`
  --> /checkout/src/test/incremental/issue-39828/auxiliary/generic.rs:4:1
   |
LL | #![rustc_partition_reused(module="generic-fallback.cgu", cfg="rpass2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/hygiene/load_cached_hygiene.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hygiene/load_cached_hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/load_cached_hygiene.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `load_cached_hygiene-call_unchanged_function` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="load_cached_hygiene-call_unchanged_function", cfg="rpass2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/lto-in-linker.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/lto-in-linker.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto-in-linker/lto-in-linker.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto-in-linker" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "rlib" "-C" "linker-plugin-lto" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/lto-in-linker/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C rpath -C debuginfo=0 -C linker-plugin-lto --crate-type rlib
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `lto_in_linker` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "lto_in_linker", cfg = "cfail2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/krate-inherent.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent/krate-inherent.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `krate_inherent-x` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "krate_inherent-x", cfg = "cfail2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/krate-inlined.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inlined.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/krate-inlined.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `krate_inlined-x` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "krate_inlined-x", cfg = "rpass2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `main` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="main", cfg="rpass2")]


error: CGU-reuse for `main-some_mod` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="main-some_mod", cfg="rpass2")]

error: aborting due to 2 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/rlib-lto.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/rlib-lto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib-lto/rlib-lto.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib-lto" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--crate-type" "rlib" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/rlib-lto/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0 -C lto --crate-type rlib
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `rlib_lto` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module = "rlib_lto", cfg = "cfail2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove-private-item-cross-crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `main` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="main", cfg="rpass2")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/spike.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spike.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/spike.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `spike` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="spike", cfg="rpass2")]


error: CGU-reuse for `spike-y` is `No` but should be at least `PreLto`
   |
   |
LL | #![rustc_partition_reused(module="spike-y", cfg="rpass2")]

error: aborting due to 2 previous errors; 1 warning emitted



------------------------------------------


---- [incremental] incremental/thinlto/cgu_invalidated_via_import.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_via_import.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/cgu_invalidated_via_import.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `cgu_invalidated_via_import-bar` is `No` but should be `PreLto`
   |
   |
LL | / #![rustc_expected_cgu_reuse(module="cgu_invalidated_via_import-bar",
LL | |                             cfg="cfail2",
LL | |                             kind="pre-lto")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/thinlto/cgu_keeps_identical_fn.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_keeps_identical_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn/cgu_keeps_identical_fn.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


error: CGU-reuse for `cgu_keeps_identical_fn-bar` is `No` but should be `PostLto`
   |
   |
LL | / #![rustc_expected_cgu_reuse(module="cgu_keeps_identical_fn-bar",
LL | |                             cfg="cfail2",
LL | |                             kind="post-lto")]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [incremental] incremental/thinlto/independent_cgus_dont_affect_each_other.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/independent_cgus_dont_affect_each_other.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/independent_cgus_dont_affect_each_other.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-nightly (7289dc057 2021-03-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
warning: could not decode incremental cache: Any


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

error: aborting due to 2 previous errors; 1 warning emitted


---
test result: FAILED. 101 passed; 26 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.38s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:54
