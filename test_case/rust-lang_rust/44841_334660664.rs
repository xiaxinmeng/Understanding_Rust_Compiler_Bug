
...
[00:58:39] ---- [incremental] incremental/spike.rs stdout ----
[00:58:39] 	
[00:58:39] error in revision `rpass2`: compilation failed!
[00:58:39] status: exit code: 101
[00:58:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spike.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-Z" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike.inc" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Z" "query-dep-graph" "-Zincremental-info" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike.stage2-x86_64-unknown-linux-gnu.incremental.libaux"
[00:58:39] stdout:
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] stderr:
[00:58:39] ------------------------------------------
[00:58:39] error: internal compiler error: unexpected panic
[00:58:39] 
[00:58:39] note: the compiler unexpectedly panicked. this is a bug.
[00:58:39] 
[00:58:39] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:58:39] 
[00:58:39] note: rustc 1.22.0-dev running on x86_64-unknown-linux-gnu
[00:58:39] 
[00:58:39] incremental: session directory: 6 files hard-linked
[00:58:39] incremental: session directory: 0 files copied
[00:58:39] thread 'rustc' panicked at 'no entry found for key', /checkout/src/libcore/option.rs:839:4
[00:58:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:39] 
[00:58:39] 
[00:58:39] ------------------------------------------
[00:58:39] 
[00:58:39] thread '[incremental] incremental/spike.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2433:8
[00:58:39] 
[00:58:39] 
[00:58:39] failures:
[00:58:39]     [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs
[00:58:39]     [incremental] incremental/cache_file_headers.rs
[00:58:39]     [incremental] incremental/change_add_field/struct_point.rs
[00:58:39]     [incremental] incremental/change_private_fn/struct_point.rs
[00:58:39]     [incremental] incremental/change_private_fn_cc/struct_point.rs
[00:58:39]     [incremental] incremental/change_private_impl_method/struct_point.rs
[00:58:39]     [incremental] incremental/change_private_impl_method_cc/struct_point.rs
[00:58:39]     [incremental] incremental/change_pub_inherent_method_body/struct_point.rs
[00:58:39]     [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs
[00:58:39]     [incremental] incremental/change_symbol_export_status.rs
[00:58:39]     [incremental] incremental/commandline-args.rs
[00:58:39]     [incremental] incremental/issue-35593.rs
[00:58:39]     [incremental] incremental/issue-38222.rs
[00:58:39]     [incremental] incremental/krate-inherent.rs
[00:58:39]     [incremental] incremental/krate-inlined.rs
[00:58:39]     [incremental] incremental/remapped_paths_cc/main.rs
[00:58:39]     [incremental] incremental/remove-private-item-cross-crate/main.rs
[00:58:39]     [incremental] incremental/spans_in_type_debuginfo.rs
[00:58:39]     [incremental] incremental/spike.rs
[00:58:39] 
[00:58:39] test result: [31mFAILED(B[m. 59 passed; 19 failed; 0 ignored; 0 measured; 0 filtered out
