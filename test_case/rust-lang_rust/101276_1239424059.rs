plain
.................................................................i...................... 792/13491
........................................................................i............... 880/13491
........................................................................................ 968/13491
........................................................................................ 1056/13491
...................................F.................................F.................. 1144/13491
.................................................................i...................F.. 1320/13491
.................................................................i...................F.. 1320/13491
...............................F.................................F...................... 1408/13491
........................................................................................ 1584/13491
........................................................................................ 1672/13491
...........................................................................i......ii.... 1760/13491
........................................................................................ 1848/13491
---
.......F....................................i........................................... 3080/13491
..........i............................................................................. 3168/13491
........................................................................................ 3256/13491
..........................................................................iiiii......... 3344/13491
...........................................................F.......F.................... 3432/13491
........................................................................................ 3608/13491
........................................................................................ 3696/13491
........................................................................................ 3784/13491
........................................................................................ 3872/13491
---
........................................................................................ 4928/13491
.................................................................F...................... 5016/13491
........................................................................................ 5104/13491
........................................................................................ 5192/13491
............iF...................................................................i...... 5280/13491
........................................................................................ 5456/13491
........................................................................................ 5544/13491
....................F................................................................... 5632/13491
........................................................................................ 5720/13491
---
........................................................................................ 8008/13491
........................................................................................ 8096/13491
....ii.................i.....i..ii...................................................... 8184/13491
........................................................................................ 8272/13491
..........................F........F..........F...F....F....F............F..F.F..F...... 8360/13491
...............F.....................................F..............FF.F.FF...........F. 8448/13491
......................F...F................F.....................................F...FF. 8536/13491
F..F..F.F.F......F..FF.FFFFFF.FFFFFF.F.F..........i..ii................................. 8624/13491
...................................iiii................................................. 8800/13491
.............................................................................i.......... 8888/13491
..............................i......................................................... 8976/13491
..........i............................................................................. 9064/13491
---
........................................................................................ 9856/13491
........................................................................................ 9944/13491
...........................ii...............i........................................... 10032/13491
........................................................................................ 10120/13491
................................F.......................................F.....F......... 10208/13491
............................................F................................F.......... 10296/13491
...F.F........................F......................................................... 10384/13491
........................................................................................ 10560/13491
........................................................................................ 10648/13491
.......................................................iiiii...i....i.i................. 10736/13491
........................................................................................ 10824/13491
---
........................................................................................ 11792/13491
........................................................................................ 11880/13491
................i.......i........i.....i.....................i.......................... 11968/13491
........................................................................................ 12056/13491
....................................F................................F.................. 12144/13491
........................................................................................ 12320/13491
........................................................................................ 12408/13491
........................................................................................ 12496/13491
........................................................................................ 12584/13491
---
---- [ui] src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs stdout ----
diff of stderr:

10 LL |     y
11    |     - returning this value requires that `*x` is borrowed for `'1`
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
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that *x is borrowed for `'1`
22 LL |     })()
23    |     - return type of async closure is &'1 i32

34 LL |         y
34 LL |         y
35    |         - returning this value requires that `*x` is borrowed for `'1`
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
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that *x is borrowed for `'1`
46 LL |     }
47    |     - return type of async block is &'1 i32


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
   |     - returning this value requires that `*x` is borrowed for `'1`

error[E0506]: cannot assign to *x because it is borrowed
   |
   |
LL |         let y = &*x;
   |                 --- borrow of *x occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed *x occurs here
LL |         y
   |         - returning this value requires that *x is borrowed for `'1`
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
   |         - returning this value requires that `*x` is borrowed for `'1`

error[E0506]: cannot assign to *x because it is borrowed
   |
   |
LL |         let y = &*x;
   |                 --- borrow of *x occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed *x occurs here
LL |         y
   |         - returning this value requires that *x is borrowed for `'1`
LL |     }
   |     - return type of async block is &'1 i32
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-fn-in-const-c.rs stdout ----
diff of stderr:

2   --> $DIR/borrowck-fn-in-const-c.rs:17:16
3    |
4 LL |         return &local.inner;
-    |                ^^^^^^^^^^^^ returning this value requires that `local.inner` is borrowed for `'static`
+    |                ^^^^^^^^^^^^ returning this value requires that local.inner is borrowed for `'static`
6 LL |     }
7    |     - here, drop of `local` needs exclusive access to `local.inner`, because the type `DropString` implements the `Drop` trait


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-fn-in-const-c/borrowck-fn-in-const-c.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-fn-in-const-c.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-fn-in-const-c.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-fn-in-const-c" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-fn-in-const-c/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0713]: borrow may still be in use when destructor runs
   |
   |
LL |         return &local.inner; //~ borrow may still be in use when destructor runs
   |                ^^^^^^^^^^^^ returning this value requires that local.inner is borrowed for `'static`
LL |     }
   |     - here, drop of `local` needs exclusive access to `local.inner`, because the type `DropString` implements the `Drop` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0713`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs stdout ----
diff of stderr:

5    |     ----------^^^^^^^^-
6    |     |         |
7    |     |         borrowed value does not live long enough
-    |     assignment requires that `z.1` is borrowed for `'static`
+    |     assignment requires that z.1 is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `z.1` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn/borrowck-local-borrow-with-panic-outlives-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `z.1` does not live long enough
   |
   |
LL |     *x = Some(&mut z.1);
   |     |         |
   |     |         borrowed value does not live long enough
   |     |         borrowed value does not live long enough
   |     assignment requires that z.1 is borrowed for `'static`
LL | }
LL | }
   | - `z.1` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/mut-borrow-in-loop.rs stdout ----
diff of stderr:

16    |             ------------^^^-
17    |             |           |
18    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |             argument requires that *arg is borrowed for `'a`
20 
21 error[E0499]: cannot borrow `*arg` as mutable more than once at a time

28    |             ------------^^^-
29    |             |           |
29    |             |           |
30    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |             argument requires that *arg is borrowed for `'a`
32 
33 error[E0499]: cannot borrow `*arg` as mutable more than once at a time

40    |             ------------^^^-
41    |             |           |
41    |             |           |
42    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |             argument requires that *arg is borrowed for `'a`
45 error: aborting due to 3 previous errors; 1 warning emitted
46 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop/mut-borrow-in-loop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/mut-borrow-in-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { //~ WARN denote infinite loops with
   |         ^^^^^^^^^^ help: use `loop`
   = note: `#[warn(while_true)]` on by default


error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |             ------------^^^-
   |             |           |
   |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
   |             argument requires that *arg is borrowed for `'a`

error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |             ------------^^^-
   |             |           |
   |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
   |             argument requires that *arg is borrowed for `'a`

error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |             ------------^^^-
   |             |           |
   |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
   |             argument requires that *arg is borrowed for `'a`
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0499`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/two-phase-surprise-no-conflict.rs stdout ----
diff of stderr:

79    |     ^^^^^^^^^^^^^^^^^^-----------------------------------------^
80    |     |                 |                         |
81    |     |                 |                         immutable borrow occurs here
-    |     |                 cast requires that `reg.sess_mut` is borrowed for `'a`
+    |     |                 cast requires that reg.sess_mut is borrowed for `'a`
83    |     mutable borrow occurs here
84 
85 error[E0502]: cannot borrow `*reg` as mutable because it is also borrowed as immutable
122    |     ^^^^^^^^^^^^^^^^^^-------------------------------------------------^
123    |     |                 |                             |
124    |     |                 |                             first mutable borrow occurs here
124    |     |                 |                             first mutable borrow occurs here
-    |     |                 cast requires that `reg.sess_mut` is borrowed for `'a`
+    |     |                 cast requires that reg.sess_mut is borrowed for `'a`
126    |     second mutable borrow occurs here
127 
128 error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict/two-phase-surprise-no-conflict.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/two-phase-surprise-no-conflict.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-surprise-no-conflict.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0503]: cannot use `self.cx` because it was mutably borrowed
   |
   |
LL |         let _mut_borrow = &mut *self;
   |                           ---------- borrow of `*self` occurs here
