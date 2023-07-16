plain
[00:50:16] normalized stderr:
[00:50:16] error[E0621]: explicit lifetime required in the type of `x`
[00:50:16]   --> $DIR/generator-region-requirements.rs:15:51
[00:50:16]    |
[00:50:16] LL | fn dangle(x: &mut i32) -> &'static mut i32 {
[00:50:16]    |              -------- help: add explicit lifetime `'static` to the type of `x`: `&'static mut i32`
[00:50:16] ...
[00:50:16] LL |             GeneratorState::Complete(c) => return c,
[00:50:16]    |                                                   ^ lifetime `'static` required
[00:50:16] error: aborting due to previous error
[00:50:16] 
[00:50:16] For more information about this error, try `rustc --explain E0621`.
[00:50:16] 
[00:50:16] 
[00:50:16] 
[00:50:16] 
[00:50:16] The actual stderr differed from the expected stderr.
[00:50:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/generator-region-requirements.nll.stderr
[00:50:16] To update references, rerun the tests and pass the `--bless` flag
[00:50:16] To only update this specific test, also pass `--test-args generator/generator-region-requirements.rs`
[00:50:16] 
[00:50:16] error in revision `nll`: 1 errors occurred comparing output.
[00:50:16] status: exit code: 1
[00:50:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/generator-region-requirements.rs" "--target=i586-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/generator-region-requirements.nll/auxiliary" "-A" "unused"
[00:50:16] ------------------------------------------
[00:50:16] 
[00:50:16] ------------------------------------------
[00:50:16] stderr:
[00:50:16] stderr:
[00:50:16] ------------------------------------------
[00:50:16] {"message":"explicit lifetime required in the type of `x`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n