plain
.................................................................................................... 900/12527
.................................................................................................... 1000/12527
..........................................F......................................................... 1100/12527
.................................................................................................... 1200/12527
........i......................F...............................F............................F....... 1300/12527
.................................................................................................... 1500/12527
.................................................................................................... 1600/12527
i................................................................................................... 1700/12527
.................................................................................................... 1800/12527
---
..........................................................i......................................... 7400/12527
.................................................................................................... 7500/12527
.................................................................................................... 7600/12527
............ii................i....i..ii............................................................ 7700/12527
................................................................................................FF.. 7800/12527
.............F.F.................F.....F............................................................ 7900/12527
.........F.F.F.....F.......................F........F............................F.........F......F. 8000/12527
...F.FFFFF...FF............FFF..F...........................i.ii.................................... 8100/12527
........iiii........................................................................................ 8300/12527
.............................i.....................................i................................ 8400/12527
..............................i..................................................................... 8500/12527
....................................................................................i............... 8600/12527
---
.................................................................................................... 9200/12527
.............................................................................................iiii.ii 9300/12527
iii..................................................................ii...............i............. 9400/12527
.................................................................................................... 9500/12527
.......................................................F..F......................................... 9600/12527
.......................................................................F............................ 9700/12527
..................F................................................................................. 9800/12527
.................................................................................................... 10000/12527
..............................................................................i..ii.i............... 10100/12527
.................................................................................................... 10200/12527
..i..............................................................................................iii 10300/12527
..i..............................................................................................iii 10300/12527
iii.i..iiiiii.i............................................................................F........ 10400/12527
.................................................................................................... 10500/12527
.................................................................................................... 10600/12527
.................................................................................................... 10700/12527
............F.......................F............................................................... 10800/12527
.................................................................................................... 11000/12527
.................................................................................................... 11100/12527
............................................................ii............................i......... 11200/12527
.................................................................................................... 11300/12527
.................................................................................................... 11300/12527
...............................F.......................F............................................ 11400/12527
.................................................................................................... 11600/12527
.................................................................................................... 11700/12527
....................................................i............................................... 11800/12527
.................................................................................................... 11900/12527
---

---- [ui] ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs stdout ----
diff of stderr:

