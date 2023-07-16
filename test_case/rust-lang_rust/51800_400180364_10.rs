\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/feature-gate-async-await.rs","byte_start":509,"byte_end":526,"line_start":14,"line_end":14,"column_start":1,"column_end":18,"is_primary":true,"text":[{"text":"async fn foo() {} //~ ERROR async fn is unstable","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(async_await)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: async fn is unstable (see issue #50547)\n  --> /checkout/src/test/ui/feature-gate-async-await.rs:14:1\n   |\nLL | async fn foo() {} //~ ERROR async fn is unstable\n   | ^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(async_await)] to the crate attributes to enable\n\n"}
[00:47:46] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:47:46] {"message":"Some errors occurred: E0422, E0425, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0422, E0425, E0658.\n"}
[00:47:46] {"message":"For more information about an error, try `rustc --explain E0422`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0422`.\n"}
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] thread '[ui] ui/feature-gate-async-await.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3154:9
[00:47:46] 
[00:47:46] 
[00:47:46] ---- [ui] ui/lint-anon-param-edition.rs stdout ----
[00:47:46] diff of stderr:
[00:47:46] 
[00:47:46] - warning: anonymous parameters are deprecated and will be removed in the next edition.
[00:47:46] -    |
[00:47:46] -    |
[00:47:46] - LL |     fn foo(u8);
[00:47:46] -    |            ^^ help: Try naming the parameter or explicitly ignoring it: `_: u8`
[00:47:46] -    |
[00:47:46] -    = note: #[warn(anonymous_parameters)] on by default
[00:47:46] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:47:46] -    = note: for more information, see issue #4168n is experimental (see issue #23416)
[00:47:46] +   --> $DIR/no-args-non-move-async-closure.rs:16:20
[00:47:46] +    |
[00:47:46] + LL |     let _ = async |x: u8| {};
[00:47:46] +    |
[00:47:46] +    |
[00:47:46] +    = help: add #![feature(type_ascription)] to the crate attributes to enable
[00:47:46] - For more information about this error, try `rustc --explain E0708`.
[00:47:46] + error: aborting due to 3 previous errors
[00:47:46] + 
[00:47:46] + Some errors occurred: E0425, E0658.
[00:47:46] + Some errors occurred: E0425, E0658.
[00:47:46] + For more information about an error, try `rustc --explain E0425`.
[00:47:46] 12 
[00:47:46] 
[00:47:46] 
[00:47:46] The actual stderr differed from the expected stderr.
[00:47:46] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
[00:47:46] To update references, rerun the tests and pass the `--bless` flag
[00:47:46] To only update this specific test, also pass `--test-args no-args-non-move-async-closure.rs`
[00:47:46] error: 1 errors occurred comparing output.
[00:47:46] status: exit code: 101
[00:47:46] status: exit code: 101
[00:47:46] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-args-non-move-async-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-args-non-move-async-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-args-non-move-async-closure/auxiliary" "-A" "unused"
[00:47:46] ------------------------------------------
[00:47:46] 
[00:47:46] ------------------------------------------
[00:47:46] stderr:
[00:47:46] stderr:
[00:47:46] ------------------------------------------
[00:47:46] {"message":"cannot find value `async` in this scope","code":{"code":"E0425","explanation":"\nAn unresolved name was used.\n\nErroneous code examples:\n\n