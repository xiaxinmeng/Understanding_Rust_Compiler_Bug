plain
[00:50:02] ....................................................................................................
[00:50:04] ....................................................................................................
[00:50:07] ....................................................................................................
[00:50:11] ........i...........................................................................................
[00:50:16] .......................F......................................................................i.....
[00:50:19] ..i.................................................................................................
[00:50:25] ....................................................................................................
[00:50:27] ....................................................................................................
[00:50:30] ....................................................................................................
[00:50:32] ....................................................................................................
---
[00:52:01] 
[00:52:01] ---- [ui] ui/consts/const-eval/const_raw_ptr_ops.rs stdout ----
[00:52:01] diff of stderr:
[00:52:01] 
[00:52:01] 22 LL | const Z2: i32 = unsafe { *(42 as *const i32) }; //~ ERROR cannot be used
[00:52:01] 24    |                          |
[00:52:01] 24    |                          |
[00:52:01] -    |                          tried to access memory with alignment 2, but alignment 4 is required
[00:52:01] +    |                          a memory access tried to interpret some bytes as a pointer
[00:52:01] 27 error: this constant cannot be used
[00:52:01] 28   --> $DIR/const_raw_ptr_ops.rs:27:1
[00:52:01] 
[00:52:01] 
[00:52:01] 
[00:52:01] The actual stderr differed from the expected stderr.
[00:52:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/const_raw_ptr_ops.stderr
[00:52:01] To update references, rerun the tests and pass the `--bless` flag
[00:52:01] To only update this specific test, also pass `--test-args consts/const-eval/const_raw_ptr_ops.rs`
[00:52:01] error: 1 errors occurred comparing output.
[00:52:01] status: exit code: 1
[00:52:01] status: exit code: 1
[00:52:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/auxiliary" "-A" "unused"
[00:52:01] ------------------------------------------
[00:52:01] 
[00:52:01] ------------------------------------------
[00:52:01] stderr:
[00:52:01] stderr:
[00:52:01] ------------------------------------------
[00:52:01] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs","byte_start":632,"byte_end":668,"line_start":16,"line_end":16,"column_start":17,"column_end":53,"is_primary":false,"text":[{"text":"const X: bool = &1 as *const i32 == &2 as *const i32; //~ ERROR cannot be used","highlight_start":17,"highlight_end":53}],"label":"\"pointer arithmetic or comparison\" needs an rfc before being allowed inside constants","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs","byte_start":616,"byte_end":669,"line_start":16,"line_end":16,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"const X: bool = &1 as *const i32 == &2 as *const i32; //~ ERROR cannot be used","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:16:1\n   |\nLL | const X: bool = &1 as *const i32 == &2 as *const i32; //~ ERROR cannot be used\n   | ^^^^^^^^^^^^^^^^------------------------------------^\n   |                 |\n   |                 \"pointer arithmetic or comparison\" needs an rfc before being allowed inside constants\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[00:52:01] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs","byte_start":905,"byte_end":934,"line_start":22,"line_end":22,"column_start":19,"column_end":48,"is_primary":false,"text":[{"text":"const Y2: usize = &1 as *const i32 as usize + 1; //~ ERROR cannot be used","highlight_start":19,"highlight_end":48}],"label":"\"pointer arithmetic or comparison\" needs an rfc before being allowed inside constants","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs","byte_start":887,"byte_end":935,"line_start":22,"line_end":22,"column_start":1,"column_end":49,"i:null,"expansion":null}],"children":[],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:26:1\n   |\nLL | const Z2: i32 = unsafe { *(42 as *const i32) }; //~ ERROR cannot be used\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^-------------------^^^\n   |                          |\n   |                          a memory access tried to interpret some bytes as a pointer\n\n"}
[00:52:01] {"message":"this constant cannot be used","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs","byte_start":1170,"byte_end":1189,"line_start":27,"line_end":27,"column_start":26,"column_end":45,"is_primary":false,"text":[{"text":"const Z3: i32 = unsafe { *(44 as *const i32) }; //~ ERROR cannot be used","highlight_start":26,"highlight_end":45}],"label":"a memory access tried to interpret some bytes as a pointer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs","byte_start":1145,"byte_end":1192,"line_start":27,"line_end":27,"column_start":1,"column_end":48,"is_primary":true,"text":[{"text":"const Z3: i32 = unsafe { *(44 as *const i32) }; //~ ERROR cannot be used","highlight_start":1,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this constant cannot be used\n  --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:27:1\n   |\nLL | const Z3: i32 = unsafe { *(44 as *const i32) }; //~ ERROR cannot be used\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^-------------------^^^\n   |                          |\n   |                          a memory access tried to interpret some bytes as a pointer\n\n"}
[00:52:01] {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors\n\n"}
[00:52:01] ------------------------------------------
[00:52:01] 
[00:52:01] thread '[ui] ui/consts/const-eval/const_raw_ptr_ops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[00:52:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:52:01] 
[00:52:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:52:01] 
[00:52:01] 
[00:52:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--Fri, 17 Aug 2018 18:14:55 GMT

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
