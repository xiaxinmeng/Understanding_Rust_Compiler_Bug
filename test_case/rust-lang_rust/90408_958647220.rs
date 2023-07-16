plain
............F....................................................................................... 600/12372
...........................................F...............................i........................ 700/12372
...................................................ii............................................... 800/12372
.................................................................................................... 900/12372
......................................................................F............................. 1000/12372
..........F......................................................................................... 1100/12372
.......................................................................i..................F......... 1200/12372
..........................F.................................F....................................... 1300/12372
.........................................................................................F.......... 1400/12372
........................................................i........................................... 1600/12372
.................................................................................................... 1700/12372
..............................................................i..................................... 1800/12372
.................................................................................................... 1900/12372
---
.................................................................................................... 7400/12372
.................................................................................................... 7500/12372
....................................ii................i....i..ii.................................... 7600/12372
.................................................................................................... 7700/12372
.................................FFF....................FF...............F..F....................... 7800/12372
............................................FF....F.FF.........................F...F................ 7900/12372
.................F..F................F.F.FFFFF.F..F.......F......F...FFF............................ 8000/12372
.................................................................................................... 8200/12372
................................................................................i................... 8300/12372
..................i............................................................i.................... 8400/12372
.................................................................................................... 8500/12372
---
.................................................................................................... 9100/12372
.................................................................................................... 9200/12372
..................iiii.iiiii.................................................................ii..... 9300/12372
..........i......................................................................................... 9400/12372
..............................................................................FF.................... 9500/12372
.............................................................................................F...... 9600/12372
......................................F...........F................................................. 9700/12372
.................................................................................................... 9900/12372
................................................................................i..ii.i............. 10000/12372
.................................................................................................... 10100/12372
..................................................................................iiiiii.i..iiiiii.i 10200/12372
---

---- [ui] ui/async-await/async-borrowck-escaping-block-error.rs stdout ----
diff of stderr:

- error[E0373]: async block may outlive the current function, but it borrows `x`, which is owned by the current function
-   --> $DIR/async-borrowck-escaping-block-error.rs:6:20
+ error[E0597]: `x` does not live long enough
3    |
3    |
4 LL |     Box::new(async { x } )
-    |                    ^^-^^
+    |                    --^--
6    |                    | |
-    |                    | `x` is borrowed here
-    |                    may outlive borrowed value `x`
- note: async block is returned here
-   --> $DIR/async-borrowck-escaping-block-error.rs:4:20
-    |
-    |
- LL | fn test_boxed() -> Box<impl std::future::Future<Output = u32>> {
-    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- help: to force the async block to take ownership of `x` (and any other referenced variables), use the `move` keyword
-    |
- LL |     Box::new(async move { x } )
-    |                    ++++
+    |                    | borrowed value does not live long enough
+    |                    value captured here by generator
+    |                    requires that `x` is borrowed for `'static`
+ LL |
+ LL | }
+    | - `x` dropped here while still borrowed
19 
20 error[E0373]: async block may outlive the current function, but it borrows `x`, which is owned by the current function

38 
39 error: aborting due to 2 previous errors
40 
---
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/async-borrowck-escaping-block-error.stderr
diff of fixed:

3 
4 fn test_boxed() -> Box<impl std::future::Future<Output = u32>> {
5     let x = 0u32;
-     Box::new(async move { x } )
+     Box::new(async { x } )
7     //~^ ERROR E0373
9 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/async-borrowck-escaping-block-error.fixed
To only update this specific test, also pass `--test-args async-await/async-borrowck-escaping-block-error.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     Box::new(async { x } )
   |                    --^--
   |                    | |
   |                    | borrowed value does not live long enough
   |                    value captured here by generator
   |                    requires that `x` is borrowed for `'static`
LL |     //~^ ERROR E0373
LL | }
   | - `x` dropped here while still borrowed

error[E0373]: async block may outlive the current function, but it borrows `x`, which is owned by the current function
   |
   |
LL |     async { *x }
   |           ^^--^^
   |           | |
   |           | `x` is borrowed here
   |           may outlive borrowed value `x`
note: async block is returned here
  --> /checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.rs:11:5
   |
   |
LL |     async { *x }
   |     ^^^^^^^^^^^^
help: to force the async block to take ownership of `x` (and any other referenced variables), use the `move` keyword
   |
LL |     async move { *x }

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0373, E0597.
---

---- [ui] ui/async-await/issues/issue-78938-async-block.rs stdout ----
diff of stderr:

- error[E0373]: async block may outlive the current function, but it borrows `room_ref`, which is owned by the current function
-   --> $DIR/issue-78938-async-block.rs:8:39
+ error[E0597]: `room_ref` does not live long enough
3    |
3    |
4 LL |       let gameloop_handle = spawn(async {
+    |  _______________________________________-
+    |  _______________________________________-
6 LL | |         game_loop(Arc::clone(&room_ref))
-    | |                               -------- `room_ref` is borrowed here
+    | |                               ^^^^^^^^ borrowed value does not live long enough
8 LL | |     });
-    | |_____^ may outlive borrowed value `room_ref`
-    |
-    = note: async blocks are not executed immediately and must either take a reference or ownership of outside variables they use
- help: to force the async block to take ownership of `room_ref` (and any other referenced variables), use the `move` keyword
-    |
- LL |     let gameloop_handle = spawn(async move {
+    | |     -
+    | |     |
+    | |     |
+    | |_____value captured here by generator
+    |       requires that `room_ref` is borrowed for `'static`
+ LL |       gameloop_handle.await;
+ LL |   }
+    |   - `room_ref` dropped here while still borrowed
17 error: aborting due to previous error
18 

- For more information about this error, try `rustc --explain E0373`.
---
To only update this specific test, also pass `--test-args async-await/issues/issue-78938-async-block.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-78938-async-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78938-async-block" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-78938-async-block/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `room_ref` does not live long enough
   |
   |
LL |       let gameloop_handle = spawn(async { //~ ERROR E0373
   |  _______________________________________-
LL | |         game_loop(Arc::clone(&room_ref))
   | |                               ^^^^^^^^ borrowed value does not live long enough
LL | |     });
   | |     |
   | |     |
   | |_____value captured here by generator
   |       requires that `room_ref` is borrowed for `'static`
LL |       gameloop_handle.await;
LL |   }
   |   - `room_ref` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/borrowck/borrowck-escaping-closure-error-1.rs stdout ----
diff of stderr:

- error[E0373]: closure may outlive the current function, but it borrows `books`, which is owned by the current function
-   --> $DIR/borrowck-escaping-closure-error-1.rs:13:11
+ error[E0597]: `books` does not live long enough
3    |
3    |
4 LL |     spawn(|| books.push(4));
-    |           ^^ ----- `books` is borrowed here
-    |           |
-    |           may outlive borrowed value `books`
-    |
- note: function requires argument type to outlive `'static`
-   --> $DIR/borrowck-escaping-closure-error-1.rs:13:5
-    |
- LL |     spawn(|| books.push(4));
-    |     ^^^^^^^^^^^^^^^^^^^^^^^
- help: to force the closure to take ownership of `books` (and any other referenced variables), use the `move` keyword
-    |
- LL |     spawn(move || books.push(4));
+    |           ---^^^^^--------
+    |           |  |
+    |           |  |
+    |           |  borrowed value does not live long enough
+    |           value captured here
+    |           requires that `books` is borrowed for `'static`
+ LL |
+ LL | }
+    | - `books` dropped here while still borrowed
19 error: aborting due to previous error
20 

- For more information about this error, try `rustc --explain E0373`.
- For more information about this error, try `rustc --explain E0373`.
+ For more information about this error, try `rustc --explain E0597`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-1/borrowck-escaping-closure-error-1.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-escaping-closure-error-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-escaping-closure-error-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-escaping-closure-error-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `books` does not live long enough
   |
   |
LL |     spawn(|| books.push(4));
   |           ---^^^^^--------
   |           |  |
   |           |  borrowed value does not live long enough
   |           value captured here
   |           requires that `books` is borrowed for `'static`
LL |     //~^ ERROR E0373
LL | }
   | - `books` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/borrowck-local-borrow-with-panic-outlives-fn.rs:3:15
3    |
4 LL |     *x = Some(&mut z.1);
-    |     |         |
-    |     |         |
-    |     |         borrowed value does not live long enough
-    |     assignment requires that `z.1` is borrowed for `'static`
+    |               |
+    |               |
+    |               borrowed value does not live long enough
+    |               requires that `z.1` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `z.1` does not live long enough
   |
   |
LL |     *x = Some(&mut z.1);
   |               |
   |               |
   |               borrowed value does not live long enough
   |               requires that `z.1` is borrowed for `'static`
LL | }
LL | }
   | - `z.1` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/borrowck/mut-borrow-in-loop.rs stdout ----
diff of stderr:

13    |      -- lifetime `'a` defined here
14 ...
15 LL |             (self.func)(arg)
-    |             ------------^^^-
-    |             |           |
-    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |                         |
+    |                         |
+    |                         `*arg` was mutably borrowed here in the previous iteration of the loop
+    |                         requires that `*arg` is borrowed for `'a`
20 
21 error[E0499]: cannot borrow `*arg` as mutable more than once at a time


25    |      -- lifetime `'a` defined here
26 ...
27 LL |             (self.func)(arg)
-    |             ------------^^^-
-    |             |           |
-    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |                         |
+    |                         |
+    |                         `*arg` was mutably borrowed here in the previous iteration of the loop
+    |                         requires that `*arg` is borrowed for `'a`
32 
33 error[E0499]: cannot borrow `*arg` as mutable more than once at a time


