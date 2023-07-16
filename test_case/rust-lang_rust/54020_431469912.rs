plain
[00:47:25] .................................................................................................... 1400/4637
[00:47:28] ......................................................................i............................. 1500/4637
[00:47:31] .......................................i............................................................ 1600/4637
[00:47:34] .................................................................................................... 1700/4637
[00:47:38] ................................F................................................................... 1800/4637
[00:47:44] .................................................................................................... 2000/4637
[00:47:48] .................................................................................................... 2100/4637
[00:47:52] .................................................................................................... 2200/4637
[00:47:56] .................................................................................................... 2300/4637
---
[00:48:40] .................................................................................................... 3600/4637
[00:48:43] .................................................................................................... 3700/4637
[00:48:46] .................................................................................................... 3800/4637
[00:48:49] ...............................................i.................................................... 3900/4637
[00:48:55] .....................................................................F.............................. 4000/4637
[00:49:01] .................................................................................................... 4200/4637
[00:49:04] .....................i.............................................................................. 4300/4637
[00:49:08] .................................................................................................... 4400/4637
[00:49:11] .................................................................................................... 4500/4637
[00:49:11] .................................................................................................... 4500/4637
[00:49:13] ............................................................................i....................... 4600/4637
[00:49:14] .....................................
[00:49:14] failures:
[00:49:14] 
[00:49:14] ---- [ui] ui/issue-53737.rs stdout ----
[00:49:14] normalized stderr:
[00:49:14] error: extern location for myfoo does not exist: $TEST_BUILD_DIR/issue-53737/auxiliary/libbazso
[00:49:14] 
[00:49:14] error[E0463]: can't find crate for `myfoo`
[00:49:14]    |
[00:49:14]    |
[00:49:14] LL | use myfoo::Yellow;
[00:49:14] 
[00:49:14] error: aborting due to 2 previous errors
[00:49:14] 
[00:49:14] For more information about this error, try `rustc --explain E0463`.
[00:49:14] For more information about this error, try `rustc --explain E0463`.
[00:49:14] 
[00:49:14] 
[00:49:14] 
[00:49:14] The actual stderr differed from the expected stderr.
[00:49:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-53737/issue-53737.stderr
[00:49:14] To update references, rerun the tests and pass the `--bless` flag
[00:49:14] To only update this specific test, also pass `--test-args issue-53737.rs`
[00:49:14] error: 1 errors occurred comparing output.
[00:49:14] status: exit code: 1
[00:49:14] status: exit code: 1
[00:49:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-53.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-53737.rs","byte_start":515,"byte_end":520,"line_start":15,"line_end":15,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"use myfoo::Yellow;","highlight_start":5,"highlight_end":10}],"label":"can't find crate","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0463]: can't find crate for `myfoo`\n  --> /checkout/src/test/ui/issue-53737.rs:15:5\n   |\nLL | use myfoo::Yellow;\n   |     ^^^^^ can't find crate\n\n"}
[00:49:14] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:49:14] {"message":"For more information about this error, try `rustc --explain E0463`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0463`.\n"}
[00:49:14] ------------------------------------------
[00:49:14] 
[00:49:14] thread '[ui] ui/issue-53737.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3302:9
[00:49:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:14] 
[00:49:14] ---- [ui] ui/rust-2018/use-suggestions-extern-prelude.rs stdout ----
[00:49:14] diff of stderr:
[00:49:14] 
[00:49:14] + error: extern location for netted does not exist: $TEST_BUILD_DIR/rust-2018/use-suggestions-extern-prelude/auxiliary/libep_nested_libso
[00:49:14] + 
[00:49:14] 1 error[E0422]: cannot find struct, variant or union type `Bazz` in this scope
[00:49:14] 3    |
[00:49:14] 
[00:49:14] 
[00:49:14] 4 LL |     let _x = Bazz{};
[00:49:14] - help: possible candidate is found in another module, you can import it into scope
[00:49:14] -    |
[00:49:14] -    |
[00:49:14] - LL | use netted::foo::bar::Bazz;
[00:49:14] 10 
[00:49:14] - error: aborting due to previous error
[00:49:14] + error: aborting due to 2 previous errors
[00:49:14] 12 
[00:49:14] 12 
[00:49:14] 13 For more information about this error, try `rustc --explain E0422`.
[00:49:14] 14 
[00:49:14] 
[00:49:14] 
[00:49:14] The actual stderr differed from the expected stderr.
[00:49:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/use-suggestions-extern-prelude/use-suggestions-extern-prelude.stderr
[00:49:14] To update references, rerun the tests and pass the `--bless` flag
[00:49:14] To only update this specific test, also pass `--test-args rust-2018/use-suggestions-extern-prelude.rs`
[00:49:14] error: 1 errors occurred comparing output.
[00:49:14] status: exit code: 1
[00:49:14] status: exit code: 1
[00:49:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/use-suggestions-extern-prelude.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/use-suggestions-extern-prelude/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/use-suggestions-extern-prelude/auxiliary" "-A" "unused" "--extern" "netted=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/use-suggestions-extern-prelude/auxiliary/libep_nested_libso"
[00:49:14] ------------------------------------------
[00:49:14] 
[00:49:14] ------------------------------------------
[00:49:14] stderr:
[00:49:14] stderr:
[00:49:14] ------------------------------------------
[00:49:14] {"message":"extern location for netted does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/use-suggestions-extern-prelude/auxiliary/libep_nested_libso","code":null,"level":"error","spans":[],"children":[],"rendered":"error: extern location for netted does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/use-suggestions-extern-prelude/auxiliary/libep_nested_libso\n\n"}
[00:49:14] {"message":"cannot find struct, variant or union type `Bazz` in this scope","code":{"code":"E0422","explanation":"\nYou are trying to use an identifier that is either undefined or not a struct.\nErroneous code example:\n\n