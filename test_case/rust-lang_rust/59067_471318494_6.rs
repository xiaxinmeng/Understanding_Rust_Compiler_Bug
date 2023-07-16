\n\nYou can find more information about borrowing in the rust-book:\nhttp://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/borrowck/borrowck-in-static.rs","byte_start":161,"byte_end":162,"line_start":5,"line_end":5,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    Box::new(|| x) //~ ERROR cannot move out of captured outer variable","highlight_start":17,"highlight_end":18}],"label":"cannot move out of captured variable in an `Fn` closure","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/borrowck/borrowck-in-static.rs","byte_start":128,"byte_end":129,"line_start":4,"line_end":4,"column_start":9,"column_end":10,"is_primary":false,"text":[{"text":"    let x = Box::new(0);","highlight_start":9,"highlight_end":10}],"label":"captured outer variable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this error has been downgraded to a warning for backwards compatibility with previous releases","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"this represents potential undefined behavior in your code and this warning will become a hard error in the future","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning[E0507]: cannot move out of captured variable in an `Fn` closure\n  --> /checkout/src/test/ui/borrowck/borrowck-in-static.rs:5:17\n   |\nLL |     let x = Box::new(0);\n   |         - captured outer variable\nLL |     Box::new(|| x) //~ ERROR cannot move out of captured outer variable\n   |                 ^ cannot move out of captured variable in an `Fn` closure\n   |\n   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases\n   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future\n\n"}
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] thread '[ui (nll)] ui/borrowck/borrowck-in-static.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3319:9
[01:38:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:38:31] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:38:31] 
[01:38:31] ---- [ui (nll)] ui/borrowck/borrowck-move-in-irrefut-pat.rs#ast stdout ----
[01:38:31] diff of stderr:
[01:38:31] 
[01:38:31] 14 LL | fn arg_item(&_x: &String) {}
[01:38:31] 16 
[01:38:31] - error[E0507]: cannot move out of borrowed content
[01:38:31] - error[E0507]: cannot move out of borrowed content
[01:38:31] + warning[E0507]: cannot move out of borrowed content
[01:38:31] 19    |
[01:38:31] 19    |
[01:38:31] 20 LL |     with(|&_x| ())
[01:38:31] 29    |
[01:38:31] 29    |
[01:38:31] 30 LL |     with(|&_x| ())
[01:38:31] +    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:38:31] +    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:38:31] 32 
[01:38:31] 33 error[E0507]: cannot move out of borrowed content
[01:38:31] 33 error[E0507]: cannot move out of borrowed content
[01:38:31] 34   --> $DIR/borrowck-move-in-irrefut-pat.rs:17:15
[01:38:31] 
[01:38:31] 45 LL |     let &_x = &"hi".to_string();
[01:38:31] 47 
[01:38:31] - error: aborting due to 3 previous errors
[01:38:31] + error: aborting due to 2 previous errors
[01:38:31] 49 
[01:38:31] 49 
[01:38:31] 50 For more information about this error, try `rustc --explain E0507`.
[01:38:31] 51 
[01:38:31] 
[01:38:31] 
[01:38:31] The actual stderr differed from the expected stderr.
[01:38:31] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-in-irrefut-pat.ast.nll/borrowck-move-in-irrefut-pat.ast.nll.stderr
[01:38:31] To update references, rerun the tests and pass the `--bless` flag
[01:38:31] To only update this specific test, also pass `--test-args borrowck/borrowck-move-in-irrefut-pat.rs`
[01:38:31] 
[01:38:31] error in revision `ast`: 1 errors occurred comparing output.
[01:38:31] status: exit code: 1
[01:38:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-in-irrefut-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-in-irrefut-pat.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-in-irrefut-pat.ast.nll/auxiliary" "-A" "unused"
[01:38:31] ------------------------------------------
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] stderr:
[01:38:31] stderr:
[01:38:31] ------------------------------------------
[01:38:31] {"message":"cannot move out of borrowed content","code":{"code":"E0507","explanation":"\nYou tried to move out of a value which was borrowed. Erroneous code example:\n\n