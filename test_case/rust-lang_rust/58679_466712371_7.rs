\n\nTo determine if a `T` is `Foo`, we need to check if `Bar<T>` is `Foo`. However,\nto do this check, we need to determine that `Bar<Bar<T>>` is `Foo`. To\ndetermine this, we check if `Bar<Bar<Bar<T>>>` is `Foo`, and so on. This is\nclearly a recursive requirement that can't be resolved directly.\n\nConsider changing your trait bounds so that they're less self-referential.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-21946.rs","byte_start":51,"byte_end":54,"line_start":7,"line_end":7,"column_start":6,"column_end":9,"is_primary":true,"text":[{"text":"impl Foo for FooStruct {","highlight_start":6,"highlight_end":9}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0275]: overflow evaluating the requirement `<FooStruct as Foo>::A`\n  --> /checkout/src/test/ui/issues/issue-21946.rs:7:6\n   |\nLL | impl Foo for FooStruct {\n   |      ^^^\n\n"}
[01:08:00] {"message":"For more information about this error, try `rustc --explain E0275`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0275`.\n"}
[01:08:00] 
[01:08:00] ------------------------------------------
[01:08:00] 
[01:08:00] 
[01:08:00] thread '[ui] ui/issues/issue-21946.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:08:00] 
[01:08:00] ---- [ui] ui/issues/issue-23122-1.rs stdout ----
[01:08:00] diff of stderr:
[01:08:00] 
[01:08:00] 4 LL | impl<T: Next> Next for GetNext<T> {
[01:08:00] 6 
[01:08:00] 6 
[01:08:00] - error[E0275]: overflow evaluating the requirement `<GetNext<T> as Next>::Next`
[01:08:00] -    |
[01:08:00] -    |
[01:08:00] - LL |     type Next = <GetNext<T> as Next>::Next;
[01:08:00] - 
[01:08:00] - error: aborting due to 2 previous errors
[01:08:00] + error: aborting due to previous error
[01:08:00] 14 
[01:08:00] 14 
[01:08:00] 15 For more information about this error, try `rustc --explain E0275`.
[01:08:00] 16 
[01:08:00] 
[01:08:00] 
[01:08:00] The actual stderr differed from the expected stderr.
[01:08:00] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-1/issue-23122-1.stderr
[01:08:00] To update references, rerun the tests and pass the `--bless` flag
[01:08:00] To only update this specific test, also pass `--test-args issues/issue-23122-1.rs`
[01:08:00] error: 1 errors occurred comparing output.
[01:08:00] status: exit code: 1
[01:08:00] status: exit code: 1
[01:08:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23122-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-1/auxiliary" "-A" "unused"
[01:08:00] ------------------------------------------
[01:08:00] 
[01:08:00] ------------------------------------------
[01:08:00] stderr:
[01:08:00] stderr:
[01:08:00] ------------------------------------------
[01:08:00] {"message":"overflow evaluating the requirement `<GetNext<T> as Next>::Next`","code":{"code":"E0275","explanation":"\nThis error occurs when there was a recursive trait requirement that overflowed\nbefore it could be evaluated. Often this means that there is unbounded\nrecursion in resolving some type bounds.\n\nFor example, in the following code:\n\n