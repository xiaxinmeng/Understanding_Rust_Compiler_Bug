plain
diff of stderr:

2   --> $DIR/issue-52742.rs:17:9
3    |
4 LL |     fn take_bar(&mut self, b: Bar<'_>) {
-    |                 ---------         -- let's call this `'1`
+    |                 ---------  - has type `Bar<'1>`
7    |                 has type `&mut Foo<'_, '2>`
7    |                 has type `&mut Foo<'_, '2>`
8 LL |         self.y = b.z

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742.nll/issue-52742.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52742.rs`

error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn take_bar(&mut self, b: Bar<'_>) {
   |                 ---------  - has type `Bar<'1>`
   |                 has type `&mut Foo<'_, '2>`
   |                 has type `&mut Foo<'_, '2>`
LL |         self.y = b.z
   |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
error: aborting due to previous error
------------------------------------------