37    |      -- lifetime `'a` defined here
38 ...
39 LL |             (self.func)(arg)
-    |             ------------^^^-
-    |             |           |
-    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |                         |
+    |                         |
+    |                         `*arg` was mutably borrowed here in the previous iteration of the loop
+    |                         requires that `*arg` is borrowed for `'a`
45 error: aborting due to 3 previous errors; 1 warning emitted
46 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop/mut-borrow-in-loop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/mut-borrow-in-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { //~ WARN denote infinite loops with
   |         ^^^^^^^^^^ help: use `loop`
   |
   = note: `#[warn(while_true)]` on by default

error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |                         |
   |                         |
   |                         `*arg` was mutably borrowed here in the previous iteration of the loop
   |                         requires that `*arg` is borrowed for `'a`

error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |                         |
   |                         |
   |                         `*arg` was mutably borrowed here in the previous iteration of the loop
   |                         requires that `*arg` is borrowed for `'a`

error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |                         |
   |                         |
   |                         `*arg` was mutably borrowed here in the previous iteration of the loop
   |                         requires that `*arg` is borrowed for `'a`
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/borrowck/two-phase-surprise-no-conflict.rs stdout ----
diff of stderr:

76    |                     -- lifetime `'a` defined here
77 ...
78 LL |     reg.register_univ(Box::new(CapturePass::new(&reg.sess_mut)));
-    |     |                 |                         |
-    |     |                 |                         |
-    |     |                 |                         immutable borrow occurs here
-    |     |                 cast requires that `reg.sess_mut` is borrowed for `'a`
+    |     |                                           |
+    |     |                                           |
+    |     |                                           immutable borrow occurs here
+    |     |                                           requires that `reg.sess_mut` is borrowed for `'a`
83    |     mutable borrow occurs here
84 
85 error[E0502]: cannot borrow `*reg` as mutable because it is also borrowed as immutable

119    |                     -- lifetime `'a` defined here
120 ...
121 LL |     reg.register_univ(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
-    |     |                 |                             |
-    |     |                 |                             |
-    |     |                 |                             first mutable borrow occurs here
-    |     |                 cast requires that `reg.sess_mut` is borrowed for `'a`
+    |     |                                               |
+    |     |                                               |
+    |     |                                               first mutable borrow occurs here
+    |     |                                               requires that `reg.sess_mut` is borrowed for `'a`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-surprise-no-conflict.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0503]: cannot use `self.cx` because it was mutably borrowed
   |
   |
LL |         let _mut_borrow = &mut *self;
   |                           ---------- borrow of `*self` occurs here
LL |         let _access = self.cx;
   |                       ^^^^^^^ use of borrowed `*self`
LL |         //~^ ERROR cannot use `self.cx` because it was mutably borrowed [E0503]
LL |         _mut_borrow;
   |         ----------- borrow later used here

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
   |     |   first borrow later used by call
   |     first mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_bound(Box::new(TrivialPass::new_mut(&mut reg.sess_mut)));
   |     |   |                                            |
   |     |   |                                            |
   |     |   |                                            second mutable borrow occurs here
   |     |   first borrow later used by call
   |     first mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_univ(Box::new(TrivialPass::new_mut(&mut reg.sess_mut)));
   |     |   |                                           |
   |     |   |                                           |
   |     |   |                                           second mutable borrow occurs here
   |     |   first borrow later used by call
   |     first mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_ref(&TrivialPass::new_mut(&mut reg.sess_mut));
   |     |   |                                  |
   |     |   |                                  |
   |     |   |                                  second mutable borrow occurs here
   |     |   first borrow later used by call
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
   |     |                                           |
   |     |                                           |
   |     |                                           immutable borrow occurs here
   |     |                                           requires that `reg.sess_mut` is borrowed for `'a`
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
   |     |   first borrow later used by call
   |     first mutable borrow occurs here

error[E0499]: cannot borrow `*reg` as mutable more than once at a time
   |
   |
LL | fn register_plugins<'a>(mk_reg: impl Fn() -> &'a mut Registry<'a>) {
   |                     -- lifetime `'a` defined here
...
LL |     reg.register_univ(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
   |     |                                               |
   |     |                                               |
   |     |                                               first mutable borrow occurs here
   |     |                                               requires that `reg.sess_mut` is borrowed for `'a`
   |     second mutable borrow occurs here

error[E0499]: cannot borrow `reg.sess_mut` as mutable more than once at a time
   |
   |
LL |     reg.register_univ(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
   |     |   |                                           |
   |     |   |                                           |
   |     |   |                                           second mutable borrow occurs here
   |     |   first borrow later used by call
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
   |     |   first borrow later used by call
   |     first mutable borrow occurs here
error: aborting due to 15 previous errors

Some errors have detailed explanations: E0499, E0502, E0503.
For more information about an error, try `rustc --explain E0499`.
For more information about an error, try `rustc --explain E0499`.

------------------------------------------


---- [ui] ui/c-variadic/variadic-ffi-4.rs stdout ----
diff of stderr:

36    |                           has type `VaList<'1, '_>`
38 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:22:5
+   --> $DIR/variadic-ffi-4.rs:21:47
40    |
40    |
41 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
44    |                                               has type `&mut VaListImpl<'1>`
44    |                                               has type `&mut VaListImpl<'1>`
- LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'1` must outlive `'2`
+    |                                               requires that `'1` must outlive `'2`
+    |
+    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
48 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:22:5
+   --> $DIR/variadic-ffi-4.rs:21:73
50    |
50    |
51 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ^^^^^^^
+    |                                               |                         |
+    |                                               |                         has type `VaListImpl<'2>`
+    |                                               |                         has type `VaListImpl<'2>`
+    |                                               |                         requires that `'2` must outlive `'1`
54    |                                               has type `&mut VaListImpl<'1>`
- LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'2` must outlive `'1`
58 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:28:5
+   --> $DIR/variadic-ffi-4.rs:27:47
60    |
60    |
61 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
64    |                                               has type `&mut VaListImpl<'1>`
64    |                                               has type `&mut VaListImpl<'1>`
- LL |     ap0 = &mut ap1;
-    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
+    |                                               requires that `'1` must outlive `'2`
67    |
68    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
69    = note: mutable references are invariant over their type parameter

70    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
72 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:28:5
+   --> $DIR/variadic-ffi-4.rs:27:73
74    |
74    |
75 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ^^^^^^^
+    |                                               |                         |
+    |                                               |                         has type `VaListImpl<'2>`
+    |                                               |                         has type `VaListImpl<'2>`
+    |                                               |                         requires that `'2` must outlive `'1`
78    |                                               has type `&mut VaListImpl<'1>`
- LL |     ap0 = &mut ap1;
-    |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
-    |
-    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
-    = note: mutable references are invariant over their type parameter
-    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
85 
86 error[E0597]: `ap1` does not live long enough


98    | - `ap1` dropped here while still borrowed
100 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:35:12
+   --> $DIR/variadic-ffi-4.rs:34:47
102    |
102    |
103 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
106    |                                               has type `&mut VaListImpl<'1>`
106    |                                               has type `&mut VaListImpl<'1>`
- LL |     *ap0 = ap1.clone();
-    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                                               requires that `'1` must outlive `'2`
+    |
+    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
110 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:35:12
+   --> $DIR/variadic-ffi-4.rs:34:73
112    |
112    |
113 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ^^^^^^^
+    |                                               |                         |
+    |                                               |                         has type `VaListImpl<'2>`
+    |                                               |                         has type `VaListImpl<'2>`
+    |                                               |                         requires that `'2` must outlive `'1`
116    |                                               has type `&mut VaListImpl<'1>`
- LL |     *ap0 = ap1.clone();
-    |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`
120 error: aborting due to 11 previous errors
121 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/variadic-ffi-4.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
---
diff of stderr:

20    |     ^^
21    |     |
22    |     borrowed value does not live long enough
-    |     using this value as a static requires that `x` is borrowed for `'static`
+    |     requires that `x` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/generic-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/generic-slice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL | impl<'a, T: 'static> Generic<'a, T> {
   |      -- lifetime `'a` defined here
LL |         &x
   |         ^^
   |         |
   |         |
   |         borrowed value does not live long enough
   |         using this value as a constant requires that `x` is borrowed for `'a`
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
   |     requires that `x` is borrowed for `'static`
LL |     //~^ ERROR `x` does not live long enough
LL | };
   | - `x` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

8   --> $DIR/mut_ref_in_final.rs:17:40
9    |
10 LL | const B3: Option<&mut i32> = Some(&mut 42);
-    |                              ----------^^-
-    |                              |         | temporary value is freed at the end of this statement
-    |                              |         | temporary value is freed at the end of this statement
-    |                              |         creates a temporary which is freed while still in use
-    |                              using this value as a constant requires that borrow lasts for `'static`
+    |                                   -----^^- temporary value is freed at the end of this statement
+    |                                   |    |
+    |                                   |    creates a temporary which is freed while still in use
+    |                                   requires that borrow lasts for `'static`
16 
17 error[E0716]: temporary value dropped while borrowed

19    |
19    |
20 LL | const B4: Option<&mut i32> = helper(&mut 42);
-    |                              ------------^^-
-    |                              |           | temporary value is freed at the end of this statement
-    |                              |           | temporary value is freed at the end of this statement
-    |                              |           creates a temporary which is freed while still in use
-    |                              using this value as a constant requires that borrow lasts for `'static`
+    |                                     -----^^- temporary value is freed at the end of this statement
+    |                                     |    |
+    |                                     |    creates a temporary which is freed while still in use
+    |                                     requires that borrow lasts for `'static`
26 
27 error[E0716]: temporary value dropped while borrowed

29    |
29    |
30 LL | const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
-    |                                  -------------------------------^^--
-    |                                  |                              |  temporary value is freed at the end of this statement
-    |                                  |                              |  temporary value is freed at the end of this statement
-    |                                  |                              creates a temporary which is freed while still in use
-    |                                  using this value as a constant requires that borrow lasts for `'static`
+    |                                                            -----^^ - temporary value is freed at the end of this statement
+    |                                                            |    |
+    |                                                            |    creates a temporary which is freed while still in use
+    |                                                            requires that borrow lasts for `'static`
36 
37 error[E0716]: temporary value dropped while borrowed

39    |
39    |
40 LL | static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
-    |                                    -------------------------------^^--
-    |                                    |                              |  temporary value is freed at the end of this statement
-    |                                    |                              |  temporary value is freed at the end of this statement
-    |                                    |                              creates a temporary which is freed while still in use
-    |                                    using this value as a static requires that borrow lasts for `'static`
+    |                                                              -----^^ - temporary value is freed at the end of this statement
+    |                                                              |    |
+    |                                                              |    creates a temporary which is freed while still in use
+    |                                                              requires that borrow lasts for `'static`
46 
47 error[E0716]: temporary value dropped while borrowed

49    |
49    |
50 LL | static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
-    |                                        -------------------------------^^--
-    |                                        |                              |  temporary value is freed at the end of this statement
-    |                                        |                              |  temporary value is freed at the end of this statement
-    |                                        |                              creates a temporary which is freed while still in use
-    |                                        using this value as a static requires that borrow lasts for `'static`
+    |                                                                  -----^^ - temporary value is freed at the end of this statement
+    |                                                                  |    |
+    |                                                                  |    creates a temporary which is freed while still in use
+    |                                                                  requires that borrow lasts for `'static`
57 error: aborting due to 6 previous errors
58 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/mut_ref_in_final.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-mut-refs/mut_ref_in_final.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0764]: mutable references are not allowed in the final value of constants
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:11:21
   |
LL | const B: *mut i32 = &mut 4; //~ ERROR mutable references are not allowed


error[E0716]: temporary value dropped while borrowed
   |
   |
LL | const B3: Option<&mut i32> = Some(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                                   -----^^- temporary value is freed at the end of this statement
   |                                   |    |
   |                                   |    creates a temporary which is freed while still in use
   |                                   requires that borrow lasts for `'static`

error[E0716]: temporary value dropped while borrowed
   |
   |
LL | const B4: Option<&mut i32> = helper(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                                     -----^^- temporary value is freed at the end of this statement
   |                                     |    |
   |                                     |    creates a temporary which is freed while still in use
   |                                     requires that borrow lasts for `'static`

error[E0716]: temporary value dropped while borrowed
   |
   |
LL | const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                                            -----^^ - temporary value is freed at the end of this statement
   |                                                            |    |
   |                                                            |    creates a temporary which is freed while still in use
   |                                                            requires that borrow lasts for `'static`

error[E0716]: temporary value dropped while borrowed
   |
   |
LL | static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                                              -----^^ - temporary value is freed at the end of this statement
   |                                                              |    |
   |                                                              |    creates a temporary which is freed while still in use
   |                                                              requires that borrow lasts for `'static`

error[E0716]: temporary value dropped while borrowed
   |
   |
LL | static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                                                  -----^^ - temporary value is freed at the end of this statement
   |                                                                  |    |
   |                                                                  |    creates a temporary which is freed while still in use
   |                                                                  requires that borrow lasts for `'static`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0716, E0764.
For more information about an error, try `rustc --explain E0716`.
---
diff of stderr:

2   --> $DIR/issue-54224.rs:1:39
3    |
4 LL | const FOO: Option<&[[u8; 3]]> = Some(&[*b"foo"]);
-    |                                 |     |        |
-    |                                 |     |        temporary value is freed at the end of this statement
-    |                                 |     |        temporary value is freed at the end of this statement
-    |                                 |     creates a temporary which is freed while still in use
-    |                                 using this value as a constant requires that borrow lasts for `'static`
+    |                                      -^^^^^^^^^- temporary value is freed at the end of this statement
+    |                                      ||
+    |                                      |creates a temporary which is freed while still in use
+    |                                      requires that borrow lasts for `'static`
10 
11 error[E0716]: temporary value dropped while borrowed

13    |
13    |
14 LL | pub const Z: Cow<'static, [ [u8; 3] ]> = Cow::Borrowed(&[*b"ABC"]);
-    |                                          |              |        |
-    |                                          |              |        temporary value is freed at the end of this statement
-    |                                          |              |        temporary value is freed at the end of this statement
-    |                                          |              creates a temporary which is freed while still in use
-    |                                          using this value as a constant requires that borrow lasts for `'static`
+    |                                                        -^^^^^^^^^- temporary value is freed at the end of this statement
+    |                                                        ||
+    |                                                        |creates a temporary which is freed while still in use
+    |                                                        requires that borrow lasts for `'static`
21 error: aborting due to 2 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/issue-54224.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-54224.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-54224.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
   |
   |
LL | const FOO: Option<&[[u8; 3]]> = Some(&[*b"foo"]); //~ ERROR temporary value dropped while borrowed
   |                                      -^^^^^^^^^- temporary value is freed at the end of this statement
   |                                      ||
   |                                      |creates a temporary which is freed while still in use
   |                                      requires that borrow lasts for `'static`

error[E0716]: temporary value dropped while borrowed
   |
   |
LL | pub const Z: Cow<'static, [ [u8; 3] ]> = Cow::Borrowed(&[*b"ABC"]);
   |                                                        -^^^^^^^^^- temporary value is freed at the end of this statement
   |                                                        ||
   |                                                        |creates a temporary which is freed while still in use
   |                                                        requires that borrow lasts for `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.

---
diff of stderr:

2   --> $DIR/promote-not.rs:8:50
3    |
4 LL | static mut TEST1: Option<&mut [i32]> = Some(&mut [1, 2, 3]);
-    |                                        |         |        |
-    |                                        |         |        temporary value is freed at the end of this statement
-    |                                        |         |        temporary value is freed at the end of this statement
-    |                                        |         creates a temporary which is freed while still in use
-    |                                        using this value as a static requires that borrow lasts for `'static`
+    |                                             -----^^^^^^^^^- temporary value is freed at the end of this statement
+    |                                             |    |
+    |                                             |    creates a temporary which is freed while still in use
+    |                                             requires that borrow lasts for `'static`
10 
11 error[E0716]: temporary value dropped while borrowed

13    |
13    |
14 LL |     let x = &mut [1,2,3];
-    |                  ^^^^^^^ creates a temporary which is freed while still in use
+    |             |    |
+    |             |    |
+    |             |    creates a temporary which is freed while still in use
+    |             assignment requires that borrow lasts for `'static`
16 LL |     x
-    |     - using this value as a static requires that borrow lasts for `'static`
18 LL | };
19    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not/promote-not.stderr
To only update this specific test, also pass `--test-args consts/promote-not.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote-not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
   |
   |
LL | static mut TEST1: Option<&mut [i32]> = Some(&mut [1, 2, 3]); //~ ERROR temporary value dropped while borrowed
   |                                             -----^^^^^^^^^- temporary value is freed at the end of this statement
   |                                             |    |
   |                                             |    creates a temporary which is freed while still in use
   |                                             requires that borrow lasts for `'static`

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let x = &mut [1,2,3]; //~ ERROR temporary value dropped while borrowed
   |             |    |
   |             |    |
   |             |    creates a temporary which is freed while still in use
   |             assignment requires that borrow lasts for `'static`
LL | };
LL | };
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |         let _x: &'static () = &foo(); //~ ERROR temporary value dropped while borrowed
   |                 -----------    ^^^^^ creates a temporary which is freed while still in use
   |                 |
   |                 type annotation requires that borrow lasts for `'static`
   |     - temporary value is freed at the end of this statement