LL |         let _access = self.cx;
   |                       ^^^^^^^ use of borrowed `*self`
LL |         //~^ ERROR cannot use `self.cx` because it was mutably borrowed [E0503]
LL |         _mut_borrow;


error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
   |
   |
LL |                 self.hash_expr(&self.cx_mut.body(eid).value);
   |                 ^^^^^---------^^---------------------^^^^^^^
   |                 |    |          |
   |                 |    |          immutable borrow occurs here
   |                 |    immutable borrow later used by call
   |                 mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_static(Box::new(TrivialPass::new(&mut reg.sess_mut)));
   |     |   |                                         |
   |     |   |                                         |
   |     |   |                                         second mutable borrow occurs here
   |     first mutable borrow occurs here


error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_bound(Box::new(TrivialPass::new_mut(&mut reg.sess_mut)));
   |     |   |                                            |
   |     |   |                                            |
   |     |   |                                            second mutable borrow occurs here
   |     first mutable borrow occurs here


error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_univ(Box::new(TrivialPass::new_mut(&mut reg.sess_mut)));
   |     |   |                                           |
   |     |   |                                           |
   |     |   |                                           second mutable borrow occurs here
   |     first mutable borrow occurs here


error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_ref(&TrivialPass::new_mut(&mut reg.sess_mut));
   |     |   |                                  |
   |     |   |                                  |
   |     |   |                                  second mutable borrow occurs here
   |     first mutable borrow occurs here


error[E0502]: cannot borrow `*reg` as mutable because it is also borrowed as immutable
   |
   |
LL |     reg.register_bound(Box::new(CapturePass::new(&reg.sess_mut)));
   |     |   |                                        |
   |     |   |                                        |
   |     |   |                                        immutable borrow occurs here
   |     |   immutable borrow later used by call
   |     mutable borrow occurs here

error[E0502]: cannot borrow `*reg` as mutable because it is also borrowed as immutable
   |
   |
LL | fn register_plugins<'a>(mk_reg: impl Fn() -> &'a mut Registry<'a>) {
   |                     -- lifetime `'a` defined here
...
LL |     reg.register_univ(Box::new(CapturePass::new(&reg.sess_mut)));
   |     |                 |                         |
   |     |                 |                         |
   |     |                 |                         immutable borrow occurs here
   |     |                 cast requires that reg.sess_mut is borrowed for `'a`
   |     mutable borrow occurs here

error[E0502]: cannot borrow `*reg` as mutable because it is also borrowed as immutable
   |
   |
LL |     reg.register_ref(&CapturePass::new(&reg.sess_mut));
   |     |   |                              |
   |     |   |                              |
   |     |   |                              immutable borrow occurs here
   |     |   immutable borrow later used by call
   |     mutable borrow occurs here

error[E0499]: cannot borrow `*reg` as mutable more than once at a time
   |
   |
LL |     reg.register_bound(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
   |     |   |                                            |
   |     |   |                                            |
   |     |   |                                            first mutable borrow occurs here
   |     |   first borrow later used by call
   |     second mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_bound(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
   |     |   |                                            |
   |     |   |                                            |
   |     |   |                                            second mutable borrow occurs here
   |     first mutable borrow occurs here


error[E0499]: cannot borrow `*reg` as mutable more than once at a time
   |
   |
LL | fn register_plugins<'a>(mk_reg: impl Fn() -> &'a mut Registry<'a>) {
   |                     -- lifetime `'a` defined here
...
LL |     reg.register_univ(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
   |     |                 |                             |
   |     |                 |                             first mutable borrow occurs here
   |     |                 |                             first mutable borrow occurs here
   |     |                 cast requires that reg.sess_mut is borrowed for `'a`
   |     second mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_univ(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
   |     |   |                                           |
   |     |   |                                           |
   |     |   |                                           second mutable borrow occurs here
   |     first mutable borrow occurs here


error[E0499]: cannot borrow `*reg` as mutable more than once at a time
   |
   |
LL |     reg.register_ref(&CapturePass::new_mut(&mut reg.sess_mut));
   |     |   |                                  |
   |     |   |                                  |
   |     |   |                                  first mutable borrow occurs here
   |     |   first borrow later used by call
   |     second mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_ref(&CapturePass::new_mut(&mut reg.sess_mut));
   |     |   |                                  |
   |     |   |                                  |
   |     |   |                                  second mutable borrow occurs here
   |     first mutable borrow occurs here

error: aborting due to 15 previous errors

---
diff of stderr:

112    |     ------^^^^^^^^
113    |     |     |
114    |     |     borrowed value does not live long enough
-    |     assignment requires that `ap1` is borrowed for `'3`
+    |     assignment requires that ap1 is borrowed for `'3`
117 LL | }
117 LL | }
118    | - `ap1` dropped here while still borrowed

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/variadic-ffi-4.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'f`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'f` but it is returning data with lifetime `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:14:5
   |
   |
LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
   |                                               -- has type `VaListImpl<'1>`
LL |     ap //~ ERROR: lifetime may not live long enough
   |     ^^ returning this value requires that `'1` must outlive `'static`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:18:31
   |
   |
LL |     let _ = ap.with_copy(|ap| ap); //~ ERROR: lifetime may not live long enough
   |                           --- ^^ returning this value requires that `'1` must outlive `'2`
   |                           | |
   |                           | return type of closure is VaList<'2, '_>
   |                           has type `VaList<'1, '_>`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of a mutable reference to `VaListImpl<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of a mutable reference to `VaListImpl<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error[E0597]: `ap1` does not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                                        - let's call the lifetime of this reference `'3`
LL |     ap0 = &mut ap1;
   |     |     |
   |     |     borrowed value does not live long enough
   |     |     borrowed value does not live long enough
   |     assignment requires that ap1 is borrowed for `'3`
LL | }
LL | }
   | - `ap1` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/generic-slice.rs stdout ----
diff of stderr:

8    |         ^^
9    |         |
10    |         borrowed value does not live long enough
-    |         using this value as a constant requires that `x` is borrowed for `'a`
+    |         using this value as a constant requires that x is borrowed for `'a`
13 LL |     };
13 LL |     };
14    |     - `x` dropped here while still borrowed
20    |     ^^
21    |     |
22    |     borrowed value does not live long enough
22    |     borrowed value does not live long enough
-    |     using this value as a static requires that `x` is borrowed for `'static`
+    |     using this value as a static requires that x is borrowed for `'static`
25 LL | };
25 LL | };
26    | - `x` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/generic-slice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/generic-slice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/generic-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL | impl<'a, T: 'static> Generic<'a, T> {
   |      -- lifetime `'a` defined here
LL |         &x
   |         ^^
   |         |
   |         borrowed value does not live long enough
   |         borrowed value does not live long enough
   |         using this value as a constant requires that x is borrowed for `'a`
LL |         //~^ ERROR `x` does not live long enough
LL |     };
   |     - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
LL |     &x
   |     ^^
   |     |
   |     |
   |     borrowed value does not live long enough
   |     using this value as a static requires that x is borrowed for `'static`
LL |     //~^ ERROR `x` does not live long enough
LL | };
   | - `x` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/promote_const_let.rs stdout ----
diff of stderr:

2   --> $DIR/promote_const_let.rs:4:9
3    |
4 LL |     let x: &'static u32 = {
-    |            ------------ type annotation requires that `y` is borrowed for `'static`
+    |            ------------ type annotation requires that y is borrowed for `'static`
6 LL |         let y = 42;
7 LL |         &y
8    |         ^^ borrowed value does not live long enough

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let/promote_const_let.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/promote_const_let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote_const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `y` does not live long enough
   |
LL |     let x: &'static u32 = {
LL |     let x: &'static u32 = {
   |            ------------ type annotation requires that y is borrowed for `'static`
LL |         let y = 42;
LL |         &y //~ ERROR does not live long enough
   |         ^^ borrowed value does not live long enough
LL |     };
   |     - `y` dropped here while still borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote_const_let.rs:6:28
   |
   |
