plain
........................................................................................ 352/13492
........................................................................................ 440/13492
........................................................................................ 528/13492
........................................................................................ 616/13492
.........................................................F.F............................ 704/13492
........................................................................i............... 880/13492
........................................................................................ 968/13492
........................................................................................ 1056/13492
................................................................F....................... 1144/13492
................................................................F....................... 1144/13492
....................................................................................F... 1232/13492
.................................................................i..........F..F........ 1320/13492
....................................i................................................... 1496/13492
........................................................................................ 1584/13492
........................................................................................ 1672/13492
...........................................................................i......ii.... 1760/13492
---
........................................................................................ 3432/13492
........................................................................................ 3520/13492
........................................................................................ 3608/13492
........................................................................................ 3696/13492
.................................F..F................................................... 3784/13492
....i..........i..........i............................................................. 3960/13492
........................................................................................ 4048/13492
.ii..................................................................................... 4136/13492
...................................................i.................................... 4224/13492
---
.............................................................................ii......... 7832/13492
........................................................................................ 7920/13492
........................................................................................ 8008/13492
........................................................................................ 8096/13492
....ii................i......i..ii................................F..F...F.............. 8184/13492
..........F............................................................................. 8360/13492
........................................................................................ 8448/13492
.........................F.............................................................. 8536/13492
..................................................i..ii................................. 8624/13492
---

---- [ui] src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs stdout ----
diff of stderr:

8 LL |     *x += 1;
9    |     ^^^^^^^ assignment to borrowed `*x` occurs here
10 LL |     y
-    |     - returning this value requires that `*x` is borrowed for `'1`
+    |     - returning this value requires that ``*x`` is borrowed for `'1`
12 
- error[E0506]: cannot assign to `*x` because it is borrowed
+ error[E0506]: cannot assign to *x because it is borrowed
15    |
15    |
16 LL |         let y = &*x;

-    |                 --- borrow of `*x` occurs here
+    |                 --- borrow of *x occurs here
18 LL |         *x += 1;
-    |         ^^^^^^^ assignment to borrowed `*x` occurs here
+    |         ^^^^^^^ assignment to borrowed *x occurs here
20 LL |         y
21    |         - returning this value requires that `*x` is borrowed for `'1`
22 LL |     })()

32 LL |         *x += 1;
33    |         ^^^^^^^ assignment to borrowed `*x` occurs here
34 LL |         y
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that ``*x`` is borrowed for `'1`
36 
- error[E0506]: cannot assign to `*x` because it is borrowed
+ error[E0506]: cannot assign to *x because it is borrowed
39    |
39    |
40 LL |         let y = &*x;

-    |                 --- borrow of `*x` occurs here
+    |                 --- borrow of *x occurs here
42 LL |         *x += 1;
-    |         ^^^^^^^ assignment to borrowed `*x` occurs here
+    |         ^^^^^^^ assignment to borrowed *x occurs here
44 LL |         y
45    |         - returning this value requires that `*x` is borrowed for `'1`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations/issue-74072-lifetime-name-annotations.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations/issue-74072-lifetime-name-annotations.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-74072-lifetime-name-annotations.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `*x` because it is borrowed
   |
   |
LL | pub async fn async_fn(x: &mut i32) -> &i32 {
   |                          - let's call the lifetime of this reference `'1`
LL |     let y = &*x;
   |             --- borrow of `*x` occurs here
LL |     *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |     ^^^^^^^ assignment to borrowed `*x` occurs here
LL |     y
   |     - returning this value requires that ``*x`` is borrowed for `'1`

error[E0506]: cannot assign to *x because it is borrowed
   |
   |
LL |         let y = &*x;
   |                 --- borrow of *x occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed *x occurs here
LL |         y
   |         - returning this value requires that `*x` is borrowed for `'1`
LL |     })()
   |     - return type of async closure is &'1 i32

error[E0506]: cannot assign to `*x` because it is borrowed
   |
   |
