plain
[00:43:58] ....................................................................................................
[00:44:01] ....................................................................................................
[00:44:03] ....................................................................................................
[00:44:06] ....................................................................................................
[00:44:08] ............iiiiiiiii...............................................................................
[00:44:11] .....................................................................................F..............
[00:44:17] ..................i.................................................................................
[00:44:20] ............................i.......................................................................
[00:44:22] ....................................................................................................
[00:44:25] ....................................................................................................
---
[00:44:30] diff of stderr:
[00:44:30] 
[00:44:30] 8   --> $DIR/ex2a-push-one-existing-name-early-bound.rs:17:5
[00:44:30] 9    |
[00:44:30] 10 LL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)
[00:44:30] -    |                                       - consider changing the type of `y` to `&'a T`
[00:44:30] +    |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`
[00:44:30] 12 ...
[00:44:30] 13 LL |     x.push(y); //~ ERROR explicit lifetime required
[00:44:30] 14    |     ^^^^^^^^^ lifetime `'a` required
[00:44:30] 
[00:44:30] The actual stderr differed from the expected stderr.
[00:44:30] The actual stderr differed from the expected stderr.
[00:44:30] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/ex2a-push-one-existing-name-early-bound.nll.stderr
[00:44:30] To update references, rerun the tests and pass the `--bless` flag
[00:44:30] To only update this specific test, also pass `--test-args lifetime-errors/ex2a-push-one-existing-name-early-bound.rs`
[00:44:30] error: 1 errors occurred comparing output.
[00:44:30] status: exit code: 1
[00:44:30] status: exit code: 1
[00:44:30] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/auxiliary" "-A" "unused"
[00:44:30] ------------------------------------------
[00:44:30] 
[00:44:30] ------------------------------------------
[00:44:30] stderr:
[00:44:30] stderr:
[00:44:30] ------------------------------------------
[00:44:30] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":618,"byte_end":619,"line_start":17,"line_end":17,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":12,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:12\n   |\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |            ^\n\n"}
[00:44:30] {"message":"explicit lifetime required in the type of `y`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n