LL |       let x: &'static u32 = &{ //~ ERROR temporary value dropped while borrowed
   |  ____________------------____^
   | |            |
   | |            type annotation requires that borrow lasts for `'static`
LL | |         let y = 42;
LL | |         y
LL | |     };
   | |_____^ creates a temporary which is freed while still in use
LL |   }

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0716.
---
diff of stderr:

2   --> $DIR/dropck_trait_cycle_checked.rs:111:13
3    |
4 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                     -------- cast requires that `o2` is borrowed for `'static`
+    |                                                                     -------- cast requires that o2 is borrowed for `'static`
6 LL |     o1.set0(&o2);
7    |             ^^^ borrowed value does not live long enough

13   --> $DIR/dropck_trait_cycle_checked.rs:112:13
14    |
14    |
15 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                     -------- cast requires that `o3` is borrowed for `'static`
+    |                                                                     -------- cast requires that o3 is borrowed for `'static`
17 LL |     o1.set0(&o2);
18 LL |     o1.set1(&o3);
19    |             ^^^ borrowed value does not live long enough
25   --> $DIR/dropck_trait_cycle_checked.rs:113:13
26    |
26    |
27 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                               -------- cast requires that `o2` is borrowed for `'static`
+    |                                                                               -------- cast requires that o2 is borrowed for `'static`
29 ...
30 LL |     o2.set0(&o2);
31    |             ^^^ borrowed value does not live long enough
37   --> $DIR/dropck_trait_cycle_checked.rs:114:13
38    |
38    |
39 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                               -------- cast requires that `o3` is borrowed for `'static`
+    |                                                                               -------- cast requires that o3 is borrowed for `'static`
41 ...
42 LL |     o2.set1(&o3);
43    |             ^^^ borrowed value does not live long enough
49   --> $DIR/dropck_trait_cycle_checked.rs:115:13
50    |
50    |
51 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                                         -------- cast requires that `o1` is borrowed for `'static`
+    |                                                                                         -------- cast requires that o1 is borrowed for `'static`
53 ...
54 LL |     o3.set0(&o1);
55    |             ^^^ borrowed value does not live long enough
61   --> $DIR/dropck_trait_cycle_checked.rs:116:13
62    |
62    |
63 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                                         -------- cast requires that `o2` is borrowed for `'static`
+    |                                                                                         -------- cast requires that o2 is borrowed for `'static`
65 ...
66 LL |     o3.set1(&o2);
67    |             ^^^ borrowed value does not live long enough

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked/dropck_trait_cycle_checked.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dropck/dropck_trait_cycle_checked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `o2` does not live long enough
   |
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                                                                     -------- cast requires that o2 is borrowed for `'static`
LL |     o1.set0(&o2); //~ ERROR `o2` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o2` dropped here while still borrowed

error[E0597]: `o3` does not live long enough
   |
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                                                                     -------- cast requires that o3 is borrowed for `'static`
LL |     o1.set0(&o2); //~ ERROR `o2` does not live long enough
LL |     o1.set1(&o3); //~ ERROR `o3` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o3` dropped here while still borrowed

error[E0597]: `o2` does not live long enough
   |
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                                                                               -------- cast requires that o2 is borrowed for `'static`
...
LL |     o2.set0(&o2); //~ ERROR `o2` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o2` dropped here while still borrowed

error[E0597]: `o3` does not live long enough
   |
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                                                                               -------- cast requires that o3 is borrowed for `'static`
...
LL |     o2.set1(&o3); //~ ERROR `o3` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o3` dropped here while still borrowed

error[E0597]: `o1` does not live long enough
   |
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                                                                                         -------- cast requires that o1 is borrowed for `'static`
...
LL |     o3.set0(&o1); //~ ERROR `o1` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL |     o3.set1(&o2); //~ ERROR `o2` does not live long enough
LL | }
   | - `o1` dropped here while still borrowed

error[E0597]: `o2` does not live long enough
   |
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                                                                                         -------- cast requires that o2 is borrowed for `'static`
...
LL |     o3.set1(&o2); //~ ERROR `o2` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
   | - `o2` dropped here while still borrowed
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/dst/dst-bad-coerce3.rs stdout ----
diff of stderr:

7 LL |     let f2: &Fat<[isize; 3]> = &f1;
8    |                                ^^^ borrowed value does not live long enough
9 LL |     let f3: &'a Fat<[isize]> = f2;
-    |             ---------------- type annotation requires that `f1` is borrowed for `'a`
+    |             ---------------- type annotation requires that f1 is borrowed for `'a`
12 LL | }
12 LL | }
13    | - `f1` dropped here while still borrowed

21 LL |     let f2: &Fat<Foo> = &f1;
22    |                         ^^^ borrowed value does not live long enough
23 LL |     let f3: &'a Fat<dyn Bar> = f2;
-    |             ---------------- type annotation requires that `f1` is borrowed for `'a`
+    |             ---------------- type annotation requires that f1 is borrowed for `'a`
26 LL | }
26 LL | }
27    | - `f1` dropped here while still borrowed

35 LL |     let f2: &([isize; 3],) = &f1;
36    |                              ^^^ borrowed value does not live long enough
37 LL |     let f3: &'a ([isize],) = f2;
-    |             -------------- type annotation requires that `f1` is borrowed for `'a`
+    |             -------------- type annotation requires that f1 is borrowed for `'a`
40 LL | }
40 LL | }
41    | - `f1` dropped here while still borrowed

49 LL |     let f2: &(Foo,) = &f1;
50    |                       ^^^ borrowed value does not live long enough
51 LL |     let f3: &'a (dyn Bar,) = f2;
-    |             -------------- type annotation requires that `f1` is borrowed for `'a`
+    |             -------------- type annotation requires that f1 is borrowed for `'a`
53 LL | }
54    | - `f1` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coerce3/dst-bad-coerce3.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coerce3/dst-bad-coerce3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dst/dst-bad-coerce3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dst/dst-bad-coerce3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coerce3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-bad-coerce3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `f1` does not live long enough
   |
   |
LL | fn baz<'a>() {
   |        -- lifetime `'a` defined here
...
LL |     let f2: &Fat<[isize; 3]> = &f1; //~ ERROR `f1` does not live long enough
   |                                ^^^ borrowed value does not live long enough
LL |     let f3: &'a Fat<[isize]> = f2;
   |             ---------------- type annotation requires that f1 is borrowed for `'a`
LL | }
LL | }
   | - `f1` dropped here while still borrowed

error[E0597]: `f1` does not live long enough
   |
   |
LL | fn baz<'a>() {
   |        -- lifetime `'a` defined here
...
LL |     let f2: &Fat<Foo> = &f1; //~ ERROR `f1` does not live long enough
   |                         ^^^ borrowed value does not live long enough
LL |     let f3: &'a Fat<dyn Bar> = f2;
   |             ---------------- type annotation requires that f1 is borrowed for `'a`
LL | }
LL | }
   | - `f1` dropped here while still borrowed

error[E0597]: `f1` does not live long enough
   |
   |
LL | fn baz<'a>() {
   |        -- lifetime `'a` defined here
...
LL |     let f2: &([isize; 3],) = &f1; //~ ERROR `f1` does not live long enough
   |                              ^^^ borrowed value does not live long enough
LL |     let f3: &'a ([isize],) = f2;
   |             -------------- type annotation requires that f1 is borrowed for `'a`
LL | }
LL | }
   | - `f1` dropped here while still borrowed

error[E0597]: `f1` does not live long enough
   |
   |
LL | fn baz<'a>() {
   |        -- lifetime `'a` defined here
...
LL |     let f2: &(Foo,) = &f1; //~ ERROR `f1` does not live long enough
   |                       ^^^ borrowed value does not live long enough
LL |     let f3: &'a (dyn Bar,) = f2;
   |             -------------- type annotation requires that f1 is borrowed for `'a`
LL | }
   | - `f1` dropped here while still borrowed
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/issue-74684-1.rs stdout ----
diff of stderr:

