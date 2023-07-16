plain

---- [ui] src/test/ui/error-codes/E0374.rs stdout ----
diff of stderr:

- error[E0374]: the trait `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced, none found
+ error[E0374]: implementing the trait `CoerceUnsized` requires multiple coercions
2   --> $DIR/E0374.rs:8:1
3    |
4 LL | impl<T, U> CoerceUnsized<Foo<U>> for Foo<T>

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0374/E0374.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0374.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0374.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0374" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0374/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0374]: implementing the trait `CoerceUnsized` requires multiple coercions
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL | impl<T, U> CoerceUnsized<Foo<U>> for Foo<T> //~ ERROR E0374

error: aborting due to previous error

For more information about this error, try `rustc --explain E0374`.
