plain
[00:51:26] 
[00:51:26] ---- [ui] ui/range/issue-54505-no-std.rs stdout ----
[00:51:26] diff of stderr:
[00:51:26] 
[00:51:26] 1 error: `#[panic_handler]` function required, but not found
[00:51:26] 2 
[00:51:26] - error: language item required, but not found: `eh_personality`
[00:51:26] 5 error[E0308]: mismatched types
[00:51:26] 6   --> $DIR/issue-54505-no-std.rs:21:16
[00:51:26] 7    |
[00:51:26] 
[00:51:26] 
[00:51:26] 74    = note: expected type `&_`
[00:51:26] 75               found type `core::ops::RangeToInclusive<{integer}>`
[00:51:26] - error: aborting due to 8 previous errors
[00:51:26] + error: aborting due to 7 previous errors
[00:51:26] 78 
[00:51:26] 79 For more information about this error, try `rustc --explain E0308`.
[00:51:26] 79 For more information about this error, try `rustc --explain E0308`.
[00:51:26] 80 
[00:51:26] 
[00:51:26] 
[00:51:26] The actual stderr differed from the expected stderr.
[00:51:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/issue-54505-no-std.stderr
[00:51:26] To update references, rerun the tests and pass the `--bless` flag
[00:51:26] To only update this specific test, also pass `--test-args range/issue-54505-no-std.rs`
[00:51:26] error: 1 errors occurred comparing output.
[00:51:26] status: exit code: 1
[00:51:26] status: exit code: 1
[00:51:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/issue-54505-no-std.rs" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/issue-54505-no-std/auxiliary" "-A" "unused"
[00:51:26] ------------------------------------------
[00:51:26] 
[00:51:26] ------------------------------------------
[00:51:26] stderr:
[00:51:26] stderr:
[00:51:26] ------------------------------------------
[00:51:26] {"message":"`#[panic_handler]` function required, but not found","code":null,"level":"error","spans":[],"children":[],"rendered":"error: `#[panic_handler]` function required, but not found\n\n"}
[00:51:26] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n