8    |              ------------^^-
9    |              |           |
10    |              |           borrowed value does not live long enough
-    |              argument requires that `a` is borrowed for `'a`
+    |              argument requires that a is borrowed for `'a`
13 LL | }
13 LL | }
14    | - `a` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-74684-1/issue-74684-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-74684-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-74684-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-74684-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-74684-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL | fn bug<'a, T: ?Sized + Fun<F<'a> = [u8]>>(_ : Box<T>) -> &'static T::F<'a> {
   |        -- lifetime `'a` defined here
LL |     let a = [0; 1];
LL |     let _x = T::identity(&a);
   |              ------------^^-
   |              |           borrowed value does not live long enough
   |              |           borrowed value does not live long enough
   |              argument requires that a is borrowed for `'a`
LL | }
LL | }
   | - `a` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs stdout ----
diff of stderr:

5    |                          -----^^------
6    |                          |    |
7    |                          |    borrowed value does not live long enough
-    |                          argument requires that `x` is borrowed for `'static`
+    |                          argument requires that x is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `x` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty/assoc-ty-wf-used-to-get-assoc-ty.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/implied-bounds/assoc-ty-wf-used-to-get-assoc-ty/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     let _: &'static u8 = test(&x, &&3);
   |                          -----^^------
   |                          |    borrowed value does not live long enough
   |                          |    borrowed value does not live long enough
   |                          argument requires that x is borrowed for `'static`
LL | }
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/inline-const/const-expr-lifetime-err.rs stdout ----
diff of stderr:

8    |            ------------------^^-
9    |            |                 |
10    |            |                 borrowed value does not live long enough
-    |            argument requires that `y` is borrowed for `'a`
+    |            argument requires that y is borrowed for `'a`
13 LL | }
13 LL | }
14    | - `y` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-lifetime-err/const-expr-lifetime-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inline-const/const-expr-lifetime-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-expr-lifetime-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-lifetime-err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-lifetime-err/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `y` does not live long enough
   |
   |
LL | fn foo<'a>() {
   |        -- lifetime `'a` defined here
LL |     let y = ();
LL |     equate(InvariantRef::new(&y), const { InvariantRef::<'a>::NEW });
   |            ------------------^^-
   |            |                 borrowed value does not live long enough
   |            |                 borrowed value does not live long enough
   |            argument requires that y is borrowed for `'a`
LL |     //~^ ERROR `y` does not live long enough [E0597]
LL | }
   | - `y` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-18118.rs stdout ----
diff of stderr:

5    |         ^^
6    |         |
7    |         borrowed value does not live long enough
-    |         using this value as a constant requires that `p` is borrowed for `'static`
+    |         using this value as a constant requires that p is borrowed for `'static`
9 LL |     };
10    |     - `p` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/issue-18118.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/issue-18118.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18118.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `p` does not live long enough
   |
   |
LL |         &p //~ ERROR `p` does not live long enough
   |         |
   |         borrowed value does not live long enough
   |         borrowed value does not live long enough
   |         using this value as a constant requires that p is borrowed for `'static`
LL |     };
   |     - `p` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lifetimes/issue-90600-expected-return-static-indirect.rs stdout ----
diff of stderr:

5    |                                ^^^^^^^^ borrowed value does not live long enough
6 LL |
7 LL |     let read = &refcell as &RefCell<dyn Read>;
-    |                -------- cast requires that `foo` is borrowed for `'static`
+    |                -------- cast requires that foo is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `foo` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect/issue-90600-expected-return-static-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-90600-expected-return-static-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-90600-expected-return-static-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `foo` does not live long enough
   |
   |
LL |     let refcell = RefCell::new(&mut foo);
   |                                ^^^^^^^^ borrowed value does not live long enough
LL |     //~^ ERROR `foo` does not live long enough
LL |     let read = &refcell as &RefCell<dyn Read>;
   |                -------- cast requires that foo is borrowed for `'static`
LL | }
LL | }
   | - `foo` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90600-expected-return-static-indirect.rs:9:16
   |
   |
LL | fn inner(mut foo: &[u8]) {
   |                   - let's call the lifetime of this reference `'1`
...
LL |     let read = &refcell as &RefCell<dyn Read>;
   |                ^^^^^^^^ cast requires that `'1` must outlive `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs stdout ----
diff of stderr:

71 LL | /     foo(cell, |cell_a, cell_x| {
72 LL | |         cell_x.set(cell_a.get()); // forces 'a: 'x, implies 'a = 'static -> borrow error
73 LL | |     })
-    | |______- argument requires that `a` is borrowed for `'static`
+    | |______- argument requires that a is borrowed for `'static`
75 LL |   }
76    |   - `a` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/propagate-approximated-shorter-to-static-comparing-against-free.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/propagate-approximated-shorter-to-static-comparing-against-free.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     foo(cell, |cell_a, cell_x| {
---
diff of stderr:

2   --> $DIR/propagate-multiple-requirements.rs:15:14
3    |
4 LL |     let mut out: &mut &'static [i32] = &mut (&[1] as _);
-    |                  ------------------- type annotation requires that `local_arr` is borrowed for `'static`
+    |                  ------------------- type annotation requires that local_arr is borrowed for `'static`
6 LL |     once(|mut z: &[i32], mut out_val: &mut &[i32]| {
7    |          ----------------------------------------- value captured here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-multiple-requirements/propagate-multiple-requirements.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-multiple-requirements/propagate-multiple-requirements.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-multiple-requirements.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-multiple-requirements.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-multiple-requirements" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-multiple-requirements/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `local_arr` does not live long enough
   |
   |
LL |     let mut out: &mut &'static [i32] = &mut (&[1] as _);
   |                  ------------------- type annotation requires that local_arr is borrowed for `'static`
LL |     once(|mut z: &[i32], mut out_val: &mut &[i32]| {
   |          ----------------------------------------- value captured here
...
LL |         z = &local_arr; //~ ERROR
   |              ^^^^^^^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `local_arr` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj.rs stdout ----
diff of stderr:

4 LL |     let a = (Foo(&s),);
5    |                  ^^ borrowed value does not live long enough
6 LL |     drop(a.0);
-    |          --- copying this value requires that `s` is borrowed for `'static`
+    |          --- copying this value requires that s is borrowed for `'static`
8 LL |     drop(a.0);
9 LL | }
10    | - `s` dropped here while still borrowed

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj/do-not-ignore-lifetime-bounds-in-copy-proj.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/do-not-ignore-lifetime-bounds-in-copy-proj.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |     let a = (Foo(&s),); //~ ERROR `s` does not live long enough [E0597]
   |                  ^^ borrowed value does not live long enough
LL |     drop(a.0);
   |          --- copying this value requires that s is borrowed for `'static`
LL |     drop(a.0);
LL | }
   | - `s` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy.rs stdout ----
diff of stderr:

4 LL |     let a = Foo(&s);
5    |                 ^^ borrowed value does not live long enough
6 LL |     drop(a);
-    |          - copying this value requires that `s` is borrowed for `'static`
+    |          - copying this value requires that s is borrowed for `'static`
8 LL |     drop(a);
9 LL | }
10    | - `s` dropped here while still borrowed

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy/do-not-ignore-lifetime-bounds-in-copy.stderr
To only update this specific test, also pass `--test-args nll/do-not-ignore-lifetime-bounds-in-copy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |     let a = Foo(&s); //~ ERROR `s` does not live long enough [E0597]
   |                 ^^ borrowed value does not live long enough
LL |     drop(a);
   |          - copying this value requires that s is borrowed for `'static`
LL |     drop(a);
LL | }
   | - `s` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/enum-drop-access.rs stdout ----
diff of stderr:

7 LL |         DropOption::Some(&mut ref mut r) => {
9 LL |             Some(r)
9 LL |             Some(r)
-    |             ------- returning this value requires that `*opt.0` is borrowed for `'1`
+    |             ------- returning this value requires that *opt.0 is borrowed for `'1`
12 LL | }
12 LL | }
13    | - here, drop of `opt` needs exclusive access to `*opt.0`, because the type `DropOption<&mut i32>` implements the `Drop` trait

21 LL |         Some(DropOption::Some(&mut ref mut r)) => {
23 LL |             Some(r)
23 LL |             Some(r)
-    |             ------- returning this value requires that `*opt.0.0` is borrowed for `'1`
+    |             ------- returning this value requires that *opt.0.0 is borrowed for `'1`
26 LL | }
26 LL | }
27    | - here, drop of `opt` needs exclusive access to `*opt.0.0`, because the type `DropOption<&mut i32>` implements the `Drop` trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/enum-drop-access/enum-drop-access.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/enum-drop-access.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/enum-drop-access.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/enum-drop-access" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/enum-drop-access/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0713]: borrow may still be in use when destructor runs
   |
   |