error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |             |
   |             type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |             |
   |             type annotation requires that borrow lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).0; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(1/0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(1/(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(1%0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(1%(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &([1,2,3][4]+1); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &TEST_DROP;
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------    ^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------     ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------    ^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------      ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &[&TEST_DROP; 1];
   |               ----------    ^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let _val: &'static _ = &[&TEST_DROP; 1];
   |               ----------      ^^^^^^^^^    - temporary value is freed at the end of this statement
   |               |               |
   |               |               creates a temporary which is freed while still in use
   |               type annotation requires that borrow lasts for `'static`
error: aborting due to 20 previous errors

For more information about this error, try `rustc --explain E0716`.


------------------------------------------


---- [ui] ui/dropck/dropck_trait_cycle_checked.rs stdout ----
diff of stderr:

1 error[E0597]: `o2` does not live long enough
3    |
3    |
- LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                     -------- cast requires that `o2` is borrowed for `'static`
6 LL |     o1.set0(&o2);
-    |             ^^^ borrowed value does not live long enough
+    |             |
+    |             |
+    |             borrowed value does not live long enough
+    |             requires that `o2` is borrowed for `'static`
9 LL | }
9 LL | }
10    | - `o2` dropped here while still borrowed

12 error[E0597]: `o3` does not live long enough
14    |
14    |
- LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                     -------- cast requires that `o3` is borrowed for `'static`
- LL |     o1.set0(&o2);
18 LL |     o1.set1(&o3);
-    |             ^^^ borrowed value does not live long enough
+    |             |
+    |             |
+    |             borrowed value does not live long enough
+    |             requires that `o3` is borrowed for `'static`
21 LL | }
21 LL | }
22    | - `o3` dropped here while still borrowed

24 error[E0597]: `o2` does not live long enough
26    |
26    |
- LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                               -------- cast requires that `o2` is borrowed for `'static`
- ...
30 LL |     o2.set0(&o2);
-    |             ^^^ borrowed value does not live long enough
+    |             |
+    |             |
+    |             borrowed value does not live long enough
+    |             requires that `o2` is borrowed for `'static`
33 LL | }
33 LL | }
34    | - `o2` dropped here while still borrowed

36 error[E0597]: `o3` does not live long enough
38    |
38    |
- LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                               -------- cast requires that `o3` is borrowed for `'static`
- ...
42 LL |     o2.set1(&o3);
-    |             ^^^ borrowed value does not live long enough
+    |             |
+    |             |
+    |             borrowed value does not live long enough
+    |             requires that `o3` is borrowed for `'static`
45 LL | }
45 LL | }
46    | - `o3` dropped here while still borrowed

48 error[E0597]: `o1` does not live long enough
50    |
50    |
- LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                                         -------- cast requires that `o1` is borrowed for `'static`
- ...
54 LL |     o3.set0(&o1);
-    |             ^^^ borrowed value does not live long enough
+    |             |
+    |             |
+    |             borrowed value does not live long enough
+    |             requires that `o1` is borrowed for `'static`
56 LL |     o3.set1(&o2);
57 LL | }
58    | - `o1` dropped here while still borrowed

60 error[E0597]: `o2` does not live long enough
62    |
62    |
- LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                                         -------- cast requires that `o2` is borrowed for `'static`
- ...
66 LL |     o3.set1(&o2);
-    |             ^^^ borrowed value does not live long enough
+    |             |
+    |             |
+    |             borrowed value does not live long enough
+    |             requires that `o2` is borrowed for `'static`
68 LL | }
69    | - `o2` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked/dropck_trait_cycle_checked.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked/dropck_trait_cycle_checked.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dropck/dropck_trait_cycle_checked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `o2` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:111:13
   |
LL |     o1.set0(&o2); //~ ERROR `o2` does not live long enough
   |             |
   |             |
   |             borrowed value does not live long enough
   |             requires that `o2` is borrowed for `'static`
LL | }
LL | }
   | - `o2` dropped here while still borrowed

error[E0597]: `o3` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:112:13
   |
LL |     o1.set1(&o3); //~ ERROR `o3` does not live long enough
   |             |
   |             |
   |             borrowed value does not live long enough
   |             requires that `o3` is borrowed for `'static`
LL | }
LL | }
   | - `o3` dropped here while still borrowed

error[E0597]: `o2` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:113:13
   |
LL |     o2.set0(&o2); //~ ERROR `o2` does not live long enough
   |             |
   |             |
   |             borrowed value does not live long enough
   |             requires that `o2` is borrowed for `'static`
LL | }
LL | }
   | - `o2` dropped here while still borrowed

error[E0597]: `o3` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:114:13
   |
LL |     o2.set1(&o3); //~ ERROR `o3` does not live long enough
   |             |
   |             |
   |             borrowed value does not live long enough
   |             requires that `o3` is borrowed for `'static`
LL | }
LL | }
   | - `o3` dropped here while still borrowed

error[E0597]: `o1` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:115:13
   |
LL |     o3.set0(&o1); //~ ERROR `o1` does not live long enough
   |             |
   |             |
   |             borrowed value does not live long enough
   |             requires that `o1` is borrowed for `'static`
LL |     o3.set1(&o2); //~ ERROR `o2` does not live long enough
LL | }
   | - `o1` dropped here while still borrowed

error[E0597]: `o2` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:116:13
   |
LL |     o3.set1(&o2); //~ ERROR `o2` does not live long enough
   |             |
   |             |
   |             borrowed value does not live long enough
   |             requires that `o2` is borrowed for `'static`
LL | }
   | - `o2` dropped here while still borrowed
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/impl-trait/does-not-live-long-enough.rs stdout ----
diff of stderr:

- error[E0373]: closure may outlive the current function, but it borrows `prefix`, which is owned by the current function
-   --> $DIR/does-not-live-long-enough.rs:6:33
+ error[E0597]: `prefix` does not live long enough
+   --> $DIR/does-not-live-long-enough.rs:6:51
3    |
- LL |         self.data.iter().filter(|s| s.starts_with(prefix)).map(|s| s.as_ref())
-    |                                 ^^^               ------ `prefix` is borrowed here
-    |                                 |
-    |                                 may outlive borrowed value `prefix`
- note: closure is returned here
-   --> $DIR/does-not-live-long-enough.rs:5:55
-    |
-    |
12 LL |     fn started_with<'a>(&'a self, prefix: &'a str) -> impl Iterator<Item=&'a str> {
-    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
- help: to force the closure to take ownership of `prefix` (and any other referenced variables), use the `move` keyword
-    |
- LL |         self.data.iter().filter(move |s| s.starts_with(prefix)).map(|s| s.as_ref())
-    |                                 ++++
+    |                     -- lifetime `'a` defined here
+ LL |         self.data.iter().filter(|s| s.starts_with(prefix)).map(|s| s.as_ref())
+    |                                 |                 |
+    |                                 |                 |
+    |                                 |                 borrowed value does not live long enough
+    |                                 value captured here
+    |                                 requires that `prefix` is borrowed for `'a`
+ LL |
+ LL |     }
+    |     - `prefix` dropped here while still borrowed
19 error: aborting due to previous error
20 

- For more information about this error, try `rustc --explain E0373`.
- For more information about this error, try `rustc --explain E0373`.
+ For more information about this error, try `rustc --explain E0597`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/does-not-live-long-enough/does-not-live-long-enough.stderr
---
diff of stderr:

5    |         ^^
6    |         |
7    |         borrowed value does not live long enough
-    |         using this value as a constant requires that `p` is borrowed for `'static`
+    |         requires that `p` is borrowed for `'static`
9 LL |     };
10    |     - `p` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/issue-18118.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/issue-18118.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18118.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `p` does not live long enough
   |
   |
LL |         &p //~ ERROR `p` does not live long enough
   |         |
   |         |
   |         borrowed value does not live long enough
   |         requires that `p` is borrowed for `'static`
LL |     };
   |     - `p` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/issue-52049.rs:6:10
3    |
4 LL |     foo(&unpromotable(5u32));
-    |     |    |
-    |     |    |
-    |     |    creates a temporary which is freed while still in use
-    |     argument requires that borrow lasts for `'static`
+    |         -^^^^^^^^^^^^^^^^^^
+    |         ||
+    |         |creates a temporary which is freed while still in use
+    |         requires that borrow lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/issue-52049.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/issue-52049.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-52049.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52049.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     foo(&unpromotable(5u32));
   |         -^^^^^^^^^^^^^^^^^^
   |         ||
   |         |creates a temporary which is freed while still in use
   |         requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.

---
diff of stderr:

5    |              ^^
6    |              |
7    |              borrowed value does not live long enough
-    |              this usage requires that `x` is borrowed for `'static`
+    |              requires that `x` is borrowed for `'static`
10 LL |     }
10 LL |     }
11    |     - `x` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61882-2/issue-61882-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-61882-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-61882-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61882-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61882-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `x` does not live long enough
   |
LL |         Self(&x);
   |              ^^
   |              |
   |              |
   |              borrowed value does not live long enough
   |              requires that `x` is borrowed for `'static`
LL |         //~^ ERROR `x` does not live long enough
LL |     }
   |     - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---

34    |
35    = note: defining type: supply
36 
- error[E0521]: borrowed data escapes outside of function
-   --> $DIR/propagate-approximated-shorter-to-static-no-bound.rs:32:5
+ error: lifetime may not live long enough
39    |
39    |
- LL |   fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
-    |             --      ------ `cell_a` is a reference that is only valid in the function body
-    |             |
-    |             lifetime `'a` defined here
- LL | /     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
- LL | |
- LL | |
- LL | |         // Only works if 'x: 'y:
- LL | |         demand_y(x, y, x.get())
- LL | |     });
-    | |      |
-    | |      |
-    | |______`cell_a` escapes the function body here
-    |        argument requires that `'a` must outlive `'static`
+ LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
+    |           --      ^^^^^^ requires that `'a` must outlive `'static`
+    |           |
+    |           lifetime `'a` defined here
55 error: aborting due to previous error
56 

