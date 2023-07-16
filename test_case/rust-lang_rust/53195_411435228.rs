plain
[00:49:46] ....................................................................................................
[00:49:49] ....................................................................................................
[00:49:51] ....................................................................................................
[00:49:54] ....................................................................................................
[00:49:57] .............iiiiiiiii..............................................................................
[00:50:00] ...................................................................................F................
[00:50:06] ..................i.................................................................................
[00:50:09] ............................i.......................................................................
[00:50:12] ....................................................................................................
[00:50:15] ....................................................................................................
---
[00:50:21] diff of stderr:
[00:50:21] 
[00:50:21] 8   --> $DIR/ex2a-push-one-existing-name-early-bound.rs:17:5
[00:50:21] 9    |
[00:50:21] 10 LL | fn baz<'a, 'b, T>(x: &mut Vec<&'a T>, y: &T)
[00:50:21] -    |                                       - consider changing the type of `y` to `&'a T`
[00:50:21] +    |                                          -- help: add explicit lifetime `'a` to the type of `y`: `&'a T`
[00:50:21] 12 ...
[00:50:21] 13 LL |     x.push(y); //~ ERROR explicit lifetime required
[00:50:21] 14    |     ^^^^^^^^^ lifetime `'a` required
[00:50:21] 
[00:50:21] The actual stderr differed from the expected stderr.
[00:50:21] The actual stderr differed from the expected stderr.
[00:50:21] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/ex2a-push-one-existing-name-early-bound.nll.stderr
[00:50:21] To update references, rerun the tests and pass the `--bless` flag
[00:50:21] To only update this specific test, also pass `--test-args lifetime-errors/ex2a-push-one-existing-name-early-bound.rs`
[00:50:21] error: 1 errors occurred comparing output.
[00:50:21] status: exit code: 1
[00:50:21] status: exit code: 1
[00:50:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.nll/auxiliary" "-A" "unused"
[00:50:21] ------------------------------------------
[00:50:21] 
[00:50:21] ------------------------------------------
[00:50:21] stderr:
[00:50:21] stderr:
[00:50:21] ------------------------------------------
[00:50:21] {"message":"not reporting region error due to nll","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs","byte_start":618,"byte_end":619,"line_start":17,"line_end":17,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    x.push(y); //~ ERROR explicit lifetime required","highlight_start":12,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: not reporting region error due to nll\n  --> /checkout/src/test/ui/lifetime-errors/ex2a-push-one-existing-name-early-bound.rs:17:12\n   |\nLL |     x.push(y); //~ ERROR explicit lifetime required\n   |            ^\n\n"}
[00:50:21] {"message":"explicit lifetime required in the type of `y`","code":{"code":"E0621","explanation":"\nThis error code indicates a mismatch between the lifetimes appearing in the\nfunction signature (i.e., the parameter types and the return type) and the\ndata-flow found in the function body.\n\nErroneous code example:\n\n