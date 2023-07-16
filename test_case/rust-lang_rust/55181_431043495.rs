plain
[00:48:49] ............................................iiiiiiii................................................ 1100/4631
[00:48:51] .................................................................................................... 1200/4631
[00:48:54] .................................................................................................... 1300/4631
[00:48:56] .................................................................................................... 1400/4631
[00:48:59] ....................................................................i............F.................. 1500/4631
[00:49:05] .................................................................................................... 1700/4631
[00:49:08] .................................................................................................... 1800/4631
[00:49:11] .......................................................................i............................ 1900/4631
[00:49:15] .................................................................................................... 2000/4631
[00:49:15] .................................................................................................... 2000/4631
[00:49:19] .................................................................................................... 2100/4631
[00:49:23] .................................................................................................... 2200/4631
[00:49:27] .................................i.................................................................. 2300/4631
[00:49:30] .................................................................................................... 2400/4631
[00:49:34] .................................................................................................... 2500/4631
[00:49:37] ................................................iiiiiiiii........................................... 2600/4631
[00:49:44] .................................................................................................... 2800/4631
[00:49:47] .................................................................................................... 2900/4631
[00:49:50] .............................................................................i...................... 3000/4631
[00:49:53] .................................................................................................... 3100/4631
---
[00:50:41] 1 error[E0621]: explicit lifetime required in the type of `x`
[00:50:41] -   --> $DIR/generator-region-requirements.rs:8:9
[00:50:41] +   --> $DIR/generator-region-requirements.rs:12:51
[00:50:41] 3    |
[00:50:41] 4 LL | fn dangle(x: &mut i32) -> &'static mut i32 {
[00:50:41] 5    |              -------- help: add explicit lifetime `'static` to the type of `x`: `&'static mut i32`
[00:50:41] 6 ...
[00:50:41] 6 ...
[00:50:41] - LL |         x
[00:50:41] -    |         ^ lifetime `'static` required
[00:50:41] + LL |             GeneratorState::Complete(c) => return c,
[00:50:41] +    |                                                   ^ lifetime `'static` required
[00:50:41] 10 error: aborting due to previous error
[00:50:41] 11 
[00:50:41] 
[00:50:41] 
[00:50:41] 
[00:50:41] The actual stderr differed from the expected stderr.
[00:50:41] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/generator-region-requirements.nll.stderr
[00:50:41] To update references, rerun the tests and pass the `--bless` flag
[00:50:41] To only update this specific test, also pass `--test-args generator/generator-region-requirements.rs`
[00:50:41] error: 1 errors occurred comparing output.
[00:50:41] status: exit code: 1
[00:50:41] status: exit code: 1
[00:50:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/generator-region-requirements.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/auxiliary" "-A" "unused"
[00:50:41] ------------------------------------------
[00:50:41] 
[00:50:41] ------------------------------------------
[00:50:41] stderr:
[00:50:41] stderr:
[00:50:41] ------------------------------------------
[00:50:41] {"message":"explicit lifetime required in the type of `x`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n