- For more information about this error, try `rustc --explain E0521`.
---
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:32:47
   |
LL |       establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |  _______________________________________________^
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |     });
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) u32>)),
               (),
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where '_#1r: '_#0r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:31:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |     });
LL | | }
   | |_^
   |
   |
   = note: defining type: supply

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs:31:19
   |
LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |           --      ^^^^^^ requires that `'a` must outlive `'static`
   |           |
   |           lifetime `'a` defined here
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs stdout ----
diff of stderr:

73   --> $DIR/propagate-approximated-shorter-to-static-comparing-against-free.rs:30:26
74    |
75 LL |     let cell = Cell::new(&a);
-    |                ----------^^-
-    |                |         |
-    |                |         borrowed value does not live long enough
-    |                argument requires that `a` is borrowed for `'static`
+    |                          |
+    |                          |
+    |                          borrowed value does not live long enough
+    |                          requires that `a` is borrowed for `'static`
81 LL | }
81 LL | }
82    | - `a` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/propagate-approximated-shorter-to-static-comparing-against-free.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:21:15
   |
LL |       foo(cell, |cell_a, cell_x| {
   |  _______________^
LL | |         cell_a.set(cell_x.get()); // forces 'x: 'a, error in closure
LL | |         //~^ ERROR
LL | |     })
   |
   |
   = note: defining type: case1::{closure#0} with closure substs [
               i32,
               for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>)),
               (),


error[E0521]: borrowed data escapes outside of closure
   |
   |
LL |     foo(cell, |cell_a, cell_x| {
   |                ------  ------ `cell_x` is a reference that is only valid in the closure body
   |                |
   |                `cell_a` declared here, outside of the closure body
LL |         cell_a.set(cell_x.get()); // forces 'x: 'a, error in closure
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ `cell_x` escapes the closure body here
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:18:1
   |
LL | / fn case1() {
LL | / fn case1() {
LL | |     let a = 0;
LL | |     let cell = Cell::new(&a);
LL | |     foo(cell, |cell_a, cell_x| {
LL | |     })
LL | | }
   | |_^
   |
   |
   = note: defining type: case1

note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:35:15
   |
LL |       foo(cell, |cell_a, cell_x| {
   |  _______________^
LL | |         cell_x.set(cell_a.get()); // forces 'a: 'x, implies 'a = 'static -> borrow error
LL | |     })
   |
   |
   = note: defining type: case2::{closure#0} with closure substs [
               i32,
               for<'r> extern "rust-call" fn((std::cell::Cell<&'_#1r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>)),
               (),
           ]
   = note: number of external vids: 2
   = note: where '_#1r: '_#0r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-comparing-against-free.rs:28:1
   |
LL | / fn case2() {
LL | / fn case2() {
LL | |     let a = 0;
LL | |     let cell = Cell::new(&a);
LL | |     //~^ ERROR `a` does not live long enough
LL | |     })
LL | | }
   | |_^
   |
   |
   = note: defining type: case2

error[E0597]: `a` does not live long enough
   |
   |
LL |     let cell = Cell::new(&a);
   |                          |
   |                          |
   |                          borrowed value does not live long enough
   |                          requires that `a` is borrowed for `'static`
LL | }
LL | }
   | - `a` dropped here while still borrowed
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0521, E0597.
For more information about an error, try `rustc --explain E0521`.
---

34    |
35    = note: defining type: supply
36 
- error[E0521]: borrowed data escapes outside of function
-   --> $DIR/propagate-approximated-shorter-to-static-wrong-bound.rs:35:5
+ error: lifetime may not live long enough
39    |
39    |
- LL |   fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
-    |             --      ------ `cell_a` is a reference that is only valid in the function body
-    |             |
-    |             lifetime `'a` defined here
- LL | /     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
- LL | |
- LL | |
- LL | |         // Only works if 'x: 'y:
- LL | |         demand_y(x, y, x.get())
- LL | |     });
-    | |      |
-    | |      |
-    | |______`cell_a` escapes the function body here
-    |        argument requires that `'a` must outlive `'static`
+ LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
+    |           --      ^^^^^^ requires that `'a` must outlive `'static`
+    |           |
+    |           lifetime `'a` defined here
55 error: aborting due to previous error
56 

- For more information about this error, try `rustc --explain E0521`.
---
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:35:47
   |
LL |       establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |  _______________________________________________^
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |     });
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 5, kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#0r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:34:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         //~^ ERROR borrowed data escapes outside of function
LL | |
LL | |     });
LL | | }
   | |_^
   |
   |
   = note: defining type: supply

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs:34:19
   |
LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |           --      ^^^^^^ requires that `'a` must outlive `'static`
   |           |
   |           lifetime `'a` defined here
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj.rs stdout ----
diff of stderr:

2   --> $DIR/do-not-ignore-lifetime-bounds-in-copy-proj.rs:9:18
3    |
4 LL |     let a = (Foo(&s),);
-    |                  ^^ borrowed value does not live long enough
- LL |     drop(a.0);
-    |          --- copying this value requires that `s` is borrowed for `'static`
- LL |     drop(a.0);
+    |                  |
+    |                  |
+    |                  borrowed value does not live long enough
+    |                  requires that `s` is borrowed for `'static`
9 LL | }
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |     let a = (Foo(&s),); //~ ERROR `s` does not live long enough [E0597]
   |                  |
   |                  |
   |                  borrowed value does not live long enough
   |                  requires that `s` is borrowed for `'static`
LL | }
LL | }
   | - `s` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/do-not-ignore-lifetime-bounds-in-copy.rs:8:17
3    |
4 LL |     let a = Foo(&s);
-    |                 ^^ borrowed value does not live long enough
- LL |     drop(a);
-    |          - copying this value requires that `s` is borrowed for `'static`
- LL |     drop(a);
+    |                 |
+    |                 |
+    |                 borrowed value does not live long enough
+    |                 requires that `s` is borrowed for `'static`
9 LL | }
9 LL | }
10    | - `s` dropped here while still borrowed


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy/do-not-ignore-lifetime-bounds-in-copy.stderr
To only update this specific test, also pass `--test-args nll/do-not-ignore-lifetime-bounds-in-copy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |     let a = Foo(&s); //~ ERROR `s` does not live long enough [E0597]
   |                 |
   |                 |
   |                 borrowed value does not live long enough
   |                 requires that `s` is borrowed for `'static`
LL | }
LL | }
   | - `s` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
1 error: lifetime may not live long enough
-   --> $DIR/issue-42574-diagnostic-in-nested-closure.rs:8:8
+   --> $DIR/issue-42574-diagnostic-in-nested-closure.rs:8:5
3    |
4 LL |     || doit(data);
-    |     -- ^^^^^^^^^^ argument requires that `'1` must outlive `'static`
+    |     --^^^^^^^^^^^
6    |     |
7    |     lifetime `'1` represents this closure's body
+    |     requires that `'1` must outlive `'static`
8    |
9    = note: closure implements `FnMut`, so references to captured variables can't escape the closure

12   --> $DIR/issue-42574-diagnostic-in-nested-closure.rs:8:13
13    |
13    |
14 LL |     || doit(data);
-    |     -- -----^^^^-
-    |     |  |    |
-    |     |  |    borrowed value does not live long enough
-    |     |  argument requires that `data` is borrowed for `'static`
+    |     |       |
+    |     |       |
+    |     |       borrowed value does not live long enough
19    |     value captured here
+    |     requires that `data` is borrowed for `'static`
21 LL | }
21 LL | }
22    |  - `data` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure/issue-42574-diagnostic-in-nested-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-42574-diagnostic-in-nested-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-42574-diagnostic-in-nested-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-42574-diagnostic-in-nested-closure.rs:8:5
   |
LL |     || doit(data);
   |     --^^^^^^^^^^^
   |     |
   |     lifetime `'1` represents this closure's body
   |     requires that `'1` must outlive `'static`
   |
   = note: closure implements `FnMut`, so references to captured variables can't escape the closure

error[E0597]: `data` does not live long enough
   |
   |
LL |     || doit(data);
   |     |       |
   |     |       |
   |     |       borrowed value does not live long enough
   |     value captured here
   |     requires that `data` is borrowed for `'static`
LL | }
LL | }
   |  - `data` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

5    |                        ^^
6    |                        |
7    |                        borrowed value does not live long enough
-    |                        this usage requires that `a` is borrowed for `'static`
+    |                        requires that `a` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-46036.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-46036" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-46036/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     let foo = Foo { x: &a }; //~ ERROR E0597
   |                        |
   |                        |
   |                        borrowed value does not live long enough
   |                        requires that `a` is borrowed for `'static`
LL |     loop { }
LL | }
   | - `a` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

20    |                          ^^^^^^^^^^^^^^^^^^^^^
21    |                          |
22    |                          `list[_].next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list[_].next` is borrowed for `'1`
+    |                          requires that `list[_].next` is borrowed for `'1`
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index/issue-62007-assign-const-index.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-62007-assign-const-index.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-62007-assign-const-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `list[_].value` as mutable more than once at a time
   |
   |
LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
   |                          - let's call the lifetime of this reference `'1`
...
LL |         result.push(&mut list[0].value); //~ ERROR cannot borrow `list[_].value` as mutable
   |                     ^^^^^^^^^^^^^^^^^^ `list[_].value` was mutably borrowed here in the previous iteration of the loop
LL |             return result;
LL |             return result;
   |                    ------ returning this value requires that `list[_].value` is borrowed for `'1`

error[E0499]: cannot borrow `list[_].next` as mutable more than once at a time
   |
   |
LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
   |                          - let's call the lifetime of this reference `'1`
...
LL |         if let Some(n) = list[0].next.as_mut() { //~ ERROR cannot borrow `list[_].next` as mutable
   |                          |
   |                          |
   |                          `list[_].next` was mutably borrowed here in the previous iteration of the loop
   |                          requires that `list[_].next` is borrowed for `'1`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0499`.

---
diff of stderr:

2   --> $DIR/issue-57265-return-type-wf-check.rs:20:23
3    |
4 LL |     let (_, z) = foo(&"hello".to_string());
-    |                  -----^^^^^^^^^^^^^^^^^^^-- temporary value is freed at the end of this statement
-    |                  |    |
-    |                  |    creates a temporary which is freed while still in use
-    |                  argument requires that borrow lasts for `'static`
+    |                      -^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
+    |                      ||
+    |                      |creates a temporary which is freed while still in use
+    |                      requires that borrow lasts for `'static`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57265-return-type-wf-check/issue-57265-return-type-wf-check.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-57265-return-type-wf-check.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-57265-return-type-wf-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57265-return-type-wf-check" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57265-return-type-wf-check/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let (_, z) = foo(&"hello".to_string());
   |                      -^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                      ||
   |                      |creates a temporary which is freed while still in use
   |                      requires that borrow lasts for `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.

---
diff of stderr:

20    |                          ^^^^^^^^^^^^^^^^^^^^^^
21    |                          |
22    |                          `list.0.next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list.0.next` is borrowed for `'a`
+    |                          requires that `list.0.next` is borrowed for `'a`
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-differing-fields/issue-62007-assign-differing-fields.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-62007-assign-differing-fields.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-62007-assign-differing-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-differing-fields" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-differing-fields/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `list.0.value` as mutable more than once at a time
   |
   |
