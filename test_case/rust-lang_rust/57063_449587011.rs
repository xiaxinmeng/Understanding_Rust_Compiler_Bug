plain
[01:19:16] diff of stderr:
[01:19:16] 
[01:19:16] 10    |             ------ first borrow later used here
[01:19:16] 11 
[01:19:16] 12 error[E0499]: cannot borrow `x` as mutable more than once at a time
[01:19:16] +   --> $DIR/borrowck-describe-lvalue.rs:306:20
[01:19:16] 14    |
[01:19:16] 14    |
[01:19:16] 15 LL |                    let y = &mut x;
[01:19:16] 16    |                            ------ first mutable borrow occurs here
[01:19:16] 21    |                    ------ first borrow later used here
[01:19:16] 22 
[01:19:16] 22 
[01:19:16] 23 error: captured variable cannot escape `FnMut` closure body
[01:19:16] +   --> $DIR/borrowck-describe-lvalue.rs:304:16
[01:19:16] 25    |
[01:19:16] 25    |
[01:19:16] 26 LL |              || {
[01:19:16] 27    |               - inferred to be a `FnMut` closure
[01:19:16] 368    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:19:16] 369 
[01:19:16] 369 
[01:19:16] 370 error[E0382]: use of moved value: `x`
[01:19:16] +   --> $DIR/borrowck-describe-lvalue.rs:317:22
[01:19:16] 372    |
[01:19:16] 372    |
[01:19:16] 373 LL |                 drop(x);
[01:19:16] 374    |                      - value moved here
[01:19:16] 
[01:19:16] The actual stderr differed from the expected stderr.
[01:19:16] The actual stderr differed from the expected stderr.
[01:19:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.ast.nll/borrowck-describe-lvalue.ast.nll.stderr
[01:19:16] To update references, rerun the tests and pass the `--bless` flag
[01:19:16] To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`
[01:19:16] 
[01:19:16] error in revision `ast`: 1 errors occurred comparing output.
[01:19:16] status: exit code: 1
[01:19:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "ast" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.ast.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue.ast.nll/auxiliary" "-A" "unused"
[01:19:16] ------------------------------------------
[01:19:16] 
[01:19:16] ------------------------------------------
[01:19:16] stderr:
[01:19:16] stderr:
[01:19:16] ------------------------------------------
[01:19:16] {"message":"cannot borrow `x` as mutable more than once at a time","code":{"code":"E0499","explanation":"\nA variable was borrowed as mutable more than once. Erroneous code example:\n\n