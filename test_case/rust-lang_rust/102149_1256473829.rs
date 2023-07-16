plain
diff of stderr:

2   --> $DIR/issue-101984.rs:21:13
3    |
4 LL |         let (cmp, router) = self.router.at()?;
-    |             ^^^^^^^^^^^^^   ----------------- this expression has type `Match<&(for<'r> fn(&'r ()), Box<Wrapper>)>`
+    |             ^^^^^^^^^^^^^   ----------------- this expression has type `Match<&(for<'a> fn(&'a ()), Box<Wrapper>)>`
7    |             expected struct `Match`, found tuple
8    |


-    = note: expected struct `Match<&(for<'r> fn(&'r ()), Box<Wrapper>)>`
+    = note: expected struct `Match<&(for<'a> fn(&'a ()), Box<Wrapper>)>`
10                found tuple `(_, _)`
12 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-101984/issue-101984.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-101984.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-101984.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-101984" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-101984/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/issue-101984.rs:21:13
   |
   |
LL |         let (cmp, router) = self.router.at()?;
   |             ^^^^^^^^^^^^^   ----------------- this expression has type `Match<&(for<'a> fn(&'a ()), Box<Wrapper>)>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |             expected struct `Match`, found tuple
   |
   |
   = note: expected struct `Match<&(for<'a> fn(&'a ()), Box<Wrapper>)>`
               found tuple `(_, _)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