LL | fn to_refs<'a, T>(mut list: (&'a mut List<T>, &'a mut List<T>)) -> Vec<&'a mut T> {
   |            -- lifetime `'a` defined here
...
LL |         result.push(&mut (list.0).value); //~ ERROR cannot borrow `list.0.value` as mutable
   |                     ^^^^^^^^^^^^^^^^^^^ `list.0.value` was mutably borrowed here in the previous iteration of the loop
LL |             return result;
LL |             return result;
   |                    ------ returning this value requires that `list.0.value` is borrowed for `'a`

error[E0499]: cannot borrow `list.0.next` as mutable more than once at a time
   |
   |
LL | fn to_refs<'a, T>(mut list: (&'a mut List<T>, &'a mut List<T>)) -> Vec<&'a mut T> {
   |            -- lifetime `'a` defined here
...
LL |         if let Some(n) = (list.0).next.as_mut() { //~ ERROR cannot borrow `list.0.next` as mutable
   |                          |
   |                          |
   |                          `list.0.next` was mutably borrowed here in the previous iteration of the loop
   |                          requires that `list.0.next` is borrowed for `'a`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0499`.

---
diff of stderr:

2   --> $DIR/issue-69114-static-ty.rs:7:9
3    |
4 LL |     FOO(&n);
-    |     ----^^-
-    |     |   |
-    |     |   borrowed value does not live long enough
-    |     argument requires that `n` is borrowed for `'static`
+    |         |
+    |         |
+    |         borrowed value does not live long enough
+    |         requires that `n` is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `n` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-ty/issue-69114-static-ty.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-69114-static-ty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-69114-static-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-ty/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `n` does not live long enough
   |
LL |     FOO(&n);
   |         ^^
   |         |
   |         |
   |         borrowed value does not live long enough
   |         requires that `n` is borrowed for `'static`
LL |     //~^ ERROR does not live long enough
LL | }
   | - `n` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
3    |
4 LL |         BAR = &n;
-    |         ------^^
-    |         |     |
-    |         |     borrowed value does not live long enough
-    |         assignment requires that `n` is borrowed for `'static`
+    |               |
+    |               |
+    |               borrowed value does not live long enough
+    |               requires that `n` is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `n` dropped here while still borrowed
14   --> $DIR/issue-69114-static-mut-ty.rs:27:22
15    |
15    |
16 LL |         BAR_ELIDED = &n;
-    |         |            |
-    |         |            |
-    |         |            borrowed value does not live long enough
-    |         assignment requires that `n` is borrowed for `'static`
+    |                      |
+    |                      |
+    |                      borrowed value does not live long enough
+    |                      requires that `n` is borrowed for `'static`
22 LL | }
22 LL | }
23    | - `n` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-mut-ty/issue-69114-static-mut-ty.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-69114-static-mut-ty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-69114-static-mut-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-mut-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-mut-ty/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `n` does not live long enough
   |
LL |         BAR = &n;
   |               ^^
   |               |
   |               |
   |               borrowed value does not live long enough
   |               requires that `n` is borrowed for `'static`
LL | }
LL | }
   | - `n` dropped here while still borrowed

error[E0597]: `n` does not live long enough
   |
   |
LL |         BAR_ELIDED = &n;
   |                      |
   |                      |
   |                      borrowed value does not live long enough
   |                      requires that `n` is borrowed for `'static`
LL | }
LL | }
   | - `n` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/nll/outlives-suggestion-simple.rs stdout ----
diff of stderr:

