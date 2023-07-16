\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/codemap_tests/one_line.rs","byte_start":531,"byte_end":532,"line_start":13,"line_end":13,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    v.push(v.pop().unwrap()); //~ ERROR cannot borrow","highlight_start":5,"highlight_end":6}],"label":"first mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/codemap_tests/one_line.rs","byte_start":538,"byte_end":539,"line_start":13,"line_end":13,"column_start":12,"column_end":13,"is_primary":true,"text":[{"text":"    v.push(v.pop().unwrap()); //~ ERROR cannot borrow","highlight_start":12,"highlight_end":13}],"label":"second mutable borrow occurs here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/codemap_tests/one_line.rs","byte_start":531,"byte_end":555,"line_start":13,"line_end":13,"column_start":5,"column_end":29,"is_primary":false,"text":[{"text":"    v.push(v.pop().unwrap()); //~ ERROR cannot borrow","highlight_start":5,"highlight_end":29}],"label":"borrow later used here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider extracting this into a `let`-binding","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/codemap_tests/tdout ----
[00:51:17] 
[00:51:17] 20    |                      ^ immutable borrow occurs here
[00:51:17] 20    |                      ^ immutable borrow occurs here
[00:51:17] 21 LL |       b.resume();
[00:51:17] +    |
[00:51:17] +    |
[00:51:17] + note: consider extracting this into a `let`-binding
[00:51:17] +   --> $DIR/yield-while-iterating.rs:67:20
[00:51:17] +    |
[00:51:17] + LL |     println!("{}", x[0]); //~ ERROR
[00:51:17] 23 
[00:51:17] 24 error: aborting due to 2 previous errors
[00:51:17] 25 
[00:51:17] 
[00:51:17] 
[00:51:17] 
[00:51:17] The actual stderr differed from the expected stderr.
[00:51:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.nll/yield-while-iterating.nll.stderr
[00:51:17] To update references, rerun the tests and pass the `--bless` flag
[00:51:17] To only update this specific test, also pass `--test-args generator/yield-while-iterating.rs`
[00:51:17] error: 1 errors occurred comparing output.
[00:51:17] status: exit code: 1
[00:51:17] status: exit code: 1
[00:51:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yield-while-iterating.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.nll/a" "-Zborrowck=mir" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yield-while-iterating.nll/auxiliary" "-A" "unused"
[00:51:17] ------------------------------------------
[00:51:17] 
[00:51:17] ------------------------------------------
[00:51:17] stderr:
[00:51:17] stderr:
[00:51:17] ------------------------------------------
[00:51:17] {"message":"borrow may still be in use when generator yields","code":{"code":"E0626","explanation":"\nThis error occurs because a borrow in a generator persists across a\nyield point.\n\n