1 error[E0597]: `z.1` does not live long enough
3    |
3    |
+ LL | fn cplusplus_mode_exceptionally_unsafe(x: &mut Option<&'static mut isize>) {
+    |                                        - requires that `z.1` is borrowed for `'static`
+ LL |     let mut z = (0, 0);
4 LL |     *x = Some(&mut z.1);
-    |     |         |
-    |     |         |
-    |     |         borrowed value does not live long enough
-    |     assignment requires that `z.1` is borrowed for `'static`
+    |               ^^^^^^^^ borrowed value does not live long enough
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-local-borrow-with-panic-outlives-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `z.1` does not live long enough
   |
   |
LL | fn cplusplus_mode_exceptionally_unsafe(x: &mut Option<&'static mut isize>) {
   |                                        - requires that `z.1` is borrowed for `'static`
LL |     let mut z = (0, 0);
LL |     *x = Some(&mut z.1);
   |               ^^^^^^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `z.1` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/borrowck/mut-borrow-in-loop.rs stdout ----
diff of stderr:

11    |
12 LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
13    |      -- lifetime `'a` defined here
- ...
+ LL |     fn in_loop(self, arg : &'a mut T) {
+    |                ---- requires that `*arg` is borrowed for `'a`
+ LL |         loop {
15 LL |             (self.func)(arg)
-    |             ------------^^^-
-    |             |           |
-    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |                         ^^^ `*arg` was mutably borrowed here in the previous iteration of the loop
20 
21 error[E0499]: cannot borrow `*arg` as mutable more than once at a time


24 LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
25    |      -- lifetime `'a` defined here
26 ...
+ LL |     fn in_while(self, arg : &'a mut T) {
+    |                 ---- requires that `*arg` is borrowed for `'a`
+ LL |         while true {
27 LL |             (self.func)(arg)
-    |             ------------^^^-
-    |             |           |
-    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |                         ^^^ `*arg` was mutably borrowed here in the previous iteration of the loop
32 
33 error[E0499]: cannot borrow `*arg` as mutable more than once at a time


36 LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
37    |      -- lifetime `'a` defined here
38 ...
+ LL |     fn in_for(self, arg : &'a mut T) {
+    |               ---- requires that `*arg` is borrowed for `'a`
+ ...
39 LL |             (self.func)(arg)
-    |             ------------^^^-
-    |             |           |
-    |             |           `*arg` was mutably borrowed here in the previous iteration of the loop
-    |             argument requires that `*arg` is borrowed for `'a`
+    |                         ^^^ `*arg` was mutably borrowed here in the previous iteration of the loop
45 error: aborting due to 3 previous errors; 1 warning emitted
46 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop/mut-borrow-in-loop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/mut-borrow-in-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mut-borrow-in-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-in-loop/auxiliary"
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
LL |     fn in_loop(self, arg : &'a mut T) {
   |                ---- requires that `*arg` is borrowed for `'a`
LL |         loop {
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |                         ^^^ `*arg` was mutably borrowed here in the previous iteration of the loop

error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |     fn in_while(self, arg : &'a mut T) {
   |                 ---- requires that `*arg` is borrowed for `'a`
LL |         while true { //~ WARN denote infinite loops with
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |                         ^^^ `*arg` was mutably borrowed here in the previous iteration of the loop

error[E0499]: cannot borrow `*arg` as mutable more than once at a time
   |
   |
LL | impl<'a, T : 'a> FuncWrapper<'a, T> {
   |      -- lifetime `'a` defined here
...
LL |     fn in_for(self, arg : &'a mut T) {
   |               ---- requires that `*arg` is borrowed for `'a`
...
LL |             (self.func)(arg) //~ ERROR cannot borrow
   |                         ^^^ `*arg` was mutably borrowed here in the previous iteration of the loop
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/borrowck/two-phase-surprise-no-conflict.rs stdout ----
diff of stderr:

75 LL | fn register_plugins<'a>(mk_reg: impl Fn() -> &'a mut Registry<'a>) {
76    |                     -- lifetime `'a` defined here
77 ...
+ LL |     let reg = mk_reg();
+    |               -------- assignment requires that `reg.sess_mut` is borrowed for `'a`
78 LL |     reg.register_univ(Box::new(CapturePass::new(&reg.sess_mut)));
-    |     |                 |                         |
-    |     |                 |                         |
-    |     |                 |                         immutable borrow occurs here
-    |     |                 cast requires that `reg.sess_mut` is borrowed for `'a`
+    |     |                                           |
+    |     |                                           |
+    |     |                                           immutable borrow occurs here
83    |     mutable borrow occurs here
84 
85 error[E0502]: cannot borrow `*reg` as mutable because it is also borrowed as immutable

118 LL | fn register_plugins<'a>(mk_reg: impl Fn() -> &'a mut Registry<'a>) {
119    |                     -- lifetime `'a` defined here
120 ...
+ LL |     let reg = mk_reg();
+    |               -------- assignment requires that `reg.sess_mut` is borrowed for `'a`
121 LL |     reg.register_univ(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
-    |     |                 |                             |
-    |     |                 |                             |
-    |     |                 |                             first mutable borrow occurs here
-    |     |                 cast requires that `reg.sess_mut` is borrowed for `'a`
+    |     |                                               |
+    |     |                                               |
+    |     |                                               first mutable borrow occurs here
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-surprise-no-conflict.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-surprise-no-conflict/auxiliary"
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
LL |     let reg = mk_reg();
   |               -------- assignment requires that `reg.sess_mut` is borrowed for `'a`
LL |     reg.register_univ(Box::new(CapturePass::new(&reg.sess_mut)));
   |     |                                           |
   |     |                                           |
   |     |                                           immutable borrow occurs here
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
LL |     let reg = mk_reg();
   |               -------- assignment requires that `reg.sess_mut` is borrowed for `'a`
LL |     reg.register_univ(Box::new(CapturePass::new_mut(&mut reg.sess_mut)));
   |     |                                               |
   |     |                                               |
   |     |                                               first mutable borrow occurs here
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

48    |                           has type `VaList<'1, '_>`
50 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:22:5
+   --> $DIR/variadic-ffi-4.rs:21:47
52    |
52    |
53 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
56    |                                               has type `&mut VaListImpl<'1>`
56    |                                               has type `&mut VaListImpl<'1>`
- LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'1` must outlive `'2`
+    |                                               requires that `'1` must outlive `'2`
59    |
-    = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
-    = note: the struct VaListImpl<'f> is invariant over the parameter 'f
+    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
+    = note: mutable references are invariant over their type parameter
62    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
64 error: lifetime may not live long enough

-   --> $DIR/variadic-ffi-4.rs:22:5
+   --> $DIR/variadic-ffi-4.rs:21:47
+   --> $DIR/variadic-ffi-4.rs:21:47
66    |
67 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
70    |                                               has type `&mut VaListImpl<'1>`
70    |                                               has type `&mut VaListImpl<'1>`
- LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'2` must outlive `'1`
+    |                                               requires that `'2` must outlive `'1`
73    |
-    = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
-    = note: the struct VaListImpl<'f> is invariant over the parameter 'f
+    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
+    = note: mutable references are invariant over their type parameter
76    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
78 error: lifetime may not live long enough

-   --> $DIR/variadic-ffi-4.rs:28:5
+   --> $DIR/variadic-ffi-4.rs:27:47
+   --> $DIR/variadic-ffi-4.rs:27:47
80    |
81 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
84    |                                               has type `&mut VaListImpl<'1>`
84    |                                               has type `&mut VaListImpl<'1>`
- LL |     ap0 = &mut ap1;
-    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
+    |                                               requires that `'1` must outlive `'2`
87    |
88    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
89    = note: mutable references are invariant over their type parameter

90    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
92 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:28:5
+   --> $DIR/variadic-ffi-4.rs:27:47
94    |
94    |
95 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
98    |                                               has type `&mut VaListImpl<'1>`
98    |                                               has type `&mut VaListImpl<'1>`
- LL |     ap0 = &mut ap1;
-    |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
+    |                                               requires that `'2` must outlive `'1`
101    |
102    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
103    = note: mutable references are invariant over their type parameter

118    | - `ap1` dropped here while still borrowed
120 error: lifetime may not live long enough
-   --> $DIR/variadic-ffi-4.rs:35:12
+   --> $DIR/variadic-ffi-4.rs:34:47
122    |
122    |
123 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
126    |                                               has type `&mut VaListImpl<'1>`
126    |                                               has type `&mut VaListImpl<'1>`
- LL |     *ap0 = ap1.clone();
-    |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                                               requires that `'1` must outlive `'2`
129    |
-    = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
-    = note: the struct VaListImpl<'f> is invariant over the parameter 'f
+    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
+    = note: mutable references are invariant over their type parameter
132    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
134 error: lifetime may not live long enough

-   --> $DIR/variadic-ffi-4.rs:35:12
+   --> $DIR/variadic-ffi-4.rs:34:47
+   --> $DIR/variadic-ffi-4.rs:34:47
136    |
137 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               ^^^^^^^                   ------- has type `VaListImpl<'2>`
140    |                                               has type `&mut VaListImpl<'1>`
140    |                                               has type `&mut VaListImpl<'1>`
---
diff of stderr:

8   --> $DIR/mut_ref_in_final.rs:16:40
9    |
10 LL | const B3: Option<&mut i32> = Some(&mut 42);
-    |                              ----------^^-
-    |                              |         | temporary value is freed at the end of this statement
-    |                              |         creates a temporary which is freed while still in use
-    |                              |         creates a temporary which is freed while still in use
-    |                              using this value as a constant requires that borrow lasts for `'static`
+    |           ----------------             ^^- temporary value is freed at the end of this statement
+    |           |                            creates a temporary which is freed while still in use
+    |           |                            creates a temporary which is freed while still in use
+    |           requires that borrow lasts for `'static`
17 error[E0716]: temporary value dropped while borrowed
18   --> $DIR/mut_ref_in_final.rs:19:42

19    |
19    |
20 LL | const B4: Option<&mut i32> = helper(&mut 42);
-    |                              ------------^^-
-    |                              |           | temporary value is freed at the end of this statement
-    |                              |           creates a temporary which is freed while still in use
-    |                              |           creates a temporary which is freed while still in use
-    |                              using this value as a constant requires that borrow lasts for `'static`
+    |           ----------------               ^^- temporary value is freed at the end of this statement
+    |           |                              creates a temporary which is freed while still in use
+    |           |                              creates a temporary which is freed while still in use
+    |           requires that borrow lasts for `'static`
27 error[E0716]: temporary value dropped while borrowed
28   --> $DIR/mut_ref_in_final.rs:34:65

29    |
29    |
30 LL | const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
-    |                                  -------------------------------^^--
-    |                                  |                              |  temporary value is freed at the end of this statement
-    |                                  |                              creates a temporary which is freed while still in use
-    |                                  |                              creates a temporary which is freed while still in use
-    |                                  using this value as a constant requires that borrow lasts for `'static`
+    |            -------------------                                  ^^ - temporary value is freed at the end of this statement
+    |            |                                                    creates a temporary which is freed while still in use
+    |            |                                                    creates a temporary which is freed while still in use
+    |            requires that borrow lasts for `'static`
37 error[E0716]: temporary value dropped while borrowed
38   --> $DIR/mut_ref_in_final.rs:37:67

39    |
39    |
40 LL | static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
-    |                                    -------------------------------^^--
-    |                                    |                              |  temporary value is freed at the end of this statement
-    |                                    |                              creates a temporary which is freed while still in use
-    |                                    |                              creates a temporary which is freed while still in use
-    |                                    using this value as a static requires that borrow lasts for `'static`
+    |              -------------------                                  ^^ - temporary value is freed at the end of this statement
+    |              |                                                    creates a temporary which is freed while still in use
+    |              |                                                    creates a temporary which is freed while still in use
+    |              requires that borrow lasts for `'static`
47 error[E0716]: temporary value dropped while borrowed
48   --> $DIR/mut_ref_in_final.rs:40:71

49    |
49    |
50 LL | static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
-    |                                        -------------------------------^^--
-    |                                        |                              |  temporary value is freed at the end of this statement
-    |                                        |                              creates a temporary which is freed while still in use
-    |                                        |                              creates a temporary which is freed while still in use
-    |                                        using this value as a static requires that borrow lasts for `'static`
+    |                  -------------------                                  ^^ - temporary value is freed at the end of this statement
+    |                  |                                                    creates a temporary which is freed while still in use
+    |                  |                                                    creates a temporary which is freed while still in use
+    |                  requires that borrow lasts for `'static`
57 error: aborting due to 6 previous errors
58 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/mut_ref_in_final.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-mut-refs/mut_ref_in_final.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0764]: mutable references are not allowed in the final value of constants
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:10:21
   |
LL | const B: *mut i32 = &mut 4; //~ ERROR mutable references are not allowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:16:40
   |
   |
LL | const B3: Option<&mut i32> = Some(&mut 42); //~ ERROR temporary value dropped while borrowed
   |           ----------------             ^^- temporary value is freed at the end of this statement
   |           |                            creates a temporary which is freed while still in use
   |           |                            creates a temporary which is freed while still in use
   |           requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:19:42
   |
   |
LL | const B4: Option<&mut i32> = helper(&mut 42); //~ ERROR temporary value dropped while borrowed
   |           ----------------               ^^- temporary value is freed at the end of this statement
   |           |                              creates a temporary which is freed while still in use
   |           |                              creates a temporary which is freed while still in use
   |           requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:34:65
   |
   |
LL | const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |            -------------------                                  ^^ - temporary value is freed at the end of this statement
   |            |                                                    creates a temporary which is freed while still in use
   |            |                                                    creates a temporary which is freed while still in use
   |            requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:37:67
   |
   |
LL | static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |              -------------------                                  ^^ - temporary value is freed at the end of this statement
   |              |                                                    creates a temporary which is freed while still in use
   |              |                                                    creates a temporary which is freed while still in use
   |              requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:40:71
   |
   |
LL | static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                  -------------------                                  ^^ - temporary value is freed at the end of this statement
   |                  |                                                    creates a temporary which is freed while still in use
   |                  |                                                    creates a temporary which is freed while still in use
   |                  requires that borrow lasts for `'static`
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
-    |                                 |     creates a temporary which is freed while still in use
-    |                                 |     creates a temporary which is freed while still in use
-    |                                 using this value as a constant requires that borrow lasts for `'static`
+    |            ------------------         ^^^^^^^^^- temporary value is freed at the end of this statement
+    |            |                          creates a temporary which is freed while still in use
+    |            |                          creates a temporary which is freed while still in use
+    |            requires that borrow lasts for `'static`
11 error[E0716]: temporary value dropped while borrowed
12   --> $DIR/issue-54224.rs:9:57

13    |
13    |
14 LL | pub const Z: Cow<'static, [ [u8; 3] ]> = Cow::Borrowed(&[*b"ABC"]);
-    |                                          |              |        |
-    |                                          |              |        temporary value is freed at the end of this statement
-    |                                          |              creates a temporary which is freed while still in use
-    |                                          |              creates a temporary which is freed while still in use
-    |                                          using this value as a constant requires that borrow lasts for `'static`
+    |              -------------------------                  ^^^^^^^^^- temporary value is freed at the end of this statement
+    |              |                                          creates a temporary which is freed while still in use
+    |              |                                          creates a temporary which is freed while still in use
+    |              requires that borrow lasts for `'static`
21 error: aborting due to 2 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/issue-54224.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-54224.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-54224.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/issue-54224.rs:1:39
   |
LL | const FOO: Option<&[[u8; 3]]> = Some(&[*b"foo"]); //~ ERROR temporary value dropped while borrowed
   |            ------------------         ^^^^^^^^^- temporary value is freed at the end of this statement
   |            |                          creates a temporary which is freed while still in use
   |            |                          creates a temporary which is freed while still in use
   |            requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/issue-54224.rs:9:57
   |
   |
LL | pub const Z: Cow<'static, [ [u8; 3] ]> = Cow::Borrowed(&[*b"ABC"]);
   |              -------------------------                  ^^^^^^^^^- temporary value is freed at the end of this statement
   |              |                                          creates a temporary which is freed while still in use
   |              |                                          creates a temporary which is freed while still in use
   |              requires that borrow lasts for `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.

---
diff of stderr:

2   --> $DIR/promote-not.rs:8:50
3    |
4 LL | static mut TEST1: Option<&mut [i32]> = Some(&mut [1, 2, 3]);
-    |                                        |         |        |
-    |                                        |         |        temporary value is freed at the end of this statement
-    |                                        |         creates a temporary which is freed while still in use
-    |                                        |         creates a temporary which is freed while still in use
-    |                                        using this value as a static requires that borrow lasts for `'static`
+    |                   ------------------             ^^^^^^^^^- temporary value is freed at the end of this statement
+    |                   |                              creates a temporary which is freed while still in use
+    |                   |                              creates a temporary which is freed while still in use
+    |                   requires that borrow lasts for `'static`
11 error[E0716]: temporary value dropped while borrowed
12   --> $DIR/promote-not.rs:11:18

13    |
13    |
+ LL | static mut TEST2: &'static mut [i32] = {
+    |                   ------------------ requires that borrow lasts for `'static`
14 LL |     let x = &mut [1,2,3];
15    |                  ^^^^^^^ creates a temporary which is freed while still in use


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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote-not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:8:50
   |
LL | static mut TEST1: Option<&mut [i32]> = Some(&mut [1, 2, 3]); //~ ERROR temporary value dropped while borrowed
   |                   ------------------             ^^^^^^^^^- temporary value is freed at the end of this statement
   |                   |                              creates a temporary which is freed while still in use
   |                   |                              creates a temporary which is freed while still in use
   |                   requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:11:18
   |
   |
LL | static mut TEST2: &'static mut [i32] = {
   |                   ------------------ requires that borrow lasts for `'static`
LL |     let x = &mut [1,2,3]; //~ ERROR temporary value dropped while borrowed
   |                  ^^^^^^^ creates a temporary which is freed while still in use
LL | };
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:20:32
   |
LL |         let _x: &'static () = &foo(); //~ ERROR temporary value dropped while borrowed
   |                 -----------    ^^^^^ creates a temporary which is freed while still in use
   |                 |
   |                 type annotation requires that borrow lasts for `'static`
   |     - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:28:29
  --> /checkout/src/test/ui/consts/promote-not.rs:28:29
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |             |
   |             type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:33:29
   |
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |             |
   |             type annotation requires that borrow lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:39:29
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:46:29
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).0; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:47:29
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:50:29
   |
LL |     let _val: &'static _ = &(1/0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:51:29
   |
LL |     let _val: &'static _ = &(1/(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:52:29
   |
LL |     let _val: &'static _ = &(1%0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:53:29
   |
LL |     let _val: &'static _ = &(1%(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:54:29
   |
LL |     let _val: &'static _ = &([1,2,3][4]+1); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:57:29
   |
LL |     let _val: &'static _ = &TEST_DROP;
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:59:29
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------    ^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:59:30
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------     ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:62:29
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------    ^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:62:31
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------      ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:65:29
   |
LL |     let _val: &'static _ = &[&TEST_DROP; 1];
   |               ----------    ^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:65:31
   |
LL |     let _val: &'static _ = &[&TEST_DROP; 1];
   |               |               |
   |               |               creates a temporary which is freed while still in use
   |               |               creates a temporary which is freed while still in use
   |               type annotation requires that borrow lasts for `'static`
error: aborting due to 20 previous errors

For more information about this error, try `rustc --explain E0716`.

---
diff of stderr:

2   --> $DIR/dropck_trait_cycle_checked.rs:111:13
3    |
4 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                     -------- cast requires that `o2` is borrowed for `'static`
+    |          -- assignment requires that `o2` is borrowed for `'static`
6 LL |     o1.set0(&o2);
7    |             ^^^ borrowed value does not live long enough

13   --> $DIR/dropck_trait_cycle_checked.rs:112:13
14    |
14    |
15 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                     -------- cast requires that `o3` is borrowed for `'static`
+    |          -- assignment requires that `o3` is borrowed for `'static`
17 LL |     o1.set0(&o2);
18 LL |     o1.set1(&o3);
19    |             ^^^ borrowed value does not live long enough
25   --> $DIR/dropck_trait_cycle_checked.rs:113:13
26    |
26    |
27 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                               -------- cast requires that `o2` is borrowed for `'static`
+    |              -- assignment requires that `o2` is borrowed for `'static`
29 ...
30 LL |     o2.set0(&o2);
31    |             ^^^ borrowed value does not live long enough
37   --> $DIR/dropck_trait_cycle_checked.rs:114:13
38    |
38    |
39 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                               -------- cast requires that `o3` is borrowed for `'static`
+    |              -- assignment requires that `o3` is borrowed for `'static`
41 ...
42 LL |     o2.set1(&o3);
43    |             ^^^ borrowed value does not live long enough
49   --> $DIR/dropck_trait_cycle_checked.rs:115:13
50    |
50    |
51 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                                         -------- cast requires that `o1` is borrowed for `'static`
+    |                  -- assignment requires that `o1` is borrowed for `'static`
53 ...
54 LL |     o3.set0(&o1);
55    |             ^^^ borrowed value does not live long enough
61   --> $DIR/dropck_trait_cycle_checked.rs:116:13
62    |
62    |
63 LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
-    |                                                                                         -------- cast requires that `o2` is borrowed for `'static`
+    |                  -- assignment requires that `o2` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dropck/dropck_trait_cycle_checked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `o2` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:111:13
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |          -- assignment requires that `o2` is borrowed for `'static`
LL |     o1.set0(&o2); //~ ERROR `o2` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o2` dropped here while still borrowed

error[E0597]: `o3` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:112:13
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |          -- assignment requires that `o3` is borrowed for `'static`
LL |     o1.set0(&o2); //~ ERROR `o2` does not live long enough
LL |     o1.set1(&o3); //~ ERROR `o3` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o3` dropped here while still borrowed

error[E0597]: `o2` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:113:13
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |              -- assignment requires that `o2` is borrowed for `'static`
...
LL |     o2.set0(&o2); //~ ERROR `o2` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o2` dropped here while still borrowed

error[E0597]: `o3` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:114:13
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |              -- assignment requires that `o3` is borrowed for `'static`
...
LL |     o2.set1(&o3); //~ ERROR `o3` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
LL | }
   | - `o3` dropped here while still borrowed

error[E0597]: `o1` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:115:13
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                  -- assignment requires that `o1` is borrowed for `'static`
...
LL |     o3.set0(&o1); //~ ERROR `o1` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL |     o3.set1(&o2); //~ ERROR `o2` does not live long enough
LL | }
   | - `o1` dropped here while still borrowed

error[E0597]: `o2` does not live long enough
  --> /checkout/src/test/ui/dropck/dropck_trait_cycle_checked.rs:116:13
   |
LL |     let (o1, o2, o3): (Box<dyn Obj>, Box<dyn Obj>, Box<dyn Obj>) = (O::new(), O::new(), O::new());
   |                  -- assignment requires that `o2` is borrowed for `'static`
...
LL |     o3.set1(&o2); //~ ERROR `o2` does not live long enough
   |             ^^^ borrowed value does not live long enough
LL | }
   | - `o2` dropped here while still borrowed
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/issues/issue-18118.rs stdout ----
diff of stderr:

1 error[E0597]: `p` does not live long enough
3    |
3    |
+ LL |     const z: &'static isize = {
+    |              -------------- requires that `p` is borrowed for `'static`
+ LL |         let p = 3;
4 LL |         &p
-    |         |
-    |         |
-    |         borrowed value does not live long enough
-    |         using this value as a constant requires that `p` is borrowed for `'static`
+    |         ^^ borrowed value does not live long enough
9 LL |     };
10    |     - `p` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/issue-18118.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/issue-18118.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18118.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18118.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18118/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `p` does not live long enough
   |
   |
LL |     const z: &'static isize = {
   |              -------------- requires that `p` is borrowed for `'static`
LL |         let p = 3;
LL |         &p //~ ERROR `p` does not live long enough
   |         ^^ borrowed value does not live long enough
LL |     };
   |     - `p` dropped here while still borrowed
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
54    |
-    = note: requirement occurs because of the type Cell<&'_#10r u32>, which makes the generic argument &'_#10r u32 invariant
+    = note: requirement occurs because of the type Cell<&'_#1r u32>, which makes the generic argument &'_#1r u32 invariant
56    = note: the struct Cell<T> is invariant over the parameter T
57    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

59 error: aborting due to previous error
60 
- For more information about this error, try `rustc --explain E0521`.
---
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-no-bound/auxiliary"
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
   |
   = note: requirement occurs because of the type Cell<&'_#1r u32>, which makes the generic argument &'_#1r u32 invariant
   = note: the struct Cell<T> is invariant over the parameter T
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error


------------------------------------------
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
54    |
-    = note: requirement occurs because of the type Cell<&'_#11r u32>, which makes the generic argument &'_#11r u32 invariant
+    = note: requirement occurs because of the type Cell<&'_#1r u32>, which makes the generic argument &'_#1r u32 invariant
56    = note: the struct Cell<T> is invariant over the parameter T
57    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

59 error: aborting due to previous error
60 
- For more information about this error, try `rustc --explain E0521`.
---
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-shorter-to-static-wrong-bound/auxiliary"
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
   |
   = note: requirement occurs because of the type Cell<&'_#1r u32>, which makes the generic argument &'_#1r u32 invariant
   = note: the struct Cell<T> is invariant over the parameter T
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
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
+    |             -----^^---
+    |             |    |
+    |             |    borrowed value does not live long enough
+    |             assignment requires that `s` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy-proj/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |     let a = (Foo(&s),); //~ ERROR `s` does not live long enough [E0597]
   |             -----^^---
   |             |    |
   |             |    borrowed value does not live long enough
   |             assignment requires that `s` is borrowed for `'static`
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
+    |             ----^^-
+    |             |   |
+    |             |   borrowed value does not live long enough
+    |             requires that `s` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/do-not-ignore-lifetime-bounds-in-copy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |     let a = Foo(&s); //~ ERROR `s` does not live long enough [E0597]
   |             ----^^-
   |             |   |
   |             |   borrowed value does not live long enough
   |             requires that `s` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-42574-diagnostic-in-nested-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure/auxiliary"
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

2   --> $DIR/issue-46036.rs:8:24
3    |
4 LL |     let foo = Foo { x: &a };
-    |                        |
-    |                        |
-    |                        borrowed value does not live long enough
-    |                        this usage requires that `a` is borrowed for `'static`
+    |               ---------^^--
+    |               |        |
+    |               |        borrowed value does not live long enough
+    |               requires that `a` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-46036.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-46036" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-46036/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     let foo = Foo { x: &a }; //~ ERROR E0597
   |               ---------^^--
   |               |        |
   |               |        borrowed value does not live long enough
   |               requires that `a` is borrowed for `'static`
LL |     loop { }
LL | }
   | - `a` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

14   --> $DIR/issue-62007-assign-const-index.rs:24:26
15    |
16 LL | fn to_refs<T>(mut list: [&mut List<T>; 2]) -> Vec<&mut T> {
-    |                          - let's call the lifetime of this reference `'1`
+    |               --------   - let's call the lifetime of this reference `'1`
+    |               |
+    |               requires that `list[_].next` is borrowed for `'1`
18 ...
19 LL |         if let Some(n) = list[0].next.as_mut() {
-    |                          |
-    |                          |
-    |                          `list[_].next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list[_].next` is borrowed for `'1`
+    |                          ^^^^^^^^^^^^^^^^^^^^^ `list[_].next` was mutably borrowed here in the previous iteration of the loop
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index/issue-62007-assign-const-index.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-62007-assign-const-index.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-62007-assign-const-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-const-index/auxiliary"
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
   |               --------   - let's call the lifetime of this reference `'1`
   |               |
   |               requires that `list[_].next` is borrowed for `'1`
...
LL |         if let Some(n) = list[0].next.as_mut() { //~ ERROR cannot borrow `list[_].next` as mutable
   |                          ^^^^^^^^^^^^^^^^^^^^^ `list[_].next` was mutably borrowed here in the previous iteration of the loop
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
+    |     --- ^^ borrowed value does not live long enough
+    |     |
+    |     requires that `n` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-69114-static-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-ty/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `n` does not live long enough
   |
LL |     FOO(&n);
LL |     FOO(&n);
   |     --- ^^ borrowed value does not live long enough
   |     |
   |     requires that `n` is borrowed for `'static`
LL |     //~^ ERROR does not live long enough
LL | }
   | - `n` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

14   --> $DIR/issue-62007-assign-differing-fields.rs:17:26
15    |
16 LL | fn to_refs<'a, T>(mut list: (&'a mut List<T>, &'a mut List<T>)) -> Vec<&'a mut T> {
-    |            -- lifetime `'a` defined here
+    |            --     -------- requires that `list.0.next` is borrowed for `'a`
+    |            |
+    |            lifetime `'a` defined here
18 ...
19 LL |         if let Some(n) = (list.0).next.as_mut() {
-    |                          |
-    |                          |
-    |                          `list.0.next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list.0.next` is borrowed for `'a`
+    |                          ^^^^^^^^^^^^^^^^^^^^^^ `list.0.next` was mutably borrowed here in the previous iteration of the loop
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-differing-fields/issue-62007-assign-differing-fields.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-62007-assign-differing-fields.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-62007-assign-differing-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-differing-fields" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-62007-assign-differing-fields/auxiliary"
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
   |            --     -------- requires that `list.0.next` is borrowed for `'a`
   |            |
   |            lifetime `'a` defined here
...
LL |         if let Some(n) = (list.0).next.as_mut() { //~ ERROR cannot borrow `list.0.next` as mutable
   |                          ^^^^^^^^^^^^^^^^^^^^^^ `list.0.next` was mutably borrowed here in the previous iteration of the loop
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0499`.

---
3    |
4 LL |         BAR = &n;
-    |         ------^^
-    |         |     |
-    |         |     borrowed value does not live long enough
-    |         assignment requires that `n` is borrowed for `'static`
+    |         ---   ^^ borrowed value does not live long enough
+    |         |
+    |         requires that `n` is borrowed for `'static`
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
+    |         ----------   ^^ borrowed value does not live long enough
+    |         |
+    |         requires that `n` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-69114-static-mut-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-mut-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-69114-static-mut-ty/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `n` does not live long enough
   |
LL |         BAR = &n;
LL |         BAR = &n;
   |         ---   ^^ borrowed value does not live long enough
   |         |
   |         requires that `n` is borrowed for `'static`
LL | }
LL | }
   | - `n` dropped here while still borrowed

error[E0597]: `n` does not live long enough
   |
   |
LL |         BAR_ELIDED = &n;
   |         ----------   ^^ borrowed value does not live long enough
   |         |
   |         requires that `n` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/auxiliary"
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

15    |
16 LL | fn assignment_to_field_projection<'a, T>(
17    |                                   -- lifetime `'a` defined here
+ LL |     mut list: (&'a mut List<T>, &'a mut List<T>),
+    |     -------- requires that `list.0.next` is borrowed for `'a`
18 ...
19 LL |         if let Some(n) = (list.0).next.as_mut() {
-    |                          |
-    |                          |
-    |                          `list.0.next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list.0.next` is borrowed for `'a`
+    |                          ^^^^^^^^^^^^^^^^^^^^^^ `list.0.next` was mutably borrowed here in the previous iteration of the loop
24 
25 error[E0499]: cannot borrow `list.0.0.0.0.0.value` as mutable more than once at a time

39    |
39    |
40 LL | fn assignment_through_projection_chain<'a, T>(
41    |                                        -- lifetime `'a` defined here
+ LL |     mut list: (((((Box<&'a mut List<T>>, Box<&'a mut List<T>>),),),),),
+    |     -------- requires that `list.0.0.0.0.0.next` is borrowed for `'a`
42 ...
43 LL |         if let Some(n) = ((((list.0).0).0).0).0.next.as_mut() {
-    |                          |
-    |                          |
-    |                          `list.0.0.0.0.0.next` was mutably borrowed here in the previous iteration of the loop
-    |                          argument requires that `list.0.0.0.0.0.next` is borrowed for `'a`
+    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `list.0.0.0.0.0.next` was mutably borrowed here in the previous iteration of the loop
49 error: aborting due to 4 previous errors
50 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/assignment-to-differing-field/assignment-to-differing-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/polonius/assignment-to-differing-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/polonius/assignment-to-differing-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/assignment-to-differing-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "polonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/assignment-to-differing-field/auxiliary"
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
LL |     mut list: (&'a mut List<T>, &'a mut List<T>),
   |     -------- requires that `list.0.next` is borrowed for `'a`
...
LL |         if let Some(n) = (list.0).next.as_mut() {
   |                          ^^^^^^^^^^^^^^^^^^^^^^ `list.0.next` was mutably borrowed here in the previous iteration of the loop

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
LL |     mut list: (((((Box<&'a mut List<T>>, Box<&'a mut List<T>>),),),),),
   |     -------- requires that `list.0.0.0.0.0.next` is borrowed for `'a`
...
LL |         if let Some(n) = ((((list.0).0).0).0).0.next.as_mut() {
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `list.0.0.0.0.0.next` was mutably borrowed here in the previous iteration of the loop
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/nll/ty-outlives/projection-one-region-closure.rs stdout ----
diff of stderr:

28    |
29    = note: defining type: no_relationships_late::<'_#1r, T>
+ error: lifetime may not live long enough
+   --> $DIR/projection-one-region-closure.rs:41:37
+    |
+    |
+ LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
+    |                          --  --     ^^^^ requires that `'b` must outlive `'a`
+    |                          |   |
+    |                          |   lifetime `'b` defined here
+    |                          lifetime `'a` defined here
+    |
+    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of the type Cell<&'_#2r ()>, which makes the generic argument &'_#2r () invariant
+    = note: the struct Cell<T> is invariant over the parameter T
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
31 error[E0309]: the parameter type `T` may not live long enough
32   --> $DIR/projection-one-region-closure.rs:45:29
33    |


36    |
37    = help: consider adding an explicit lifetime bound `T: 'a`...
- error: lifetime may not live long enough
-   --> $DIR/projection-one-region-closure.rs:45:39
-    |
-    |
- LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
-    |                          --  -- lifetime `'b` defined here
-    |                          |
-    |                          lifetime `'a` defined here
- ...
- LL |     with_signature(cell, t, |cell, t| require(cell, t));
-    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
-    |
-    = help: consider adding the following bound: `'b: 'a`
52 note: external requirements
53   --> $DIR/projection-one-region-closure.rs:56:29
54    |


78    |
79    = note: defining type: no_relationships_early::<'_#1r, '_#2r, T>
+ error: lifetime may not live long enough
+   --> $DIR/projection-one-region-closure.rs:51:38
+    |
+    |
+ LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
+    |                           --  --     ^^^^ requires that `'b` must outlive `'a`
+    |                           |   |
+    |                           |   lifetime `'b` defined here
+    |                           lifetime `'a` defined here
+    |
+    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of the type Cell<&'_#1r ()>, which makes the generic argument &'_#1r () invariant
+    = note: the struct Cell<T> is invariant over the parameter T
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
81 error[E0309]: the parameter type `T` may not live long enough
82   --> $DIR/projection-one-region-closure.rs:56:29
83    |


85    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^
86    |
87    = help: consider adding an explicit lifetime bound `T: 'a`...
- error: lifetime may not live long enough
-   --> $DIR/projection-one-region-closure.rs:56:39
-    |
-    |
- LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
-    |                           --  -- lifetime `'b` defined here
-    |                           |
-    |                           lifetime `'a` defined here
- ...
- LL |     with_signature(cell, t, |cell, t| require(cell, t));
-    |                                       ^^^^^^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
-    |
-    = help: consider adding the following bound: `'b: 'a`
102 note: external requirements
103   --> $DIR/projection-one-region-closure.rs:70:29



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/projection-one-region-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/projection-one-region-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/projection-one-region-closure/auxiliary"
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
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:41:37
   |
   |
LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                          --  --     ^^^^ requires that `'b` must outlive `'a`
   |                          |   |
   |                          |   lifetime `'b` defined here
   |                          lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type Cell<&'_#2r ()>, which makes the generic argument &'_#2r () invariant
   = note: the struct Cell<T> is invariant over the parameter T
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:45:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
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
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:51:38
   |
   |
LL | fn no_relationships_early<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
   |                           --  --     ^^^^ requires that `'b` must outlive `'a`
   |                           |   |
   |                           |   lifetime `'b` defined here
   |                           lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type Cell<&'_#1r ()>, which makes the generic argument &'_#1r () invariant
   = note: the struct Cell<T> is invariant over the parameter T
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/ty-outlives/projection-one-region-closure.rs:56:29
   |
   |
LL |     with_signature(cell, t, |cell, t| require(cell, t));
   |
   |
   = help: consider adding an explicit lifetime bound `T: 'a`...
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
+   --> $DIR/projection-one-region-trait-bound-closure.rs:33:37
32    |
32    |
33 LL | fn no_relationships_late<'a, 'b, T>(cell: Cell<&'a ()>, t: T)
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
+   --> $DIR/type-check-pointer-comparisons.rs:5:26
18    |
18    |
19 LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
-    |                  --  -- lifetime `'b` defined here
-    |                  |
+    |                  --  --  ^ requires that `'b` must outlive `'a`
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
+   --> $DIR/type-check-pointer-comparisons.rs:11:24
50    |
50    |
51 LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
-    |                --  -- lifetime `'b` defined here
-    |                |
+    |                --  --  ^ requires that `'b` must outlive `'a`
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
+   --> $DIR/type-check-pointer-comparisons.rs:17:31
82    |
82    |
83 LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
-    |                   --  -- lifetime `'b` defined here
-    |                   |
+    |                   --  --      ^ requires that `'b` must outlive `'a`
+    |                   |   |
+    |                   |   lifetime `'b` defined here
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/type-check-pointer-comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/auxiliary"
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
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:5:26
   |
   |
LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
   |                  --  --  ^ requires that `'b` must outlive `'a`
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
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:11:24
   |
   |
LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
   |                --  --  ^ requires that `'b` must outlive `'a`
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
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:17:31
   |
   |
LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
   |                   --  --      ^ requires that `'b` must outlive `'a`
   |                   |   |
   |                   |   lifetime `'b` defined here
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


---- [ui] ui/nll/user-annotations/adt-brace-enums.rs stdout ----
diff of stderr:

2   --> $DIR/adt-brace-enums.rs:25:48
3    |
4 LL |     SomeEnum::SomeVariant::<&'static u32> { t: &c };
-    |                                                |
-    |                                                |
-    |                                                borrowed value does not live long enough
-    |                                                this usage requires that `c` is borrowed for `'static`
+    |     -------------------------------------------^^--
+    |     |                                          |
+    |     |                                          borrowed value does not live long enough
+    |     requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/adt-brace-enums.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/adt-brace-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-brace-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-brace-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-brace-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeEnum::SomeVariant::<&'static u32> { t: &c }; //~ ERROR
   |     -------------------------------------------^^--
   |     |                                          |
   |     |                                          borrowed value does not live long enough
   |     requires that `c` is borrowed for `'static`
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


------------------------------------------


---- [ui] ui/nll/user-annotations/adt-nullary-enums.rs stdout ----
diff of stderr:

1 error[E0597]: `c` does not live long enough
2   --> $DIR/adt-nullary-enums.rs:33:41
3    |
- LL |         SomeEnum::SomeVariant(Cell::new(&c)),
-    |                               ----------^^-
-    |                               |         |
-    |                               |         borrowed value does not live long enough
-    |                               argument requires that `c` is borrowed for `'static`
- LL | }
- LL | }
-    | - `c` dropped here while still borrowed
+ LL | /     combine(
+ LL | |         SomeEnum::SomeVariant(Cell::new(&c)),
+    | |                                         ^^ borrowed value does not live long enough
+ LL | |         SomeEnum::SomeOtherVariant::<Cell<&'static u32>>,
+ LL | |     );
+    | |_____- argument requires that `c` is borrowed for `'static`
+ LL |   }
+    |   - `c` dropped here while still borrowed
12 
13 error[E0597]: `c` does not live long enough
14   --> $DIR/adt-nullary-enums.rs:41:41

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/adt-nullary-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-nullary-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-nullary-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-nullary-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
LL | /     combine(
LL | /     combine(
LL | |         SomeEnum::SomeVariant(Cell::new(&c)), //~ ERROR
   | |                                         ^^ borrowed value does not live long enough
LL | |         SomeEnum::SomeOtherVariant::<Cell<&'static u32>>,
LL | |     );
   | |_____- argument requires that `c` is borrowed for `'static`
LL |   }
   |   - `c` dropped here while still borrowed

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

2   --> $DIR/adt-tuple-enums.rs:28:43
3    |
4 LL |     SomeEnum::SomeVariant::<&'static u32>(&c);
-    |                                           |
-    |                                           |
-    |                                           borrowed value does not live long enough
-    |                                           this usage requires that `c` is borrowed for `'static`
+    |     --------------------------------------^^-
+    |     |                                     |
+    |     |                                     borrowed value does not live long enough
+    |     requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/adt-tuple-enums.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/adt-tuple-enums.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-enums.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeEnum::SomeVariant::<&'static u32>(&c); //~ ERROR
   |     --------------------------------------^^-
   |     |                                     |
   |     |                                     borrowed value does not live long enough
   |     requires that `c` is borrowed for `'static`
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

---
diff of stderr:

2   --> $DIR/adt-tuple-struct.rs:23:32
3    |
4 LL |     SomeStruct::<&'static u32>(&c);
-    |                                |
-    |                                |
-    |                                borrowed value does not live long enough
-    |                                this usage requires that `c` is borrowed for `'static`
+    |     ---------------------------^^-
+    |     |                          |
+    |     |                          borrowed value does not live long enough
+    |     requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/adt-tuple-struct.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/adt-tuple-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     SomeStruct::<&'static u32>(&c); //~ ERROR
   |     ---------------------------^^-
   |     |                          |
   |     |                          borrowed value does not live long enough
   |     requires that `c` is borrowed for `'static`
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


------------------------------------------


---- [ui] ui/nll/user-annotations/adt-tuple-struct-calls.rs stdout ----
diff of stderr:

1 error[E0597]: `c` does not live long enough
2   --> $DIR/adt-tuple-struct-calls.rs:27:7
3    |
+ LL |     let f = SomeStruct::<&'static u32>;
+    |             -------------------------- assignment requires that `c` is borrowed for `'static`
4 LL |     f(&c);
-    |     --^^-
-    |     | |
-    |     | borrowed value does not live long enough
-    |     argument requires that `c` is borrowed for `'static`
+    |       ^^ borrowed value does not live long enough
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls/adt-tuple-struct-calls.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls/adt-tuple-struct-calls.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/adt-tuple-struct-calls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/adt-tuple-struct-calls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/adt-tuple-struct-calls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     let f = SomeStruct::<&'static u32>;
   |             -------------------------- assignment requires that `c` is borrowed for `'static`
LL |     f(&c); //~ ERROR
   |       ^^ borrowed value does not live long enough
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/closure-substs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/closure-substs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/closure-substs/auxiliary"
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


---- [ui] ui/nll/user-annotations/constant-in-expr-inherent-2.rs stdout ----
diff of stderr:

26   --> $DIR/constant-in-expr-inherent-2.rs:25:28
27    |
28 LL |     B::ALSO_ASSOCIATED_FUN(&x);
-    |     -----------------------^^-
-    |     |                      |
-    |     |                      borrowed value does not live long enough
-    |     argument requires that `x` is borrowed for `'static`
+    |     ---------------------- ^^ borrowed value does not live long enough
+    |     |
+    |     requires that `x` is borrowed for `'static`
33 LL |     <_>::TRAIT_ASSOCIATED_FUN(&x);
34 LL | }
35    | - `x` dropped here while still borrowed

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-2/constant-in-expr-inherent-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-inherent-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-inherent-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-inherent-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     FUN(&x);                        //~ ERROR `x` does not live long enough
   |     ----^^-
   |     |   |
   |     |   borrowed value does not live long enough
   |     argument requires that `x` is borrowed for `'static`
LL | }
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     A::ASSOCIATED_FUN(&x);          //~ ERROR `x` does not live long enough
   |     ------------------^^-
   |     |                 |
   |     |                 borrowed value does not live long enough
   |     argument requires that `x` is borrowed for `'static`
LL | }
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     B::ALSO_ASSOCIATED_FUN(&x);     //~ ERROR `x` does not live long enough
   |     ---------------------- ^^ borrowed value does not live long enough
   |     |
   |     requires that `x` is borrowed for `'static`
LL |     <_>::TRAIT_ASSOCIATED_FUN(&x);  //~ ERROR `x` does not live long enough
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     <_>::TRAIT_ASSOCIATED_FUN(&x);  //~ ERROR `x` does not live long enough
   |     --------------------------^^-
   |     |                         |
   |     |                         borrowed value does not live long enough
   |     argument requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed
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
+    |     ----------------------- ^^ borrowed value does not live long enough
+    |     |
+    |     requires that `c` is borrowed for `'static`
9 LL | }
10    | - `c` dropped here while still borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns/fns.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns/fns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/fns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/fns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     some_fn::<&'static u32>(&c); //~ ERROR
   |     ----------------------- ^^ borrowed value does not live long enough
   |     |
   |     requires that `c` is borrowed for `'static`
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


------------------------------------------


---- [ui] ui/nll/user-annotations/method-ufcs-1.rs stdout ----
diff of stderr:

1 error[E0597]: `a` does not live long enough
3    |
3    |
+ LL |     let x = <&'static u32 as Bazoom<_>>::method;
+    |             ----------------------------------- assignment requires that `a` is borrowed for `'static`
4 LL |     x(&a, b, c);
-    |     --^^-------
-    |     | |
-    |     | borrowed value does not live long enough
-    |     argument requires that `a` is borrowed for `'static`
+    |       ^^ borrowed value does not live long enough
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/method-ufcs-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     let x = <&'static u32 as Bazoom<_>>::method;
   |             ----------------------------------- assignment requires that `a` is borrowed for `'static`
LL |     x(&a, b, c); //~ ERROR
   |       ^^ borrowed value does not live long enough
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


---- [ui] ui/nll/user-annotations/method-ufcs-2.rs stdout ----
diff of stderr:

1 error[E0597]: `a` does not live long enough
3    |
3    |
+ LL |     let x = <&'static u32 as Bazoom<_>>::method;
+    |             ----------------------------------- assignment requires that `a` is borrowed for `'static`
4 LL |     x(&a, b, c);
-    |     --^^-------
-    |     | |
-    |     | borrowed value does not live long enough
-    |     argument requires that `a` is borrowed for `'static`
+    |       ^^ borrowed value does not live long enough
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/method-ufcs-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `a` does not live long enough
   |
   |
LL |     let x = <&'static u32 as Bazoom<_>>::method;
   |             ----------------------------------- assignment requires that `a` is borrowed for `'static`
LL |     x(&a, b, c); //~ ERROR
   |       ^^ borrowed value does not live long enough
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

2   --> $DIR/method-ufcs-3.rs:36:53
3    |
4 LL |     <_ as Bazoom<_>>::method::<&'static u32>(&a, b, &c);
-    |     ------------------------------------------------^^-
-    |     |                                               |
-    |     |                                               borrowed value does not live long enough
-    |     argument requires that `c` is borrowed for `'static`
+    |     ----------------------------------------        ^^ borrowed value does not live long enough
+    |     |
+    |     requires that `c` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/method-ufcs-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/method-ufcs-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `c` does not live long enough
   |
   |
LL |     <_ as Bazoom<_>>::method::<&'static u32>(&a, b, &c); //~ ERROR
   |     ----------------------------------------        ^^ borrowed value does not live long enough
   |     |
   |     requires that `c` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-78262.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "polonius" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.polonius" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-Z" "polonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.polonius/auxiliary"
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-78262.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-78262.nll/auxiliary"
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-proc-static-upvar.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-proc-static-upvar/auxiliary"
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
+    |           ---^^^^-
+    |           |  |
+    |           |  borrowed value does not live long enough
+    |           requires that `line` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-pattern-typing-issue-19552.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-pattern-typing-issue-19552/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `line` does not live long enough
   |
   |
LL |     match [&*line] { //~ ERROR `line` does not live long enough
   |           ---^^^^-
   |           |  |
   |           |  borrowed value does not live long enough
   |           requires that `line` is borrowed for `'static`
LL | }
LL | }
   | - `line` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
3    |
4 LL |         Self(&x);
-    |              ^^
-    |              |
-    |              borrowed value does not live long enough
-    |              this usage requires that `x` is borrowed for `'static`
+    |         -----^^-
+    |         |    |
+    |         |    borrowed value does not live long enough
+    |         requires that `x` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/issue-61882-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/issue-61882-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/issue-61882-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `x` does not live long enough
   |
LL |         Self(&x);
   |         -----^^-
   |         |    |
   |         |    |
   |         |    borrowed value does not live long enough
   |         requires that `x` is borrowed for `'static`
LL |         //~^ ERROR `x` does not live long enough
LL |     }
   |     - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.

---
diff of stderr:

10   --> $DIR/static-drop-scope.rs:7:60
11    |
12 LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
-    |                                                      |     |       |
-    |                                                      |     |       temporary value is freed at the end of this statement
-    |                                                      |     creates a temporary which is freed while still in use
-    |                                                      |     creates a temporary which is freed while still in use
-    |                                                      using this value as a static requires that borrow lasts for `'static`
+    |                          -------------------------         ^^^^^^^^- temporary value is freed at the end of this statement
+    |                          |                                 creates a temporary which is freed while still in use
+    |                          |                                 creates a temporary which is freed while still in use
+    |                          requires that borrow lasts for `'static`
19 error[E0493]: destructors cannot be evaluated at compile-time
20   --> $DIR/static-drop-scope.rs:11:59

28   --> $DIR/static-drop-scope.rs:11:59
28   --> $DIR/static-drop-scope.rs:11:59
29    |
30 LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
-    |                                                     |     |       |
-    |                                                     |     |       temporary value is freed at the end of this statement
-    |                                                     |     creates a temporary which is freed while still in use
-    |                                                     |     creates a temporary which is freed while still in use
-    |                                                     using this value as a constant requires that borrow lasts for `'static`
+    |                         -------------------------         ^^^^^^^^- temporary value is freed at the end of this statement
+    |                         |                                 creates a temporary which is freed while still in use
+    |                         |                                 creates a temporary which is freed while still in use
+    |                         requires that borrow lasts for `'static`
37 error[E0493]: destructors cannot be evaluated at compile-time
38   --> $DIR/static-drop-scope.rs:15:28



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/static-drop-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-drop-scope.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-drop-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/auxiliary"
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
  --> /checkout/src/test/ui/static/static-drop-scope.rs:7:60
  --> /checkout/src/test/ui/static/static-drop-scope.rs:7:60
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                          -------------------------         ^^^^^^^^- temporary value is freed at the end of this statement
   |                          |                                 creates a temporary which is freed while still in use
   |                          |                                 creates a temporary which is freed while still in use
   |                          requires that borrow lasts for `'static`
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/static/static-drop-scope.rs:11:59
   |
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                           ^^^^^^^^- value is dropped here
   |                                                           constants cannot evaluate destructors

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-drop-scope.rs:11:59
  --> /checkout/src/test/ui/static/static-drop-scope.rs:11:59
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                         -------------------------         ^^^^^^^^- temporary value is freed at the end of this statement
   |                         |                                 creates a temporary which is freed while still in use
   |                         |                                 creates a temporary which is freed while still in use
   |                         requires that borrow lasts for `'static`
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

2   --> $DIR/static-region-bound.rs:10:14
3    |
4 LL |     let x = &id(3);
-    |              ^^^^^ creates a temporary which is freed while still in use
+    |             -^^^^^
+    |             |creates a temporary which is freed while still in use
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-region-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-region-bound.rs:10:14
   |
LL |     let x = &id(3); //~ ERROR temporary value dropped while borrowed
   |             -^^^^^
   |             |creates a temporary which is freed while still in use
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
+    |             -------------------- ^^ borrowed value does not live long enough
+    |             |
+    |             requires that `s` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `s` does not live long enough
   |
   |
LL |         z = f::<dyn X<Y = &str>>(&s);
   |             -------------------- ^^ borrowed value does not live long enough
   |             |
   |             requires that `s` is borrowed for `'static`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/coercion-generic-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/coercion-generic-regions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/coercion-generic-regions/auxiliary"
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
test result: FAILED. 12362 passed; 46 failed; 119 ignored; 0 measured; 0 filtered out; finished in 144.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:51
