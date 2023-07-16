plain

---- [ui] src/test/ui/generic-associated-types/bugs/hrtb-implied-1.rs stdout ----
diff of stderr:

8    |     -------------------------------------- argument requires that borrow lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement
-    |
- note: due to current limitations in the borrow checker, this implies a `'static` lifetime
-   --> $DIR/hrtb-implied-1.rs:26:26
-    |
- LL |     for<'a> I::Item<'a>: Debug,
17 
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/hrtb-implied-1/hrtb-implied-1.stderr
To only update this specific test, also pass `--test-args generic-associated-types/bugs/hrtb-implied-1.rs`

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/hrtb-implied-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/hrtb-implied-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/hrtb-implied-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/generic-associated-types/bugs/hrtb-implied-1.rs:31:22
   |
LL |     let slice = &mut ();
   |                      ^^ creates a temporary which is freed while still in use
   |                      ^^ creates a temporary which is freed while still in use
...
LL |     print_items::<WindowsMut<'_>>(windows);
   |     -------------------------------------- argument requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