LL | fn drop_enum(opt: DropOption<&mut i32>) -> Option<&mut i32> {
   |                              - let's call the lifetime of this reference `'1`
LL |     match opt {
LL |         DropOption::Some(&mut ref mut r) => { //~ ERROR
LL |             Some(r)
LL |             Some(r)
   |             ------- returning this value requires that *opt.0 is borrowed for `'1`
LL | }
LL | }
   | - here, drop of `opt` needs exclusive access to `*opt.0`, because the type `DropOption<&mut i32>` implements the `Drop` trait

error[E0713]: borrow may still be in use when destructor runs
   |
   |
LL | fn optional_drop_enum(opt: Option<DropOption<&mut i32>>) -> Option<&mut i32> {
   |                                              - let's call the lifetime of this reference `'1`
LL |     match opt {
LL |         Some(DropOption::Some(&mut ref mut r)) => { //~ ERROR
LL |             Some(r)
LL |             Some(r)
   |             ------- returning this value requires that *opt.0.0 is borrowed for `'1`
LL | }
LL | }
   | - here, drop of `opt` needs exclusive access to `*opt.0.0`, because the type `DropOption<&mut i32>` implements the `Drop` trait
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0713`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/get_default.rs stdout ----
diff of stderr:

8    |               --------- immutable borrow occurs here
9 LL |             Some(v) => {
10 LL |                 return v;
-    |                        - returning this value requires that `*map` is borrowed for `'1`
+    |                        - returning this value requires that *map is borrowed for `'1`
12 ...
13 LL |                 map.set(String::new()); // Ideally, this would not error.
14    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

26    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
28 LL |                 return v;
28 LL |                 return v;
-    |                        - returning this value requires that `*map` is borrowed for `'1`
+    |                        - returning this value requires that *map is borrowed for `'1`
30 
31 error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable


38    |               --------- immutable borrow occurs here
40 LL |                 return v;
40 LL |                 return v;
-    |                        - returning this value requires that `*map` is borrowed for `'1`
+    |                        - returning this value requires that *map is borrowed for `'1`
42 ...
43 LL |                 map.set(String::new()); // Ideally, just AST would error here
44    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default/get_default.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/get_default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/get_default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable
   |
   |
LL | fn ok(map: &mut Map) -> &String {
   |            - let's call the lifetime of this reference `'1`
LL |     loop {
LL |         match map.get() {
   |               --------- immutable borrow occurs here
LL |             Some(v) => {
LL |                 return v;
   |                        - returning this value requires that *map is borrowed for `'1`
...
LL |                 map.set(String::new()); // Ideally, this would not error.
   |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable
   |
   |
LL | fn err(map: &mut Map) -> &String {
   |             - let's call the lifetime of this reference `'1`
LL |     loop {
LL |         match map.get() {
   |               --------- immutable borrow occurs here
LL |             Some(v) => {
LL |                 map.set(String::new()); // Both AST and MIR error here
   |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
LL |                 //~^ ERROR borrowed as immutable
LL |                 return v;
   |                        - returning this value requires that *map is borrowed for `'1`

error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable
   |
   |
LL | fn err(map: &mut Map) -> &String {
   |             - let's call the lifetime of this reference `'1`
LL |     loop {
LL |         match map.get() {
   |               --------- immutable borrow occurs here
LL |                 return v;
LL |                 return v;
   |                        - returning this value requires that *map is borrowed for `'1`
...
LL |                 map.set(String::new()); // Ideally, just AST would error here
   |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0502`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-42574-diagnostic-in-nested-closure.rs stdout ----
diff of stderr:

15    |     -- -----^^^^-
16    |     |  |    |
17    |     |  |    borrowed value does not live long enough
-    |     |  argument requires that `data` is borrowed for `'static`
+    |     |  argument requires that data is borrowed for `'static`
20 ...
21 LL | }



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure/issue-42574-diagnostic-in-nested-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-42574-diagnostic-in-nested-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-42574-diagnostic-in-nested-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     || doit(data);
   |     -- ^^^^^^^^^^ argument requires that `'1` must outlive `'static`
   |     |
   |     lifetime `'1` represents this closure's body
   |
   = note: closure implements `FnMut`, so references to captured variables can't escape the closure
error[E0597]: `data` does not live long enough
  --> /checkout/src/test/ui/nll/issue-42574-diagnostic-in-nested-closure.rs:6:13
   |
   |
LL |     || doit(data);
   |     -- -----^^^^-
   |     |  |    borrowed value does not live long enough
   |     |  |    borrowed value does not live long enough
   |     |  argument requires that data is borrowed for `'static`
...
LL | }
LL | }
   |  - `data` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-31567.rs stdout ----
diff of stderr:

6 LL |     let s_inner: &'a S = &*v.0;
8    |                  |
8    |                  |
-    |                  type annotation requires that `*v.0` is borrowed for `'a`
+    |                  type annotation requires that *v.0 is borrowed for `'a`
10 LL |     &s_inner.0
11 LL | }
12    | - here, drop of `v` needs exclusive access to `*v.0`, because the type `VecWrapper<'_>` implements the `Drop` trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-31567/issue-31567.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-31567.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-31567.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-31567" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-31567/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0713]: borrow may still be in use when destructor runs
   |
   |
LL | fn get_dangling<'a>(v: VecWrapper<'a>) -> &'a u32 {
   |                 -- lifetime `'a` defined here
LL |     let s_inner: &'a S = &*v.0; //~ ERROR borrow may still be in use when destructor runs [E0713]
   |                  |
   |                  |
   |                  type annotation requires that *v.0 is borrowed for `'a`
LL |     &s_inner.0
LL | }
   | - here, drop of `v` needs exclusive access to `*v.0`, because the type `VecWrapper<'_>` implements the `Drop` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0713`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-46036.rs stdout ----
diff of stderr:

5    |                        ^^
6    |                        |
7    |                        borrowed value does not live long enough
-    |                        this usage requires that `a` is borrowed for `'static`
+    |                        this usage requires that a is borrowed for `'static`
9 LL |     loop { }
10 LL | }
11    | - `a` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-46036/issue-46036.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-46036.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-46036.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-46036" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-46036/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     let foo = Foo { x: &a }; //~ ERROR E0597
   |                        |
   |                        borrowed value does not live long enough
   |                        borrowed value does not live long enough
   |                        this usage requires that a is borrowed for `'static`
LL |     loop { }
LL | }
   | - `a` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-45696-scribble-on-boxed-borrow.rs stdout ----
diff of stderr:

4 LL | fn scribbled<'a>(s: Scribble<'a>) -> &'a mut u32 {
5    |              -- lifetime `'a` defined here
6 LL |     &mut *s.0
-    |     ^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
+    |     ^^^^^^^^^ returning this value requires that *s.0 is borrowed for `'a`
8 LL | }
9    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait


14 LL | fn boxed_scribbled<'a>(s: Box<Scribble<'a>>) -> &'a mut u32 {
15    |                    -- lifetime `'a` defined here
16 LL |     &mut *(*s).0
-    |     ^^^^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
+    |     ^^^^^^^^^^^^ returning this value requires that *s.0 is borrowed for `'a`
18 LL | }
19    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait


