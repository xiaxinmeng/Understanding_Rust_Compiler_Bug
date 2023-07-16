\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/generic/generic-arg-mismatch-recover.rs","byte_start":678,"byte_end":680,"line_start":19,"line_end":19,"column_start":29,"column_end":31,"is_primary":true,"text":[{"text":"    Bar::<'static, 'static, ()>(&()); //~ ERROR wrong number of lifetime arguments","high:53] 3    |
[00:46:53] 4 LL |     Foo::<isize>::new();
[00:46:53] 6 
[00:46:53] 7 error: aborting due to previous error
[00:46:53] 8 
[00:46:53] - For more information about this error, try `rustc --explain E0243`.
[00:46:53] - For more information about this error, try `rustc --explain E0243`.
[00:46:53] + For more information about this error, try `rustc --explain E0107`.
[00:46:53] 10 
[00:46:53] 
[00:46:53] 
[00:46:53] The actual stderr differed from the expected stderr.
[00:46:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-impl-less-params-with-defaults/generic-impl-less-params-with-defaults.stderr
[00:46:53] To update references, rerun the tests and pass the `--bless` flag
[00:46:53] To only update this specific test, also pass `--test-args generic/generic-impl-less-params-with-defaults.rs`
[00:46:53] error: 1 errors occurred comparing output.
[00:46:53] status: exit code: 1
[00:46:53] status: exit code: 1
[00:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic/generic-impl-less-params-with-defaults.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-impl-less-params-with-defaults/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-impl-less-params-with-defaults/auxiliary" "-A" "unused"
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] 
[00:46:53] -------------------------------------st 2, found 3
[00:46:53] 3    |
[00:46:53] 3    |
[00:46:53] 4 LL |     Vec::<isize, Heap, bool>::new();
[00:46:53] 6 
[00:46:53] 7 error: aborting due to previous error
[00:46:53] 8 
[00:46:53] - For more information about this error, try `rustc --explain E0244`.
[00:46:53] - For more information about this error, try `rustc --explain E0244`.
[00:46:53] + For more information about this error, try `rustc --explain E0107`.
[00:46:53] 10 
[00:46:53] 
[00:46:53] 
[00:46:53] The actual stderr differed from the expected stderr.
[00:46:53] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-impl-more-params-with-defaults/generic-impl-more-params-with-defaults.stderr
[00:46:53] To update references, rerun the tests and pass the `--bless` flag
[00:46:53] To only update this specific test, also pass `--test-args generic/generic-impl-more-params-with-defaults.rs`
[00:46:53] error: 1 errors occurred comparing output.
[00:46:53] status: exit code: 1
[00:46:53] status: exit code: 1
[00:46:53] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic/generic-impl-more-params-with-defaults.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-impl-more-params-with-defaults/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic/generic-impl-more-params-with-defaults/auxiliary" "-A" "unused"
[00:46:53] ------------------------------------------
[00:46:53] 
[00:46:53] ------------------------------------------
[00:46:53] stderr:
[00:46:53] stderr:
[00:46:53] ------------------------------------------
[00:46:53] {"message":"wrong number of type arguments: expected at most 2, found 3","code":{"code":"E0107","explanation":"\nThis error means that an incorrect number of type or lifetime parameters\nwere provided:\n\n