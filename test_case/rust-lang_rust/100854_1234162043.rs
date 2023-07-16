plain
---- [ui] src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs stdout ----
diff of stderr:

53    | |
54 LL | |
55 LL | |         ref mut b,
-    | |         --------- another mutable borrow, by `b`, occurs here
+    | |         --------- another mutable borrow, by `c`, occurs here
57 LL | |         [
58 LL | |             ref mut c,
59    | |             --------- another mutable borrow, by `c`, occurs here
60 LL | |             ref mut d,
60 LL | |             ref mut d,
-    | |             --------- another mutable borrow, by `d`, occurs here
+    | |             --------- another mutable borrow, by `c`, occurs here
62 LL | |             ref e,
63    | |             ----- also borrowed as immutable, by `e`, here
64 LL | |         ]
75    | |
76 LL | |
77 LL | |             ref mut b,
77 LL | |             ref mut b,
-    | |             --------- another mutable borrow, by `b`, occurs here
+    | |             --------- another mutable borrow, by `c`, occurs here
79 LL | |             [
80 LL | |                 ref mut c,
81    | |                 --------- another mutable borrow, by `c`, occurs here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
82 LL | |                 ref mut d,
82 LL | |                 ref mut d,
-    | |                 --------- another mutable borrow, by `d`, occurs here
+    | |                 --------- another mutable borrow, by `c`, occurs here
84 LL | |                 ref e,
85    | |                 ----- also borrowed as immutable, by `e`, here
86 LL | |             ]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice/borrowck-pat-ref-mut-twice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
LL |       let ref mut a @ (
   |           ^--------
   |           |
   |           |
   |  _________first mutable borrow, by `a`, occurs here
   | |
LL | |     //~^ ERROR cannot borrow value as mutable more than once at a time
LL | |         ref mut b,
   | |         --------- another mutable borrow, by `c`, occurs here
LL | |         [
LL | |             ref mut c,
   | |             --------- another mutable borrow, by `c`, occurs here
LL | |             ref mut d,
   | |             --------- another mutable borrow, by `c`, occurs here
LL | |             ref e,
   | |             ----- also borrowed as immutable, by `e`, here
LL | |         ]
LL | |     ) = (U, [U, U, U]);


error: cannot borrow value as mutable more than once at a time
   |
LL |       let ref mut a @ (
   |           ^--------
   |           |
   |           |
   |  _________first mutable borrow, by `a`, occurs here
   | |
LL | |         //~^ ERROR cannot borrow value as mutable more than once at a time
LL | |             ref mut b,
   | |             --------- another mutable borrow, by `c`, occurs here
LL | |             [
LL | |                 ref mut c,
   | |                 --------- another mutable borrow, by `c`, occurs here
LL | |                 ref mut d,
   | |                 --------- another mutable borrow, by `c`, occurs here
LL | |                 ref e,
   | |                 ----- also borrowed as immutable, by `e`, here
LL | |             ]
LL | |         ) = (u(), [u(), u(), u()]);

error: borrow of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs:64:9
   |
   |
LL |     let a @ (ref mut b, ref mut c) = (U, U);
   |         -^^^^---------^^---------^
   |         |    |          value borrowed here after move
   |         |    value borrowed here after move
   |         value moved into `a` here
   |         value moved into `a` here
   |         move occurs because `a` has type `(U, U)` which does not implement the `Copy` trait
error: borrow of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs:67:9
   |
   |
LL |     let a @ (b, [c, d]) = &mut val; // Same as ^--
   |         -^^^^-^^^-^^-^^
   |         |    |   |  |
   |         |    |   |  value borrowed here after move
   |         |    value borrowed here after move
   |         value moved into `a` here
   |         value moved into `a` here
   |         move occurs because `a` has type `&mut (U, [U; 2])` which does not implement the `Copy` trait
error: borrow of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs:70:9
   |
   |
LL |     let a @ &mut ref mut b = &mut U;
   |         -^^^^^^^^---------
   |         |        value borrowed here after move
   |         value moved into `a` here
   |         move occurs because `a` has type `&mut U` which does not implement the `Copy` trait


error: borrow of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs:72:9
   |
LL |     let a @ &mut (ref mut b, ref mut c) = &mut (U, U);
   |         -^^^^^^^^^---------^^---------^
   |         |         |          value borrowed here after move
   |         |         value borrowed here after move
   |         value moved into `a` here
   |         value moved into `a` here
   |         move occurs because `a` has type `&mut (U, U)` which does not implement the `Copy` trait

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
   |                                     first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
   |                                     first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
   |                                     first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
   |                                     first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     fn f1(ref mut a @ ref mut b: U) {}
   |           ---------^^^---------
   |           |           |
   |           |           another mutable borrow, by `b`, occurs here
   |           first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     fn f2(ref mut a @ ref mut b: U) {}
   |           ---------^^^---------
   |           |           |
   |           |           another mutable borrow, by `b`, occurs here
   |           first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
LL |           ref mut a @ [
   |           ^--------
   |           |
   |           |
   |  _________first mutable borrow, by `a`, occurs here
   | |
LL | |         //~^ ERROR cannot borrow value as mutable more than once at a time
LL | |             [ref b @ .., _],
   | |              ---------- also borrowed as immutable, by `b`, here
LL | |             [_, ref mut mid @ ..],
   | |                 ---------------- another mutable borrow, by `mid`, occurs here
LL | |             ..,
LL | |             [..],
LL | |         ] : [[U; 4]; 5]


error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     fn f4_also_moved(ref mut a @ ref mut b @ c: U) {}
   |                      ---------^^^-------------
   |                      |           |           also moved into `c` here
   |                      |           |           also moved into `c` here
   |                      |           another mutable borrow, by `b`, occurs here
   |                      first mutable borrow, by `a`, occurs here
error: cannot move out of value because it is borrowed
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs:21:34
   |
   |
LL |     fn f4_also_moved(ref mut a @ ref mut b @ c: U) {}
   |                                  ---------^^^-
   |                                  |           value moved into `c` here
   |                                  |           value moved into `c` here
   |                                  value borrowed, by `b`, here

error[E0499]: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         |           |
   |         |           |
   |         |           first mutable borrow occurs here
   |         second mutable borrow occurs here
LL |     drop(b);
   |          - first borrow later used here


error[E0499]: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         |           |
   |         |           |
   |         |           first mutable borrow occurs here
   |         second mutable borrow occurs here
...
LL |     *b = U;


error[E0499]: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              second mutable borrow occurs here
   |         first mutable borrow occurs here
...
LL |             *a = Err(U);


error[E0499]: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               second mutable borrow occurs here
   |                                     first mutable borrow occurs here
...
LL |             *a = Err(U);


error[E0499]: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              second mutable borrow occurs here
   |         first mutable borrow occurs here
LL |             drop(a);
   |                  - first borrow later used here


error[E0499]: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               second mutable borrow occurs here
   |                                     first mutable borrow occurs here
LL |             drop(a);
   |                  - first borrow later used here

error[E0382]: borrow of moved value
error[E0382]: borrow of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs:21:34
   |
LL |     fn f4_also_moved(ref mut a @ ref mut b @ c: U) {}
   |                      |           |           |
   |                      |           |           value moved here
   |                      |           value borrowed here after move
   |                      move occurs because value has type `U`, which does not implement the `Copy` trait
