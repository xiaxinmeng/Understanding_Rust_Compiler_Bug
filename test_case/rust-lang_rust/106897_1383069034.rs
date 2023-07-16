plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 72 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F...F..................i................................................

---- [ui] checkout/tests/ui-fulldeps/dropck-tarena-unsound-drop.rs stdout ----
diff of stderr:


1 error[E0597]: `arena` does not live long enough
3    |
3    |
+ LL |     let arena: TypedArena<C> = TypedArena::default();
+    |         ----- binding `arena` declared here
4 LL |     f(&arena);
5    |       ^^^^^^ borrowed value does not live long enough
6 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck-tarena-unsound-drop/dropck-tarena-unsound-drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dropck-tarena-unsound-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/dropck-tarena-unsound-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck-tarena-unsound-drop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck-tarena-unsound-drop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `arena` does not live long enough
   |
   |
LL |     let arena: TypedArena<C> = TypedArena::default();
   |         ----- binding `arena` declared here
LL |     f(&arena);
   |       ^^^^^^ borrowed value does not live long enough
LL | } //~^ ERROR `arena` does not live long enough
   | |
   | |
   | `arena` dropped here while still borrowed
   | borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `TypedArena`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] checkout/tests/ui-fulldeps/dropck-tarena-cycle-checked.rs stdout ----
diff of stderr:

1 error[E0597]: `arena` does not live long enough
3    |
3    |
+ LL |     let arena = TypedArena::default();
+    |         ----- binding `arena` declared here
4 LL |     f(&arena);
5    |       ^^^^^^ borrowed value does not live long enough
6 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck-tarena-cycle-checked/dropck-tarena-cycle-checked.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dropck-tarena-cycle-checked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui-fulldeps/dropck-tarena-cycle-checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck-tarena-cycle-checked" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/dropck-tarena-cycle-checked/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `arena` does not live long enough
   |
   |
LL |     let arena = TypedArena::default();
   |         ----- binding `arena` declared here
LL |     f(&arena);
   |       ^^^^^^ borrowed value does not live long enough
LL | } //~^ ERROR `arena` does not live long enough
   | |
   | |
   | `arena` dropped here while still borrowed
   | borrow might be used here, when `arena` is dropped and runs the `Drop` code for type `TypedArena`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
