plain
travis_time:end:14aebca0:start=1552913387814986384,finish=1552913390145520017,duration=2330533633
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:19:03] 
[01:19:03] running 99 tests
[01:19:19] .F.F.F..FFFF.F.F...................................F.F..........F.F.F..F..F.....F............FF....
[01:19:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:19:19] 
[01:19:19] ---- [incremental] incremental/change_add_field/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_add_field/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_add_field/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-call_fn_with_type_in_body` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_add_field/struct_point.rs","byte_start":1053,"byte_end":1142,"line_start":24,"line_end":24,"column_start":1,"column_end":90,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-call_fn_with_type_in_body\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":90}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-call_fn_with_type_in_body` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_add_field/struct_point.rs:24:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-call_fn_with_type_in_body\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/change_add_field/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:19:19] 
[01:19:19] ---- [incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_free_fn` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs","byte_start":467,"byte_end":547,"line_start":16,"line_end":16,"column_start":1,"column_end":81,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_free_fn\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":81}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_free_fn` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs:16:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_free_fn\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs","byte_start":373,"byte_end":466,"line_start":15,"line_end":15,"column_start":1,"column_end":94,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":94}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs:15:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs","byte_start":626,"byte_end":704,"line_start":18,"line_end":18,"column_start":1,"column_end":79,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_write_field\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/add_private_fn_at_krate_root_cc/struct_point.rs:18:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_write_field\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/add_private_fn_at_krate_root_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/change_private_fn/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_fn/struct_point.rs","byte_start":394,"byte_end":487,"line_start":15,"line_end":15,"column_start":1,"column_end":94,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":94}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_fn/struct_point.rs:15:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_fn/struct_point.rs","byte_start":488,"byte_end":584,"line_start":16,"line_end":16,"column_start":1,"column_end":97,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":97}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_fn/struct_point.rs:16:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/change_private_fn/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/change_private_fn_cc/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_fn_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn_cc/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_fn_cc/struct_point.rs","byte_start":342,"byte_end":435,"line_start":14,"line_end":14,"column_start":1,"column_end":94,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":94}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_fn_cc/struct_point.rs:14:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_fn_cc/struct_point.rs","byte_start":436,"byte_end":532,"line_start":15,"line_end":15,"column_start":1,"column_end":97,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":97}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_fn_cc/struct_point.rs:15:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/change_private_fn_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/change_private_impl_method/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_impl_method/struct_point.rs","byte_start":394,"byte_end":487,"line_start":15,"line_end":15,"column_start":1,"column_end":94,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":94}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_impl_method/struct_point.rs:15:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_impl_method/struct_point.rs","byte_start":488,"byte_end":584,"line_start":16,"line_end":16,"column_start":1,"column_end":97,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":97}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_impl_method/struct_point.rs:16:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/change_private_impl_method/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/change_pub_inherent_method_body/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_body/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs","byte_start":412,"byte_end":499,"line_start":15,"line_end":15,"column_start":1,"column_end":88,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_another_method\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":88}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs:15:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_another_method\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_changed_method` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs","byte_start":324,"byte_end":411,"line_start":14,"line_end":14,"column_start":1,"column_end":88,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_changed_method\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":88}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_changed_method` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs:14:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_changed_method\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs","byte_start":657,"byte_end":735,"line_start":18,"line_end":18,"column_start":1,"column_end":79,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_write_field\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_pub_inherent_method_body/struct_point.rs:18:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_write_field\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/change_pub_inherent_method_body/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/change_pub_inherent_method_sig/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_pub_inherent_method_sig/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs","byte_start":464,"byte_end":551,"line_start":16,"line_end":16,"column_start":1,"column_end":88,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_another_method\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":88}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_another_method` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs:16:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_another_method\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs","byte_start":709,"byte_end":787,"line_start":19,"line_end":19,"column_start":1,"column_end":79,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_write_field\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_write_field` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_pub_inherent_method_sig/struct_point.rs:19:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_write_field\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/change_pub_inherent_method_sig/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/change_private_impl_method_cc/struct_point.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_impl_method_cc/struct_point/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs","byte_start":579,"byte_end":672,"line_start":18,"line_end":18,"column_start":1,"column_end":94,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":94}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_same_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs:18:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_same_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs","byte_start":673,"byte_end":769,"line_start":19,"line_end":19,"column_start":1,"column_end":97,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":97}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `struct_point-fn_calls_methods_in_another_impl` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/change_private_impl_method_cc/struct_point.rs:19:1\n   |\nLL | #![rustc_partition_reused(module=\"struct_point-fn_calls_methods_in_another_impl\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/change_private_impl_method_cc/struct_point.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/commandline-args.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass3`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/commandline-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/commandline-args.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=2" "--verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/commandline-args/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `commandline_args` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/commandline-args.rs","byte_start":289,"byte_end":356,"line_start":10,"line_end":10,"column_start":1,"column_end":68,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"commandline_args\", cfg=\"rpass3\")]","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `commandline_args` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/commandline-args.rs:10:1\n   |\nLL | #![rustc_partition_reused(module=\"commandline_args\", cfg=\"rpass3\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/commandline-args.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/issue-35593.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass2`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-35593.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/issue-35593.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-35593/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `issue_35593` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/issue-35593.rs","byte_start":179,"byte_end":241,"line_start":8,"line_end":8,"column_start":1,"column_end":63,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"issue_35593\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `issue_35593` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/issue-35593.rs:8:1\n   |\nLL | #![rustc_partition_reused(module=\"issue_35593\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/issue-35593.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/issue-35593.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/issue-38222.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass2`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-38222.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/issue-38222.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-C" "debuginfo=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-38222/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `issue_38222` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/issue-38222.rs","byte_start":399,"byte_end":461,"line_start":15,"line_end":15,"column_start":1,"column_end":63,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"issue_38222\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `issue_38222` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/issue-38222.rs:15:1\n   |\nLL | #![rustc_partition_reused(module=\"issue_38222\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/issue-38222.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/issue-38222.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/krate-inherent.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent/krate-inherent.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inherent/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `krate_inherent-x` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/krate-inherent.rs","byte_start":127,"byte_end":194,"line_start":7,"line_end":7,"column_start":1,"column_end":68,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"krate_inherent-x\", cfg=\"cfail2\")]","highlight_start":1,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `krate_inherent-x` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/krate-inherent.rs:7:1\n   |\nLL | #![rustc_partition_reused(module=\"krate_inherent-x\", cfg=\"cfail2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/krate-inherent.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/krate-inherent.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/krate-inlined.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass2`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/krate-inlined.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/krate-inlined.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/krate-inlined/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `krate_inlined-x` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/krate-inlined.rs","byte_start":307,"byte_end":373,"line_start":10,"line_end":10,"column_start":1,"column_end":67,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"krate_inlined-x\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `krate_inlined-x` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/krate-inlined.rs:10:1\n   |\nLL | #![rustc_partition_reused(module=\"krate_inlined-x\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/krate-inlined.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/krate-inlined.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass2`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `main-some_mod` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/remapped_paths_cc/main.rs","byte_start":330,"byte_end":394,"line_start":11,"line_end":11,"column_start":1,"column_end":65,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"main-some_mod\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `main-some_mod` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/remapped_paths_cc/main.rs:11:1\n   |\nLL | #![rustc_partition_reused(module=\"main-some_mod\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `main` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/remapped_paths_cc/main.rs","byte_start":274,"byte_end":329,"line_start":10,"line_end":10,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"main\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `main` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/remapped_paths_cc/main.rs:10:1\n   |\nLL | #![rustc_partition_reused(module=\"main\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/remapped_paths_cc/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/remapped_paths_cc/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/remove-private-item-cross-crate/main.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass2`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remove-private-item-cross-crate/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/main.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zquery-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/remove-private-item-cross-crate/main/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `main` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/remove-private-item-cross-crate/main.rs","byte_start":249,"byte_end":304,"line_start":11,"line_end":11,"column_start":1,"column_end":56,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"main\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `main` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/remove-private-item-cross-crate/main.rs:11:1\n   |\nLL | #![rustc_partition_reused(module=\"main\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/remove-private-item-cross-crate/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/spans_in_type_debuginfo.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass2`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_in_type_debuginfo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/spans_in_type_debuginfo.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"variant is never constructed: `B`","code":{"code":"dead_code","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/incremental/spans_in_type_debuginfo.rs","byte_start":708,"byte_end":714,"line_start":38,"line_end":38,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        B(u32),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(dead_code)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variant is never constructed: `B`\n  --> /checkout/src/test/incremental/spans_in_type_debuginfo.rs:38:9\n   |\nLL |         B(u32),\n   |         ^^^^^^\n   |\n   = note: #[warn(dead_code)] on by default\n\n"}
[01:19:19] {"message":"CGU-reuse for `spans_in_type_debuginfo-enums` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/spans_in_type_debuginfo.rs","byte_start":246,"byte_end":326,"line_start":8,"line_end":8,"column_start":1,"column_end":81,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"spans_in_type_debuginfo-enums\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":81}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `spans_in_type_debuginfo-enums` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/spans_in_type_debuginfo.rs:8:1\n   |\nLL | #![rustc_partition_reused(module=\"spans_in_type_debuginfo-enums\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `spans_in_type_debuginfo-structs` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/spans_in_type_debuginfo.rs","byte_start":163,"byte_end":245,"line_start":7,"line_end":7,"column_start":1,"column_end":83,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"spans_in_type_debuginfo-structs\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":83}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `spans_in_type_debuginfo-structs` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/spans_in_type_debuginfo.rs:7:1\n   |\nLL | #![rustc_partition_reused(module=\"spans_in_type_debuginfo-structs\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/spans_in_type_debuginfo.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/spans_in_type_debuginfo.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/spike.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `rpass2`: compilation failed!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spike.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/spike.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spike/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `spike-y` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/spike.rs","byte_start":397,"byte_end":455,"line_start":12,"line_end":12,"column_start":1,"column_end":59,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"spike-y\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `spike-y` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/spike.rs:12:1\n   |\nLL | #![rustc_partition_reused(module=\"spike-y\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] {"message":"CGU-reuse for `spike` is `No` but should be at least `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/spike.rs","byte_start":278,"byte_end":334,"line_start":10,"line_end":10,"column_start":1,"column_end":57,"is_primary":true,"text":[{"text":"#![rustc_partition_reused(module=\"spike\", cfg=\"rpass2\")]","highlight_start":1,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `spike` is `No` but should be at least `PreLto`\n  --> /checkout/src/test/incremental/spike.rs:10:1\n   |\nLL | #![rustc_partition_reused(module=\"spike\", cfg=\"rpass2\")]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/spike.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/spike.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/thinlto/cgu_invalidated_via_import.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_invalidated_via_import.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/cgu_invalidated_via_import.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `cgu_invalidated_via_import-bar` is `No` but should be `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/thinlto/cgu_invalidated_via_import.rs","byte_start":630,"byte_end":785,"line_start":19,"line_end":21,"column_start":1,"column_end":45,"is_primary":true,"text":[{"text":"#![rustc_expected_cgu_reuse(module=\"cgu_invalidated_via_import-bar\",","highlight_start":1,"highlight_end":69},{"text":"                            cfg=\"cfail2\",","highlight_start":1,"highlight_end":42},{"text":"                            kind=\"pre-lto\")]","highlight_start":1,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `cgu_invalidated_via_import-bar` is `No` but should be `PreLto`\n  --> /checkout/src/test/incremental/thinlto/cgu_invalidated_via_import.rs:19:1\n   |\nLL | / #![rustc_expected_cgu_reuse(module=\"cgu_invalidated_via_import-bar\",\nLL | |                             cfg=\"cfail2\",\nLL | |                             kind=\"pre-lto\")]\n   | |____________________________________________^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/thinlto/cgu_invalidated_via_import.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] thread '[incremental] incremental/thinlto/cgu_invalidated_via_import.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
[01:19:19] 
[01:19:19] ---- [incremental] incremental/thinlto/independent_cgus_dont_affect_each_other.rs stdout ----
[01:19:19] 
[01:19:19] error in revision `cfail2`: test compilation failed although it shouldn't!
[01:19:19] status: exit code: 1
[01:19:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/thinlto/independent_cgus_dont_affect_each_other.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/independent_cgus_dont_affect_each_other.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/independent_cgus_dont_affect_each_other/auxiliary"
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] stderr:
[01:19:19] stderr:
[01:19:19] ------------------------------------------
[01:19:19] {"message":"CGU-reuse for `independent_cgus_dont_affect_each_other-bar` is `No` but should be `PreLto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/thinlto/independent_cgus_dont_affect_each_other.rs","byte_start":587,"byte_end":755,"line_start":18,"line_end":20,"column_start":1,"column_end":45,"is_primary":true,"text":[{"text":"#![rustc_expected_cgu_reuse(module=\"independent_cgus_dont_affect_each_other-bar\",","highlight_start":1,"highlight_end":82},{"text":"                            cfg=\"cfail2\",","highlight_start":1,"highlight_end":42},{"text":"                            kind=\"pre-lto\")]","highlight_start":1,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: CGU-reuse for `independent_cgus_dont_affect_each_other-bar` is `No` but should be `PreLto`\n  --> /checkout/src/test/incremental/thinlto/independent_cgus_dont_affect_each_other.rs:18:1\n   |\nLL | / #![rustc_expected_cgu_reuse(module=\"independent_cgus_dont_affect_each_other-bar\",\nLL | |                             cfg=\"cfail2\",\nLL | |                             kind=\"pre-lto\")]\n   | |____________________________________________^\n\n"}
[01:19:19] 
[01:19:19] ------------------------------------------
[01:19:19] 
[01:19:19] thread '[incremental] incremental/thinlto/independent_cgus_dont_affect_each_other.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3325:9
---
[01:19:19] test result: FAILED. 80 passed; 19 failed; 0 ignored; 0 measured; 0 filtered out
[01:19:19] 
[01:19:19] 
[01:19:19] 
[01:19:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:19:19] 
[01:19:19] 
[01:19:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:19] Build completed unsuccessfully in 0:11:41
[01:19:19] Build completed unsuccessfully in 0:11:41
[01:19:19] make: *** [check] Error 1
[01:19:19] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0155b918
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Mar 18 14:09:20 UTC 2019
---
travis_time:end:06802970:start=1552918162320948455,finish=1552918162325695112,duration=4746657
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1499ac90
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0be2b4a8
travis_time:start:0be2b4a8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a52d961
$ dmesg | grep -i kill
