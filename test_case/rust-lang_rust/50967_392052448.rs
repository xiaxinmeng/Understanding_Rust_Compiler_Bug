plain
[00:50:58] 
[00:50:58] ---- [ui] ui/const-eval/ref_to_int_match.rs stdout ----
[00:50:58] diff of stderr:
[00:50:58] 
[00:50:58] - error[E0030]: lower range bound must be less than or equal to upper
[00:50:58] + error[E0308]: mismatched types
[00:50:58] 2   --> $DIR/ref_to_int_match.rs:14:9
[00:50:58] 3    |
[00:50:58] 4 LL |         10...BAR => {}, //~ ERROR lower range bound must be less than or equal to upper
[00:50:58] 
[00:50:58] -    |         ^^ lower bound larger than upper bound
[00:50:58] +    |         ^^^^^^^^ expected u64, found u32
[00:50:58] 7 error: aborting due to previous error
[00:50:58] 8 
[00:50:58] 
[00:50:58] - For more information about this error, try `rustc --explain E0030`.
[00:50:58] - For more information about this error, try `rustc --explain E0030`.
[00:50:58] + For more information about this error, try `rustc --explain E0308`.
[00:50:58] 10 
[00:50:58] 
[00:50:58] 
[00:50:58] The actual stderr differed from the expected stderr.
[00:50:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/ref_to_int_match/ref_to_int_match.stderr
[00:50:58] To update references, rerun the tests and pass the `--bless` flag
[00:50:58] To only update this specific test, also pass `--test-args const-eval/ref_to_int_match.rs`
[00:50:58] error: 1 errors occurred comparing output.
[00:50:58] status: exit code: 101
[00:50:58] status: exit code: 101
[00:50:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/ref_to_int_match.rs" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/ref_to_int_match/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/ref_to_int_match/auxiliary" "-A" "unused"
[00:50:58] ------------------------------------------
[00:50:58] 
[00:50:58] ------------------------------------------
[00:50:58] stderr:
[00:50:58] stderr:
[00:50:58] ------------------------------------------
[00:50:58] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n