89    |
90    = help: consider adding the following bound: `'b: 'a`
91 
- error[E0521]: borrowed data escapes outside of associated function
-   --> $DIR/outlives-suggestion-simple.rs:73:9
+ error: lifetime may not live long enough
94    |
94    |
95 LL | impl<'a> Foo2<'a> {
96    |      -- lifetime `'a` defined here

97 LL |     // should not produce outlives suggestions to name 'self
98 LL |     fn get_bar(&self) -> Bar2 {
+    |                -^^^^
100    |                |
100    |                |
-    |                `self` declared here, outside of the associated function body
-    |                `self` is a reference that is only valid in the associated function body
103    |                let's call the lifetime of this reference `'1`
- LL |         Bar2::new(&self)
-    |         |
-    |         |
-    |         `self` escapes the associated function body here
-    |         argument requires that `'1` must outlive `'a`
+    |                requires that `'1` must outlive `'a`
110 error: aborting due to 9 previous errors
111 

- For more information about this error, try `rustc --explain E0521`.
---
To only update this specific test, also pass `--test-args nll/outlives-suggestion-simple.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:6:5
   |
LL | fn foo1<'a, 'b>(x: &'a usize) -> &'b usize {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     x //~ERROR lifetime may not live long enough
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:10:5
   |
   |
LL | fn foo2<'a>(x: &'a usize) -> &'static usize {
   |         -- lifetime `'a` defined here
LL |     x //~ERROR lifetime may not live long enough
   |     ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
   |
   |
LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:14:5
   |
   |
LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:22:5
   |
   |
LL | fn foo4<'a, 'b, 'c>(x: &'a usize) -> (&'b usize, &'c usize) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |     (x, x) //~ERROR lifetime may not live long enough
   |     ^^^^^^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:31:9
   |
   |
LL |     pub fn foo<'a>(x: &'a usize) -> Self {
   |                -- lifetime `'a` defined here
LL |         Foo { x } //~ERROR lifetime may not live long enough
   |         ^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:41:9
   |
   |
LL | impl<'a> Bar<'a> {
   |      -- lifetime `'a` defined here
LL |     pub fn get<'b>(&self) -> &'b usize {
   |                -- lifetime `'b` defined here
LL |         self.x //~ERROR lifetime may not live long enough
   |         ^^^^^^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:52:9
   |
   |
LL | impl<'a> Baz<'a> {
   |      -- lifetime `'a` defined here
LL |     fn get<'b>(&'b self) -> &'a i32 {
   |            -- lifetime `'b` defined here
LL |         self.x //~ERROR lifetime may not live long enough
   |         ^^^^^^ returning this value requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:72:16
   |
   |
LL | impl<'a> Foo2<'a> {
   |      -- lifetime `'a` defined here
LL |     // should not produce outlives suggestions to name 'self
LL |     fn get_bar(&self) -> Bar2 {
   |                -^^^^
   |                |
   |                let's call the lifetime of this reference `'1`
   |                requires that `'1` must outlive `'a`
error: aborting due to 9 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/nll/polonius/assignment-to-differing-field.rs stdout ----
diff of stderr:

20    |                          ^^^^^^^^^^^^^^^^^^^^^^
21    |                          |
22    |                          `list.0.next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list.0.next` is borrowed for `'a`
+    |                          requires that `list.0.next` is borrowed for `'a`
24 
25 error[E0499]: cannot borrow `list.0.0.0.0.0.value` as mutable more than once at a time

44    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
45    |                          |
45    |                          |
46    |                          `list.0.0.0.0.0.next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list.0.0.0.0.0.next` is borrowed for `'a`
+    |                          requires that `list.0.0.0.0.0.next` is borrowed for `'a`
49 error: aborting due to 4 previous errors
50 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/assignment-to-differing-field/assignment-to-differing-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/polonius/assignment-to-differing-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/polonius/assignment-to-differing-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/assignment-to-differing-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "polonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/assignment-to-differing-field/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `list.0.value` as mutable more than once at a time
   |
   |
LL | fn assignment_to_field_projection<'a, T>(
   |                                   -- lifetime `'a` defined here
...
LL |         result.push(&mut (list.0).value);
   |                     ^^^^^^^^^^^^^^^^^^^ `list.0.value` was mutably borrowed here in the previous iteration of the loop
LL |             return result;
LL |             return result;
   |                    ------ returning this value requires that `list.0.value` is borrowed for `'a`

error[E0499]: cannot borrow `list.0.next` as mutable more than once at a time
   |
   |
LL | fn assignment_to_field_projection<'a, T>(
   |                                   -- lifetime `'a` defined here
...
LL |         if let Some(n) = (list.0).next.as_mut() {
   |                          |
   |                          |
   |                          `list.0.next` was mutably borrowed here in the previous iteration of the loop
   |                          requires that `list.0.next` is borrowed for `'a`

error[E0499]: cannot borrow `list.0.0.0.0.0.value` as mutable more than once at a time
   |
   |
LL | fn assignment_through_projection_chain<'a, T>(
   |                                        -- lifetime `'a` defined here
...
LL |         result.push(&mut ((((list.0).0).0).0).0.value);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `list.0.0.0.0.0.value` was mutably borrowed here in the previous iteration of the loop
LL |             return result;
LL |             return result;
   |                    ------ returning this value requires that `list.0.0.0.0.0.value` is borrowed for `'a`

error[E0499]: cannot borrow `list.0.0.0.0.0.next` as mutable more than once at a time
   |
   |
LL | fn assignment_through_projection_chain<'a, T>(
   |                                        -- lifetime `'a` defined here
...
LL |         if let Some(n) = ((((list.0).0).0).0).0.next.as_mut() {
   |                          |
   |                          |
   |                          `list.0.0.0.0.0.next` was mutably borrowed here in the previous iteration of the loop
   |                          requires that `list.0.0.0.0.0.next` is borrowed for `'a`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/nll/ty-outlives/projection-one-region-closure.rs stdout ----
diff of stderr:

37    = help: consider adding an explicit lifetime bound `T: 'a`...
39 error: lifetime may not live long enough
-   --> $DIR/projection-one-region-closure.rs:45:39
+   --> $DIR/projection-one-region-closure.rs:45:29
41    |
41    |
42 LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
43    |                          --  -- lifetime `'b` defined here

45    |                          lifetime `'a` defined here
46 ...
47 LL |     with_signature(cell, t, |cell, t| require(cell, t));
-    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
49    |
50    = help: consider adding the following bound: `'b: 'a`


87    = help: consider adding an explicit lifetime bound `T: 'a`...
89 error: lifetime may not live long enough
-   --> $DIR/projection-one-region-closure.rs:56:39
+   --> $DIR/projection-one-region-closure.rs:56:29
91    |
91    |
92 LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
93    |                           --  -- lifetime `'b` defined here

95    |                           lifetime `'a` defined here
96 ...
97 LL |     with_signature(cell, t, |cell, t| require(cell, t));
-    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
99    |
100    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/projection-one-region-closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/projection-one-region-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:45:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where T: '_#2r
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:41:1
   |
   |
LL | / fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | | {
...  |
LL | |     //~| ERROR
LL | | }
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>

error[E0309]: the parameter type `T` may not live long enough
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:45:29
   |
   |
LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                          --  -- lifetime `'b` defined here
   |                          |
   |                          lifetime `'a` defined here
...
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:56:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where T: '_#3r
   = note: where '_#2r: '_#3r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:51:1
   |
   |
LL | / fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     'a: 'a,
...  |
LL | |     //~| ERROR
LL | | }
   |
   |
   = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>

error[E0309]: the parameter type `T` may not live long enough
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:56:29
   |
   |
LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                           --  -- lifetime `'b` defined here
   |                           |
   |                           lifetime `'a` defined here
...
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:70:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: projection_outlives::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where <T as Anything<ReEarlyBound(1, 'b)>>::AssocType: '_#3r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:62:1
   |
   |
LL | / fn projection_outlives<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     T::AssocType: 'a,
...  |
LL | |     with_signature(cell, t, |cell, t| require(cell, t));
LL | | }
   |
   |
   = note: defining type: projection_outlives::<'_#1r, '_#2r, T>
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:80:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: elements_outlive::<'_#1r, '_#2r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#3r ()>, T)),
               (),
           ]
   = note: number of external vids: 4
   = note: where T: '_#3r
   = note: where '_#2r: '_#3r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:74:1
   |
   |
LL | / fn elements_outlive<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
LL | | where
LL | |     T: Anything<'b>,
LL | |     T: 'a,
...  |
LL | |     with_signature(cell, t, |cell, t| require(cell, t));
LL | | }
   |
   |
   = note: defining type: elements_outlive::<'_#1r, '_#2r, T>
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0309`.


------------------------------------------


---- [ui] ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs stdout ----
diff of stderr:

28    = note: defining type: no_relationships_late::<'_#1r, T>
30 error: lifetime may not live long enough
-   --> $DIR/projection-one-region-trait-bound-closure.rs:37:39
+   --> $DIR/projection-one-region-trait-bound-closure.rs:37:29
32    |
32    |
33 LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
34    |                          --  -- lifetime `'b` defined here

36    |                          lifetime `'a` defined here
37 ...
38 LL |     with_signature(cell, t, |cell, t| require(cell, t));
-    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
40    |
41    = help: consider adding the following bound: `'b: 'a`


69    = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>
71 error: lifetime may not live long enough
-   --> $DIR/projection-one-region-trait-bound-closure.rs:47:39
+   --> $DIR/projection-one-region-trait-bound-closure.rs:47:29
73    |
73    |
74 LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
75    |                           --  -- lifetime `'b` defined here

77    |                           lifetime `'a` defined here
78 ...
79 LL |     with_signature(cell, t, |cell, t| require(cell, t));
-    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
+    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
81    |
82    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure/projection-one-region-trait-bound-closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure/projection-one-region-trait-bound-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-trait-bound-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
note: external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:37:29
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = note: defining type: no_relationships_late::<'_#1r, T>::{closure#0} with closure substs [
               i32,
               extern "rust-call" fn((std::cell::Cell<&'_#2r ()>, T)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#3r
   = note: number of external vids: 4
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-trait-bound-closure.rs:33:1
---
1 error: lifetime may not live long enough
-   --> $DIR/type-check-pointer-comparisons.rs:6:5
+   --> $DIR/type-check-pointer-comparisons.rs:5:26
3    |
4 LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
-    |                  --  -- lifetime `'b` defined here
-    |                  |
+    |                  --  --  ^ requires that `'a` must outlive `'b`
+    |                  |   |
+    |                  |   lifetime `'b` defined here
7    |                  lifetime `'a` defined here
- LL |     x == y;
-    |     ^ requires that `'a` must outlive `'b`
10    |
11    = help: consider adding the following bound: `'a: 'b`
12    = note: requirement occurs because of a mutable reference to &i32

14    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
16 error: lifetime may not live long enough
-   --> $DIR/type-check-pointer-comparisons.rs:6:10
+   --> $DIR/type-check-pointer-comparisons.rs:5:50
18    |
18    |
19 LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
-    |                  --  -- lifetime `'b` defined here
-    |                  |
+    |                  --  --                          ^ requires that `'b` must outlive `'a`
+    |                  |   |
+    |                  |   lifetime `'b` defined here
22    |                  lifetime `'a` defined here
- LL |     x == y;
-    |          ^ requires that `'b` must outlive `'a`
25    |
26    = help: consider adding the following bound: `'b: 'a`
27    = note: requirement occurs because of a mutable reference to &i32

31 help: `'a` and `'b` must be the same: replace one with the other
33 error: lifetime may not live long enough
-   --> $DIR/type-check-pointer-comparisons.rs:12:5
+   --> $DIR/type-check-pointer-comparisons.rs:11:24
35    |
35    |
36 LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
-    |                --  -- lifetime `'b` defined here
-    |                |
+    |                --  --  ^ requires that `'a` must outlive `'b`
+    |                |   |
+    |                |   lifetime `'b` defined here
39    |                lifetime `'a` defined here
- LL |     x == y;
-    |     ^ requires that `'a` must outlive `'b`
42    |
43    = help: consider adding the following bound: `'a: 'b`
44    = note: requirement occurs because of a mutable pointer to &i32

46    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
48 error: lifetime may not live long enough
-   --> $DIR/type-check-pointer-comparisons.rs:12:10
+   --> $DIR/type-check-pointer-comparisons.rs:11:41
50    |
50    |
51 LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
-    |                --  -- lifetime `'b` defined here
-    |                |
+    |                --  --                   ^ requires that `'b` must outlive `'a`
+    |                |   |
+    |                |   lifetime `'b` defined here
54    |                lifetime `'a` defined here
- LL |     x == y;
-    |          ^ requires that `'b` must outlive `'a`
57    |
58    = help: consider adding the following bound: `'b: 'a`
59    = note: requirement occurs because of a mutable pointer to &i32

63 help: `'a` and `'b` must be the same: replace one with the other
65 error: lifetime may not live long enough
-   --> $DIR/type-check-pointer-comparisons.rs:18:5
+   --> $DIR/type-check-pointer-comparisons.rs:17:31
67    |
67    |
68 LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
-    |                   --  -- lifetime `'b` defined here
-    |                   |
+    |                   --  --      ^ requires that `'a` must outlive `'b`
+    |                   |   |
+    |                   |   lifetime `'b` defined here
71    |                   lifetime `'a` defined here
- LL |     f == g;
-    |     ^ requires that `'a` must outlive `'b`
74    |
75    = help: consider adding the following bound: `'a: 'b`
76    = note: requirement occurs because of a mutable reference to &i32

78    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
80 error: lifetime may not live long enough
-   --> $DIR/type-check-pointer-comparisons.rs:18:10
+   --> $DIR/type-check-pointer-comparisons.rs:17:55
82    |
82    |
83 LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
-    |                   --  -- lifetime `'b` defined here
+    |                   --  -- lifetime `'b` defined here   ^ requires that `'b` must outlive `'a`
85    |                   |
86    |                   lifetime `'a` defined here
- LL |     f == g;
-    |          ^ requires that `'b` must outlive `'a`
89    |
90    = help: consider adding the following bound: `'b: 'a`
91    = note: requirement occurs because of a mutable reference to &i32

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/type-check-pointer-comparisons.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/type-check-pointer-comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/type-check-pointer-comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:5:26
   |
LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
   |                  --  --  ^ requires that `'a` must outlive `'b`
   |                  |   |
   |                  |   lifetime `'b` defined here
   |                  lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:5:50
   |
   |
LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
   |                  --  --                          ^ requires that `'b` must outlive `'a`
   |                  |   |
   |                  |   lifetime `'b` defined here
   |                  lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:11:24
   |
   |
LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
   |                --  --  ^ requires that `'a` must outlive `'b`
   |                |   |
   |                |   lifetime `'b` defined here
   |                lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable pointer to &i32
   = note: mutable pointers are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:11:41
   |
   |
LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
   |                --  --                   ^ requires that `'b` must outlive `'a`
   |                |   |
   |                |   lifetime `'b` defined here
   |                lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable pointer to &i32
   = note: mutable pointers are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:17:31
   |
   |
LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
   |                   --  --      ^ requires that `'a` must outlive `'b`
   |                   |   |
   |                   |   lifetime `'b` defined here
   |                   lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:17:55
   |
   |
LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
   |                   --  -- lifetime `'b` defined here   ^ requires that `'b` must outlive `'a`
   |                   |
   |                   lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 6 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/nll/user-annotations/adt-brace-structs.rs stdout ----
diff of stderr:

5    |                                     ^^
6    |                                     |
7    |                                     borrowed value does not live long enough
-    |                                     this usage requires that `c` is borrowed for `'static`
+    |                                     requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/adt-brace-structs.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/adt-brace-structs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-brace-structs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-brace-structs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-structs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeStruct::<&'static u32> { t: &c }; //~ ERROR
   |                                     |
   |                                     |
   |                                     borrowed value does not live long enough
   |                                     requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeStruct::<&'a u32> { t: &c }; //~ ERROR
   |                                |
   |                                |
   |                                borrowed value does not live long enough
   |                                this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         SomeStruct::<&'a u32> { t: &c }; //~ ERROR
   |                                    |
   |                                    |
   |                                    borrowed value does not live long enough
   |                                    this usage requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/adt-nullary-enums.rs:33:41
3    |
4 LL |         SomeEnum::SomeVariant(Cell::new(&c)),
-    |                               ----------^^-
-    |                               |         |
-    |                               |         borrowed value does not live long enough
-    |                               argument requires that `c` is borrowed for `'static`
+    |                                         |
+    |                                         |
+    |                                         borrowed value does not live long enough
+    |                                         requires that `c` is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `c` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/adt-nullary-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-nullary-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |         SomeEnum::SomeVariant(Cell::new(&c)), //~ ERROR
   |                                         |
   |                                         |
   |                                         borrowed value does not live long enough
   |                                         requires that `c` is borrowed for `'static`
LL | }
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
...
LL |         SomeEnum::SomeVariant(Cell::new(&c)), //~ ERROR
   |                               ----------^^-
   |                               |         |
   |                               |         borrowed value does not live long enough
   |                               argument requires that `c` is borrowed for `'a`
LL | }
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |             SomeEnum::SomeVariant(Cell::new(&c)), //~ ERROR
   |                                   ----------^^-
   |                                   |         |
   |                                   |         borrowed value does not live long enough
   |                                   argument requires that `c` is borrowed for `'a`
LL |     };
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

5    |                                                ^^
6    |                                                |
7    |                                                borrowed value does not live long enough
-    |                                                this usage requires that `c` is borrowed for `'static`
+    |                                                requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/adt-brace-enums.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/adt-brace-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-brace-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-brace-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeEnum::SomeVariant::<&'static u32> { t: &c }; //~ ERROR
   |                                                |
   |                                                |
   |                                                borrowed value does not live long enough
   |                                                requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeEnum::SomeVariant::<&'a u32> { t: &c }; //~ ERROR
   |                                           |
   |                                           |
   |                                           borrowed value does not live long enough
   |                                           this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         SomeEnum::SomeVariant::<&'a u32> { t: &c }; //~ ERROR
   |                                               |
   |                                               |
   |                                               borrowed value does not live long enough
   |                                               this usage requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

5    |                                           ^^
6    |                                           |
7    |                                           borrowed value does not live long enough
-    |                                           this usage requires that `c` is borrowed for `'static`
+    |                                           requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/adt-tuple-enums.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/adt-tuple-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeEnum::SomeVariant::<&'static u32>(&c); //~ ERROR
   |                                           |
   |                                           |
   |                                           borrowed value does not live long enough
   |                                           requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeEnum::SomeVariant::<&'a u32>(&c); //~ ERROR
   |                                      |
   |                                      |
   |                                      borrowed value does not live long enough
   |                                      this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         SomeEnum::SomeVariant::<&'a u32>(&c); //~ ERROR
   |                                          |
   |                                          |
   |                                          borrowed value does not live long enough
   |                                          this usage requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/nll/user-annotations/closure-substs.rs stdout ----
diff of stderr:

16    |                ^ returning this value requires that `'1` must outlive `'static`
18 error: lifetime may not live long enough
-   --> $DIR/closure-substs.rs:22:9
+   --> $DIR/closure-substs.rs:21:6
20    |
20    |
21 LL | fn bar<'a>() {
22    |        -- lifetime `'a` defined here
- ...
- ...
- LL |         b(x);
-    |         ^^^^ argument requires that `'a` must outlive `'static`
+ LL |     // Here `x` is free in the closure sig:
+ LL |     |x: &'a i32, b: fn(&'static i32)| {
+    |      ^ requires that `'a` must outlive `'static`
26 
- error[E0521]: borrowed data escapes outside of closure
-   --> $DIR/closure-substs.rs:29:9
+ error: lifetime may not live long enough
29    |
29    |
30 LL |     |x: &i32, b: fn(&'static i32)| {
-    |      -  - let's call the lifetime of this reference `'1`
+    |      ^  - let's call the lifetime of this reference `'1`
32    |      |
-    |      `x` is a reference that is only valid in the closure body
- LL |         b(x);
-    |         |
-    |         |
-    |         `x` escapes the closure body here
-    |         argument requires that `'1` must outlive `'static`
+    |      requires that `'1` must outlive `'static`
40 error: aborting due to 4 previous errors
41 

- For more information about this error, try `rustc --explain E0521`.
---
To only update this specific test, also pass `--test-args nll/user-annotations/closure-substs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/closure-substs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/closure-substs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/closure-substs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:8:16
   |
LL | fn foo<'a>() {
   |        -- lifetime `'a` defined here
...
LL |         return x; //~ ERROR lifetime may not live long enough
   |                ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:15:16
   |
   |
LL |     |x: &i32| -> &'static i32 {
   |         - let's call the lifetime of this reference `'1`
LL |         return x; //~ ERROR lifetime may not live long enough
   |                ^ returning this value requires that `'1` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:21:6
   |
   |
LL | fn bar<'a>() {
   |        -- lifetime `'a` defined here
LL |     // Here `x` is free in the closure sig:
LL |     |x: &'a i32, b: fn(&'static i32)| {
   |      ^ requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:28:6
   |
   |
LL |     |x: &i32, b: fn(&'static i32)| {
   |      ^  - let's call the lifetime of this reference `'1`
   |      |
   |      requires that `'1` must outlive `'static`
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/nll/user-annotations/adt-tuple-struct.rs stdout ----
diff of stderr:

5    |                                ^^
6    |                                |
7    |                                borrowed value does not live long enough
-    |                                this usage requires that `c` is borrowed for `'static`
+    |                                requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/adt-tuple-struct.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/adt-tuple-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeStruct::<&'static u32>(&c); //~ ERROR
   |                                |
   |                                |
   |                                borrowed value does not live long enough
   |                                requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     SomeStruct::<&'a u32>(&c); //~ ERROR
   |                           |
   |                           |
   |                           borrowed value does not live long enough
   |                           this usage requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         SomeStruct::<&'a u32>(&c); //~ ERROR
   |                               |
   |                               |
   |                               borrowed value does not live long enough
   |                               this usage requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/constant-in-expr-inherent-2.rs:23:9
3    |
4 LL |     FUN(&x);
-    |     ----^^-
-    |     |   |
-    |     |   borrowed value does not live long enough
-    |     argument requires that `x` is borrowed for `'static`
+    |         |
+    |         |
+    |         borrowed value does not live long enough
+    |         requires that `x` is borrowed for `'static`
10 LL | }
10 LL | }
11    | - `x` dropped here while still borrowed
14   --> $DIR/constant-in-expr-inherent-2.rs:24:23
15    |
15    |
16 LL |     A::ASSOCIATED_FUN(&x);
-    |     ------------------^^-
-    |     |                 |
-    |     |                 borrowed value does not live long enough
-    |     argument requires that `x` is borrowed for `'static`
+    |                       |
+    |                       |
+    |                       borrowed value does not live long enough
+    |                       requires that `x` is borrowed for `'static`
22 LL | }
22 LL | }
23    | - `x` dropped here while still borrowed
26   --> $DIR/constant-in-expr-inherent-2.rs:25:28
27    |
27    |
28 LL |     B::ALSO_ASSOCIATED_FUN(&x);
-    |     -----------------------^^-
-    |     |                      |
-    |     |                      borrowed value does not live long enough
-    |     argument requires that `x` is borrowed for `'static`
+    |                            |
+    |                            |
+    |                            borrowed value does not live long enough
+    |                            requires that `x` is borrowed for `'static`
33 LL |     <_>::TRAIT_ASSOCIATED_FUN(&x);
34 LL | }
35    | - `x` dropped here while still borrowed
38   --> $DIR/constant-in-expr-inherent-2.rs:26:31
39    |
39    |
40 LL |     <_>::TRAIT_ASSOCIATED_FUN(&x);
-    |     --------------------------^^-
-    |     |                         |
-    |     |                         borrowed value does not live long enough
-    |     argument requires that `x` is borrowed for `'static`
+    |                               |
+    |                               |
+    |                               borrowed value does not live long enough
+    |                               requires that `x` is borrowed for `'static`
45 LL | }
46    | - `x` dropped here while still borrowed


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-2/constant-in-expr-inherent-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-inherent-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-inherent-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     FUN(&x);                        //~ ERROR `x` does not live long enough
   |         |
   |         |
   |         borrowed value does not live long enough
   |         requires that `x` is borrowed for `'static`