24 LL | fn boxed_boxed_scribbled<'a>(s: Box<Box<Scribble<'a>>>) -> &'a mut u32 {
25    |                          -- lifetime `'a` defined here
26 LL |     &mut *(**s).0
-    |     ^^^^^^^^^^^^^ returning this value requires that `*s.0` is borrowed for `'a`
+    |     ^^^^^^^^^^^^^ returning this value requires that *s.0 is borrowed for `'a`
28 LL | }
29    | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-45696-scribble-on-boxed-borrow/issue-45696-scribble-on-boxed-borrow.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-45696-scribble-on-boxed-borrow/issue-45696-scribble-on-boxed-borrow.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-45696-scribble-on-boxed-borrow.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-45696-scribble-on-boxed-borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-45696-scribble-on-boxed-borrow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-45696-scribble-on-boxed-borrow/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0713]: borrow may still be in use when destructor runs
   |
   |
LL | fn scribbled<'a>(s: Scribble<'a>) -> &'a mut u32 {
   |              -- lifetime `'a` defined here
LL |     &mut *s.0 //~ ERROR borrow may still be in use when destructor runs [E0713]
   |     ^^^^^^^^^ returning this value requires that *s.0 is borrowed for `'a`
LL | }
   | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait

error[E0713]: borrow may still be in use when destructor runs
   |
   |
LL | fn boxed_scribbled<'a>(s: Box<Scribble<'a>>) -> &'a mut u32 {
   |                    -- lifetime `'a` defined here
LL |     &mut *(*s).0 //~ ERROR borrow may still be in use when destructor runs [E0713]
   |     ^^^^^^^^^^^^ returning this value requires that *s.0 is borrowed for `'a`
LL | }
   | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait

error[E0713]: borrow may still be in use when destructor runs
   |
   |
LL | fn boxed_boxed_scribbled<'a>(s: Box<Box<Scribble<'a>>>) -> &'a mut u32 {
   |                          -- lifetime `'a` defined here
LL |     &mut *(**s).0 //~ ERROR borrow may still be in use when destructor runs [E0713]
   |     ^^^^^^^^^^^^^ returning this value requires that *s.0 is borrowed for `'a`
LL | }
   | - here, drop of `s` needs exclusive access to `*s.0`, because the type `Scribble<'_>` implements the `Drop` trait
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0713`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-52059-report-when-borrow-and-drop-conflict.rs stdout ----
diff of stderr:

4 LL | fn finish_1(s: S) -> &mut String {
5    |             - has type `S<'1>`
6 LL |     s.url
-    |     ^^^^^ returning this value requires that `*s.url` is borrowed for `'1`
+    |     ^^^^^ returning this value requires that *s.url is borrowed for `'1`
---

---- [ui] src/test/ui/nll/relate_tys/var-appears-twice.rs stdout ----
diff of stderr:

4 LL |     let x: DoubleCell<_> = make_cell(&b);
5    |            -------------             ^^ borrowed value does not live long enough
6    |            |
-    |            type annotation requires that `b` is borrowed for `'static`
+    |            type annotation requires that b is borrowed for `'static`
9 LL | }
9 LL | }
10    | - `b` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/var-appears-twice/var-appears-twice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/relate_tys/var-appears-twice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/relate_tys/var-appears-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/var-appears-twice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/relate_tys/var-appears-twice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `b` does not live long enough
   |
   |
LL |     let x: DoubleCell<_> = make_cell(&b); //~ ERROR
   |            -------------             ^^ borrowed value does not live long enough
   |            |
   |            type annotation requires that b is borrowed for `'static`
LL | }
LL | }
   | - `b` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/user-annotations/adt-brace-enums.rs stdout ----
diff of stderr:

5    |                                                ^^
6    |                                                |
7    |                                                borrowed value does not live long enough
-    |                                                this usage requires that `c` is borrowed for `'static`
+    |                                                this usage requires that c is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed

19    |                                           ^^
20    |                                           |
21    |                                           borrowed value does not live long enough
21    |                                           borrowed value does not live long enough
-    |                                           this usage requires that `c` is borrowed for `'a`
+    |                                           this usage requires that c is borrowed for `'a`
23 LL | }
24    | - `c` dropped here while still borrowed

33    |                                               ^^
34    |                                               |
35    |                                               borrowed value does not live long enough
35    |                                               borrowed value does not live long enough
-    |                                               this usage requires that `c` is borrowed for `'a`
+    |                                               this usage requires that c is borrowed for `'a`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
38    |     - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/adt-brace-enums.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/adt-brace-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-brace-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-brace-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeEnum::SomeVariant::<&'static u32> { t: &c }; //~ ERROR
   |                                                |
   |                                                borrowed value does not live long enough
   |                                                borrowed value does not live long enough
   |                                                this usage requires that c is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-brace-enums.rs:30:43
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeEnum::SomeVariant::<&'a u32> { t: &c }; //~ ERROR
   |                                           |
   |                                           borrowed value does not live long enough
   |                                           borrowed value does not live long enough
   |                                           this usage requires that c is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-brace-enums.rs:40:47
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         SomeEnum::SomeVariant::<&'a u32> { t: &c }; //~ ERROR
   |                                               |
   |                                               borrowed value does not live long enough
   |                                               borrowed value does not live long enough
   |                                               this usage requires that c is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/user-annotations/adt-brace-structs.rs stdout ----
diff of stderr:

5    |                                     ^^
6    |                                     |
7    |                                     borrowed value does not live long enough
-    |                                     this usage requires that `c` is borrowed for `'static`
+    |                                     this usage requires that c is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed

19    |                                ^^
20    |                                |
21    |                                borrowed value does not live long enough
21    |                                borrowed value does not live long enough
-    |                                this usage requires that `c` is borrowed for `'a`
+    |                                this usage requires that c is borrowed for `'a`
23 LL | }
24    | - `c` dropped here while still borrowed

33    |                                    ^^
34    |                                    |
35    |                                    borrowed value does not live long enough
35    |                                    borrowed value does not live long enough
-    |                                    this usage requires that `c` is borrowed for `'a`
+    |                                    this usage requires that c is borrowed for `'a`
37 LL |     };
38    |     - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/adt-brace-structs.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/adt-brace-structs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-brace-structs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-brace-structs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeStruct::<&'static u32> { t: &c }; //~ ERROR
   |                                     |
   |                                     borrowed value does not live long enough
   |                                     borrowed value does not live long enough
   |                                     this usage requires that c is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-brace-structs.rs:28:32
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeStruct::<&'a u32> { t: &c }; //~ ERROR
   |                                |
   |                                borrowed value does not live long enough
   |                                borrowed value does not live long enough
   |                                this usage requires that c is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-brace-structs.rs:38:36
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         SomeStruct::<&'a u32> { t: &c }; //~ ERROR
   |                                    |
   |                                    borrowed value does not live long enough
   |                                    borrowed value does not live long enough
   |                                    this usage requires that c is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/user-annotations/adt-tuple-enums.rs stdout ----
diff of stderr:

5    |                                           ^^
6    |                                           |
7    |                                           borrowed value does not live long enough
-    |                                           this usage requires that `c` is borrowed for `'static`
+    |                                           this usage requires that c is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed

19    |                                      ^^
20    |                                      |
21    |                                      borrowed value does not live long enough
21    |                                      borrowed value does not live long enough
-    |                                      this usage requires that `c` is borrowed for `'a`
+    |                                      this usage requires that c is borrowed for `'a`
23 LL | }
24    | - `c` dropped here while still borrowed

33    |                                          ^^
34    |                                          |
35    |                                          borrowed value does not live long enough
35    |                                          borrowed value does not live long enough
-    |                                          this usage requires that `c` is borrowed for `'a`
+    |                                          this usage requires that c is borrowed for `'a`
37 LL |     };
38    |     - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/adt-tuple-enums.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/adt-tuple-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeEnum::SomeVariant::<&'static u32>(&c); //~ ERROR
   |                                           |
   |                                           borrowed value does not live long enough
   |                                           borrowed value does not live long enough
   |                                           this usage requires that c is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-tuple-enums.rs:33:38
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeEnum::SomeVariant::<&'a u32>(&c); //~ ERROR
   |                                      |
   |                                      borrowed value does not live long enough
   |                                      borrowed value does not live long enough
   |                                      this usage requires that c is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-tuple-enums.rs:43:42
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         SomeEnum::SomeVariant::<&'a u32>(&c); //~ ERROR
   |                                          |
   |                                          borrowed value does not live long enough
   |                                          borrowed value does not live long enough
   |                                          this usage requires that c is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/user-annotations/adt-nullary-enums.rs stdout ----
