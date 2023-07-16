\n\nThis will fail because the compiler does not know which instance of `Foo` to\ncall `bar` on. Change `Foo::bar()` to `Foo::<T>::bar()` to resolve the error.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-54974.rs","byte_start":195,"byte_end":198,"line_start":12,"line_end":12,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"async fn my_task(obj: Arc<SomeTrait>) {","highlight_start":18,"highlight_end":21}],"label":"cannot infer type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0282]: type annotations needed\n  --> /checkout/src/test/ui/issues/issue-54974.rs:12:18\n   |\nLL | async fn my_task(obj: Arc<SomeTrait>) {\n   |                  ^^^ cannot infer type\n\n"}
[01:07:25] {"message":"For more information about this error, try `rustc --explain E0282`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0282`.\n"}
[01:07:25] 
[01:07:25] ------------------------------------------
[01:07:25] 
[01:07:25] 
[01:07:25] thread '[ui] ui/issues/issue-54974.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
[01:07:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:25] 
[01:07:25] ---- [ui] ui/issues/issue-55324.rs stdout ----
[01:07:25] 
[01:07:25] error: test compilation failed although it shouldn't!
[01:07:25] status: exit code: 1
[01:07:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55324.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55324/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55324/auxiliary" "-A" "unused"
[01:07:25] ------------------------------------------
[01:07:25] 
[01:07:25] ------------------------------------------
[01:07:25] stderr:
[01:07:25] stderr:
[01:07:25] ------------------------------------------
[01:07:25] {"message":"type annotations needed","code":{"code":"E0282","explanation":"\nThis error indicates that type inference did not result in one unique possible\ntype, and extra information is required. In most cases this can be provided\nby adding a type annotation. Sometimes you need to specify a generic type\nparameter manually.\n\nA common example is the `collect` method on `Iterator`. It has a generic type\nparameter with a `FromIterator` bound, which for a `char` iterator is\nimplemented by `Vec` and `String` among others. Consider the following snippet\nthat reverses the characters of a string:\n\n