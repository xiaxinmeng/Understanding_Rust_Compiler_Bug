plain
[00:51:55] ...............................................................................................i....
[00:52:01] ..........................................................................i.........................
[00:52:06] ....................................................................................................
[00:52:12] ....................................................................................................
[00:52:19] ..............................................................................F.....................
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/issue-51244.stderr
[00:52:23] To update references, rerun the tests and pass the `--bless` flag
[00:52:23] To only update this specific test, also pass `--test-args suggestions/issue-51244.rs`
[00:52:23] error: 1 errors occurred comparing output.
[00:52:23] status: exit code: 101
[00:52:23] status: exit code: 101
[00:52:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-51244.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-51244/auxiliary" "-A" "unused"
[00:52:23] ------------------------------------------
[00:52:23] 
[00:52:23] ------------------------------------------
[00:52:23] stderr:
[00:52:23] stderr:
[00:52:23] ------------------------------------------
[00:52:23] {"message":"cannot assign to immutable borrowed content `*my_ref`","code":{"code":"E0594","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":511,"byte_end":522,"line_start":13,"line_end":13,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    *my_ref = 0;","highlight_start":5,"highlight_end":16}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/suggestions/issue-51244.rs","byte_start":487,"byte_end":501,"line_start":12,"line_end":12,"column_start":9,"column_end":23,"is_primary":false,"text":[{"text":"    let ref my_ref @ _ = 0;","highlight_start":9,"highlight_end":23}],"label":"consider changing this to `ref mut my_ref @ _`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0594]: cannot assign to immutable borrowed content `*my_ref`\n  --> /checkout/src/test/ui/suggestions/issue-51244.rs:13:5\n   |\nLL |     let ref my_ref @ _ = 0;\n   |         -------------- consider changing this to `ref mut my_ref @ _`\nLL |     *my_ref = 0;\n   |     ^^^^^^^^^^^ cannot borrow as mutable\n\n"}
[00:52:23] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:52:23] {"message":"For more information about this error, try `rustc --explain E0594`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0594`.\n"}
[00:52:23] ------------------------------------------
[00:52:23] 
[00:52:23] thread '[ui] ui/suggestions/issue-51244.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3096:9
[00:52:23] note: Run with `RUST_BACKTRACE=1` for a backtrace.
