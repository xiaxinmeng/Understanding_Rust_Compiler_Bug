plain
---- [ui] src/test/ui/suggestions/option_as_ref.rs stdout ----
diff of stderr:

5    |     ^^^^----------------------
6    |     |   |
7    |     |   `*opt` moved due to this method call
+    |     help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
8    |     move occurs because `*opt` has type `Option<Box<i32>>`, which does not implement the `Copy` trait
9    |
10 note: this function takes ownership of the receiver `self`, which moves `*opt`
12    |
12    |
13 LL |     pub const fn map<U, F>(self, f: F) -> Option<U>
14    |                            ^^^^
- help: consider calling `.as_ref()` to borrow the type's contents
-    |
- LL |     opt.as_ref().map(|x| x.to_string()).unwrap_or_else(String::new)
19 
20 error: aborting due to previous error
21 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/option_as_ref/option_as_ref.stderr
diff of fixed:

1 // run-rustfix
2 
3 fn _foo(opt: &Option<Box<i32>>) -> String {
-     opt.as_ref().map(|x| x.to_string()).unwrap_or_else(String::new)
+     opt.map(|x| x.to_string()).unwrap_or_else(String::new)
5     //~^ cannot move out of `*opt` which is behind a shared reference
7 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/option_as_ref/option_as_ref.fixed
To only update this specific test, also pass `--test-args suggestions/option_as_ref.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/option_as_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/option_as_ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/option_as_ref/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of `*opt` which is behind a shared reference
   |
   |
LL |     opt.map(|x| x.to_string()).unwrap_or_else(String::new)
   |     |   |
   |     |   |
   |     |   `*opt` moved due to this method call
   |     help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
   |     move occurs because `*opt` has type `Option<Box<i32>>`, which does not implement the `Copy` trait
   |
note: this function takes ownership of the receiver `self`, which moves `*opt`
   |
   |
LL |     pub const fn map<U, F>(self, f: F) -> Option<U>

error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