LL | }
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     A::ASSOCIATED_FUN(&x);          //~ ERROR `x` does not live long enough
   |                       |
   |                       |
   |                       borrowed value does not live long enough
   |                       requires that `x` is borrowed for `'static`
LL | }
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     B::ALSO_ASSOCIATED_FUN(&x);     //~ ERROR `x` does not live long enough
   |                            |
   |                            |
   |                            borrowed value does not live long enough
   |                            requires that `x` is borrowed for `'static`
LL |     <_>::TRAIT_ASSOCIATED_FUN(&x);  //~ ERROR `x` does not live long enough
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     <_>::TRAIT_ASSOCIATED_FUN(&x);  //~ ERROR `x` does not live long enough
   |                               |
   |                               |
   |                               borrowed value does not live long enough
   |                               requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/adt-tuple-struct-calls.rs:27:7
3    |
4 LL |     f(&c);
-    |     --^^-
-    |     | |
-    |     | borrowed value does not live long enough
-    |     argument requires that `c` is borrowed for `'static`
+    |       |
+    |       |
+    |       borrowed value does not live long enough
+    |       requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls/adt-tuple-struct-calls.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls/adt-tuple-struct-calls.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-struct-calls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-struct-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     f(&c); //~ ERROR
   |       |
   |       |
   |       borrowed value does not live long enough
   |       requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
...
LL |     f(&c); //~ ERROR
   |     --^^-
   |     | |
   |     | borrowed value does not live long enough
   |     argument requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         f(&c); //~ ERROR
   |         --^^-
   |         | |
   |         | borrowed value does not live long enough
   |         argument requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL |     let f = SomeStruct::<&'a u32>;
   |         - lifetime `'1` appears in the type of `f`
...
LL |         f(&c); //~ ERROR
   |         --^^-
   |         | |
   |         | borrowed value does not live long enough
   |         argument requires that `c` is borrowed for `'1`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/fns.rs:23:29
3    |
4 LL |     some_fn::<&'static u32>(&c);
-    |     ------------------------^^-
-    |     |                       |
-    |     |                       borrowed value does not live long enough
-    |     argument requires that `c` is borrowed for `'static`
+    |                             |
+    |                             |
+    |                             borrowed value does not live long enough
+    |                             requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns/fns.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns/fns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/fns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     some_fn::<&'static u32>(&c); //~ ERROR
   |                             |
   |                             |
   |                             borrowed value does not live long enough
   |                             requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
LL |     let c = 66;
LL |     some_fn::<&'a u32>(&c); //~ ERROR
   |     -------------------^^-
   |     |                  |
   |     |                  borrowed value does not live long enough
   |     argument requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         some_fn::<&'a u32>(&c); //~ ERROR
   |         -------------------^^-
   |         |                  |
   |         |                  borrowed value does not live long enough
   |         argument requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/method-ufcs-2.rs:30:7
3    |
4 LL |     x(&a, b, c);
-    |     --^^-------
-    |     | |
-    |     | borrowed value does not live long enough
-    |     argument requires that `a` is borrowed for `'static`
+    |       |
+    |       |
+    |       borrowed value does not live long enough
+    |       requires that `a` is borrowed for `'static`
9 LL | }
10    | - `a` dropped here while still borrowed


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-2/method-ufcs-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/method-ufcs-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/method-ufcs-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     x(&a, b, c); //~ ERROR
   |       |
   |       |
   |       borrowed value does not live long enough
   |       requires that `a` is borrowed for `'static`
LL | }
   | - `a` dropped here while still borrowed

error[E0597]: `b` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
...
LL |     <_ as Bazoom<&'a u32>>::method(a, &b, c); //~ ERROR
   |     ----------------------------------^^----
   |     |                                 |
   |     |                                 borrowed value does not live long enough
   |     argument requires that `b` is borrowed for `'a`
LL | }
   | - `b` dropped here while still borrowed

error[E0597]: `b` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |     let _closure = || {
   |                    -- value captured here
LL |         let c = 66;
LL |         <_ as Bazoom<&'a u32>>::method(a, &b, c); //~ ERROR
   |         -----------------------------------^----
   |         |                                  |
   |         |                                  borrowed value does not live long enough
   |         argument requires that `b` is borrowed for `'a`
LL | }
LL | }
   | - `b` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/method-call.rs:36:34
3    |
4 LL |     a.method::<&'static u32>(b,  &c);
-    |     -----------------------------^^-
-    |     |                            |
-    |     |                            borrowed value does not live long enough
-    |     argument requires that `c` is borrowed for `'static`
+    |                                  |
+    |                                  |
+    |                                  borrowed value does not live long enough
+    |                                  requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-call/method-call.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-call/method-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/method-call.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-call/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     a.method::<&'static u32>(b,  &c); //~ ERROR
   |                                  |
   |                                  |
   |                                  borrowed value does not live long enough
   |                                  requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
...
LL |     a.method::<&'a u32>(b,  &c); //~ ERROR
   |     ------------------------^^-
   |     |                       |
   |     |                       borrowed value does not live long enough
   |     argument requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         a.method::<&'a u32>(b,  &c); //~ ERROR
   |         ------------------------^^-
   |         |                       |
   |         |                       borrowed value does not live long enough
   |         argument requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/method-ufcs-3.rs:36:53
3    |
4 LL |     <_ as Bazoom<_>>::method::<&'static u32>(&a, b, &c);
-    |     ------------------------------------------------^^-
-    |     |                                               |
-    |     |                                               borrowed value does not live long enough
-    |     argument requires that `c` is borrowed for `'static`
+    |                                                     |
+    |                                                     |
+    |                                                     borrowed value does not live long enough
+    |                                                     requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-3/method-ufcs-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/method-ufcs-3.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/method-ufcs-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     <_ as Bazoom<_>>::method::<&'static u32>(&a, b, &c); //~ ERROR
   |                                                     |
   |                                                     |
   |                                                     borrowed value does not live long enough
   |                                                     requires that `c` is borrowed for `'static`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
...
LL |     <_ as Bazoom<_>>::method::<&'a u32>(&a, b, &c); //~ ERROR
   |     -------------------------------------------^^-
   |     |                                          |
   |     |                                          borrowed value does not live long enough
   |     argument requires that `c` is borrowed for `'a`
LL | }
   | - `c` dropped here while still borrowed

error[E0597]: `c` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |         <_ as Bazoom<_>>::method::<&'a u32>(&a, b, &c); //~ ERROR
   |         -------------------------------------------^^-
   |         |                                          |
   |         |                                          borrowed value does not live long enough
   |         argument requires that `c` is borrowed for `'a`
LL |     };
   |     - `c` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/method-ufcs-1.rs:30:7
3    |
4 LL |     x(&a, b, c);
-    |     --^^-------
-    |     | |
-    |     | borrowed value does not live long enough
-    |     argument requires that `a` is borrowed for `'static`
+    |       |
+    |       |
+    |       borrowed value does not live long enough
+    |       requires that `a` is borrowed for `'static`
9 LL | }
10    | - `a` dropped here while still borrowed


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-1/method-ufcs-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/method-ufcs-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/method-ufcs-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     x(&a, b, c); //~ ERROR
   |       |
   |       |
   |       borrowed value does not live long enough
   |       requires that `a` is borrowed for `'static`
LL | }
   | - `a` dropped here while still borrowed

error[E0597]: `a` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime<'a>(_d: &'a u32) {
   |                                   -- lifetime `'a` defined here
...
LL |     <&'a u32 as Bazoom<_>>::method(&a, b, c); //~ ERROR
   |     -------------------------------^^-------
   |     |                              |
   |     |                              borrowed value does not live long enough
   |     argument requires that `a` is borrowed for `'a`
LL | }
   | - `a` dropped here while still borrowed

error[E0597]: `a` does not live long enough
   |
   |
LL | fn annot_reference_named_lifetime_in_closure<'a>(_: &'a u32) {
   |                                              -- lifetime `'a` defined here
...
LL |     let _closure = || {
   |                    -- value captured here
LL |         let c = 66;
LL |         <&'a u32 as Bazoom<_>>::method(&a, b, c); //~ ERROR
   |         --------------------------------^-------
   |         |                               |
   |         |                               borrowed value does not live long enough
   |         argument requires that `a` is borrowed for `'a`
LL | }
LL | }
   | - `a` dropped here while still borrowed
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/regions/issue-78262.rs#nll stdout ----
diff of stderr:

- error[E0521]: borrowed data escapes outside of closure
-   --> $DIR/issue-78262.rs:14:26
+ error: lifetime may not live long enough
3    |
3    |
4 LL |     let f = |x: &dyn TT| x.func();
-    |              -  -        ^^^^^^^^
-    |              |  |        |
-    |              |  |        `x` escapes the closure body here
-    |              |  |        argument requires that `'1` must outlive `'static`
-    |              |  let's call the lifetime of this reference `'1`
-    |              `x` is a reference that is only valid in the closure body
+    |              ^  - let's call the lifetime of this reference `'1`
+    |              |
+    |              requires that `'1` must outlive `'static`
12 error: aborting due to previous error
13 

- For more information about this error, try `rustc --explain E0521`.
- For more information about this error, try `rustc --explain E0521`.
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.nll/issue-78262.nll.stderr
To only update this specific test, also pass `--test-args regions/issue-78262.rs`


error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-78262.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/issue-78262.rs:14:14
   |
