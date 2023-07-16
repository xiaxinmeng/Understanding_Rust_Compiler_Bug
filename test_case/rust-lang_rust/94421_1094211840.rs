plain
........................................................................................ 10472/12916
i....................................................................................... 10560/12916
.........iiiiii.i..iiiiii.i............................................................. 10648/12916
........................................................................................ 10736/12916
.................................................................................F...... 10824/12916
..F..........................................................F.......................... 10912/12916
........................................................................................ 11088/12916
........................................................................................ 11176/12916
........................................................................................ 11264/12916
........................................................................................ 11352/12916
---
---- [ui] src/test/ui/issues/issue-18783.rs stdout ----
diff of stderr:

6    |                     |
7    |                     first mutable borrow occurs here
8 LL |     c.push(Box::new(|| y = 0));
-    |                     ^^ - second borrow occurs due to use of `y` in closure
-    |                     |
-    |                     second mutable borrow occurs here
- LL |
- LL | }
-    | - first borrow might be used here, when `c` is dropped and runs the destructor for type `RefCell<Vec<Box<dyn FnMut()>>>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |       ----          ^^ - second borrow occurs due to use of `y` in closure
+    |       |             |
+    |       |             second mutable borrow occurs here
15 
15 
16 error[E0499]: cannot borrow `y` as mutable more than once at a time

21    |                             |
21    |                             |
22    |                             first mutable borrow occurs here
23 LL |     Push::push(&c, Box::new(|| y = 0));
-    |                             ^^ - second borrow occurs due to use of `y` in closure
-    |                             |
-    |                             second mutable borrow occurs here
- LL |
- LL | }
-    | - first borrow might be used here, when `c` is dropped and runs the destructor for type `RefCell<Vec<Box<dyn FnMut()>>>`
+    |     ----------              ^^ - second borrow occurs due to use of `y` in closure
+    |     |                       |
+    |     |                       second mutable borrow occurs here
30 
31 error: aborting due to 2 previous errors
32 

---
To only update this specific test, also pass `--test-args issues/issue-18783.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18783.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18783" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18783/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0499]: cannot borrow `y` as mutable more than once at a time
   |
   |
LL |     c.push(Box::new(|| y = 0));
   |                     -- - first borrow occurs due to use of `y` in closure
   |                     |
   |                     first mutable borrow occurs here
LL |     c.push(Box::new(|| y = 0));
   |       ----          ^^ - second borrow occurs due to use of `y` in closure
   |       |             |
   |       |             second mutable borrow occurs here


error[E0499]: cannot borrow `y` as mutable more than once at a time
   |
   |
LL |     Push::push(&c, Box::new(|| y = 0));
   |                             -- - first borrow occurs due to use of `y` in closure
   |                             |
   |                             first mutable borrow occurs here
LL |     Push::push(&c, Box::new(|| y = 0));
   |     ----------              ^^ - second borrow occurs due to use of `y` in closure
   |     |                       |
   |     |                       second mutable borrow occurs here

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0499`.
For more information about this error, try `rustc --explain E0499`.
------------------------------------------


---- [ui] src/test/ui/span/dropck_vec_cycle_checked.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/dropck_vec_cycle_checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/dropck_vec_cycle_checked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/dropck_vec_cycle_checked/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/span/issue-25199.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-25199.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-25199" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-25199/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/issue-25199.rs:63:26
   |
   |
LL |             assert!(e.0, e.1);
   |
   = note: `#[warn(non_fmt_panics)]` on by default
   = note: `#[warn(non_fmt_panics)]` on by default
   = note: this usage of assert!() is deprecated; it will be a hard error in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/panic-macro-consistency.html>
   |
   |
LL |             assert!(e.0, "{}", e.1);

warning: 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/span/vec-must-not-hide-type-from-dropck.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/vec-must-not-hide-type-from-dropck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/vec-must-not-hide-type-from-dropck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/vec-must-not-hide-type-from-dropck/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/issues/issue-18783.rs
