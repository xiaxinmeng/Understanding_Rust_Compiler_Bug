plain

1 error: only lifetime parameters can be used in this context
2   --> $DIR/disallow-const.rs:4:15
3    |
- LL |     for<const N: i32> || -> () {};   
+ LL |     for<const N: i32> || -> () {};
6 
7 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/disallow-const/disallow-const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/binder/disallow-const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/binder/disallow-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/disallow-const" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/binder/disallow-const/auxiliary"
stdout: none
--- stderr -------------------------------
error: only lifetime parameters can be used in this context
   |
   |
LL |     for<const N: i32> || -> () {};

error: aborting due to previous error
------------------------------------------