diff of stderr:

6    | |                                         ^^ borrowed value does not live long enough
7 LL | |         SomeEnum::SomeOtherVariant::<Cell<&'static u32>>,
8 LL | |     );
-    | |_____- argument requires that `c` is borrowed for `'static`
+    | |_____- argument requires that c is borrowed for `'static`
10 LL |   }
11    |   - `c` dropped here while still borrowed

20    |                               ----------^^-
21    |                               |         |
22    |                               |         borrowed value does not live long enough
22    |                               |         borrowed value does not live long enough
-    |                               argument requires that `c` is borrowed for `'a`
+    |                               argument requires that c is borrowed for `'a`
25 LL | }
25 LL | }
26    | - `c` dropped here while still borrowed
37    |                                   ----------^^-
38    |                                   |         |
39    |                                   |         borrowed value does not live long enough
39    |                                   |         borrowed value does not live long enough
-    |                                   argument requires that `c` is borrowed for `'a`
+    |                                   argument requires that c is borrowed for `'a`
42 error: aborting due to 3 previous errors
43 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/adt-nullary-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-nullary-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
   |
LL | /     combine(
LL | /     combine(
LL | |         SomeEnum::SomeVariant(Cell::new(&c)), //~ ERROR
   | |                                         ^^ borrowed value does not live long enough
LL | |         SomeEnum::SomeOtherVariant::<Cell<&'static u32>>,
LL | |     );
   | |_____- argument requires that c is borrowed for `'static`
LL |   }
   |   - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs:41:41
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
...
LL |         SomeEnum::SomeVariant(Cell::new(&c)), //~ ERROR
   |                               ----------^^-
   |                               |         borrowed value does not live long enough
   |                               |         borrowed value does not live long enough
   |                               argument requires that c is borrowed for `'a`
LL | }
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs:54:45
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
LL |     let _closure = || {
   |                     - `c` dropped here while still borrowed
...
LL |             SomeEnum::SomeVariant(Cell::new(&c)), //~ ERROR
   |                                   ----------^^-
   |                                   |         borrowed value does not live long enough
   |                                   |         borrowed value does not live long enough
   |                                   argument requires that c is borrowed for `'a`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/user-annotations/cast_static_lifetime.rs stdout ----
diff of stderr:

5    |                   ^^^^----------------
6    |                   |
7    |                   borrowed value does not live long enough
-    |                   type annotation requires that `x` is borrowed for `'static`
+    |                   type annotation requires that x is borrowed for `'static`
9 LL | }
10    | - `x` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/cast_static_lifetime.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/cast_static_lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/cast_static_lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/cast_static_lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/cast_static_lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     let y: &u32 = (&x) as &'static u32; //~ ERROR `x` does not live long enough
   |                   |
   |                   borrowed value does not live long enough
   |                   borrowed value does not live long enough
   |                   type annotation requires that x is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/user-annotations/adt-tuple-struct.rs stdout ----
diff of stderr:

5    |                                ^^
6    |                                |
7    |                                borrowed value does not live long enough
-    |                                this usage requires that `c` is borrowed for `'static`
+    |                                this usage requires that c is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed

19    |                           ^^
20    |                           |
21    |                           borrowed value does not live long enough
21    |                           borrowed value does not live long enough
-    |                           this usage requires that `c` is borrowed for `'a`
+    |                           this usage requires that c is borrowed for `'a`
23 LL | }
24    | - `c` dropped here while still borrowed

33    |                               ^^
34    |                               |
35    |                               borrowed value does not live long enough
35    |                               borrowed value does not live long enough
-    |                               this usage requires that `c` is borrowed for `'a`
+    |                               this usage requires that c is borrowed for `'a`
37 LL |     };
38    |     - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/adt-tuple-struct.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/adt-tuple-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeStruct::<&'static u32>(&c); //~ ERROR
   |                                |
   |                                borrowed value does not live long enough
   |                                borrowed value does not live long enough
   |                                this usage requires that c is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-tuple-struct.rs:28:27
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeStruct::<&'a u32>(&c); //~ ERROR
   |                           |
   |                           borrowed value does not live long enough
   |                           borrowed value does not live long enough
   |                           this usage requires that c is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed
error[E0597]: `c` does not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/adt-tuple-struct.rs:38:31
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
---
diff of stderr:

5    |     --^^-
6    |     | |
7    |     | borrowed value does not live long enough
-    |     opaque type requires that `x` is borrowed for `'static`
+    |     opaque type requires that x is borrowed for `'static`
9 LL | }
10    | - `x` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/do-not-suggest-adding-bound-to-opaque-type/do-not-suggest-adding-bound-to-opaque-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/do-not-suggest-adding-bound-to-opaque-type/do-not-suggest-adding-bound-to-opaque-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/do-not-suggest-adding-bound-to-opaque-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/do-not-suggest-adding-bound-to-opaque-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/do-not-suggest-adding-bound-to-opaque-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/do-not-suggest-adding-bound-to-opaque-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     S(&x) //~ ERROR `x` does not live long enough
   |     --^^-
   |     | borrowed value does not live long enough
   |     | borrowed value does not live long enough
   |     opaque type requires that x is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/regions-addr-of-arg.rs stdout ----
diff of stderr:

4 LL |     let _p: &'static isize = &a;
5    |             --------------   ^^ borrowed value does not live long enough
6    |             |
-    |             type annotation requires that `a` is borrowed for `'static`
+    |             type annotation requires that a is borrowed for `'static`
8 LL | }
9    |  - `a` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-arg/regions-addr-of-arg.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-arg/regions-addr-of-arg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-addr-of-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-addr-of-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-arg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-arg/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     let _p: &'static isize = &a; //~ ERROR `a` does not live long enough
   |             --------------   ^^ borrowed value does not live long enough
   |             |
   |             type annotation requires that a is borrowed for `'static`
LL | }
   |  - `a` dropped here while still borrowed
error[E0515]: cannot return reference to function parameter `a`
  --> /checkout/src/test/ui/regions/regions-addr-of-arg.rs:13:5
   |
   |
LL |     &a //~ ERROR cannot return reference to function parameter `a`
   |     ^^ returns a reference to data owned by the current function
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0515, E0597.
For more information about an error, try `rustc --explain E0515`.
For more information about an error, try `rustc --explain E0515`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-addr-of-upvar-self.rs stdout ----
diff of stderr:

25 LL |             let p: &'static mut usize = &mut self.food;
26    |                    ------------------        ^^^^^^^^^ borrowed value does not live long enough
27    |                    |
-    |                    type annotation requires that `self` is borrowed for `'static`
+    |                    type annotation requires that self is borrowed for `'static`
30 LL |     }
30 LL |     }
31    |      - `self` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-upvar-self/regions-addr-of-upvar-self.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-addr-of-upvar-self.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-addr-of-upvar-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-upvar-self" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-addr-of-upvar-self/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |         let _f = || {
   |                  -- lifetime `'1` represents this closure's body
LL |             let p: &'static mut usize = &mut self.food;
   |                    ^^^^^^^^^^^^^^^^^^ type annotation requires that `'1` must outlive `'static`
   |
   = note: closure implements `FnMut`, so references to captured variables can't escape the closure
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-addr-of-upvar-self.rs:8:20
   |
   |