LL |     (async move || -> &i32 {
   |                       - let's call the lifetime of this reference `'1`
LL |         let y = &*x;
   |                 --- borrow of `*x` occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed `*x` occurs here
LL |         y
   |         - returning this value requires that ``*x`` is borrowed for `'1`

error[E0506]: cannot assign to *x because it is borrowed
   |
   |
LL |         let y = &*x;
   |                 --- borrow of *x occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed *x occurs here
LL |         y
   |         - returning this value requires that `*x` is borrowed for `'1`
LL |     }
   |     - return type of async block is &'1 i32
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/issue-75785-confusing-named-region.rs stdout ----
diff of stderr:

8 LL |     *x += 1;
9    |     ^^^^^^^ assignment to borrowed `*x` occurs here
10 LL |     (&32, y)
-    |     -------- returning this value requires that `*x` is borrowed for `'1`
+    |     -------- returning this value requires that ``*x`` is borrowed for `'1`
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-75785-confusing-named-region/issue-75785-confusing-named-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-75785-confusing-named-region.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-75785-confusing-named-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-75785-confusing-named-region" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-75785-confusing-named-region/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `*x` because it is borrowed
   |
   |
LL | pub async fn async_fn(x: &mut i32) -> (&i32, &i32) {
   |                          - let's call the lifetime of this reference `'1`
LL |     let y = &*x;
   |             --- borrow of `*x` occurs here
LL |     *x += 1; //~ ERROR cannot assign to
   |     ^^^^^^^ assignment to borrowed `*x` occurs here
LL |     (&32, y)
   |     -------- returning this value requires that ``*x`` is borrowed for `'1`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616.rs stdout ----
diff of stderr:

4 LL |     let alias: &'static mut String = s;
5    |                -------------------   - borrow of `*s` occurs here
6    |                |
-    |                type annotation requires that `*s` is borrowed for `'static`
+    |                type annotation requires that ``*s`` is borrowed for `'static`
9 LL |     *s = String::new();
9 LL |     *s = String::new();
10    |     ^^ assignment to borrowed `*s` occurs here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616/borrowck-loan-of-static-data-issue-27616.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-loan-of-static-data-issue-27616.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `*s` because it is borrowed
   |
LL |     let alias: &'static mut String = s;
LL |     let alias: &'static mut String = s;
   |                -------------------   - borrow of `*s` occurs here
   |                |
   |                type annotation requires that ``*s`` is borrowed for `'static`
...
LL |     *s = String::new(); //~ ERROR cannot assign
   |     ^^ assignment to borrowed `*s` occurs here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-vec-pattern-nesting.rs stdout ----
diff of stderr:

22 LL |             _b.use_ref();
24 
24 
- error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
+ error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
27    |
28 LL |     match vec {


44 LL ~         ] => {
46 
46 
- error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
+ error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
49    |
49    |
50 LL |     let a = vec[0];

54    |             move occurs because `vec[_]` has type `Box<isize>`, which does not implement the `Copy` trait
55    |             help: consider borrowing here: `&vec[0]`
56 
- error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
+ error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
59    |
60 LL |     match vec {


73 LL ~          _b] => {}
75 
75 
- error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
+ error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
78    |
78    |
79 LL |     let a = vec[0];
99    |
100    = note: move occurs because these variables have types that don't implement the `Copy` trait
101 
101 
- error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
+ error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
104    |
104    |
105 LL |     let a = vec[0];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting/borrowck-vec-pattern-nesting.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-vec-pattern-nesting.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-vec-pattern-nesting.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `vec[_]` because it is borrowed
   |
   |
LL |         [box ref _a, _, _] => {
   |              ------ borrow of `vec[_]` occurs here
LL |         //~^ NOTE borrow of `vec[_]` occurs here
LL |             vec[0] = Box::new(4); //~ ERROR cannot assign
   |             ^^^^^^ assignment to borrowed `vec[_]` occurs here
LL |             //~^ NOTE assignment to borrowed `vec[_]` occurs here
LL |             _a.use_ref();


error[E0506]: cannot assign to `vec[_]` because it is borrowed
   |
   |
LL |         &mut [ref _b @ ..] => {
   |               ----------- borrow of `vec[_]` occurs here
LL |         //~^ borrow of `vec[_]` occurs here
LL |             vec[0] = Box::new(4); //~ ERROR cannot assign
   |             ^^^^^^ assignment to borrowed `vec[_]` occurs here
LL |             //~^ NOTE assignment to borrowed `vec[_]` occurs here
LL |             _b.use_ref();


error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
   |
LL |     match vec {
   |           ^^^ cannot move out of here
...
...
LL |         &mut [_a,
   |               --
   |               |
   |               data moved here
   |               move occurs because `_a` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider removing the `&mut`
   |
   |
LL ~         [_a,
LL +         //~^ NOTE data moved here
LL +         //~| NOTE move occurs because `_a` has type
LL +         //~| HELP consider removing the `&mut`
LL +             ..
LL ~         ] => {


error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
   |
   |
LL |     let a = vec[0]; //~ ERROR cannot move out
   |             |
   |             cannot move out of here
   |             cannot move out of here
   |             move occurs because `vec[_]` has type `Box<isize>`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&vec[0]`

error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
   |
LL |     match vec {
   |           ^^^ cannot move out of here
...
...
LL |          _b] => {}
   |          |
   |          data moved here
   |          data moved here
   |          move occurs because `_b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider removing the `&mut`
   |
LL ~         [
LL ~         [
LL +         //~^ HELP consider removing the `&mut`
LL ~          _b] => {}


error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
   |
   |
LL |     let a = vec[0]; //~ ERROR cannot move out
   |             |
   |             cannot move out of here
   |             cannot move out of here
   |             move occurs because `vec[_]` has type `Box<isize>`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&vec[0]`

error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
   |
LL |     match vec {
   |           ^^^ cannot move out of here
...
...
LL |         &mut [_a, _b, _c] => {}
   |         |     |   |   |
   |         |     |   |   |
   |         |     |   |   ...and here
   |         |     |   ...and here
   |         |     data moved here
   |         help: consider removing the `&mut`: `[_a, _b, _c]`
   = note: move occurs because these variables have types that don't implement the `Copy` trait


error[E0508]: cannot move out of type `Box<isize>`, a non-copy slice
   |
   |
LL |     let a = vec[0]; //~ ERROR cannot move out
   |             |
   |             cannot move out of here
   |             cannot move out of here
   |             move occurs because `vec[_]` has type `Box<isize>`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&vec[0]`
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0506, E0508.
For more information about an error, try `rustc --explain E0506`.
For more information about an error, try `rustc --explain E0506`.
------------------------------------------


---- [ui] src/test/ui/borrowck/move-error-in-promoted-2.rs stdout ----
diff of stderr:

- error[E0508]: cannot move out of type `[S; 1]`, a non-copy array
+ error[E0508]: cannot move out of type `S`, a non-copy array
3    |
3    |
4 LL |     &([S][0],);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-in-promoted-2/move-error-in-promoted-2.stderr
To only update this specific test, also pass `--test-args borrowck/move-error-in-promoted-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-in-promoted-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-in-promoted-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-in-promoted-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0508]: cannot move out of type `S`, a non-copy array
   |
   |
LL |     &([S][0],);
   |       |
   |       cannot move out of here
   |       move occurs because value has type `S`, which does not implement the `Copy` trait

---

---- [ui] src/test/ui/borrowck/move-error-in-promoted.rs stdout ----
diff of stderr:

- error[E0508]: cannot move out of type `[S2; 1]`, a non-copy array
+ error[E0508]: cannot move out of type `S2`, a non-copy array
3    |
3    |
4 LL |     let _ = S1(C[0]).clone();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-in-promoted/move-error-in-promoted.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/move-error-in-promoted.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-in-promoted.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-in-promoted" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-in-promoted/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0508]: cannot move out of type `S2`, a non-copy array
   |
   |
LL |     let _ = S1(C[0]).clone();
   |                |
   |                cannot move out of here
   |                move occurs because value has type `S2`, which does not implement the `Copy` trait

---

---- [ui] src/test/ui/error-codes/E0508-fail.rs stdout ----
diff of stderr:

- error[E0508]: cannot move out of type `[NonCopy; 1]`, a non-copy array
+ error[E0508]: cannot move out of type `NonCopy`, a non-copy array
2   --> $DIR/E0508-fail.rs:5:18
4 LL |     let _value = array[0];


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0508-fail/E0508-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0508-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0508-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0508-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0508-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0508]: cannot move out of type `NonCopy`, a non-copy array
   |
   |
LL |     let _value = array[0];  //~ ERROR [E0508]
   |                  |
   |                  cannot move out of here
   |                  cannot move out of here
   |                  move occurs because `array[_]` has type `NonCopy`, which does not implement the `Copy` trait
   |                  help: consider borrowing here: `&array[0]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0508`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0508.rs stdout ----
diff of stderr:

- error[E0508]: cannot move out of type `[NonCopy; 1]`, a non-copy array
+ error[E0508]: cannot move out of type `NonCopy`, a non-copy array
2   --> $DIR/E0508.rs:5:18
4 LL |     let _value = array[0];


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0508/E0508.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0508.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0508.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0508" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0508/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0508]: cannot move out of type `NonCopy`, a non-copy array
   |
   |
LL |     let _value = array[0];  //~ ERROR [E0508]
   |                  |
   |                  cannot move out of here
   |                  cannot move out of here
   |                  move occurs because `array[_]` has type `NonCopy`, which does not implement the `Copy` trait
   |                  help: consider borrowing here: `&array[0]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0508`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/moves/move-out-of-array-1.rs stdout ----
diff of stderr:

- error[E0508]: cannot move out of type `[D; 4]`, a non-copy array
+ error[E0508]: cannot move out of type `D`, a non-copy array
3    |
4 LL |     a[i]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-array-1/move-out-of-array-1.stderr
To only update this specific test, also pass `--test-args moves/move-out-of-array-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-out-of-array-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-array-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-array-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0508]: cannot move out of type `D`, a non-copy array
   |
   |
LL |     a[i] //~ ERROR cannot move out of type `[D; 4]`, a non-copy array
   |     |
   |     cannot move out of here
   |     cannot move out of here
   |     move occurs because `a[_]` has type `D`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0508`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/moves/move-out-of-array-ref.rs stdout ----
diff of stderr:

- error[E0508]: cannot move out of type `[D; 4]`, a non-copy array
+ error[E0508]: cannot move out of type `D`, a non-copy array
3    |
3    |
4 LL |     let [_, e, _, _] = *a;
9    |             data moved here
9    |             data moved here
10    |             move occurs because `e` has type `D`, which does not implement the `Copy` trait
11 
- error[E0508]: cannot move out of type `[D; 4]`, a non-copy array
+ error[E0508]: cannot move out of type `[D; 2]`, a non-copy array
14    |
14    |
15 LL |     let [_, s @ .. , _] = *a;
20    |             data moved here
20    |             data moved here
21    |             move occurs because `s` has type `[D; 2]`, which does not implement the `Copy` trait
22 
- error[E0508]: cannot move out of type `[D; 4]`, a non-copy array
+ error[E0508]: cannot move out of type `D`, a non-copy array
25    |
25    |
26 LL |     let [_, e, _, _] = *a;
31    |             data moved here
31    |             data moved here
32    |             move occurs because `e` has type `D`, which does not implement the `Copy` trait
33 
- error[E0508]: cannot move out of type `[D; 4]`, a non-copy array
+ error[E0508]: cannot move out of type `[D; 2]`, a non-copy array
36    |
36    |
37 LL |     let [_, s @ .. , _] = *a;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-array-ref/move-out-of-array-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args moves/move-out-of-array-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-out-of-array-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-array-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-array-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0508]: cannot move out of type `D`, a non-copy array
   |
   |
LL |     let [_, e, _, _] = *a;              //~ ERROR cannot move
   |             |          |
   |             |          cannot move out of here
   |             |          help: consider borrowing here: `&*a`
   |             data moved here
   |             data moved here
   |             move occurs because `e` has type `D`, which does not implement the `Copy` trait

error[E0508]: cannot move out of type `[D; 2]`, a non-copy array
   |
   |
LL |     let [_, s @ .. , _] = *a;           //~ ERROR cannot move
   |             |             |
   |             |             cannot move out of here
   |             |             help: consider borrowing here: `&*a`
   |             data moved here
   |             data moved here
   |             move occurs because `s` has type `[D; 2]`, which does not implement the `Copy` trait

error[E0508]: cannot move out of type `D`, a non-copy array
   |
   |
LL |     let [_, e, _, _] = *a;              //~ ERROR cannot move
   |             |          |
   |             |          cannot move out of here
   |             |          help: consider borrowing here: `&*a`
   |             data moved here
   |             data moved here
   |             move occurs because `e` has type `D`, which does not implement the `Copy` trait

error[E0508]: cannot move out of type `[D; 2]`, a non-copy array
   |
   |
LL |     let [_, s @ .. , _] = *a;           //~ ERROR cannot move
   |             |             |
   |             |             cannot move out of here
   |             |             help: consider borrowing here: `&*a`
   |             data moved here
   |             data moved here
   |             move occurs because `s` has type `[D; 2]`, which does not implement the `Copy` trait
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0508`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/moves/move-out-of-slice-1.rs stdout ----
diff of stderr:

- error[E0508]: cannot move out of type `[A]`, a non-copy slice
+ error[E0508]: cannot move out of type `A`, a non-copy slice
3    |
4 LL |     match a {



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-slice-1/move-out-of-slice-1.stderr
To only update this specific test, also pass `--test-args moves/move-out-of-slice-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/move-out-of-slice-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-slice-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/move-out-of-slice-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0508]: cannot move out of type `A`, a non-copy slice
   |
   |
LL |     match a { //~ ERROR cannot move out of type `[A]`, a non-copy slice
   |           ^ cannot move out of here
LL |         box [a] => {},
   |              |
   |              data moved here
   |              move occurs because `a` has type `A`, which does not implement the `Copy` trait

---

---- [ui] src/test/ui/nll/cannot-move-block-spans.rs stdout ----
diff of stderr:

25    |                          move occurs because `*r` has type `String`, which does not implement the `Copy` trait
26    |                          help: consider borrowing here: `&*r`
27 
- error[E0508]: cannot move out of type `[String; 2]`, a non-copy array
+ error[E0508]: cannot move out of type `String`, a non-copy array
30    |
30    |
31 LL |     let x = { arr[0] };

35    |               move occurs because `arr[_]` has type `String`, which does not implement the `Copy` trait
36    |               help: consider borrowing here: `&arr[0]`
37 
- error[E0508]: cannot move out of type `[String; 2]`, a non-copy array
+ error[E0508]: cannot move out of type `String`, a non-copy array
40    |
40    |
41 LL |     let y = unsafe { arr[0] };

45    |                      move occurs because `arr[_]` has type `String`, which does not implement the `Copy` trait
46    |                      help: consider borrowing here: `&arr[0]`
47 
- error[E0508]: cannot move out of type `[String; 2]`, a non-copy array
+ error[E0508]: cannot move out of type `String`, a non-copy array
50    |
50    |
51 LL |     let z = loop { break arr[0]; };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/cannot-move-block-spans/cannot-move-block-spans.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/cannot-move-block-spans.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/cannot-move-block-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/cannot-move-block-spans" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/cannot-move-block-spans/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of `*r` which is behind a shared reference
   |
   |
LL |     let x = { *r }; //~ ERROR
   |               |
   |               |
   |               move occurs because `*r` has type `String`, which does not implement the `Copy` trait
   |               help: consider borrowing here: `&*r`

error[E0507]: cannot move out of `*r` which is behind a shared reference
   |
   |
LL |     let y = unsafe { *r }; //~ ERROR
   |                      |
   |                      |
   |                      move occurs because `*r` has type `String`, which does not implement the `Copy` trait
   |                      help: consider borrowing here: `&*r`

error[E0507]: cannot move out of `*r` which is behind a shared reference
   |
   |
LL |     let z = loop { break *r; }; //~ ERROR
   |                          |
   |                          |
   |                          move occurs because `*r` has type `String`, which does not implement the `Copy` trait
   |                          help: consider borrowing here: `&*r`

error[E0508]: cannot move out of type `String`, a non-copy array
   |
   |
LL |     let x = { arr[0] }; //~ ERROR
   |               |
   |               cannot move out of here
   |               cannot move out of here
   |               move occurs because `arr[_]` has type `String`, which does not implement the `Copy` trait
   |               help: consider borrowing here: `&arr[0]`

error[E0508]: cannot move out of type `String`, a non-copy array
   |
   |
LL |     let y = unsafe { arr[0] }; //~ ERROR
   |                      |
   |                      cannot move out of here
   |                      cannot move out of here
   |                      move occurs because `arr[_]` has type `String`, which does not implement the `Copy` trait
   |                      help: consider borrowing here: `&arr[0]`

error[E0508]: cannot move out of type `String`, a non-copy array
   |
   |
LL |     let z = loop { break arr[0]; }; //~ ERROR
   |                          |
   |                          cannot move out of here
   |                          cannot move out of here
   |                          move occurs because `arr[_]` has type `String`, which does not implement the `Copy` trait
   |                          help: consider borrowing here: `&arr[0]`

error[E0507]: cannot move out of `*r` which is behind a shared reference
   |
   |
LL |     let x = { let mut u = 0; u += 1; *r }; //~ ERROR
   |                                      |
   |                                      |
   |                                      move occurs because `*r` has type `String`, which does not implement the `Copy` trait
   |                                      help: consider borrowing here: `&*r`

error[E0507]: cannot move out of `*r` which is behind a shared reference
   |
   |
LL |     let y = unsafe { let mut u = 0; u += 1; *r }; //~ ERROR
   |                                             |
   |                                             |
   |                                             move occurs because `*r` has type `String`, which does not implement the `Copy` trait
   |                                             help: consider borrowing here: `&*r`

error[E0507]: cannot move out of `*r` which is behind a shared reference
   |
   |
LL |     let z = loop { let mut u = 0; u += 1; break *r; u += 2; }; //~ ERROR
   |                                                 |
   |                                                 |
   |                                                 move occurs because `*r` has type `String`, which does not implement the `Copy` trait
   |                                                 help: consider borrowing here: `&*r`
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
For more information about an error, try `rustc --explain E0507`.
------------------------------------------


---- [ui] src/test/ui/nll/move-errors.rs stdout ----
diff of stderr:

7    |             move occurs because `*a` has type `A`, which does not implement the `Copy` trait
8    |             help: consider borrowing here: `&*a`
9 
- error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
+ error[E0508]: cannot move out of type `A`, a non-copy array
12    |
13 LL |     let b = a[0];

35    |             move occurs because value has type `A`, which does not implement the `Copy` trait
35    |             move occurs because value has type `A`, which does not implement the `Copy` trait
36    |             help: consider borrowing here: `&*r`
37 
- error[E0508]: cannot move out of type `[A; 1]`, a non-copy array
+ error[E0508]: cannot move out of type `A`, a non-copy array
40    |
40    |
41 LL |     let a = [A("".to_string())][0];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors/move-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/move-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/move-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/move-errors/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of `*a` which is behind a shared reference
   |
LL |     let b = *a;
   |             ^^
   |             |
   |             |
   |             move occurs because `*a` has type `A`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&*a`

error[E0508]: cannot move out of type `A`, a non-copy array
   |
LL |     let b = a[0];
   |             ^^^^
   |             |
   |             |
   |             cannot move out of here
   |             move occurs because `a[_]` has type `A`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&a[0]`

error[E0507]: cannot move out of `**r` which is behind a shared reference
   |
LL |     let s = **r;
   |             ^^^
   |             |
   |             |
   |             move occurs because `**r` has type `A`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&**r`
error[E0507]: cannot move out of an `Rc`
  --> /checkout/src/test/ui/nll/move-errors.rs:27:13
   |
LL |     let s = *r;
LL |     let s = *r;
   |             ^^
   |             |
   |             move occurs because value has type `A`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&*r`

error[E0508]: cannot move out of type `A`, a non-copy array
   |
   |
LL |     let a = [A("".to_string())][0];
   |             |
   |             cannot move out of here
   |             move occurs because value has type `A`, which does not implement the `Copy` trait
   |             move occurs because value has type `A`, which does not implement the `Copy` trait
   |             help: consider borrowing here: `&[A("".to_string())][0]`
error[E0507]: cannot move out of `a` which is behind a shared reference
  --> /checkout/src/test/ui/nll/move-errors.rs:38:16
   |
   |
LL |     let A(s) = *a;
   |           -    ^^ help: consider borrowing here: `&*a`
   |           data moved here
   |           data moved here
   |           move occurs because `s` has type `String`, which does not implement the `Copy` trait

error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
   |
   |
LL |     let C(D(s)) = c;
   |             -     ^ cannot move out of here
   |             data moved here
   |             data moved here
   |             move occurs because `s` has type `String`, which does not implement the `Copy` trait

error[E0507]: cannot move out of `*a` which is behind a shared reference
   |
LL |     b = *a;
LL |     b = *a;
   |         ^^ move occurs because `*a` has type `A`, which does not implement the `Copy` trait

error[E0508]: cannot move out of type `[B; 1]`, a non-copy array
   |
   |
LL |     match x[0] {
   |           |
   |           cannot move out of here
   |           help: consider borrowing here: `&x[0]`
   |           help: consider borrowing here: `&x[0]`
LL |     //~^ ERROR
LL |         B::U(d) => (),
   |              - data moved here
LL |         B::V(s) => (),
   |              - ...and here
   = note: move occurs because these variables have types that don't implement the `Copy` trait


error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
   |
LL |     match x {
   |           ^ cannot move out of here
...
...
LL |         B::U(D(s)) => (),
   |                |
   |                data moved here
   |                data moved here
   |                move occurs because `s` has type `String`, which does not implement the `Copy` trait

error[E0509]: cannot move out of type `D`, which implements the `Drop` trait
   |
LL |     match x {
   |           ^ cannot move out of here
...
...
LL |         (D(s), &t) => (),
   |            |
   |            data moved here
   |            data moved here
   |            move occurs because `s` has type `String`, which does not implement the `Copy` trait

error[E0507]: cannot move out of `*x.1` which is behind a shared reference
   |
LL |     match x {
   |           ^
...
...
LL |         (D(s), &t) => (),
   |                 |
   |                 data moved here
   |                 data moved here
   |                 move occurs because `t` has type `String`, which does not implement the `Copy` trait

error[E0509]: cannot move out of type `F`, which implements the `Drop` trait
   |
LL |     match x {
   |           ^ cannot move out of here
   |           ^ cannot move out of here
LL |     //~^ ERROR
LL |         F(s, mut t) => (),
   |           -  ----- ...and here
   |           data moved here
   |
   = note: move occurs because these variables have types that don't implement the `Copy` trait


error[E0507]: cannot move out of `x` as enum variant `Err` which is behind a shared reference
   |
LL |     match *x {
   |           ^^ help: consider borrowing here: `&*x`
   |           ^^ help: consider borrowing here: `&*x`
LL |     //~^ ERROR
LL |         Ok(s) | Err(s) => (),
   |            |
   |            data moved here
   |            data moved here
   |            move occurs because `s` has type `String`, which does not implement the `Copy` trait
error: aborting due to 14 previous errors

Some errors have detailed explanations: E0507, E0508, E0509.
For more information about an error, try `rustc --explain E0507`.
---
diff of stderr:

44   --> $DIR/unboxed-closures-failed-recursive-fn-1.rs:33:5
45    |
46 LL |     let mut factorial: Option<Box<dyn Fn(u32) -> u32 + 'static>> = None;
-    |                        ----------------------------------------- type annotation requires that `factorial` is borrowed for `'static`
+    |                        ----------------------------------------- type annotation requires that ``factorial`` is borrowed for `'static`
48 LL |
49 LL |     let f = |x: u32| -> u32 {
50    |             --------------- borrow of `factorial` occurs here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1/unboxed-closures-failed-recursive-fn-1.stderr
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-failed-recursive-fn-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `factorial` does not live long enough
   |
   |
LL |     let f = |x: u32| -> u32 {
   |             --------------- value captured here
LL |         let g = factorial.as_ref().unwrap();
   |                 ^^^^^^^^^ borrowed value does not live long enough
LL | }
   | -
   | |
   | |
   | `factorial` dropped here while still borrowed
   | borrow might be used here, when `factorial` is dropped and runs the destructor for type `Option<Box<dyn Fn(u32) -> u32>>`

error[E0506]: cannot assign to `factorial` because it is borrowed
   |
   |
LL |     let f = |x: u32| -> u32 {
   |             --------------- borrow of `factorial` occurs here
LL |         let g = factorial.as_ref().unwrap();
   |                 --------- borrow occurs due to use in closure
LL |     factorial = Some(Box::new(f));
   |     ^^^^^^^^^
   |     |
   |     |
   |     assignment to borrowed `factorial` occurs here
   |     borrow later used here

error[E0597]: `factorial` does not live long enough
   |
   |
LL |     let mut factorial: Option<Box<dyn Fn(u32) -> u32 + 'static>> = None;
   |                        ----------------------------------------- type annotation requires that `factorial` is borrowed for `'static`
LL |
LL |     let f = |x: u32| -> u32 {
   |             --------------- value captured here
LL |         let g = factorial.as_ref().unwrap();
   |                 ^^^^^^^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `factorial` dropped here while still borrowed

error[E0506]: cannot assign to `factorial` because it is borrowed
   |
   |
LL |     let mut factorial: Option<Box<dyn Fn(u32) -> u32 + 'static>> = None;
   |                        ----------------------------------------- type annotation requires that ``factorial`` is borrowed for `'static`
LL |
LL |     let f = |x: u32| -> u32 {
   |             --------------- borrow of `factorial` occurs here
LL |         let g = factorial.as_ref().unwrap();
   |                 --------- borrow occurs due to use in closure
LL |     factorial = Some(Box::new(f));
LL |     factorial = Some(Box::new(f));
   |     ^^^^^^^^^ assignment to borrowed `factorial` occurs here
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0506, E0597.
For more information about an error, try `rustc --explain E0506`.