LL |     let f = |x: &dyn TT| x.func(); //[default]~ ERROR: mismatched types
   |              ^  - let's call the lifetime of this reference `'1`
   |              |
   |              requires that `'1` must outlive `'static`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/regions/issue-78262.rs#polonius stdout ----
diff of stderr:

- error[E0521]: borrowed data escapes outside of closure
-   --> $DIR/issue-78262.rs:14:26
+ error: lifetime may not live long enough
3    |
3    |
4 LL |     let f = |x: &dyn TT| x.func();
-    |              -  -        ^^^^^^^^
-    |              |  |        |
-    |              |  |        `x` escapes the closure body here
-    |              |  |        argument requires that `'1` must outlive `'static`
-    |              |  let's call the lifetime of this reference `'1`
-    |              `x` is a reference that is only valid in the closure body
+    |              ^  - let's call the lifetime of this reference `'1`
+    |              |
+    |              requires that `'1` must outlive `'static`
12 error: aborting due to previous error
13 

- For more information about this error, try `rustc --explain E0521`.
- For more information about this error, try `rustc --explain E0521`.
15 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.polonius/issue-78262.polonius.stderr
To only update this specific test, also pass `--test-args regions/issue-78262.rs`


error in revision `polonius`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-78262.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "polonius" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.polonius" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "polonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.polonius/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/issue-78262.rs:14:14
   |
LL |     let f = |x: &dyn TT| x.func(); //[default]~ ERROR: mismatched types
   |              ^  - let's call the lifetime of this reference `'1`
   |              |
   |              requires that `'1` must outlive `'static`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/regions/regions-infer-proc-static-upvar.rs stdout ----
diff of stderr:

1 error[E0597]: `x` does not live long enough
3    |
3    |
- LL |       let y = &x;
-    |               ^^ borrowed value does not live long enough
- LL | /     foo(move|| {
- LL | |         let _a = *y;
- LL | |     });
-    | |______- argument requires that `x` is borrowed for `'static`
- LL |   }
-    |   - `x` dropped here while still borrowed
+ LL |     let y = &x;
+    |             |
+    |             |
+    |             borrowed value does not live long enough
+    |             assignment requires that `x` is borrowed for `'static`
+ LL | }
+ LL | }
+    | - `x` dropped here while still borrowed
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar/regions-infer-proc-static-upvar.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-proc-static-upvar.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-proc-static-upvar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     let y = &x; //~ ERROR `x` does not live long enough
   |             |
   |             |
   |             borrowed value does not live long enough
   |             assignment requires that `x` is borrowed for `'static`
LL | }
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/regions-pattern-typing-issue-19552.rs:5:14
3    |
4 LL |     match [&*line] {
-    |              ^^^^ borrowed value does not live long enough
- LL |         [ word ] => { assert_static(word); }
-    |                       ------------------- argument requires that `line` is borrowed for `'static`
- LL |     }
+    |              |
+    |              |
+    |              borrowed value does not live long enough
+    |              requires that `line` is borrowed for `'static`
9 LL | }
9 LL | }
10    | - `line` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552/regions-pattern-typing-issue-19552.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552/regions-pattern-typing-issue-19552.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-pattern-typing-issue-19552.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-pattern-typing-issue-19552.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `line` does not live long enough
   |
   |
LL |     match [&*line] { //~ ERROR `line` does not live long enough
   |              |
   |              |
   |              borrowed value does not live long enough
   |              requires that `line` is borrowed for `'static`
LL | }
LL | }
   | - `line` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/regions/regions-static-bound.rs#nll stdout ----
diff of stderr:

7    |     ^ returning this value requires that `'a` must outlive `'static`
9 error[E0621]: explicit lifetime required in the type of `u`
-   --> $DIR/regions-static-bound.rs:14:5
+   --> $DIR/regions-static-bound.rs:13:10
11    |
11    |
- LL |     static_id(&u);
-    |     ^^^^^^^^^^^^^ lifetime `'static` required
+ LL | fn error(u: &(), v: &()) {
+    |          ^ lifetime `'static` required
15 error[E0621]: explicit lifetime required in the type of `v`
-   --> $DIR/regions-static-bound.rs:16:5
+   --> $DIR/regions-static-bound.rs:13:18
17    |
17    |
- LL |     static_id_indirect(&v);
-    |     ^^^^^^^^^^^^^^^^^^^^^^ lifetime `'static` required
+ LL | fn error(u: &(), v: &()) {
+    |                  ^ lifetime `'static` required
21 error: aborting due to 3 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.nll/regions-static-bound.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-static-bound.rs`


error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-static-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:9:5
   |
LL | fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {
   |                        -- lifetime `'a` defined here
LL |     t //[migrate]~ ERROR E0312
   |     ^ returning this value requires that `'a` must outlive `'static`
error[E0621]: explicit lifetime required in the type of `u`
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:13:10
   |
   |
LL | fn error(u: &(), v: &()) {
   |          ^ lifetime `'static` required
error[E0621]: explicit lifetime required in the type of `v`
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:13:18
   |
   |
LL | fn error(u: &(), v: &()) {
   |                  ^ lifetime `'static` required
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0621`.

---
diff of stderr:

10   --> $DIR/static-drop-scope.rs:7:60
11    |
12 LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
-    |                                                      |     |       |
-    |                                                      |     |       temporary value is freed at the end of this statement
-    |                                                      |     |       temporary value is freed at the end of this statement
-    |                                                      |     creates a temporary which is freed while still in use
-    |                                                      using this value as a static requires that borrow lasts for `'static`
+    |                                                           -^^^^^^^^- temporary value is freed at the end of this statement
+    |                                                           ||
+    |                                                           |creates a temporary which is freed while still in use
+    |                                                           requires that borrow lasts for `'static`
19 error[E0493]: destructors cannot be evaluated at compile-time
20   --> $DIR/static-drop-scope.rs:11:59

28   --> $DIR/static-drop-scope.rs:11:59
28   --> $DIR/static-drop-scope.rs:11:59
29    |
30 LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
-    |                                                     |     |       |
-    |                                                     |     |       temporary value is freed at the end of this statement
-    |                                                     |     |       temporary value is freed at the end of this statement
-    |                                                     |     creates a temporary which is freed while still in use
-    |                                                     using this value as a constant requires that borrow lasts for `'static`
+    |                                                          -^^^^^^^^- temporary value is freed at the end of this statement
+    |                                                          ||
+    |                                                          |creates a temporary which is freed while still in use
+    |                                                          requires that borrow lasts for `'static`
37 error[E0493]: destructors cannot be evaluated at compile-time
38   --> $DIR/static-drop-scope.rs:15:28



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/static-drop-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-drop-scope.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-drop-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:7:60
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                            ^^^^^^^^- value is dropped here
   |                                                            statics cannot evaluate destructors


error[E0716]: temporary value dropped while borrowed
   |
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                           -^^^^^^^^- temporary value is freed at the end of this statement
   |                                                           ||
   |                                                           |creates a temporary which is freed while still in use
   |                                                           requires that borrow lasts for `'static`
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:11:59
   |
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                           ^^^^^^^^- value is dropped here
   |                                                           constants cannot evaluate destructors


error[E0716]: temporary value dropped while borrowed
   |
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                          -^^^^^^^^- temporary value is freed at the end of this statement
   |                                                          ||
   |                                                          |creates a temporary which is freed while still in use
   |                                                          requires that borrow lasts for `'static`
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:15:28
   |
   |
LL | static EARLY_DROP_S: i32 = (WithDtor, 0).1;
   |                            ^^^^^^^^^^^^^ - value is dropped here
   |                            statics cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:18:27
  --> /checkout/src/test/ui/static/static-drop-scope.rs:18:27
   |
LL | const EARLY_DROP_C: i32 = (WithDtor, 0).1;
   |                           ^^^^^^^^^^^^^ - value is dropped here
   |                           constants cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:21:24
  --> /checkout/src/test/ui/static/static-drop-scope.rs:21:24
   |
LL | const fn const_drop<T>(_: T) {}
   |                        ^      - value is dropped here
   |                        constant functions cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:25:5
  --> /checkout/src/test/ui/static/static-drop-scope.rs:25:5
   |
LL |     (x, ()).1
   |     ^^^^^^^ constant functions cannot evaluate destructors
LL |     //~^ ERROR destructors cannot be evaluated at compile-time
LL | }
   | - value is dropped here
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:29:34
   |
   |
LL | const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;
   |                                  ^^^^^^^^^^^^^^^^^^^ - value is dropped here
   |                                  constants cannot evaluate destructors

error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:34:43
  --> /checkout/src/test/ui/static/static-drop-scope.rs:34:43
   |
LL | const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;
   |                                           ^^^^^^^^^^^ - value is dropped here
   |                                           constants cannot evaluate destructors

error: aborting due to 10 previous errors

---
diff of stderr:

10   --> $DIR/static-lifetime-bound.rs:5:7
11    |
12 LL |     f(&x);
-    |     --^^-
-    |     | |
-    |     | borrowed value does not live long enough
-    |     argument requires that `x` is borrowed for `'static`
+    |       |
+    |       |
+    |       borrowed value does not live long enough
+    |       requires that `x` is borrowed for `'static`
17 LL | }
18    | - `x` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound/static-lifetime-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound/static-lifetime-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-lifetime-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-lifetime-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unnecessary lifetime parameter `'a`
   |
   |
LL | fn f<'a: 'static>(_: &'a i32) {} //~WARN unnecessary lifetime parameter `'a`
   |
   |
   = help: you can use the `'static` lifetime directly, in place of `'a`

error[E0597]: `x` does not live long enough
   |
   |
LL |     f(&x); //~ERROR does not live long enough
   |       |
   |       |
   |       borrowed value does not live long enough
   |       requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

2   --> $DIR/static-region-bound.rs:10:14
3    |
4 LL |     let x = &id(3);
-    |              ^^^^^ creates a temporary which is freed while still in use
+    |             -^^^^^
+    |             ||
+    |             |creates a temporary which is freed while still in use
+    |             assignment requires that borrow lasts for `'static`
6 LL |     f(x);
-    |     ---- argument requires that borrow lasts for `'static`
8 LL | }
9    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/static-region-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/static-region-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-region-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-region-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
   |
   |
LL |     let x = &id(3); //~ ERROR temporary value dropped while borrowed
   |             -^^^^^
   |             ||
   |             |creates a temporary which is freed while still in use
   |             assignment requires that borrow lasts for `'static`
LL |     f(x);
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.

---
diff of stderr:

2   --> $DIR/check-trait-object-bounds-3.rs:15:34
3    |
4 LL |         z = f::<dyn X<Y = &str>>(&s);
-    |             ---------------------^^-
-    |             |                    |
-    |             |                    borrowed value does not live long enough
-    |             argument requires that `s` is borrowed for `'static`
+    |                                  |
+    |                                  |
+    |                                  borrowed value does not live long enough
+    |                                  requires that `s` is borrowed for `'static`
10 LL |     }
10 LL |     }
11    |     - `s` dropped here while still borrowed

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-3/check-trait-object-bounds-3.stderr
To only update this specific test, also pass `--test-args traits/associated_type_bound/check-trait-object-bounds-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |         z = f::<dyn X<Y = &str>>(&s);
   |                                  |
   |                                  |
   |                                  borrowed value does not live long enough
   |                                  requires that `s` is borrowed for `'static`
LL |         //~^ ERROR `s` does not live long enough
LL |     }
   |     - `s` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

5    |                        ^^^^^^^
6    |                        |
7    |                        borrowed value does not live long enough
-    |                        assignment requires that `person` is borrowed for `'static`
+    |                        requires that `person` is borrowed for `'static`
9 LL |     let s: Box<dyn Trait<&'static str>> = Box::new(Struct { person: person });
10 LL | }
11    | - `person` dropped here while still borrowed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/coercion-generic-regions/coercion-generic-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/coercion-generic-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/coercion-generic-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/coercion-generic-regions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/coercion-generic-regions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `person` does not live long enough
   |
   |
LL |     let person: &str = &person;  //~ ERROR `person` does not live long enough
   |                        |
   |                        |
   |                        borrowed value does not live long enough
   |                        requires that `person` is borrowed for `'static`
LL |     let s: Box<dyn Trait<&'static str>> = Box::new(Struct { person: person });
LL | }
   | - `person` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
test result: FAILED. 12205 passed; 57 failed; 110 ignored; 0 measured; 0 filtered out; finished in 105.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:27
