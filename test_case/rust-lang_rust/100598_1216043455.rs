plain

---- [ui] src/test/ui/borrowck/copy-suggestion-region-vid.rs stdout ----
diff of stderr:

5    |             ------- move occurs because `helpers` has type `[Vec<&i64>; 2]`, which does not implement the `Copy` trait
6 LL |
7 LL |         HelperStruct { helpers, is_empty: helpers[0].is_empty() }
+    |                        -------            ^^^^^^^ value borrowed here after move
9    |                        |
10    |                        value moved here
11 
---
To only update this specific test, also pass `--test-args borrowck/copy-suggestion-region-vid.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/copy-suggestion-region-vid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/copy-suggestion-region-vid" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/copy-suggestion-region-vid/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `helpers`
   |
   |
LL |         let helpers = [vec![], vec![]];
   |             ------- move occurs because `helpers` has type `[Vec<&i64>; 2]`, which does not implement the `Copy` trait
LL |
LL |         HelperStruct { helpers, is_empty: helpers[0].is_empty() }
   |                        |
   |                        value moved here

error: aborting due to previous error
---
---- [ui] src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/slice-index-bounds-check-invalidation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/slice-index-bounds-check-invalidation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/slice-index-bounds-check-invalidation/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/nll/issue-62007-assign-const-index.rs stdout ----
diff of stderr:


- error[E0499]: cannot borrow `list[_].value` as mutable more than once at a time
-   --> $DIR/issue-62007-assign-const-index.rs:23:21
+ error[E0499]: cannot borrow `list` as mutable more than once at a time
3    |
3    |
4 LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
5    |                          - let's call the lifetime of this reference `'1`
6 ...
6 ...
7 LL |         result.push(&mut list[0].value);
-    |                     ^^^^^^^^^^^^^^^^^^ `list[_].value` was mutably borrowed here in the previous iteration of the loop
+    |                          ---- first mutable borrow occurs here
+ LL |         if let Some(n) = list[0].next.as_mut() {
+    |                          ^^^^ second mutable borrow occurs here
10 LL |             return result;
10 LL |             return result;
-    |                    ------ returning this value requires that `list[_].value` is borrowed for `'1`
+    |                    ------ returning this value requires that `list` is borrowed for `'1`
12 
- error[E0499]: cannot borrow `list[_].next` as mutable more than once at a time
-   --> $DIR/issue-62007-assign-const-index.rs:24:26
+ error[E0503]: cannot use `list` because it was mutably borrowed
15    |
15    |
16 LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
17    |                          - let's call the lifetime of this reference `'1`
18 ...
18 ...
+ LL |         result.push(&mut list[0].value);
+    |                          ---- borrow of `list` occurs here
19 LL |         if let Some(n) = list[0].next.as_mut() {
-    |                          |
-    |                          |
-    |                          `list[_].next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list[_].next` is borrowed for `'1`
+ LL |             list[0] = n;
+    |             ^^^^^^^ use of borrowed `list`
+ LL |         } else {
+ LL |             return result;
+    |                    ------ returning this value requires that `list` is borrowed for `'1`
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0506]: cannot assign to `list[_]` because it is borrowed
+    |
+    |
+ LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
+    |                          - let's call the lifetime of this reference `'1`
+ ...
+ LL |         result.push(&mut list[0].value);
+    |                          ---- borrow of `list[_]` occurs here
+ LL |         if let Some(n) = list[0].next.as_mut() {
+ LL |             list[0] = n;
+    |             ^^^^^^^^^^^ assignment to borrowed `list[_]` occurs here
+ LL |         } else {
+ LL |             return result;
+    |                    ------ returning this value requires that `list` is borrowed for `'1`
- For more information about this error, try `rustc --explain E0499`.
+ error[E0515]: cannot return value referencing function parameter `list`
+   --> $DIR/issue-62007-assign-const-index.rs:27:20
+    |
+    |
+ LL |         result.push(&mut list[0].value);
+    |                          ---- `list` is borrowed here
+ LL |             return result;
+    |                    ^^^^^^ returns a value referencing data owned by the current function
+ 
+ error: aborting due to 4 previous errors
---

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index/issue-62007-assign-const-index.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-62007-assign-const-index.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-62007-assign-const-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0499]: cannot borrow `list` as mutable more than once at a time
   |
   |
LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
   |                          - let's call the lifetime of this reference `'1`
...
LL |         result.push(&mut list[0].value); //~ ERROR cannot borrow `list[_].value` as mutable
   |                          ---- first mutable borrow occurs here
LL |         if let Some(n) = list[0].next.as_mut() { //~ ERROR cannot borrow `list[_].next` as mutable
   |                          ^^^^ second mutable borrow occurs here
LL |             return result;
LL |             return result;
   |                    ------ returning this value requires that `list` is borrowed for `'1`

error[E0503]: cannot use `list` because it was mutably borrowed
   |
   |
LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
   |                          - let's call the lifetime of this reference `'1`
...
LL |         result.push(&mut list[0].value); //~ ERROR cannot borrow `list[_].value` as mutable
   |                          ---- borrow of `list` occurs here
LL |         if let Some(n) = list[0].next.as_mut() { //~ ERROR cannot borrow `list[_].next` as mutable
LL |             list[0] = n;
   |             ^^^^^^^ use of borrowed `list`
LL |             return result;
LL |             return result;
   |                    ------ returning this value requires that `list` is borrowed for `'1`

error[E0506]: cannot assign to `list[_]` because it is borrowed
   |
   |
LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
   |                          - let's call the lifetime of this reference `'1`
...
LL |         result.push(&mut list[0].value); //~ ERROR cannot borrow `list[_].value` as mutable
   |                          ---- borrow of `list[_]` occurs here
LL |         if let Some(n) = list[0].next.as_mut() { //~ ERROR cannot borrow `list[_].next` as mutable
LL |             list[0] = n;
   |             ^^^^^^^^^^^ assignment to borrowed `list[_]` occurs here
LL |             return result;
LL |             return result;
   |                    ------ returning this value requires that `list` is borrowed for `'1`
error[E0515]: cannot return value referencing function parameter `list`
  --> /checkout/src/test/ui/nll/issue-62007-assign-const-index.rs:27:20
   |
   |
LL |         result.push(&mut list[0].value); //~ ERROR cannot borrow `list[_].value` as mutable
   |                          ---- `list` is borrowed here
LL |             return result;
   |                    ^^^^^^ returns a value referencing data owned by the current function

error: aborting due to 4 previous errors
