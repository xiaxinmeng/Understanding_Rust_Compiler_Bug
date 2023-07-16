plain

---- [ui] src/test/ui/error-codes/E0375.rs stdout ----
diff of stderr:

- error[E0375]: implementing the trait `CoerceUnsized` requires multiple coercions
+ error[E0375]: the trait `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced, none found
2   --> $DIR/E0375.rs:10:12
3    |
4 LL | impl<T, U> CoerceUnsized<Foo<U, T>> for Foo<T, U> {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0375/E0375.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0375.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0375" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0375/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0375]: the trait `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced, none found
   |
   |
LL | impl<T, U> CoerceUnsized<Foo<U, T>> for Foo<T, U> {}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^ requires multiple coercions
   |
   = note: `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced
   = note: currently, 2 fields need coercions: `b` (`T` -> `U`), `c` (`U` -> `T`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0375`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-26905.rs stdout ----
diff of stderr:

- error[E0375]: implementing the trait `CoerceUnsized` requires multiple coercions
+ error[E0375]: the trait `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced, none found
3    |
3    |
4 LL | impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<MyRc<U>> for MyRc<T>{ }

The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26905/issue-26905.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26905/issue-26905.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-26905.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26905.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26905" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26905/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0375]: the trait `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced, none found
   |
   |
LL | impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<MyRc<U>> for MyRc<T>{ } //~ERROR
   |                                        ^^^^^^^^^^^^^^^^^^^^^^ requires multiple coercions
   |
   = note: `CoerceUnsized` may only be implemented for a coercion between structures with one field being coerced
   = note: currently, 2 fields need coercions: `_ptr` (`*const T` -> `*const U`), `_boo` (`NotPhantomData<T>` -> `NotPhantomData<U>`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0375`.
------------------------------------------
