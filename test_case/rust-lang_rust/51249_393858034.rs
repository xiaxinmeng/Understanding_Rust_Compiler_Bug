plain
[00:50:04] ....................................................................................................
[00:50:09] ....................................................................................................
[00:50:15] ...............................................................................................i....
[00:50:21] ..........................................................................i.........................
[00:50:26] ..........................................................................................FF........
[00:50:31] ....................................................................................................
[00:50:37] ..............................................................................F.....................
[00:50:42] 10   --> $DIR/enum.rs:23:9
[00:50:42] 11    |
[00:50:42] 11    |
[00:50:42] - LL |     if let Some(x) = &Some(3) {
[00:50:42] -    |                 - consider changing this to `x`
[00:50:42] 14 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:50:42] 15    |         ^^^^^^^ cannot borrow as mutable
[00:50:42] 
[00:50:42] 
[00:50:42] 17 error[E0594]: cannot assign to immutable borrowed content `*x`
[00:50:42] 18   --> $DIR/enum.rs:29:9
[00:50:42] 19    |
[00:50:42] - LL |     while let Some(x) = &Some(3) {
[00:50:42] -    |                    - consider changing this to `x`
[00:50:42] 22 LL |         *x += 1; //~ ERROR cannot assign to immutable
[00:50:42] 23    |         ^^^^^^^ cannot borrow as mutable
[00:50:42] 
[00:50:42] 
[00:50:42] The actual stderr differed from the expected stderr.
[00:50:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/enum.stderr
[00:50:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/enum.stderr
[00:50:42] To update references, rerun the tests and pass the `--bless` flag
[00:50:42] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/enum.rs`
[00:50:42] error: 1 errors occurred comparing output.
[00:50:42] status: exit code: 101
[00:50:42] status: exit code: 101
[00:50:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/enum/auxiliary" "-A" "unused"
[00:50:42] ------------------------------------------
[00:50:42] 
[00:50:42] ------------------------------------------
[00:50:42] stderr:
[00:50:42] stderr:
[00:50:42] ------------------------------------------
[00:50:42] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":568,"byte_end":575,"line_start":19,"line_end":19,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    *x += 1; //~ ERROR cannot assign to immutable","highlight_start":5,"highlight_end":12}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:19:5\n   |\nLL |     *x += 1; //~ ERROR cannot assign to immutable\n   |     ^^^^^^^ cannot borrow as mutable\n\n"}
[00:50:42] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":656,"byte_end":663,"line_start":23,"line_end":23,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:23:9\n   |\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:50:42] {"message":"cannot assign to immutable borrowed content `*x`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs","byte_start":783,"byte_end":790,"line_start":29,"line_end":29,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        *x += 1; //~ ERROR cannot assign to immutable","highlight_start":9,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*x`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/enum.rs:29:9\n   |\nLL |         *x += 1; //~ ERROR cannot assign to immutable\n   |         ^^^^^^^ cannot borrow as mutable\n\n"}
[00:50:42] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:50:42] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E059le
[00:50:42] 
[00:50:42] 
[00:50:42] The actual stderr differed from the expected stderr.
[00:50:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/explicit-mut.stderr
[00:50:42] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/explicit-mut.stderr
[00:50:42] To update references, rerun the tests and pass the `--bless` flag
[00:50:42] To only update this specific test, also pass `--test-args rfc-2005-default-binding-mode/explicit-mut.rs`
[00:50:42] error: 1 errors occurred comparing output.
[00:50:42] status: exit code: 101
[00:50:42] status: exit code: 101
[00:50:42] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2005-default-binding-mode/explicit-mut/auxiliary" "-A" "unused"
[00:50:42] ------------------------------------------
[00:50:42] 
[00:50:42] ------------------------------------------
[00:50:42] stderr:
[00:50:42] stderr:
[00:50:42] ------------------------------------------
[00:50:42] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":659,"byte_end":666,"line_start":17,"line_end":17,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:17:13\n   |\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:50:42] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":828,"byte_end":835,"line_start":25,"line_end":25,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:25:13\n   |\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:50:42] {"message":"cannot assign to immutable borrowed content `*n`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs","byte_start":997,"byte_end":1004,"line_start":33,"line_end":33,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"            *n += 1; //~ ERROR cannot assign to immutable","highlight_start":13,"highlight_end":20}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*n`\n  --> /checkout/src/test/ui/rfc-2005-default-binding-mode/explicit-mut.rs:33:13\n   |\nLL |             *n += 1; //~ ERROR cannot assign to immutable\n   |             ^^^^^^^ cannot borrow as mutable\n\n"}
[00:50:42] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:50:42] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:50:42] ------------------------------------------
[00:50:42] 
[00:50:42] thread '[ui] ui/rfc-2005-default-binding-mode/explicit-mut.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:50:42] 
[00:50:42] 
[00:50:42] ---- [ui] ui/suggestions/issue-51244.rs stdout ----
[00:50:42] diff of stderr:
[00:50:42] 
[00:50:42] 2   --> $DIR/issue-51244.rs:13:5
[00:50:42] 3    |
[00:50:42] 4 LL |     let ref my_ref @ _ = 0;
[00:50:42] -    |         --- consider changing this to `ref mut`
[00:50:42] +    |         -------------- consider chanine_start":13,"line_end":13,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0;","highlight_start":5,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":487,"byte_end":501,"line_start":12,"line_end":12,"column_start":9,"column_end":23,"is_primary":false,"text":[{"text":"    let ref my_ref @ _ = 0;","highlight_start":9,"highlight_end":23}],"label":"consider changing this to `ref mut my_ref @ _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*my_ref`\n  --> /checkout/src/test/ui/suggestions/issue-51244.rs:13:5\n   |\nLL |     let ref my_ref @ _ = 0;\n   |         -------------- consider changing this to `ref mut my_ref @ _`\nLL |     *my_ref = 0;\n   |     ^^^^^^^^^^^ cannot borrow as mutable\n\n"}
[00:50:42] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:42] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:50:42] ------------------------------------------
[00:50:42] 
[00:50:42] thread '[ui] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:50:42] 
---
[00:50:42] 
[00:50:42] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:50:42] 
[00:50:42] 
[00:50:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:42] 
[00:50:42] 
[00:50:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:42] Build completed unsuccessfully in 0:02:52
[00:50:42] Build completed unsuccessfully in 0:02:52
[00:50:42] make: *** [check] Error 1
[00:50:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1fc5e244
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
