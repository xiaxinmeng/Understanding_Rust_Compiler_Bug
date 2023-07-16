plain
...................................................iii.............................................. 12600/12656
........................................................
failures:

---- [ui] ui/tuple/wrong_argument_ice.rs stdout ----

3    |
3    |
4 LL |         self.acc.push_back(self.current_provides, self.current_requires);
-    |                  |
-    |                  expected 1 argument
8    |
9 note: associated function defined here
9 note: associated function defined here
10   --> $SRC_DIR/alloc/src/collections/vec_deque/mod.rs:LL:COL

11    |
12 LL |     pub fn push_back(&mut self, value: T) {
13    |            ^^^^^^^^^
+ help: use parentheses to construct a tuple
+    |
+ LL |         self.acc.push_back((self.current_provides, self.current_requires));
14 
15 error: aborting due to previous error
16 



Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/wrong_argument_ice/wrong_argument_ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args tuple/wrong_argument_ice.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tuple/wrong_argument_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/wrong_argument_ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tuple/wrong_argument_ice/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/tuple/wrong_argument_ice.rs:11:18
   |
   |
LL |         self.acc.push_back(self.current_provides, self.current_requires);
   |
note: associated function defined here
  --> /checkout/library/alloc/src/collections/vec_deque/mod.rs:1525:12
   |
   |
LL |     pub fn push_back(&mut self, value: T) {
   |            ^^^^^^^^^
help: use parentheses to construct a tuple
   |
LL |         self.acc.push_back((self.current_provides, self.current_requires));

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