LL |     pub fn chase_cat(&mut self) {
   |                      - let's call the lifetime of this reference `'1`
LL |         let _f = || {
LL |             let p: &'static mut usize = &mut self.food;
   |                    ^^^^^^^^^^^^^^^^^^ type annotation requires that `'1` must outlive `'static`
error[E0597]: `self` does not live long enough
  --> /checkout/src/test/ui/regions/regions-addr-of-upvar-self.rs:8:46
   |
   |
LL |         let _f = || {
   |                  -- value captured here
LL |             let p: &'static mut usize = &mut self.food;
   |                    ------------------        ^^^^^^^^^ borrowed value does not live long enough
   |                    |
   |                    type annotation requires that self is borrowed for `'static`
LL |     }
LL |     }
   |      - `self` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/regions-free-region-ordering-caller1.rs stdout ----
diff of stderr:

21 LL |     let z: &'a & usize = &(&y);
22    |            -----------    ^^^^ borrowed value does not live long enough
23    |            |
-    |            type annotation requires that `y` is borrowed for `'a`
+    |            type annotation requires that y is borrowed for `'a`
26 LL | }
26 LL | }
27    | - `y` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-caller1/regions-free-region-ordering-caller1.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-free-region-ordering-caller1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-free-region-ordering-caller1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-caller1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-caller1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/regions/regions-free-region-ordering-caller1.rs:9:27
   |
   |
LL | fn call1<'a>(x: &'a usize) {
   |          -- lifetime `'a` defined here
...
LL |     let z: &'a & usize = &(&y);
   |            -----------    ^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'a`
LL | }
   | - temporary value is freed at the end of this statement


error[E0597]: `y` does not live long enough
   |
   |
LL | fn call1<'a>(x: &'a usize) {
   |          -- lifetime `'a` defined here
...
LL |     let z: &'a & usize = &(&y);
   |            -----------    ^^^^ borrowed value does not live long enough
   |            |
   |            type annotation requires that y is borrowed for `'a`
LL | }
LL | }
   | - `y` dropped here while still borrowed
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0716.
For more information about an error, try `rustc --explain E0597`.
For more information about an error, try `rustc --explain E0597`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-infer-proc-static-upvar.rs stdout ----
diff of stderr:

6 LL | /     foo(move|| {
7 LL | |         let _a = *y;
8 LL | |     });
-    | |______- argument requires that `x` is borrowed for `'static`
+    | |______- argument requires that x is borrowed for `'static`
10 LL |   }
11    |   - `x` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar/regions-infer-proc-static-upvar.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar/regions-infer-proc-static-upvar.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-proc-static-upvar.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-proc-static-upvar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |       let y = &x; //~ ERROR `x` does not live long enough
   |               ^^ borrowed value does not live long enough
LL | /     foo(move|| {
LL | |         let _a = *y;
LL | |     });
   | |______- argument requires that x is borrowed for `'static`
LL |   }
   |   - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/regions-nested-fns-2.rs stdout ----
diff of stderr:

7    |                        -^
8    |                        ||
9    |                        |borrowed value does not live long enough
-    |                        returning this value requires that `y` is borrowed for `'static`
+    |                        returning this value requires that y is borrowed for `'static`
12 LL | }
12 LL | }
13    | - `y` dropped here while still borrowed

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns-2/regions-nested-fns-2.stderr
To only update this specific test, also pass `--test-args regions/regions-nested-fns-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `y` does not live long enough
   |
LL |         |z| {
   |         --- value captured here
   |         --- value captured here
LL |             if false { &y } else { z }
   |                        -^
   |                        ||
   |                        |borrowed value does not live long enough
   |                        returning this value requires that y is borrowed for `'static`
LL | }
LL | }
   | - `y` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/regions-nested-fns.rs stdout ----
diff of stderr:

17    |                  ^^ borrowed value does not live long enough
18 ...
19 LL |         if false { return ay; }
-    |                           -- returning this value requires that `y` is borrowed for `'static`
+    |                           -- returning this value requires that y is borrowed for `'static`
22 LL | }
22 LL | }
23    | - `y` dropped here while still borrowed

32    |               ^ borrowed value does not live long enough
33 ...
34 LL |         if false { return ay; }
-    |                           -- returning this value requires that `y` is borrowed for `'static`
+    |                           -- returning this value requires that y is borrowed for `'static`
37 LL | }
37 LL | }
38    | - `y` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/regions-nested-fns.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-nested-fns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0521]: borrowed data escapes outside of closure
   |
   |
LL |     let mut ay = &y;
   |         ------ `ay` declared here, outside of the closure body
...
LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
   |                                                           - `z` is a reference that is only valid in the closure body
...
LL |         ay = z;
   |         ^^^^^^ `z` escapes the closure body here

error[E0597]: `y` does not live long enough
   |
   |
LL |     let mut ay = &y;
   |                  ^^ borrowed value does not live long enough
...
LL |         if false { return ay; }
   |                           -- returning this value requires that y is borrowed for `'static`
LL | }
LL | }
   | - `y` dropped here while still borrowed

error[E0597]: `y` does not live long enough
   |
   |
LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
   |                                                          --- value captured here
LL |         ay = x;
LL |         ay = &y;
   |               ^ borrowed value does not live long enough
...
LL |         if false { return ay; }
   |                           -- returning this value requires that y is borrowed for `'static`
LL | }
LL | }
   | - `y` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-nested-fns.rs:17:27
   |
   |
LL | fn nested<'x>(x: &'x isize) {
   |           -- lifetime `'x` defined here
...
LL |         if false { return x; }
   |                           ^ returning this value requires that `'x` must outlive `'static`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0521, E0597.
For more information about an error, try `rustc --explain E0521`.
For more information about an error, try `rustc --explain E0521`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-pattern-typing-issue-19552.rs stdout ----
diff of stderr:

4 LL |     match [&*line] {
5    |              ^^^^ borrowed value does not live long enough
6 LL |         [ word ] => { assert_static(word); }
-    |                       ------------------- argument requires that `line` is borrowed for `'static`
+    |                       ------------------- argument requires that line is borrowed for `'static`
9 LL | }
9 LL | }
10    | - `line` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552/regions-pattern-typing-issue-19552.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-pattern-typing-issue-19552.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-pattern-typing-issue-19552.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `line` does not live long enough
   |
   |
LL |     match [&*line] { //~ ERROR `line` does not live long enough
   |              ^^^^ borrowed value does not live long enough
LL |         [ word ] => { assert_static(word); }
   |                       ------------------- argument requires that line is borrowed for `'static`
LL | }
LL | }
   | - `line` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/rfcs/rfc-2528-type-changing-struct-update/lifetime-update.rs stdout ----
diff of stderr:

5    |                 ^^ borrowed value does not live long enough
6 ...
7 LL |     let m2: Machine<'static, State1> = Machine {
-    |             ------------------------ type annotation requires that `s` is borrowed for `'static`
+    |             ------------------------ type annotation requires that s is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `s` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type-changing-struct-update/lifetime-update/lifetime-update.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfcs/rfc-2528-type-changing-struct-update/lifetime-update.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfcs/rfc-2528-type-changing-struct-update/lifetime-update.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type-changing-struct-update/lifetime-update" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc-2528-type-changing-struct-update/lifetime-update/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `s` does not live long enough
   |
LL |         lt_str: &s,
LL |         lt_str: &s,
   |                 ^^ borrowed value does not live long enough
...
LL |     let m2: Machine<'static, State1> = Machine {
   |             ------------------------ type annotation requires that s is borrowed for `'static`
LL | }
LL | }
   | - `s` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/issue-61882-2.rs stdout ----
diff of stderr:

5    |              ^^
6    |              |
7    |              borrowed value does not live long enough
-    |              this usage requires that `x` is borrowed for `'static`
+    |              this usage requires that x is borrowed for `'static`
10 LL |     }
10 LL |     }
11    |     - `x` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/issue-61882-2/issue-61882-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/issue-61882-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/issue-61882-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/issue-61882-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/issue-61882-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
LL |         Self(&x);
   |              ^^
   |              |
   |              |
   |              borrowed value does not live long enough
   |              this usage requires that x is borrowed for `'static`
LL |         //~^ ERROR `x` does not live long enough
LL |     }
   |     - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/static/static-lifetime-bound.rs stdout ----
diff of stderr:

