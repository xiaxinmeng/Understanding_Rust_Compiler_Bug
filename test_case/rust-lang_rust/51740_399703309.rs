plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:48:39] 
[00:48:39] running 1516 tests
[00:48:44] .................F.............................................................................i....
[00:48:54] ....................................................................................................
[00:48:58] ....................................................................................................
[00:49:01] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:10] ....................................................................................................
[00:49:14] ....................................................................................................
[00:49:20] ....................................................................................................
[00:49:26] ....................................................................................................
[00:49:32] .............i..............................................................................F.i.....
[00:49:43] ....................................................................................................
[00:49:49] ....................................................................................................
[00:49:56] ................
[00:49:56] failures:
[00:49:56] failures:
[00:49:56] 
[00:49:56] ---- [ui] ui/async-fn-multiple-lifetimes.rs stdout ----
[00:49:56] diff of stderr:
[00:49:56] 
[00:49:56] - error[E0725]: multiple different lifetimes used in arguments of `async fn`
[00:49:56] + error[E0709]: multiple different lifetimes used in arguments of `async fn`
[00:49:56] 2   --> $DIR/async-fn-multiple-lifetimes.rs:17:49
[00:49:56] 3    |
[00:49:56] 4 LL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}
[00:49:56] 8    |
[00:49:56] 8    |
[00:49:56] 9    = help: `async fn` can only accept borrowed values with identical lifetimes
[00:49:56] 10 
[00:49:56] - error[E0726]: multiple elided lifetimes used in arguments of `async fn`
[00:49:56] + error[E0707]: multiple elided lifetimes used in arguments of `async fn`
[00:49:56] 12   --> $DIR/async-fn-multiple-lifetimes.rs:26:39
[00:49:56] 13    |
[00:49:56] 14 LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
[00:49:56] 28 
[00:49:56] 29 error: aborting due to 3 previous errors
[00:49:56] 30 
[00:49:56] - Some errors occurred: E0106, E0725, E0726.
[00:49:56] - Some errors occurred: E0106, E0725, E0726.
[00:49:56] + Some errors occurred: E0106, E0707, E0709.
[00:49:56] 32 For more information about an error, try `rustc --explain E0106`.
[00:49:56] 33 
[00:49:56] 
[00:49:56] 
[00:49:56] The actual stderr differed from the expected stderr.
[00:49:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/async-fn-multiple-lifetimes.stderr
[00:49:56] To update references, rerun the tests and pass the `--bless` flag
[00:49:56] To only update this specific test, also pass `--test-args async-fn-multiple-lifetimes.rs`
[00:49:56] error: 1 errors occurred comparing output.
[00:49:56] status: exit code: 101
[00:49:56] status: exit code: 101
[00:49:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-fn-multiple-lifetimes.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-fn-multiple-lifetimes/auxiliary" "-A" "unused"
[00:49:56] ------------------------------------------
[00:49:56] 
[00:49:56] ------------------------------------------
[00:49:56] stderr:
[00:49:56] stderr:
[00:49:56] ------------------------------------------
[00:49:56] {"message":"multiple different lifetimes used in arguments of `async fn`","code":{"code":"E0709","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":646,"byte_end":648,"line_start":17,"line_end":17,"column_start":47,"column_end":49,"is_primary":false,"text":[{"text":"async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}","highlight_start":47,"highlight_end":49}],"label":"first lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":657,"byte_end":659,"line_start":17,"line_end":17,"column_start":58,"column_end":60,"is_primary":false,"text":[{"text":"async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}","highlight_start":58,"highlight_end":60}],"label":"different lifetime here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":648,"byte_end":657,"line_start":17,"line_end":17,"column_start":49,"column_end":58,"is_primary":true,"text":[{"text":"async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}","highlight_start":49,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`async fn` can only accept borrowed values with identical lifetimes","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0709]: multiple different lifetimes used in arguments of `async fn`\n  --> /checkout/src/test/ui/async-fn-multiple-lifetimes.rs:17:49\n   |\nLL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}\n   |                                               --^^^^^^^^^-- different lifetime here\n   |                                               |\n   |                                               first lifetime here\n   |\n   = help: `async fn` can only accept borrowed values with identical lifetimes\n\n"}
[00:49:56] {"message":"multiple elided lifetimes used in argumentsthis error, try `rustc --explain E0727`.
[00:49:56] 12 
[00:49:56] 
[00:49:56] 
[00:49:56] The actual stderr differed from the expected stderr.
[00:49:56] The actual stderr differed from the expected stderr.
[00:49:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-args-non-move-async-closure/no-args-non-move-async-closure.stderr
[00:49:56] To update references, rerun the tests and pass the `--bless` flag
[00:49:56] To only update this specific test, also pass `--test-args no-args-non-move-async-closure.rs`
[00:49:56] error: 1 errors occurred comparing output.
[00:49:56] status: exit code: 101
[00:49:56] status: exit code: 101
[00:49:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-args-non-move-async-closure.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-args-non-move-async-closure/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-args-non-move-async-closure/auxiliary" "-A" "unused"
[00:49:56] ------------------------------------------
[00:49:56] 
[00:49:56] ------------------------------------------
[00:49:56] stderr:
[00:49:56] stderr:
[00:49:56] ------------------------------------------
[00:49:56] {"message":"`async` non-`move` closures with arguments are not currently supported","code":{"code":"E0708","explanation":null},"level":"error","spans":[{"file_name":"/c

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b009ab1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
