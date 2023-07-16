\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/empty-struct-unit-expr.rs","byte_start":660,"byte_end":668,"line_start":25,"line_end":25,"column_start":14,"column_end":22,"is_primary":true,"text":[{"text":"    let e2 = Empty2(); //~ ERROR expected function, found `Empty2`","highlight_start":14,"highlight_end":22}],"label":"not a function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/empty-struct-unit-expr.rs","byte_start":596,"byte_end":610,"line_start":18,"line_end":18,"col ------------------------------------------
[01:05:22] thread '[ui] ui/empty-struct-unit-expr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3044:9
[01:05:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:05:22] 
[01:05:22] ---- [ui] ui/error-codes/E0618.rs stdout ----
[01:05:22] ---- [ui] ui/error-codes/E0618.rs stdout ----
[01:05:22] diff of stderr:
[01:05:22] 
[01:05:22] - error[E0618]: expected function, found enum variant `X::Entry`
[01:05:22] -   --> $DIR/E0618.rs:16:5
[01:05:22] -    |
[01:05:22] - LL |     Entry,
[01:05:22] -    |     ----- `X::Entry` defined here
[01:05:22] - ...
[01:05:22] - LL |     X::Entry();
[01:05:22] -    |     ^^^^^^^^^^ not a function
[01:05:22] - help: `X::Entry` is a unit variant, you need to write it without the parenthesis
[01:05:22] -    |
[01:05:22] - LL |     X::Entry;
[01:05:22] - 
[01:05:22] - 
[01:05:22] - error[E0618]: expected function, found `i32`
[01:05:22] -   --> $DIR/E0618.rs:19:5
[01:05:22] -    |
[01:05:22] - LL |     let x = 0i32;
[01:05:22] -    |         - `i32` defined here
[01:05:22] - LL |     x();
[01:05:22] -    |     ^^^ not a function
[01:05:22] - error: aborting due to 2 previous errors
[01:05:22] - 
[01:05:22] - For more information about this error, try `rustc --explain E0618`.
[01:05:22] - 
[01:05:22] - 
[01:05:22] 
[01:05:22] 
[01:05:22] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0618/E0618.stderr`: No such file or directory (os error 2)
[01:05:22] 
[01:05:22] ---- [ui] ui/loop-break-value-no-repeat.rs stdout ----
[01:05:22] diff of stderr:
[01:05:22] 
[01:05:22] 
[01:05:22] - error[E0571]: `break` with value from a `for` loop
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |         break 22 //~ ERROR `break` with value from a `for` loop
[01:05:22] -    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
[01:05:22] - help: instead, use `break` on its own without a value inside this `for` loop
[01:05:22] -    |
[01:05:22] - LL |         break //~ ERROR `break` with value from a `for` loop
[01:05:22] - 
[01:05:22] - error: aborting due to previous error
[01:05:22] - 
[01:05:22] - For more information about this error, try `rustc --explain E0571`.
[01:05:22] - For more information about this error, try `rustc --explain E0571`.
[01:05:22] - 
[01:05:22] 
[01:05:22] 
[01:05:22] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loop-break-value-no-repeat/loop-break-value-no-repeat.stderr`: No such file or directory (os error 2)
[01:05:22] 
[01:05:22] ---- [ui] ui/mismatched_types/closure-arg-count.rs stdout ----
[01:05:22] diff of stderr:
[01:05:22] 
[01:05:22] 
[01:05:22] 14    |               |
[01:05:22] 15    |               expected closure that takes 2 arguments
[01:05:22] 16 
[01:05:22] - error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     [1, 2, 3].sort_by(|(tuple, tuple2)| panic!());
[01:05:22] -    |               ^^^^^^^ ----------------- takes a single 2-tuple as argument
[01:05:22] -    |               |
[01:05:22] -    |               expected closure that takes 2 distinct arguments
[01:05:22] - help: change the closure to take multiple arguments instead of a single tuple
[01:05:22] -    |
[01:05:22] - LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());
[01:05:22] - 
[01:05:22] - 
[01:05:22] - error[E0593]: closure is expected to take 2 distinct arguments, but it takes a single 2-tuple as argument
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     [1, 2, 3].sort_by(|(tuple, tuple2): (usize, _)| panic!());
[01:05:22] -    |               ^^^^^^^ ----------------------------- takes a single 2-tuple as argument
[01:05:22] -    |               |
[01:05:22] -    |               expected closure that takes 2 distinct arguments
[01:05:22] - help: change the closure to take multiple arguments instead of a single tuple
[01:05:22] -    |
[01:05:22] - LL |     [1, 2, 3].sort_by(|tuple, tuple2| panic!());
[01:05:22] - 
[01:05:22] - 
[01:05:22] - error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     f(|| panic!());
[01:05:22] -    |     ^ -- takes 0 arguments
[01:05:22] -    |     |
[01:05:22] -    |     expected closure that takes 1 argument
[01:05:22] - note: required by `f`
[01:05:22] - note: required by `f`
[01:05:22] -   -|               --------- takes 3 distinct arguments
[01:05:22] - LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(bar);
[01:05:22] -    |                                                     ^^^ expected closure that takes a single 2-tuple as argument
[01:05:22] - 
[01:05:22] - error[E0593]: function is expected to take a single 2-tuple as argument, but it takes 2 distinct arguments
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     let _it = vec![1, 2, 3].into_iter().enumerate().map(qux);
[01:05:22] -    |                                                     ^^^ expected function that takes a single 2-tuple as argument
[01:05:22] - ...
[01:05:22] - LL | fn qux(x: usize, y: usize) {}
[01:05:22] -    | -------------------------- takes 2 distinct arguments
[01:05:22] - 
[01:05:22] - error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     let _it = vec![1, 2, 3].into_iter().map(usize::checked_add);
[01:05:22] -    |                                         ^^^ expected function that takes 1 argument
[01:05:22] - 
[01:05:22] - error[E0593]: function is expected to take 0 arguments, but it takes 1 argument
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL |     call(Foo);
[01:05:22] -    |     ^^^^ expected function that takes 0 arguments
[01:05:22] - ...
[01:05:22] - LL | struct Foo(u8);
[01:05:22] -    | --------------- takes 1 argument
[01:05:22] -    |
[01:05:22] - note: required by `call`
[01:05:22] -    |
[01:05:22] -    |
[01:05:22] - LL | fn call<F, R>(_: F) where F: FnOnce() -> R {}
[01:05:22] - 
[01:05:22] - error: aborting due to 13 previous errors
[01:05:22] + error: aborting due to 2 previous errors
[01:05:22] 135 
[01:05:22] 135 
[01:05:22] 136 For more information about this error, try `rustc --explain E0593`.
[01:05:22] 137 
[01:05:22] 
[01:05:22] 
[01:05:22] The actual stderr differed from the expected stderr.
[01:05:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/closure-arg-count.stderr
[01:05:22] To update references, rerun the tests and pass the `--bless` flag
[01:05:22] To only update this specific test, also pass `--test-args mismatched_types/closure-arg-count.rs`
[01:05:22] error: 1 errors occurred comparing output.
[01:05:22] status: exit code: 101
[01:05:22] status: exit code: 101
[01:05:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/closure-arg-count.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/closure-arg-count/auxiliary" "-A" "unused"
[01:05:22] ------------------------------------------
[01:05:22] 
[01:05:22] 
[01:05:22] --------------------  |\n   |               expected closure that takes 2 arguments\n\n"}
[01:05:22] {"message":"closure is expected to take 2 arguments, but it takes 1 argument","code":{"code":"E0593","explanation":"\nYou tried to supply an `Fn`-based type with an incorrect number of arguments\nthan what was expected.\n\nErroneous code example:\n\n