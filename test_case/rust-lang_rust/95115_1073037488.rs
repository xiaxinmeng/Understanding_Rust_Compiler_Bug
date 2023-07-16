plain
.................................................................................................... 400/12751
.................................................................................................... 500/12751
.................................................................................................... 600/12751
.................................................................................................... 700/12751
.........F.F...........i............................................................................ 800/12751
.......i............................................................................................ 900/12751
..............................F........................................F............................ 1000/12751
....................F..F...F...F.F...........................F..................FF.F...FFF...F...... 1100/12751
...F.........F...........F................................................F................F....F... 1200/12751
.....................................i..........................F.........................F......... 1300/12751
.................................................................................................... 1500/12751
..................................................................................................F. 1600/12751
......................................................i...........F................................. 1700/12751
.................................................................................................... 1800/12751
---
.................................................................................................... 4700/12751
.................................................................................................... 4800/12751
...................................................................................i................ 4900/12751
...............................................i.................................................... 5000/12751
...................F..F............................................................................. 5100/12751
.................................................................................................... 5300/12751
.................................................................................................... 5400/12751
.................................................................................................... 5500/12751
.................................................................................................... 5600/12751
.................................................................................................... 5600/12751
.......i............................................................................................ 5700/12751
.............F...................................................................................... 5800/12751
.................................................................................................... 5900/12751
.................................................................................................... 6000/12751
.............................................F...................................................... 6100/12751
......................................................i............................................. 6200/12751
.................................................................................................... 6300/12751
...................................i..................F..............F.............................. 6400/12751
i.....................................................iiii........i...i............................. 6600/12751
.................................................................................................... 6700/12751
.............................................i....i........................................i........ 6800/12751
........i.............i...................................................i......................... 6900/12751
........i.............i...................................................i......................... 6900/12751
.............................................................................i...................... 7000/12751
.................................................................................................... 7100/12751
........................FF.F.F..F................................................................... 7200/12751
.......i............................................................................................ 7400/12751
..................................................................................i................. 7500/12751
.................................................................................................... 7600/12751
.................................................................................................... 7700/12751
.................................................................................................... 7700/12751
........................................ii................i....i..ii................................ 7800/12751
.................................................................................................... 7900/12751
............F......F..........................F.............................F..............F........ 8000/12751
...........F....................................................F.............................F..... 8100/12751
.........................................................................................F..i..iiF.. 8200/12751
..............................................iiii.................................................. 8400/12751
...................................................................i................................ 8500/12751
.......i...................................................................i........................ 8600/12751
.................................................................................................... 8700/12751
---
.................................................................................................... 12100/12751
.................................................................................................... 12200/12751
.................................................................................................... 12300/12751
.................................................................................................... 12400/12751
...............................F.................................................................... 12500/12751
.........F..F........................................................................F.............. 12600/12751
...................................................
failures:

---- [ui] ui/async-await/no-move-across-await-tuple.rs stdout ----
---- [ui] ui/async-await/no-move-across-await-tuple.rs stdout ----
diff of stderr:

8    |     ^^^ value used here after move
9    |
10    = note: move occurs because `x.1` has type `Vec<usize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.1`
+    |
+ LL |     drop(x.1.clone());
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args async-await/no-move-across-await-tuple.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-move-across-await-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-tuple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-tuple/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x.1`
   |
LL |     drop(x.1);
   |          --- value moved here
LL |     nothing().await;
LL |     nothing().await;
LL |     x.1
   |     ^^^ value used here after move
   |
   = note: move occurs because `x.1` has type `Vec<usize>`, which does not implement the `Copy` trait
help: consider cloning `x.1`
   |
LL |     drop(x.1.clone());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/async-await/no-move-across-await-struct.rs stdout ----
diff of stderr:

7    |     ^^^ value used here after move
8    |
9    = note: move occurs because `s.x` has type `Vec<usize>`, which does not implement the `Copy` trait
+ help: consider cloning `s.x`
+    |
+ LL |     needs_vec(s.x.clone()).await;
10 
11 error: aborting due to previous error
12 

---
To only update this specific test, also pass `--test-args async-await/no-move-across-await-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-move-across-await-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-struct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `s.x`
   |
   |
LL |     needs_vec(s.x).await;
   |               --- value moved here
LL |     s.x
   |     ^^^ value used here after move
   |
   = note: move occurs because `s.x` has type `Vec<usize>`, which does not implement the `Copy` trait
help: consider cloning `s.x`
   |
LL |     needs_vec(s.x.clone()).await;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/binop/binop-move-semantics.rs stdout ----
diff of stderr:

32 LL |     x.clone();
33    |     ^^^^^^^^^ value borrowed here after move
- help: consider further restricting this bound
+ help: consider cloning `x`
36    |
36    |
- LL | fn move_then_borrow<T: Add<Output=()> + Clone + Copy>(x: T) {
+ LL |     x.clone()
+    |      ++++++++
39 
39 
40 error[E0505]: cannot move out of `x` because it is borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-move-semantics/binop-move-semantics.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-move-semantics/binop-move-semantics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/binop-move-semantics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-move-semantics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-move-semantics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-move-semantics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL |   fn double_move<T: Add<Output=()>>(x: T) {
   |                                     - move occurs because `x` has type `T`, which does not implement the `Copy` trait
LL | /     x
LL | |     +
LL | |     x;  //~ ERROR: use of moved value
   | |     |
   | |     |
   | |_____value used here after move
   |       `x` moved due to usage in operator
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/arith.rs:114:12
   |
   |
LL |     fn add(self, rhs: Rhs) -> Self::Output;
help: consider further restricting this bound
   |
   |
LL | fn double_move<T: Add<Output=()> + Copy>(x: T) {


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn move_then_borrow<T: Add<Output=()> + Clone>(x: T) {
   |                                                - move occurs because `x` has type `T`, which does not implement the `Copy` trait
   |     - value moved here
LL |     +
LL |     +
LL |     x.clone();  //~ ERROR: borrow of moved value
   |     ^^^^^^^^^ value borrowed here after move
help: consider cloning `x`
   |
LL |     x.clone()
   |      ++++++++
   |      ++++++++

error[E0505]: cannot move out of `x` because it is borrowed
   |
LL |     let m = &x;
LL |     let m = &x;
   |             -- borrow of `x` occurs here
...
LL |     x  //~ ERROR: cannot move out of `x` because it is borrowed
   |     ^ move out of `x` occurs here
...
LL |     use_mut(n); use_imm(m);


error[E0505]: cannot move out of `y` because it is borrowed
   |
LL |     let n = &mut y;
LL |     let n = &mut y;
   |             ------ borrow of `y` occurs here
...
LL |     y;  //~ ERROR: cannot move out of `y` because it is borrowed
   |     ^ move out of `y` occurs here
LL |     use_mut(n); use_imm(m);


error[E0507]: cannot move out of `*m` which is behind a mutable reference
   |
   |
LL |     *m  //~ ERROR: cannot move
   |     ^^ move occurs because `*m` has type `T`, which does not implement the `Copy` trait

error[E0507]: cannot move out of `*n` which is behind a shared reference
   |
   |
LL |     *n;  //~ ERROR: cannot move
   |     ^^ move occurs because `*n` has type `T`, which does not implement the `Copy` trait

error[E0502]: cannot borrow `f` as immutable because it is also borrowed as mutable
   |
LL |       &mut f
   |       ------
   |       |
   |       |
   |  _____mutable borrow occurs here
LL | |     +
LL | |     +
LL | |     &f;  //~ ERROR: cannot borrow `f` as immutable because it is also borrowed as mutable
   | |     ^-
   | |_____||
   |       |mutable borrow later used here
   |       immutable borrow occurs here

error[E0502]: cannot borrow `f` as mutable because it is also borrowed as immutable
   |
LL |       &f
   |       --
   |       |
   |       |
   |  _____immutable borrow occurs here
LL | |     +
LL | |     +
LL | |     &mut f;  //~ ERROR: cannot borrow `f` as mutable because it is also borrowed as immutable
   | |_____|____|
   |       |    immutable borrow later used here
   |       |    immutable borrow later used here
   |       mutable borrow occurs here
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0382, E0502, E0505, E0507.
For more information about an error, try `rustc --explain E0382`.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs stdout ----
diff of stderr:

27 ...
28 LL |     &x;
29    |     ^^ value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |         a @ [.., _].clone() => (),
30 
30 
31 error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns/bindings-after-at-or-patterns-slice-patterns-box-patterns.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns/bindings-after-at-or-patterns-slice-patterns-box-patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ [.., ref mut bar] => (),
   |         |              |
   |         |              |
   |         |              mutable borrow, by `bar`, occurs here
   |         immutable borrow, by `foo`, occurs here

error: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ Some(box ref mut s) => (),
   |         |                  |
   |         |                  |
   |         |                  mutable borrow, by `s`, occurs here
   |         immutable borrow, by `foo`, occurs here

error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn bindings_after_at_slice_patterns_move_binding(x: [String; 4]) {
   |                                                  - move occurs because `x` has type `[String; 4]`, which does not implement the `Copy` trait
LL |     match x {
LL |         a @ [.., _] => (),
   |         ----------- value moved here
LL |     &x;
LL |     &x;
   |     ^^ value borrowed here after move
help: consider cloning `x`
   |
   |
LL |         a @ [.., _].clone() => (),


error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
   |
LL |         ref mut foo @ [.., _] => Some(foo),
   |         --------------------- mutable borrow occurs here
LL |     &x;
LL |     &x;
   |     ^^ immutable borrow occurs here
LL |     drop(r);
   |          - mutable borrow later used here


error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         [ref foo @ .., ref bar] => Some(foo),
   |          ------------ immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ [.., ref bar] => Some(foo),
   |         ----------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn bindings_after_at_or_patterns_move(x: Option<Test>) {
   |                                       - move occurs because `x` has type `Option<Test>`, which does not implement the `Copy` trait
LL |     match x {
LL |         foo @ Some(Test::Foo | Test::Bar) => (),
   |         |
   |         value moved here
   |         value moved here
...
...
LL |     &x;
   |     ^^ value borrowed here after move

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ Some(Test::Foo | Test::Bar) => Some(foo),
   |         ------------------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
   |
LL |         ref mut foo @ Some(Test::Foo | Test::Bar) => Some(foo),
   |         ----------------------------------------- mutable borrow occurs here
LL |     &x;
LL |     &x;
   |     ^^ immutable borrow occurs here
LL |     drop(r);
   |          - mutable borrow later used here


error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ Some(box ref s) => Some(foo),
   |         ------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn bindings_after_at_slice_patterns_or_patterns_moves(x: [Option<Test>; 4]) {
   |                                                       - move occurs because `x` has type `[Option<Test>; 4]`, which does not implement the `Copy` trait
LL |     match x {
LL |         a @ [.., Some(Test::Foo | Test::Bar)] => (),
   |         |
   |         value moved here
   |         value moved here
...
...
LL |     &x;
   |     ^^ value borrowed here after move

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref a @ [ref b @ .., Some(Test::Foo | Test::Bar)] => Some(a),
   |         ------------------------------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref a @ [ref b @ .., Some(Test::Foo | Test::Bar)] => Some(b),
   |                  ---------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         [_, ref a @ Some(box ref b), ..] => Some(a),
   |             ----------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         [_, ref a @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
   |             ------------------------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
   |
LL |         [_, ref mut a @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
   |             ----------------------------------------------- mutable borrow occurs here
LL |     &x;
LL |     &x;
   |     ^^ immutable borrow occurs here
LL |     drop(r);
   |          - mutable borrow later used here


error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref a @ [_, ref b @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
   |         ------------------------------------------------------------ immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here

error: aborting due to 17 previous errors
---

---- [ui] ui/borrowck/borrowck-consume-unsize-vec.rs stdout ----
diff of stderr:

7    |             - value moved here
8 LL |     consume(b);
9    |             ^ value used here after move
+ help: consider cloning `b`
+    |
+    |
+ LL |     consume(b.clone());
10 
11 error: aborting due to previous error
12 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-consume-unsize-vec.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-consume-unsize-vec.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-consume-unsize-vec" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-consume-unsize-vec/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `b`
   |
   |
LL | fn foo(b: Box<[i32;5]>) {
   |        - move occurs because `b` has type `Box<[i32; 5]>`, which does not implement the `Copy` trait
LL |     consume(b);
   |             - value moved here
LL |     consume(b); //~ ERROR use of moved value
   |             ^ value used here after move
help: consider cloning `b`
   |
   |
LL |     consume(b.clone());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-closures-slice-patterns.rs stdout ----
diff of stderr:

38 LL |     };
39 LL |     &x;
40    |     ^^ value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |         let [y, z @ ..] = x.clone();
41 
41 
42 error[E0502]: cannot borrow `*x` as mutable because it is also borrowed as immutable

79 LL |     };
80 LL |     &x;
80 LL |     &x;
81    |     ^^ value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |         let [y, z @ ..] = *x.clone();
82 
82 
83 error[E0502]: cannot borrow `*x` as mutable because it is also borrowed as immutable


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-slice-patterns/borrowck-closures-slice-patterns.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-slice-patterns/borrowck-closures-slice-patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-closures-slice-patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-slice-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-slice-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-slice-patterns/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
LL |     let f = || {
LL |     let f = || {
   |             -- immutable borrow occurs here
LL |         let [ref y, ref z @ ..] = x;
   |                                   - first borrow occurs due to use of `x` in closure
LL |     let r = &mut x;
LL |     let r = &mut x;
   |             ^^^^^^ mutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     f();
   |     - immutable borrow later used here

error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
LL |     let mut f = || {
LL |     let mut f = || {
   |                 -- mutable borrow occurs here
LL |         let [ref mut y, ref mut z @ ..] = x;
   |                                           - first borrow occurs due to use of `x` in closure
LL |     };
LL |     let r = &x;
   |             ^^ immutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     f();
   |     - mutable borrow later used here

error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn arr_by_move(x: [String; 3]) {
   |                - move occurs because `x` has type `[String; 3]`, which does not implement the `Copy` trait
LL |     let f = || {
   |             -- value moved into closure here
LL |         let [y, z @ ..] = x;
   |                           - variable moved due to use in closure
LL |     &x;
LL |     &x;
   |     ^^ value borrowed here after move
help: consider cloning `x`
   |
   |
LL |         let [y, z @ ..] = x.clone();


error[E0502]: cannot borrow `*x` as mutable because it is also borrowed as immutable
   |
LL |     let f = || {
LL |     let f = || {
   |             -- immutable borrow occurs here
LL |         let [ref y, ref z @ ..] = *x;
   |                                   -- first borrow occurs due to use of `x` in closure
LL |     };
LL |     let r = &mut *x;
   |             ^^^^^^^ mutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     f();
   |     - immutable borrow later used here

error[E0501]: cannot borrow `x` as immutable because previous closure requires unique access
   |
LL |     let mut f = || {
   |                 -- closure construction occurs here
   |                 -- closure construction occurs here
LL |         let [ref mut y, ref mut z @ ..] = *x;
   |                                           -- first borrow occurs due to use of `x` in closure
LL |     };
LL |     let r = &x;
   |             ^^ second borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     f();
   |     - first borrow later used here

error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn arr_box_by_move(x: Box<[String; 3]>) {
   |                    - move occurs because `x` has type `Box<[String; 3]>`, which does not implement the `Copy` trait
LL |     let f = || {
   |             -- value moved into closure here
LL |         let [y, z @ ..] = *x;
   |                           -- variable moved due to use in closure
LL |     &x;
LL |     &x;
   |     ^^ value borrowed here after move
help: consider cloning `x`
   |
   |
LL |         let [y, z @ ..] = *x.clone();


error[E0502]: cannot borrow `*x` as mutable because it is also borrowed as immutable
   |
LL |     let f = || {
LL |     let f = || {
   |             -- immutable borrow occurs here
LL |         if let [ref y, ref z @ ..] = *x {}
   |                                      -- first borrow occurs due to use of `x` in closure
LL |     };
LL |     let r = &mut *x;
   |             ^^^^^^^ mutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     f();
   |     - immutable borrow later used here

error[E0501]: cannot borrow `x` as immutable because previous closure requires unique access
   |
LL |     let mut f = || {
   |                 -- closure construction occurs here
   |                 -- closure construction occurs here
LL |         if let [ref mut y, ref mut z @ ..] = *x {}
   |                                              -- first borrow occurs due to use of `x` in closure
LL |     };
LL |     let r = &x;
   |             ^^ second borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     f();
   |     - first borrow later used here
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0382, E0501, E0502.
For more information about an error, try `rustc --explain E0382`.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-drop-from-guard.rs stdout ----
diff of stderr:

9 LL |         Some(_) => {}
10 LL |         None => { foo(my_str); }
11    |                       ^^^^^^ value used here after move
+    |
+ help: consider cloning `my_str`
+    |
+ LL |         Some(_) if { drop(my_str.clone()); false } => {}
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-drop-from-guard.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-drop-from-guard.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-drop-from-guard/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `my_str`
   |
   |
LL |     let my_str = "hello".to_owned();
   |         ------ move occurs because `my_str` has type `String`, which does not implement the `Copy` trait
LL |     match Some(42) {
LL |         Some(_) if { drop(my_str); false } => {}
   |                           ------ value moved here
LL |         Some(_) => {}
LL |         None => { foo(my_str); } //~ ERROR [E0382]
   |                       ^^^^^^ value used here after move
help: consider cloning `my_str`
   |
   |
LL |         Some(_) if { drop(my_str.clone()); false } => {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-describe-lvalue.rs stdout ----
diff of stderr:

361    |                      ^ value used here after move
362    |
363    = note: move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
+ help: consider cloning `x`
+ LL |                 drop(x.clone());
+    |                       ++++++++
364 
365 error: aborting due to 32 previous errors
---
To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |             let y = &mut x;
   |                     ------ first mutable borrow occurs here
LL |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |             ^^^^^^ second mutable borrow occurs here
LL |             *y = 1;


error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |                    let y = &mut x;
   |                            ------ first mutable borrow occurs here
LL |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |                    ^^^^^^ second mutable borrow occurs here
LL |                    *y = 1;


error: captured variable cannot escape `FnMut` closure body
   |
LL |           let mut x = 0;
   |               ----- variable defined here
LL |              || {
LL |              || {
   |               - inferred to be a `FnMut` closure
LL | /                || { //~ ERROR captured variable cannot escape `FnMut` closure body
LL | |                    let y = &mut x;
   | |                                 - variable captured here
LL | |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
LL | |                    *y = 1;
LL | |                    drop(y);
LL | |                 }
   | |_________________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape

error[E0503]: cannot use `f.x` because it was mutably borrowed
   |
   |
LL |         let x = f.x();
   |                 ----- borrow of `f` occurs here
LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
   |         ^^^ use of borrowed `f`
   |              - borrow later used here


error[E0503]: cannot use `g.0` because it was mutably borrowed
   |
LL |         let x = g.x();
LL |         let x = g.x();
   |                 ----- borrow of `g` occurs here
LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
   |         ^^^ use of borrowed `g`
   |              - borrow later used here


error[E0503]: cannot use `h.0` because it was mutably borrowed
   |
LL |         let x = &mut h.0;
LL |         let x = &mut h.0;
   |                 -------- borrow of `h.0` occurs here
LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
   |         ^^^ use of borrowed `h.0`
   |              - borrow later used here


error[E0503]: cannot use `e.0` because it was mutably borrowed
   |
   |
LL |         let x = e.x();
   |                 ----- borrow of `e` occurs here
LL |         match e {
LL |             Baz::X(value) => value //~ ERROR cannot use `e.0` because it was mutably borrowed
   |                    ^^^^^ use of borrowed `e`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `u.a` because it was mutably borrowed
   |
LL |         let x = &mut u.a;
LL |         let x = &mut u.a;
   |                 -------- borrow of `u.a` occurs here
LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
   |         ^^^ use of borrowed `u.a`
   |              - borrow later used here


error[E0503]: cannot use `f.x` because it was mutably borrowed
   |
   |
LL |         let x = f.x();
   |                 ----- borrow of `*f` occurs here
LL |         f.x; //~ ERROR cannot use `f.x` because it was mutably borrowed
   |         ^^^ use of borrowed `*f`
   |              - borrow later used here


error[E0503]: cannot use `g.0` because it was mutably borrowed
   |
LL |         let x = g.x();
LL |         let x = g.x();
   |                 ----- borrow of `*g` occurs here
LL |         g.0; //~ ERROR cannot use `g.0` because it was mutably borrowed
   |         ^^^ use of borrowed `*g`
   |              - borrow later used here


error[E0503]: cannot use `h.0` because it was mutably borrowed
   |
LL |         let x = &mut h.0;
LL |         let x = &mut h.0;
   |                 -------- borrow of `h.0` occurs here
LL |         h.0; //~ ERROR cannot use `h.0` because it was mutably borrowed
   |         ^^^ use of borrowed `h.0`
   |              - borrow later used here


error[E0503]: cannot use `e.0` because it was mutably borrowed
   |
   |
LL |         let x = e.x();
   |                 ----- borrow of `*e` occurs here
LL |         match *e {
LL |             Baz::X(value) => value
   |                    ^^^^^ use of borrowed `*e`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `u.a` because it was mutably borrowed
   |
LL |         let x = &mut u.a;
LL |         let x = &mut u.a;
   |                 -------- borrow of `u.a` occurs here
LL |         u.a; //~ ERROR cannot use `u.a` because it was mutably borrowed
   |         ^^^ use of borrowed `u.a`
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         match v {
LL |             &[x, _, .., _, _] => println!("{}", x),
   |               ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, x, .., _, _] => println!("{}", x),
   |                  ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, _, .., x, _] => println!("{}", x),
   |                         ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, _, .., _, x] => println!("{}", x),
   |                            ^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         match v {
LL |             &[x @ ..] => println!("{:?}", x),
   |               ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, x @ ..] => println!("{:?}", x),
   |                  ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[x @ .., _] => println!("{:?}", x),
   |               ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[..]` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
...
LL |             &[_, x @ .., _] => println!("{:?}", x),
   |                  ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `e` because it was mutably borrowed
   |
LL |         let x = &mut e;
LL |         let x = &mut e;
   |                 ------ borrow of `e` occurs here
LL |         match e {
   |               ^ use of borrowed `e`
LL |         drop(x);
   |              - borrow later used here


error[E0502]: cannot borrow `e.0` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut e;
LL |         let x = &mut e;
   |                 ------ mutable borrow occurs here
...
LL |             E::A(ref ax) =>
   |                  ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `e.x` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut e;
LL |         let x = &mut e;
   |                 ------ mutable borrow occurs here
...
LL |             E::B { x: ref bx } =>
   |                       ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `s.y.0` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut s;
LL |         let x = &mut s;
   |                 ------ mutable borrow occurs here
LL |         match s {
LL |             S  { y: (ref y0, _), .. } =>
   |                      ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `s.x.y` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut s;
LL |         let x = &mut s;
   |                 ------ mutable borrow occurs here
...
LL |             S  { x: F { y: ref x0, .. }, .. } =>
   |                            ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0503]: cannot use `*v` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         v[0].y;
   |         ^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0503]: cannot use `v[_].y` because it was mutably borrowed
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ borrow of `v` occurs here
LL |         v[0].y;
   |         ^^^^^^ use of borrowed `v`
LL |         drop(x);
   |              - borrow later used here


error[E0502]: cannot borrow `v[..].x` as immutable because it is also borrowed as mutable
   |
LL |         let x = &mut v;
LL |         let x = &mut v;
   |                 ------ mutable borrow occurs here
LL |         match v {
LL |             &[_, F {x: ref xf, ..}] => println!("{}", xf),
   |                        ^^^^^^ immutable borrow occurs here
LL |         drop(x);
   |              - mutable borrow later used here


error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
   |
LL |             let x = &mut block;
LL |             let x = &mut block;
   |                     ---------- mutable borrow occurs here
LL |             let p: &'a u8 = &*block.current;
   |                             ^^^^^^^^^^^^^^^ immutable borrow occurs here
LL |             drop(x);
   |                  - mutable borrow later used here


error[E0502]: cannot borrow `*block.current` as immutable because it is also borrowed as mutable
   |
LL |             let x = &mut block;
LL |             let x = &mut block;
   |                     ---------- mutable borrow occurs here
LL |             let p : *const u8 = &*(*block).current;
   |                                 ^^^^^^^^^^^^^^^^^^ immutable borrow occurs here
LL |             drop(x);
   |                  - mutable borrow later used here

error[E0382]: use of moved value: `x`
error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:274:22
   |
LL |                 drop(x);
   |                      - value moved here
LL |                 drop(x); //~ ERROR use of moved value: `x`
   |                      ^ value used here after move
   |
   = note: move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
help: consider cloning `x`
LL |                 drop(x.clone());
   |                       ++++++++

error: aborting due to 32 previous errors
---

---- [ui] ui/borrowck/borrowck-field-sensitivity.rs stdout ----
diff of stderr:

7    |          ^^^^ value used here after move
8    |
9    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     drop(x.b.clone());
10 
10 
11 error[E0382]: use of moved value: `x.b`


17    |          ^^^^ value used here after move
18    |
19    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     let y = A { a: 3, .. x }.clone();
20 
20 
21 error[E0382]: borrow of moved value: `x.b`


27    |             ^^^^ value borrowed here after move
28    |
29    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     drop(x.b.clone());
30 
30 
31 error[E0382]: borrow of moved value: `x.b`


37    |             ^^^^ value borrowed here after move
38    |
39    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     let _y = A { a: 3, .. x }.clone();
40 
40 
41 error[E0505]: cannot move out of `x.b` because it is borrowed


77    |          ^^^ value used here after move
78    |
79    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     drop(x.b.clone());
80 
80 
81 error[E0382]: use of moved value: `x.b`


87    |          ^^^ value used here after move
88    |
89    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     let _y = A { a: 3, .. x }.clone();
90 
90 
91 error[E0382]: use of moved value: `x.b`


97    |              ^^^^^^^^^^^^^^^^ value used here after move
98    |
99    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     drop(x.b.clone());
100 
100 
101 error[E0382]: use of moved value: `x.b`


107    |              ^^^^^^^^^^^^^^^^ value used here after move
108    |
109    = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
+ help: consider cloning `x.b`
+    |
+ LL |     let _y = A { a: 3, .. x }.clone();
110 
110 
111 error[E0381]: assign to part of possibly-uninitialized variable: `x`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity/borrowck-field-sensitivity.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity/borrowck-field-sensitivity.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-field-sensitivity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x.b`
   |
LL |     drop(x.b);
   |          --- value moved here
   |          --- value moved here
LL |     drop(*x.b); //~ ERROR use of moved value: `x.b`
   |          ^^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     drop(x.b.clone());


error[E0382]: use of moved value: `x.b`
   |
   |
LL |     let y = A { a: 3, .. x };
   |             ---------------- value moved here
LL |     drop(*x.b); //~ ERROR use of moved value: `x.b`
   |          ^^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     let y = A { a: 3, .. x }.clone();


error[E0382]: borrow of moved value: `x.b`
   |
LL |     drop(x.b);
   |          --- value moved here
   |          --- value moved here
LL |     let p = &x.b; //~ ERROR borrow of moved value: `x.b`
   |             ^^^^ value borrowed here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     drop(x.b.clone());


error[E0382]: borrow of moved value: `x.b`
   |
   |
LL |     let _y = A { a: 3, .. x };
   |              ---------------- value moved here
LL |     let p = &x.b; //~ ERROR borrow of moved value: `x.b`
   |             ^^^^ value borrowed here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     let _y = A { a: 3, .. x }.clone();


error[E0505]: cannot move out of `x.b` because it is borrowed
   |
   |
LL |     let p = &x.b;
   |             ---- borrow of `x.b` occurs here
LL |     drop(x.b); //~ ERROR cannot move out of `x.b` because it is borrowed
   |          ^^^ move out of `x.b` occurs here
LL |     drop(**p);


error[E0505]: cannot move out of `x.b` because it is borrowed
   |
   |
LL |     let p = &x.b;
   |             ---- borrow of `x.b` occurs here
LL |     let _y = A { a: 3, .. x }; //~ ERROR cannot move out of `x.b` because it is borrowed
   |              ^^^^^^^^^^^^^^^^ move out of `x.b` occurs here
LL |     drop(**p);


error[E0499]: cannot borrow `x.a` as mutable more than once at a time
   |
LL |     let p = &mut x.a;
LL |     let p = &mut x.a;
   |             -------- first mutable borrow occurs here
LL |     let q = &mut x.a; //~ ERROR cannot borrow `x.a` as mutable more than once at a time
   |             ^^^^^^^^ second mutable borrow occurs here
LL |     drop(*p);


error[E0382]: use of moved value: `x.b`
   |
LL |     drop(x.b);
   |          --- value moved here
   |          --- value moved here
LL |     drop(x.b);  //~ ERROR use of moved value: `x.b`
   |          ^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     drop(x.b.clone());


error[E0382]: use of moved value: `x.b`
   |
   |
LL |     let _y = A { a: 3, .. x };
   |              ---------------- value moved here
LL |     drop(x.b);  //~ ERROR use of moved value: `x.b`
   |          ^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     let _y = A { a: 3, .. x }.clone();


error[E0382]: use of moved value: `x.b`
   |
LL |     drop(x.b);
   |          --- value moved here
   |          --- value moved here
LL |     let _z = A { a: 3, .. x };  //~ ERROR use of moved value: `x.b`
   |              ^^^^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     drop(x.b.clone());


error[E0382]: use of moved value: `x.b`
   |
   |
LL |     let _y = A { a: 3, .. x };
   |              ---------------- value moved here
LL |     let _z = A { a: 4, .. x };  //~ ERROR use of moved value: `x.b`
   |              ^^^^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider cloning `x.b`
   |
LL |     let _y = A { a: 3, .. x }.clone();


error[E0381]: assign to part of possibly-uninitialized variable: `x`
   |
   |
LL |     x.a = 1; //~ ERROR assign to part of possibly-uninitialized variable: `x`
   |     ^^^^^^^ use of possibly-uninitialized `x`

error[E0381]: assign to part of possibly-uninitialized variable: `x`
   |
   |
LL |     x.a = 1; //~ ERROR assign to part of possibly-uninitialized variable: `x`
   |     ^^^^^^^ use of possibly-uninitialized `x`

error[E0381]: assign to part of possibly-uninitialized variable: `x`
   |
   |
LL |     x.b = Box::new(1); //~ ERROR assign to part of possibly-uninitialized variable: `x`
   |     ^^^ use of possibly-uninitialized `x`
error: aborting due to 14 previous errors

Some errors have detailed explanations: E0381, E0382, E0499, E0505.
For more information about an error, try `rustc --explain E0381`.
For more information about an error, try `rustc --explain E0381`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-loan-in-overloaded-op.rs stdout ----
diff of stderr:

7    |               -    ^^^^^^^^^ value borrowed here after move
9    |               value moved here
+    |
+ help: consider cloning `x`
+    |
+    |
+ LL |     let _y = {x.clone()} + x.clone(); // the `{x}` forces a move to occur
10 
11 error: aborting due to previous error
12 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-loan-in-overloaded-op.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-loan-in-overloaded-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-in-overloaded-op" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-in-overloaded-op/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `x`
   |
LL |     let x = Foo(Box::new(3));
LL |     let x = Foo(Box::new(3));
   |         - move occurs because `x` has type `Foo`, which does not implement the `Copy` trait
LL |     let _y = {x} + x.clone(); // the `{x}` forces a move to occur
   |               -    ^^^^^^^^^ value borrowed here after move
   |               value moved here
   |
help: consider cloning `x`
   |
   |
LL |     let _y = {x.clone()} + x.clone(); // the `{x}` forces a move to occur

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-move-moved-value-into-closure.rs stdout ----
diff of stderr:

12    |            ^^^^^^   -- use occurs due to use in closure
14    |            value used here after move
+    |
+ help: consider cloning `t`
+    |
+    |
+ LL |     call_f(move|| { *t.clone() + 1 });
15 
16 error: aborting due to previous error
17 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-move-moved-value-into-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-moved-value-into-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-moved-value-into-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-moved-value-into-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `t`
   |
   |
LL |     let t: Box<_> = Box::new(3);
   |         - move occurs because `t` has type `Box<isize>`, which does not implement the `Copy` trait
LL | 
LL |     call_f(move|| { *t + 1 });
   |            ------   -- variable moved due to use in closure
   |            value moved into closure here
   |            value moved into closure here
LL |     call_f(move|| { *t + 1 }); //~ ERROR use of moved value
   |            ^^^^^^   -- use occurs due to use in closure
   |            value used here after move
   |
help: consider cloning `t`
   |
   |
LL |     call_f(move|| { *t.clone() + 1 });

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-move-out-from-array-match.rs stdout ----
diff of stderr:

8    |              ^^ value used here after move
9    |
10    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _x.clone()] => {}
11 
11 
12 error[E0382]: use of partially moved value: `a[..]`


19    |              ^^ value used here after partial move
20    |
21    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [_, _, (_x.clone(), _)] => {}
22 
22 
23 error[E0382]: use of moved value: `a[..].0`


30    |               ^^ value used here after move
31    |
32    = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [_, _, (_x.clone(), _)] => {}
33 
34 error[E0382]: use of partially moved value: `a`
35   --> $DIR/borrowck-move-out-from-array-match.rs:44:11


41    |           ^ value used here after partial move
42    |
43    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_x.clone(), _, _] => {}
44 
45 error[E0382]: use of partially moved value: `a`
46   --> $DIR/borrowck-move-out-from-array-match.rs:55:11


52    |           ^ value used here after partial move
53    |
54    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [.., _x.clone()] => {}
55 
56 error[E0382]: use of partially moved value: `a`
57   --> $DIR/borrowck-move-out-from-array-match.rs:66:11


63    |           ^ value used here after partial move
64    |
65    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [(_x.clone(), _), _, _] => {}
66 
67 error[E0382]: use of partially moved value: `a`
68   --> $DIR/borrowck-move-out-from-array-match.rs:77:11


74    |           ^ value used here after partial move
75    |
76    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [.., (_x.clone(), _)] => {}
77 
77 
78 error[E0382]: use of moved value: `a[..].0`


85    |           ^^ value used here after move
86    |
87    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_y @ ...clone(), _, _] => {}
88 
88 
89 error[E0382]: use of moved value: `a[..].0`


96    |               ^^ value used here after move
97    |
98    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _y @ ...clone()] => {}
99 
100 error[E0382]: use of partially moved value: `a`
101   --> $DIR/borrowck-move-out-from-array-match.rs:110:11


107    |           ^ value used here after partial move
108    |
109    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [x @ ...clone(), _] => {}
110 
111 error: aborting due to 10 previous errors
112 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-from-array-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-from-array-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-match/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `a[..]`
   |
   |
LL |         [_, _, _x] => {}
   |                -- value moved here
...
LL |         [.., _y] => {} //~ ERROR use of moved value
   |              ^^ value used here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _x.clone()] => {}


error[E0382]: use of partially moved value: `a[..]`
   |
   |
LL |         [_, _, (_x, _)] => {}
   |                 -- value partially moved here
...
LL |         [.., _y] => {} //~ ERROR use of partially moved value
   |              ^^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [_, _, (_x.clone(), _)] => {}


error[E0382]: use of moved value: `a[..].0`
   |
   |
LL |         [_, _, (_x, _)] => {}
   |                 -- value moved here
...
LL |         [.., (_y, _)] => {} //~ ERROR use of moved value
   |               ^^ value used here after move
   |
   = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [_, _, (_x.clone(), _)] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-match.rs:44:11
   |
   |
LL |         [_x, _, _] => {}
   |          -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_x.clone(), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-match.rs:55:11
   |
   |
LL |         [.., _x] => {}
   |              -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [.., _x.clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-match.rs:66:11
   |
   |
LL |         [(_x, _), _, _] => {}
   |           -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [(_x.clone(), _), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-match.rs:77:11
   |
   |
LL |         [.., (_x, _)] => {}
   |               -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [.., (_x.clone(), _)] => {}


error[E0382]: use of moved value: `a[..].0`
   |
   |
LL |         [_y @ .., _, _] => {}
   |          ------- value moved here
...
LL |         [(_x, _), _, _] => {} //~ ERROR use of moved value
   |           ^^ value used here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_y @ ...clone(), _, _] => {}


error[E0382]: use of moved value: `a[..].0`
   |
   |
LL |         [_, _, _y @ ..] => {}
   |                ------- value moved here
...
LL |         [.., (_x, _)] => {} //~ ERROR use of moved value
   |               ^^ value used here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _y @ ...clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-match.rs:110:11
   |
   |
LL |         [x @ .., _] => {}
   |          ------ value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [x @ ...clone(), _] => {}

error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs stdout ----
diff of stderr:

8    |           ^ value used here after partial move
9    |
10    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _x.clone()] => {}
11 
12 error[E0382]: use of partially moved value: `a`
13   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:28:11


19    |           ^ value used here after partial move
20    |
21    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [_, _, (_x.clone(), _)] => {}
22 
23 error[E0382]: use of partially moved value: `a`
24   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:41:11


30    |           ^ value used here after partial move
31    |
32    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_x.clone(), _, _] => {}
33 
34 error[E0382]: use of partially moved value: `a`
35   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:52:11


41    |           ^ value used here after partial move
42    |
43    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [.., _x.clone()] => {}
44 
45 error[E0382]: use of partially moved value: `a`
46   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:63:11


52    |           ^ value used here after partial move
53    |
54    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [(_x.clone(), _), _, _] => {}
55 
56 error[E0382]: use of partially moved value: `a`
57   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:74:11


63    |           ^ value used here after partial move
64    |
65    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [.., (_x.clone(), _)] => {}
66 
67 error[E0382]: use of partially moved value: `a`
68   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:85:11


74    |           ^ value used here after partial move
75    |
76    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _y @ ...clone()] => {}
77 
78 error[E0382]: use of partially moved value: `a`
79   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:96:11


85    |           ^ value used here after partial move
86    |
87    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_y @ ...clone(), _] => {}
88 
89 error[E0382]: use of partially moved value: `a`
90   --> $DIR/borrowck-move-out-from-array-no-overlap-match.rs:109:11


96    |           ^ value used here after partial move
97    |
98    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [x @ ...clone(), _, _] => {}
99 
100 error: aborting due to 9 previous errors
101 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-from-array-no-overlap-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of partially moved value: `a`
   |
   |
LL |         [_, _, _x] => {}
   |                -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _x.clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:28:11
   |
   |
LL |         [_, _, (_x, _)] => {}
   |                 -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [_, _, (_x.clone(), _)] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:41:11
   |
   |
LL |         [_x, _, _] => {}
   |          -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_x.clone(), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:52:11
   |
   |
LL |         [.., _x] => {}
   |              -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [.., _x.clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:63:11
   |
   |
LL |         [(_x, _), _, _] => {}
   |           -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [(_x.clone(), _), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:74:11
   |
   |
LL |         [.., (_x, _)] => {}
   |               -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [.., (_x.clone(), _)] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:85:11
   |
   |
LL |         [_, _y @ ..] => {}
   |             ------- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _y @ ...clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:96:11
   |
   |
LL |         [_y @ .., _] => {}
   |          ------- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_y @ ...clone(), _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-no-overlap-match.rs:109:11
   |
   |
LL |         [x @ .., _, _] => {}
   |          ------ value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [x @ ...clone(), _, _] => {}

error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs stdout ----
diff of stderr:

8    |           ^ value used here after partial move
9    |
10    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _x.clone()] => {}
11 
12 error[E0382]: use of partially moved value: `a`
13   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:28:11


19    |           ^ value used here after partial move
20    |
21    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [_, _, (_x.clone(), _)] => {}
22 
23 error[E0382]: use of partially moved value: `a`
24   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:41:11


30    |           ^ value used here after partial move
31    |
32    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_x.clone(), _, _] => {}
33 
34 error[E0382]: use of partially moved value: `a`
35   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:52:11


41    |           ^ value used here after partial move
42    |
43    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [.., _x.clone()] => {}
44 
45 error[E0382]: use of partially moved value: `a`
46   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:63:11


52    |           ^ value used here after partial move
53    |
54    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [(_x.clone(), _), _, _] => {}
55 
56 error[E0382]: use of partially moved value: `a`
57   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:74:11


63    |           ^ value used here after partial move
64    |
65    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [.., (_x.clone(), _)] => {}
66 
67 error[E0382]: use of partially moved value: `a`
68   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:85:11


74    |           ^ value used here after partial move
75    |
76    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _y @ ...clone()] => {}
77 
78 error[E0382]: use of partially moved value: `a`
79   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:96:11


85    |           ^ value used here after partial move
86    |
87    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_y @ ...clone(), _] => {}
88 
89 error[E0382]: use of partially moved value: `a`
90   --> $DIR/borrowck-move-out-from-array-use-no-overlap-match.rs:109:11


96    |           ^ value used here after partial move
97    |
98    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [x @ ...clone(), _, _] => {}
99 
100 error: aborting due to 9 previous errors
101 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of partially moved value: `a`
   |
   |
LL |         [_, _, _x] => {}
   |                -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _x.clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:28:11
   |
   |
LL |         [_, _, (_x, _)] => {}
   |                 -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [_, _, (_x.clone(), _)] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:41:11
   |
   |
LL |         [_x, _, _] => {}
   |          -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_x.clone(), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:52:11
   |
   |
LL |         [.., _x] => {}
   |              -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [.., _x.clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:63:11
   |
   |
LL |         [(_x, _), _, _] => {}
   |           -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [(_x.clone(), _), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:74:11
   |
   |
LL |         [.., (_x, _)] => {}
   |               -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [.., (_x.clone(), _)] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:85:11
   |
   |
LL |         [_, _y @ ..] => {}
   |             ------- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _y @ ...clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:96:11
   |
   |
LL |         [_y @ .., _] => {}
   |          ------- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_y @ ...clone(), _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-no-overlap-match.rs:109:11
   |
   |
LL |         [x @ .., _, _] => {}
   |          ------ value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [x @ ...clone(), _, _] => {}

error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-move-out-from-array-use-match.rs stdout ----
diff of stderr:

8    |              ^^^^^^ value borrowed here after move
9    |
10    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _x.clone()] => {}
11 
11 
12 error[E0382]: borrow of partially moved value: `a[..]`


19    |              ^^^^^^ value borrowed here after partial move
20    |
21    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [_, _, (_x.clone(), _)] => {}
22 
22 
23 error[E0382]: borrow of moved value: `a[..].0`


30    |               ^^^^^^ value borrowed here after move
31    |
32    = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [_, _, (_x.clone(), _)] => {}
33 
34 error[E0382]: use of partially moved value: `a`
35   --> $DIR/borrowck-move-out-from-array-use-match.rs:44:11


41    |           ^ value used here after partial move
42    |
43    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_x.clone(), _, _] => {}
44 
45 error[E0382]: use of partially moved value: `a`
46   --> $DIR/borrowck-move-out-from-array-use-match.rs:55:11


52    |           ^ value used here after partial move
53    |
54    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [.., _x.clone()] => {}
55 
56 error[E0382]: use of partially moved value: `a`
57   --> $DIR/borrowck-move-out-from-array-use-match.rs:66:11


63    |           ^ value used here after partial move
64    |
65    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [(_x.clone(), _), _, _] => {}
66 
67 error[E0382]: use of partially moved value: `a`
68   --> $DIR/borrowck-move-out-from-array-use-match.rs:77:11


74    |           ^ value used here after partial move
75    |
76    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [.., (_x.clone(), _)] => {}
77 
77 
78 error[E0382]: borrow of moved value: `a[..]`


85    |           ^^^^^^ value borrowed here after move
86    |
87    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_y @ ...clone(), _, _] => {}
88 
88 
89 error[E0382]: borrow of moved value: `a[..]`


96    |               ^^^^^^ value borrowed here after move
97    |
98    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _y @ ...clone()] => {}
99 
100 error[E0382]: use of partially moved value: `a`
101   --> $DIR/borrowck-move-out-from-array-use-match.rs:110:11


107    |           ^ value used here after partial move
108    |
109    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [x @ ...clone(), _] => {}
110 
111 error[E0382]: use of partially moved value: `a`
112   --> $DIR/borrowck-move-out-from-array-use-match.rs:123:5


118    |     ^^^^ value used here after partial move
119    |
120    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _x.clone()] => {}
121 
122 error[E0382]: use of partially moved value: `a`
123   --> $DIR/borrowck-move-out-from-array-use-match.rs:131:5


129    |     ^^^^ value used here after partial move
130    |
131    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |         [_, _, (_x.clone(), _)] => {}
132 
133 error[E0382]: use of partially moved value: `a`
134   --> $DIR/borrowck-move-out-from-array-use-match.rs:139:5


140    |     ^^^^ value used here after partial move
141    |
142    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _x @ ...clone()] => {}
143 
144 error[E0382]: use of partially moved value: `a`
145   --> $DIR/borrowck-move-out-from-array-use-match.rs:147:5


151    |     ^^^^ value used here after partial move
152    |
153    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |         [_, _, _x @ ...clone()] => {}
154 
155 error: aborting due to 14 previous errors
156 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-from-array-use-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-use-match" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-use-match/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `a[..]`
   |
   |
LL |         [_, _, _x] => {}
   |                -- value moved here
...
LL |         [.., ref _y] => {} //~ ERROR [E0382]
   |              ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _x.clone()] => {}


error[E0382]: borrow of partially moved value: `a[..]`
   |
   |
LL |         [_, _, (_x, _)] => {}
   |                 -- value partially moved here
...
LL |         [.., ref _y] => {} //~ ERROR [E0382]
   |              ^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [_, _, (_x.clone(), _)] => {}


error[E0382]: borrow of moved value: `a[..].0`
   |
   |
LL |         [_, _, (_x, _)] => {}
   |                 -- value moved here
...
LL |         [.., (ref _y, _)] => {} //~ ERROR [E0382]
   |               ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [_, _, (_x.clone(), _)] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:44:11
   |
   |
LL |         [_x, _, _] => {}
   |          -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_x.clone(), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:55:11
   |
   |
LL |         [.., _x] => {}
   |              -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [.., _x.clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:66:11
   |
   |
LL |         [(_x, _), _, _] => {}
   |           -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [(_x.clone(), _), _, _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:77:11
   |
   |
LL |         [.., (_x, _)] => {}
   |               -- value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [.., (_x.clone(), _)] => {}


error[E0382]: borrow of moved value: `a[..]`
   |
   |
LL |         [_y @ .., _, _] => {}
   |          ------- value moved here
...
LL |         [(ref _x, _), _, _] => {} //~ ERROR [E0382]
   |           ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_y @ ...clone(), _, _] => {}


error[E0382]: borrow of moved value: `a[..]`
   |
   |
LL |         [_, _, _y @ ..] => {}
   |                ------- value moved here
...
LL |         [.., (ref _x, _)] => {} //~ ERROR [E0382]
   |               ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _y @ ...clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:110:11
   |
   |
LL |         [x @ .., _] => {}
   |          ------ value partially moved here
LL |     match a {
LL |     match a {
   |           ^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [x @ ...clone(), _] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:123:5
   |
   |
LL |         [_, _, _x] => {}
   |                -- value partially moved here
LL |     }
LL |     a[2] = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _x.clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:131:5
   |
   |
LL |         [_, _, (_x, _)] => {}
   |                 -- value partially moved here
LL |     }
LL |     a[2].1 = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |         [_, _, (_x.clone(), _)] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:139:5
   |
   |
LL |         [_, _, _x @ ..] => {}
   |                ------- value partially moved here
LL |     }
LL |     a[0] = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _x @ ...clone()] => {}

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use-match.rs:147:5
   |
   |
LL |         [_, _, _x @ ..] => {}
   |                ------- value partially moved here
LL |     }
LL |     a[0].1 = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |         [_, _, _x @ ...clone()] => {}

error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-move-out-from-array.rs stdout ----
diff of stderr:

7    |              ^^ value used here after move
8    |
9    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_, _, _x.clone()] = a;
10 
10 
11 error[E0382]: use of partially moved value: `a[..]`


17    |              ^^ value used here after partial move
18    |
19    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [_, _, (_x.clone(), _)] = a;
20 
20 
21 error[E0382]: use of moved value: `a[..].0`


27    |               ^^ value used here after move
28    |
29    = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [_, _, (_x.clone(), _)] = a;
30 
31 error[E0382]: use of partially moved value: `a`
32   --> $DIR/borrowck-move-out-from-array.rs:30:10


37    |          ^^^^^^^ value used here after partial move
38    |
39    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_x.clone(), _, _] = a;
40 
41 error[E0382]: use of partially moved value: `a`
42   --> $DIR/borrowck-move-out-from-array.rs:36:16


47    |                ^^^^^^^ value used here after partial move
48    |
49    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [.., _x.clone()] = a;
50 
51 error[E0382]: use of partially moved value: `a`
52   --> $DIR/borrowck-move-out-from-array.rs:42:10


57    |          ^^^^^^^ value used here after partial move
58    |
59    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [(_x.clone(), _), _, _] = a;
60 
61 error[E0382]: use of partially moved value: `a`
62   --> $DIR/borrowck-move-out-from-array.rs:48:16


67    |                ^^^^^^^ value used here after partial move
68    |
69    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [.., (_x.clone(), _)] = a;
70 
70 
71 error[E0382]: use of moved value: `a[..].0`


77    |           ^^ value used here after move
78    |
79    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_y @ ...clone(), _, _] = a;
80 
80 
81 error[E0382]: use of moved value: `a[..].0`


87    |               ^^ value used here after move
88    |
89    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_, _, _y @ ...clone()] = a;
90 
91 error[E0382]: use of partially moved value: `a`
92   --> $DIR/borrowck-move-out-from-array.rs:68:13


97    |             ^^^^^^^ value used here after partial move
98    |
99    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [x @ ...clone(), _] = a;
100 
101 error: aborting due to 10 previous errors
102 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-from-array.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-from-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `a[..]`
   |
   |
LL |     let [_, _, _x] = a;
   |                -- value moved here
LL |     let [.., _y] = a; //~ ERROR [E0382]
   |              ^^ value used here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_, _, _x.clone()] = a;


error[E0382]: use of partially moved value: `a[..]`
   |
   |
LL |     let [_, _, (_x, _)] = a;
   |                 -- value partially moved here
LL |     let [.., _y] = a; //~ ERROR [E0382]
   |              ^^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [_, _, (_x.clone(), _)] = a;


error[E0382]: use of moved value: `a[..].0`
   |
   |
LL |     let [_, _, (_x, _)] = a;
   |                 -- value moved here
LL |     let [.., (_y, _)] = a; //~ ERROR [E0382]
   |               ^^ value used here after move
   |
   = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [_, _, (_x.clone(), _)] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array.rs:30:10
   |
   |
LL |     let [_x, _, _] = a;
   |          -- value partially moved here
LL |     let [_y @ .., _, _] = a; //~ ERROR [E0382]
   |          ^^^^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_x.clone(), _, _] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array.rs:36:16
   |
   |
LL |     let [.., _x] = a;
   |              -- value partially moved here
LL |     let [_, _, _y @ ..] = a; //~ ERROR [E0382]
   |                ^^^^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [.., _x.clone()] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array.rs:42:10
   |
   |
LL |     let [(_x, _), _, _] = a;
   |           -- value partially moved here
LL |     let [_y @ .., _, _] = a; //~ ERROR [E0382]
   |          ^^^^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [(_x.clone(), _), _, _] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array.rs:48:16
   |
   |
LL |     let [.., (_x, _)] = a;
   |               -- value partially moved here
LL |     let [_, _, _y @ ..] = a; //~ ERROR [E0382]
   |                ^^^^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [.., (_x.clone(), _)] = a;


error[E0382]: use of moved value: `a[..].0`
   |
   |
LL |     let [_y @ .., _, _] = a;
   |          ------- value moved here
LL |     let [(_x, _), _, _] = a; //~ ERROR [E0382]
   |           ^^ value used here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_y @ ...clone(), _, _] = a;


error[E0382]: use of moved value: `a[..].0`
   |
   |
LL |     let [_, _, _y @ ..] = a;
   |                ------- value moved here
LL |     let [.., (_x, _)] = a; //~ ERROR [E0382]
   |               ^^ value used here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_, _, _y @ ...clone()] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array.rs:68:13
   |
   |
LL |     let [x @ .., _] = a;
   |          ------ value partially moved here
LL |     let [_, _y @ ..] = a; //~ ERROR [E0382]
   |             ^^^^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [x @ ...clone(), _] = a;

error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-move-out-from-array-use.rs stdout ----
diff of stderr:

7    |              ^^^^^^ value borrowed here after move
8    |
9    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_, _, _x.clone()] = a;
10 
10 
11 error[E0382]: borrow of partially moved value: `a[..]`


17    |              ^^^^^^ value borrowed here after partial move
18    |
19    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [_, _, (_x.clone(), _)] = a;
20 
20 
21 error[E0382]: borrow of moved value: `a[..].0`


27    |               ^^^^^^ value borrowed here after move
28    |
29    = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [_, _, (_x.clone(), _)] = a;
30 
30 
31 error[E0382]: borrow of partially moved value: `a`


37    |          ^^^^^^^^^^^ value borrowed here after partial move
38    |
39    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_x.clone(), _, _] = a;
40 
40 
41 error[E0382]: borrow of partially moved value: `a`


47    |                ^^^^^^^^^^^ value borrowed here after partial move
48    |
49    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [.., _x.clone()] = a;
50 
50 
51 error[E0382]: borrow of partially moved value: `a`


57    |          ^^^^^^^^^^^ value borrowed here after partial move
58    |
59    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [(_x.clone(), _), _, _] = a;
60 
60 
61 error[E0382]: borrow of partially moved value: `a`


67    |                ^^^^^^^^^^^ value borrowed here after partial move
68    |
69    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [.., (_x.clone(), _)] = a;
70 
70 
71 error[E0382]: borrow of moved value: `a[..]`


77    |           ^^^^^^ value borrowed here after move
78    |
79    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_y @ ...clone(), _, _] = a;
80 
80 
81 error[E0382]: borrow of moved value: `a[..]`


87    |               ^^^^^^ value borrowed here after move
88    |
89    = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_, _, _y @ ...clone()] = a;
90 
90 
91 error[E0382]: borrow of partially moved value: `a`


97    |             ^^^^^^^^^^^ value borrowed here after partial move
98    |
99    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [x @ ...clone(), _] = a;
100 
101 error[E0382]: use of partially moved value: `a`
102   --> $DIR/borrowck-move-out-from-array-use.rs:76:5


107    |     ^^^^ value used here after partial move
108    |
109    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_, _, _x.clone()] = a;
110 
111 error[E0382]: use of partially moved value: `a`
112   --> $DIR/borrowck-move-out-from-array-use.rs:82:5


117    |     ^^^^ value used here after partial move
118    |
119    = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `a[..].0`
+    |
+ LL |     let [_, _, (_x.clone(), _)] = a;
120 
121 error[E0382]: use of partially moved value: `a`
122   --> $DIR/borrowck-move-out-from-array-use.rs:88:5


127    |     ^^^^ value used here after partial move
128    |
129    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_, _, _x @ ...clone()] = a;
130 
131 error[E0382]: use of partially moved value: `a`
132   --> $DIR/borrowck-move-out-from-array-use.rs:94:5


137    |     ^^^^ value used here after partial move
138    |
139    = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
+ help: consider cloning `a[..]`
+    |
+ LL |     let [_, _, _x @ ...clone()] = a;
140 
141 error: aborting due to 14 previous errors
142 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-move-out-from-array-use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-move-out-from-array-use/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `a[..]`
   |
   |
LL |     let [_, _, _x] = a;
   |                -- value moved here
LL |     let [.., ref _y] = a; //~ ERROR [E0382]
   |              ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_, _, _x.clone()] = a;


error[E0382]: borrow of partially moved value: `a[..]`
   |
   |
LL |     let [_, _, (_x, _)] = a;
   |                 -- value partially moved here
LL |     let [.., ref _y] = a; //~ ERROR [E0382]
   |              ^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [_, _, (_x.clone(), _)] = a;


error[E0382]: borrow of moved value: `a[..].0`
   |
   |
LL |     let [_, _, (_x, _)] = a;
   |                 -- value moved here
LL |     let [.., (ref _y, _)] = a; //~ ERROR [E0382]
   |               ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [_, _, (_x.clone(), _)] = a;


error[E0382]: borrow of partially moved value: `a`
   |
   |
LL |     let [_x, _, _] = a;
   |          -- value partially moved here
LL |     let [ref _y @ .., _, _] = a; //~ ERROR [E0382]
   |          ^^^^^^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_x.clone(), _, _] = a;


error[E0382]: borrow of partially moved value: `a`
   |
   |
LL |     let [.., _x] = a;
   |              -- value partially moved here
LL |     let [_, _, ref _y @ ..] = a; //~ ERROR [E0382]
   |                ^^^^^^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [.., _x.clone()] = a;


error[E0382]: borrow of partially moved value: `a`
   |
   |
LL |     let [(_x, _), _, _] = a;
   |           -- value partially moved here
LL |     let [ref _y @ .., _, _] = a; //~ ERROR [E0382]
   |          ^^^^^^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [(_x.clone(), _), _, _] = a;


error[E0382]: borrow of partially moved value: `a`
   |
   |
LL |     let [.., (_x, _)] = a;
   |               -- value partially moved here
LL |     let [_, _, ref _y @ ..] = a; //~ ERROR [E0382]
   |                ^^^^^^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [.., (_x.clone(), _)] = a;


error[E0382]: borrow of moved value: `a[..]`
   |
   |
LL |     let [_y @ .., _, _] = a;
   |          ------- value moved here
LL |     let [(ref _x, _), _, _] = a; //~ ERROR [E0382]
   |           ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_y @ ...clone(), _, _] = a;


error[E0382]: borrow of moved value: `a[..]`
   |
   |
LL |     let [_, _, _y @ ..] = a;
   |                ------- value moved here
LL |     let [.., (ref _x, _)] = a; //~ ERROR [E0382]
   |               ^^^^^^ value borrowed here after move
   |
   = note: move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_, _, _y @ ...clone()] = a;


error[E0382]: borrow of partially moved value: `a`
   |
   |
LL |     let [x @ .., _] = a;
   |          ------ value partially moved here
LL |     let [_, ref _y @ ..] = a; //~ ERROR [E0382]
   |             ^^^^^^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [x @ ...clone(), _] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use.rs:76:5
   |
   |
LL |     let [_, _, _x] = a;
   |                -- value partially moved here
LL |     a[2] = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_, _, _x.clone()] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use.rs:82:5
   |
   |
LL |     let [_, _, (_x, _)] = a;
   |                 -- value partially moved here
LL |     a[2].1 = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..].0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `a[..].0`
   |
LL |     let [_, _, (_x.clone(), _)] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use.rs:88:5
   |
   |
LL |     let [_, _, _x @ ..] = a;
   |                ------- value partially moved here
LL |     a[0] = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_, _, _x @ ...clone()] = a;

error[E0382]: use of partially moved value: `a`
  --> /checkout/src/test/ui/borrowck/borrowck-move-out-from-array-use.rs:94:5
   |
   |
LL |     let [_, _, _x @ ..] = a;
   |                ------- value partially moved here
LL |     a[0].1 = Default::default(); //~ ERROR [E0382]
   |     ^^^^ value used here after partial move
   |
   = note: partial move occurs because `a[..]` has type `(String, String)`, which does not implement the `Copy` trait
help: consider cloning `a[..]`
   |
LL |     let [_, _, _x @ ...clone()] = a;

error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0382`.
---
diff of stderr:

40 ...
41 LL |         drop(x1);
42    |              -- use occurs due to use in closure
+ help: consider cloning `x1`
+    |
+ LL |     drop(x1.clone());
+    |            ++++++++
+    |            ++++++++
43 
44 error[E0382]: use of moved value: `x2`
45   --> $DIR/borrowck-multiple-captures.rs:27:19

53 ...
54 LL |         drop(x2);
55    |              -- use occurs due to use in closure
+ help: consider cloning `x2`
+    |
+ LL |     drop(x2.clone());
+    |            ++++++++
+    |            ++++++++
56 
57 error[E0382]: use of moved value: `x`
58   --> $DIR/borrowck-multiple-captures.rs:41:14

63    |              ^ value used here after move
64    |
65    = note: move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
+ help: consider cloning `x`
+ LL |         drop(x.clone());
+    |               ++++++++
66 
66 
67 error[E0505]: cannot move out of `x` because it is borrowed


87    |              ^ value used here after move
88    |
89    = note: move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
+ help: consider cloning `x`
+ LL |         drop(x.clone());
+    |               ++++++++
90 
91 error[E0382]: use of moved value: `x`
---
To only update this specific test, also pass `--test-args borrowck/borrowck-multiple-captures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-multiple-captures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-multiple-captures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-multiple-captures/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0505]: cannot move out of `x1` because it is borrowed
   |
LL |     let p1 = &x1;
LL |     let p1 = &x1;
   |              --- borrow of `x1` occurs here
...
LL |     thread::spawn(move|| {
   |                   ^^^^^^ move out of `x1` occurs here
LL |         drop(x1);
LL |         drop(x1);
   |              -- move occurs due to use in closure
...
LL |     borrow(&*p1);


error[E0505]: cannot move out of `x2` because it is borrowed
   |
LL |     let p2 = &x2;
LL |     let p2 = &x2;
   |              --- borrow of `x2` occurs here
LL |     thread::spawn(move|| {
   |                   ^^^^^^ move out of `x2` occurs here
LL |         drop(x2);
LL |         drop(x2);
   |              -- move occurs due to use in closure
...
LL |     borrow(&*p2);

error[E0382]: use of moved value: `x1`
  --> /checkout/src/test/ui/borrowck/borrowck-multiple-captures.rs:27:19
   |
   |
LL |     let x1: Box<_> = Box::new(1);
   |         -- move occurs because `x1` has type `Box<i32>`, which does not implement the `Copy` trait
   |          -- value moved here
...
...
LL |     thread::spawn(move|| {
   |                   ^^^^^^ value used here after move
LL |         drop(x1);
LL |         drop(x1);
   |              -- use occurs due to use in closure
help: consider cloning `x1`
   |
LL |     drop(x1.clone());
   |            ++++++++
   |            ++++++++

error[E0382]: use of moved value: `x2`
  --> /checkout/src/test/ui/borrowck/borrowck-multiple-captures.rs:27:19
   |
LL |     let x2: Box<_> = Box::new(2);
   |         -- move occurs because `x2` has type `Box<i32>`, which does not implement the `Copy` trait
   |          -- value moved here
   |          -- value moved here
LL |     thread::spawn(move|| {
   |                   ^^^^^^ value used here after move
LL |         drop(x2);
LL |         drop(x2);
   |              -- use occurs due to use in closure
help: consider cloning `x2`
   |
LL |     drop(x2.clone());
   |            ++++++++
   |            ++++++++

error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/borrowck/borrowck-multiple-captures.rs:41:14
   |
LL |         drop(x);
   |              - value moved here
LL |         drop(x); //~ ERROR use of moved value: `x`
   |              ^ value used here after move
   |
   = note: move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
help: consider cloning `x`
LL |         drop(x.clone());
   |               ++++++++


error[E0505]: cannot move out of `x` because it is borrowed
   |
LL |     let p = &x;
LL |     let p = &x;
   |             -- borrow of `x` occurs here
LL |     thread::spawn(move|| {
   |                   ^^^^^^ move out of `x` occurs here
LL |         //~^ ERROR cannot move out of `x` because it is borrowed
LL |         drop(x);
   |              - move occurs due to use in closure
...
LL |     borrow(&*p);

error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/borrowck/borrowck-multiple-captures.rs:52:14
   |
   |
LL |         drop(x);
   |              - value moved here
LL |         drop(x); //~ ERROR use of moved value: `x`
   |              ^ value used here after move
   |
   = note: move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
help: consider cloning `x`
LL |         drop(x.clone());
   |               ++++++++

error[E0382]: use of moved value: `x`
error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/borrowck/borrowck-multiple-captures.rs:49:19
   |
LL |     let x: Box<_> = Box::new(1);
   |         - move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
   |          - value moved here
   |          - value moved here
LL |     thread::spawn(move|| {
   |                   ^^^^^^ value used here after move
LL |         //~^ ERROR use of moved value: `x`
   |              - use occurs due to use in closure
   |
help: consider cloning `x`
   |
---
---- [ui] ui/borrowck/borrowck-overloaded-index-move-index.rs stdout ----
diff of stderr:

33 ...
34 LL |     f[s] = 10;
35    |       ^ value used here after move
+ help: consider cloning `s`
+    |
+    |
+ LL |     println!("{}", f[s.clone()]);
36 
37 error: aborting due to 3 previous errors
38 

---
To only update this specific test, also pass `--test-args borrowck/borrowck-overloaded-index-move-index.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-overloaded-index-move-index.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-move-index" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-overloaded-index-move-index/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0505]: cannot move out of `s` because it is borrowed
   |
   |
LL |     let rs = &mut s;
   |              ------ borrow of `s` occurs here
LL | 
LL |     println!("{}", f[s]);
   |                      ^ move out of `s` occurs here
LL |     use_mut(rs);
   |             -- borrow later used here


error[E0505]: cannot move out of `s` because it is borrowed
   |
   |
LL |     let rs = &mut s;
   |              ------ borrow of `s` occurs here
...
LL |     f[s] = 10;
   |       ^ move out of `s` occurs here
LL |     use_mut(rs);
   |             -- borrow later used here

error[E0382]: use of moved value: `s`
error[E0382]: use of moved value: `s`
  --> /checkout/src/test/ui/borrowck/borrowck-overloaded-index-move-index.rs:53:7
   |
LL |     let mut s = "hello".to_string();
   |         ----- move occurs because `s` has type `String`, which does not implement the `Copy` trait
...
LL |     println!("{}", f[s]);
   |                      - value moved here
...
LL |     f[s] = 10;
   |       ^ value used here after move
help: consider cloning `s`
   |
   |
LL |     println!("{}", f[s.clone()]);

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0382, E0505.
Some errors have detailed explanations: E0382, E0505.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/borrowck-reinit.rs stdout ----
diff of stderr:

8    |          - value moved here
9 LL |     let _ = (1,x);
10    |                ^ value used here after move
+ help: consider cloning `x`
+    |
+ LL |     drop(x.clone());
+    |           ++++++++
---
To only update this specific test, also pass `--test-args borrowck/borrowck-reinit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-reinit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-reinit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-reinit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
LL |     let mut x = Box::new(0);
LL |     let mut x = Box::new(0);
   |         ----- move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
LL |     drop(x);
   |          - value moved here
   |          - value moved here
LL |     let _ = (1,x); //~ ERROR use of moved value: `x`
   |                ^ value used here after move
help: consider cloning `x`
   |
LL |     drop(x.clone());
   |           ++++++++
---

---- [ui] ui/borrowck/copy-suggestion-region-vid.rs stdout ----
diff of stderr:

8    |                        -------            ^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
10    |                        value moved here
+    |
+    |
+ help: consider cloning `helpers`
+    |
+ LL |         HelperStruct { helpers.clone(), is_empty: helpers[0].is_empty() }
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args borrowck/copy-suggestion-region-vid.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/copy-suggestion-region-vid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/copy-suggestion-region-vid" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/copy-suggestion-region-vid/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `helpers`
   |
   |
LL |         let helpers = [vec![], vec![]];
   |             ------- move occurs because `helpers` has type `[Vec<&i64>; 2]`, which does not implement the `Copy` trait
LL | 
LL |         HelperStruct { helpers, is_empty: helpers[0].is_empty() }
   |                        -------            ^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
   |                        value moved here
   |
   |
help: consider cloning `helpers`
   |
LL |         HelperStruct { helpers.clone(), is_empty: helpers[0].is_empty() }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/issue-31287-drop-in-guard.rs stdout ----
diff of stderr:

8    |                           - value moved here
9 LL |         x => x,
10    |         ^ value used here after move
+ help: consider cloning `a`
+    |
+    |
+ LL |         Some(_) if { drop(a.clone()); false } => None,
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args borrowck/issue-31287-drop-in-guard.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-31287-drop-in-guard.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-31287-drop-in-guard" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-31287-drop-in-guard/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `a`
   |
   |
LL |     let a = Some("...".to_owned());
   |         - move occurs because `a` has type `Option<String>`, which does not implement the `Copy` trait
LL |     let b = match a {
LL |         Some(_) if { drop(a); false } => None,
   |                           - value moved here
LL |         x => x, //~ ERROR use of moved value: `a`
   |         ^ value used here after move
help: consider cloning `a`
   |
   |
LL |         Some(_) if { drop(a.clone()); false } => None,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/issue-41962.rs stdout ----
diff of stderr:

9    |
10 LL |         if let Some(ref thing) = maybe {
+ help: consider cloning value
+    |
+    |
+ LL |         if let Some(thing.clone()) = maybe {
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args borrowck/issue-41962.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-41962.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-41962" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-41962/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value
   |
   |
LL |         if let Some(thing) = maybe {
   |                     ^^^^^ value moved here, in previous iteration of loop
   |
   = note: move occurs because value has type `Vec<bool>`, which does not implement the `Copy` trait
help: borrow this field in the pattern to avoid moving `maybe.0`
   |
LL |         if let Some(ref thing) = maybe {
help: consider cloning value
   |
   |
LL |         if let Some(thing.clone()) = maybe {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/borrowck/or-patterns.rs stdout ----
diff of stderr:

8    |     ^^^^^^^ value borrowed here after move
9    |
10    = note: move occurs because `x.0.0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `x.0.0`
+    |
+ LL |         ((y.clone(), _) | (_, y),) => (),
11 
11 
12 error[E0382]: borrow of moved value: `x.0.1`
13   --> $DIR/or-patterns.rs:10:5

19    |     ^^^^^^^ value borrowed here after move
20    |
21    = note: move occurs because `x.0.1` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `x.0.1`
+    |
+ LL |         ((y, _) | (_, y.clone()),) => (),
22 
22 
23 error[E0502]: cannot borrow `x.0.0` as mutable because it is also borrowed as immutable
24   --> $DIR/or-patterns.rs:18:5

77    |     ^^^^^^^ value borrowed here after move
78    |
79    = note: move occurs because `x.0.0` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `x.0.0`
+    |
+ LL |     let ((y.clone(), _) | (_, y),) = x;
80 
80 
81 error[E0382]: borrow of moved value: `x.0.1`
82   --> $DIR/or-patterns.rs:40:5

88    |     ^^^^^^^ value borrowed here after move
89    |
90    = note: move occurs because `x.0.1` has type `String`, which does not implement the `Copy` trait
+ help: consider cloning `x.0.1`
+    |
+ LL |     let ((y, _) | (_, y.clone()),) = x;
91 
91 
92 error[E0502]: cannot borrow `x.0.0` as mutable because it is also borrowed as immutable
93   --> $DIR/or-patterns.rs:46:5

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/or-patterns/or-patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/or-patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/or-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/or-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/or-patterns/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `x.0.0`
   |
   |
LL |         ((y, _) | (_, y),) => (),
   |           - value moved here
LL |     }
LL |     &x.0 .0;
   |     ^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `x.0.0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `x.0.0`
   |
LL |         ((y.clone(), _) | (_, y),) => (),


error[E0382]: borrow of moved value: `x.0.1`
   |
   |
LL |         ((y, _) | (_, y),) => (),
   |                       - value moved here
...
LL |     &x.0 .1;
   |     ^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `x.0.1` has type `String`, which does not implement the `Copy` trait
help: consider cloning `x.0.1`
   |
LL |         ((y, _) | (_, y.clone()),) => (),


error[E0502]: cannot borrow `x.0.0` as mutable because it is also borrowed as immutable
   |
   |
LL |         ((ref y, _) | (_, ref y),) => y,
   |           ----- immutable borrow occurs here
LL |     &mut x.0 .0;
LL |     &mut x.0 .0;
   |     ^^^^^^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x.0.1` as mutable because it is also borrowed as immutable
   |
   |
LL |         ((ref y, _) | (_, ref y),) => y,
   |                           ----- immutable borrow occurs here
LL |     &mut x.0 .1;
LL |     &mut x.0 .1;
   |     ^^^^^^^^^^^ mutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x.0.0` as immutable because it is also borrowed as mutable
   |
   |
LL |         ((ref mut y, _) | (_, ref mut y),) => y,
   |           --------- mutable borrow occurs here
LL |     };
LL |     &x.0 .0;
   |     ^^^^^^^ immutable borrow occurs here
LL |     drop(r);
   |          - mutable borrow later used here


error[E0502]: cannot borrow `x.0.1` as immutable because it is also borrowed as mutable
   |
   |
LL |         ((ref mut y, _) | (_, ref mut y),) => y,
   |                               --------- mutable borrow occurs here
...
LL |     &x.0 .1;
   |     ^^^^^^^ immutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     drop(r);
   |          - mutable borrow later used here

error[E0382]: borrow of moved value: `x.0.0`
   |
   |
LL |     let ((y, _) | (_, y),) = x;
   |           - value moved here
LL |     &x.0 .0;
   |     ^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `x.0.0` has type `String`, which does not implement the `Copy` trait
help: consider cloning `x.0.0`
   |
LL |     let ((y.clone(), _) | (_, y),) = x;


error[E0382]: borrow of moved value: `x.0.1`
   |
   |
LL |     let ((y, _) | (_, y),) = x;
   |                       - value moved here
...
LL |     &x.0 .1;
   |     ^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `x.0.1` has type `String`, which does not implement the `Copy` trait
help: consider cloning `x.0.1`
   |
LL |     let ((y, _) | (_, y.clone()),) = x;


error[E0502]: cannot borrow `x.0.0` as mutable because it is also borrowed as immutable
   |
   |
LL |     let ((ref r, _) | (_, ref r),) = x;
   |           ----- immutable borrow occurs here
LL |     &mut x.0 .0;
   |     ^^^^^^^^^^^ mutable borrow occurs here
LL |     drop(r);
   |          - immutable borrow later used here


error[E0502]: cannot borrow `x.0.1` as mutable because it is also borrowed as immutable
   |
   |
LL |     let ((ref r, _) | (_, ref r),) = x;
   |                           ----- immutable borrow occurs here
LL |     &mut x.0 .1;
LL |     &mut x.0 .1;
   |     ^^^^^^^^^^^ mutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x.0.0` as immutable because it is also borrowed as mutable
   |
   |
LL |     let ((ref mut r, _) | (_, ref mut r),) = x;
   |           --------- mutable borrow occurs here
LL |     &x.0 .0;
   |     ^^^^^^^ immutable borrow occurs here
LL |     drop(r);
   |          - mutable borrow later used here


error[E0502]: cannot borrow `x.0.1` as immutable because it is also borrowed as mutable
   |
   |
LL |     let ((ref mut r, _) | (_, ref mut r),) = x;
   |                               --------- mutable borrow occurs here
...
LL |     &x.0 .1;
   |     ^^^^^^^ immutable borrow occurs here
LL |     //~^ ERROR cannot borrow
LL |     drop(r);
   |          - mutable borrow later used here
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0382, E0502.
For more information about an error, try `rustc --explain E0382`.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


---- [ui] ui/borrowck/two-phase-nonrecv-autoref.rs#nll stdout ----
diff of stderr:

16    |         - ^ value used here after move
18    |         value moved here
+    |
+ help: consider further restricting this bound
+    |
+    |
+ LL |     fn twice_ten_so<F: FnOnce(i32) -> i32 + Clone>(f: Box<F>) {
+    |                                           +++++++
+ help: ...and cloning `f`
+    |
+ LL |         f.clone()(f(10));
19 
19 
20 error[E0499]: cannot borrow `*f` as mutable more than once at a time


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/two-phase-nonrecv-autoref.nll.stderr
To only update this specific test, also pass `--test-args borrowck/two-phase-nonrecv-autoref.rs`


error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/two-phase-nonrecv-autoref.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0499]: cannot borrow `*f` as mutable more than once at a time
   |
   |
LL |         f(f(10));
   |         - ^ second mutable borrow occurs here
   |         |
   |         first mutable borrow occurs here

error[E0382]: use of moved value: `f`
  --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:59:11
   |
   |
LL |     fn twice_ten_so<F: FnOnce(i32) -> i32>(f: Box<F>) {
   |                                            - move occurs because `f` has type `Box<F>`, which does not implement the `Copy` trait
LL |         f(f(10));
   |         - ^ value used here after move
   |         value moved here
   |
help: consider further restricting this bound
   |
   |
LL |     fn twice_ten_so<F: FnOnce(i32) -> i32 + Clone>(f: Box<F>) {
   |                                           +++++++
help: ...and cloning `f`
   |
LL |         f.clone()(f(10));


error[E0499]: cannot borrow `*f` as mutable more than once at a time
   |
   |
LL |         f(f(10));
   |         - ^ second mutable borrow occurs here
   |         |
   |         first mutable borrow occurs here

error[E0382]: use of moved value: `f`
  --> /checkout/src/test/ui/borrowck/two-phase-nonrecv-autoref.rs:73:11
   |
   |
LL |     fn twice_ten_oo(f: Box<dyn FnOnce(i32) -> i32>) {
   |                     - move occurs because `f` has type `Box<dyn FnOnce(i32) -> i32>`, which does not implement the `Copy` trait
LL |         f(f(10));
   |         - ^ value used here after move
   |         value moved here


error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
   |
   |
LL |     double_access(&mut a, &a);
   |     ------------- ------  ^^ immutable borrow occurs here
   |     |             |
   |     |             mutable borrow occurs here


error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
   |
   |
LL |     i[i[3]] = 4;
   |     --^----
   |     | |
   |     | immutable borrow occurs here
   |     mutable borrow occurs here


error[E0502]: cannot borrow `i` as immutable because it is also borrowed as mutable
   |
   |
LL |     i[i[3]] = i[4];
   |     --^----
   |     | |
   |     | immutable borrow occurs here
   |     mutable borrow occurs here

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0382, E0499, E0502.
Some errors have detailed explanations: E0382, E0499, E0502.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/closures/issue-10398.rs stdout ----
diff of stderr:

7    |              ^ value used here after move
8    |
9    = note: move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
+ help: consider cloning `x`
+    |
+ LL |         let _a = x.clone();
10 
11 error: aborting due to previous error
12 

---
To only update this specific test, also pass `--test-args closures/issue-10398.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/issue-10398.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-10398" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-10398/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL |         let _a = x;
   |                  - value moved here
LL |         drop(x);
   |              ^ value used here after move
   |
   = note: move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
help: consider cloning `x`
   |
LL |         let _a = x.clone();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
---
16    |                  ^^^^
17    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider cloning `some_vec`
+    |
+ LL |     some_vec.clone().into_iter();
18 
19 error: aborting due to previous error
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab_3/tab_3.stderr
To only update this specific test, also pass `--test-args codemap_tests/tab_3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/tab_3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab_3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/tab_3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `some_vec`
   |
   |
LL |     let some_vec = vec!["hi"];
   |         -------- move occurs because `some_vec` has type `Vec<&str>`, which does not implement the `Copy` trait
LL |     some_vec.into_iter();
   |              ----------- `some_vec` moved due to this method call
LL |     {
LL |         println!("{:?}", some_vec); //~ ERROR borrow of moved
   |                          ^^^^^^^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `some_vec`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning `some_vec`
   |
LL |     some_vec.clone().into_iter();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
---
diff of stderr:

12 LL |         f();
13    |         ^
14    = note: move occurs because `f` has type `[closure@$DIR/issue-12127.rs:8:24: 8:41]`, which does not implement the `Copy` trait
+ help: consider cloning `f`
+    |
+ LL |         f.clone()();
15 
16 error: aborting due to previous error
17 

---
To only update this specific test, also pass `--test-args issues/issue-12127.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12127.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12127" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12127/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `f`
   |
LL |         f();
LL |         f();
   |         --- `f` moved due to this call
LL |         f();
   |         ^ value used here after move
   |
note: this value implements `FnOnce`, which causes it to be moved when called
   |
LL |         f();
   |         ^
   |         ^
   = note: move occurs because `f` has type `[closure@/checkout/src/test/ui/issues/issue-12127.rs:8:24: 8:41]`, which does not implement the `Copy` trait
help: consider cloning `f`
   |
LL |         f.clone()();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/issues/issue-12041.rs stdout ----
diff of stderr:

5    |                      ^^ value moved here, in previous iteration of loop
6    |
7    = note: move occurs because `tx` has type `Sender<i32>`, which does not implement the `Copy` trait
+ help: consider cloning `tx`
+    |
+ LL |             let tx = tx.clone();
8 
9 error: aborting due to previous error
10 

---
To only update this specific test, also pass `--test-args issues/issue-12041.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12041/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `tx`
   |
   |
LL |             let tx = tx;
   |                      ^^ value moved here, in previous iteration of loop
   |
   = note: move occurs because `tx` has type `Sender<i32>`, which does not implement the `Copy` trait
help: consider cloning `tx`
   |
LL |             let tx = tx.clone();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
---
diff of stderr:

9 ...
10 LL |             s
11    |             ^ value used here after move
+ help: consider cloning `s`
+    |
+    |
+ LL |         0 if { drop(s.clone()); false } => String::from("oops"),
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args issues/issue-29723.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-29723.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29723" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29723/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `s`
   |
LL |     let s = String::new();
LL |     let s = String::new();
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
LL |     let _s = match 0 {
LL |         0 if { drop(s); false } => String::from("oops"),
   |                     - value moved here
LL |             s
LL |             s
   |             ^ value used here after move
help: consider cloning `s`
   |
   |
LL |         0 if { drop(s.clone()); false } => String::from("oops"),

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/issues/issue-42796.rs stdout ----
diff of stderr:

10    |                    ^ value borrowed here after move
12    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider cloning `s`
+    |
+    |
+ LL |     let mut s_copy = s.clone();
13 
14 error: aborting due to previous error
15 

---
To only update this specific test, also pass `--test-args issues/issue-42796.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42796.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42796/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `s`
   |
   |
LL |     let s = "Hello!".to_owned();
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
LL |     let mut s_copy = s;
   |                      - value moved here
...
LL |     println!("{}", s); //~ ERROR borrow of moved value
   |                    ^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning `s`
   |
   |
LL |     let mut s_copy = s.clone();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/issues/issue-61108.rs stdout ----
diff of stderr:

18    |
19 LL |     for l in &bad_letters {
20    |              +
+ help: consider cloning `bad_letters`
+    |
+ LL |     for l in bad_letters.clone() {
21 
22 error: aborting due to previous error
23 

---
To only update this specific test, also pass `--test-args issues/issue-61108.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-61108.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61108" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61108/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `bad_letters`
   |
   |
LL |     let mut bad_letters = vec!['e', 't', 'o', 'i'];
   |         --------------- move occurs because `bad_letters` has type `Vec<char>`, which does not implement the `Copy` trait
LL |     for l in bad_letters {
   |              ----------- `bad_letters` moved due to this implicit call to `.into_iter()`
...
LL |     bad_letters.push('s'); //~ ERROR borrow of moved value: `bad_letters`
   |     ^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `bad_letters`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^
   |                  ^^^^
help: consider iterating over a slice of the `Vec<char>`'s content to avoid moving into the `for` loop
   |
LL |     for l in &bad_letters {
help: consider cloning `bad_letters`
   |
   |
LL |     for l in bad_letters.clone() {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/issues/issue-64559.rs stdout ----
diff of stderr:

19    |
20 LL |     for _val in &orig {}
+ help: consider cloning `orig`
+    |
+    |
+ LL |     for _val in orig.clone() {}
22 
23 error: aborting due to previous error
24 

---
To only update this specific test, also pass `--test-args issues/issue-64559.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-64559.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-64559" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-64559/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `orig`
   |
   |
LL |     let orig = vec![true];
   |         ---- move occurs because `orig` has type `Vec<bool>`, which does not implement the `Copy` trait
LL |     for _val in orig {}
   |                 ---- `orig` moved due to this implicit call to `.into_iter()`
LL |     let _closure = || orig;
   |                    ^^ ---- use occurs due to use in closure
   |                    value used here after move
   |
   |
note: this function takes ownership of the receiver `self`, which moves `orig`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^
   |                  ^^^^
help: consider iterating over a slice of the `Vec<bool>`'s content to avoid moving into the `for` loop
   |
LL |     for _val in &orig {}
help: consider cloning `orig`
   |
   |
LL |     for _val in orig.clone() {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/liveness/liveness-move-call-arg.rs stdout ----
diff of stderr:

6 ...
7 LL |         take(x);
8    |              ^ value moved here, in previous iteration of loop
+ help: consider cloning `x`
+    |
+ LL |         take(x.clone());
+    |               ++++++++
---
To only update this specific test, also pass `--test-args liveness/liveness-move-call-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-move-call-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-call-arg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-call-arg/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL |     let x: Box<isize> = Box::new(25);
   |         - move occurs because `x` has type `Box<isize>`, which does not implement the `Copy` trait
...
LL |         take(x); //~ ERROR use of moved value: `x`
   |              ^ value moved here, in previous iteration of loop
help: consider cloning `x`
   |
   |
LL |         take(x.clone()); //~ ERROR use of moved value: `x`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/liveness/liveness-move-in-loop.rs stdout ----
diff of stderr:

6 ...
7 LL |                     x = y;
8    |                         ^ value moved here, in previous iteration of loop
+ help: consider cloning `y`
+    |
+    |
+ LL |                     x = y.clone();
9 
10 error: aborting due to previous error
11 

---
To only update this specific test, also pass `--test-args liveness/liveness-move-in-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-move-in-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-in-loop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-in-loop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `y`
   |
   |
LL |     let y: Box<isize> = 42.into();
   |         - move occurs because `y` has type `Box<isize>`, which does not implement the `Copy` trait
...
LL |                     x = y; //~ ERROR use of moved value
   |                         ^ value moved here, in previous iteration of loop
help: consider cloning `y`
   |
   |
LL |                     x = y.clone(); //~ ERROR use of moved value

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/liveness/liveness-move-in-while.rs stdout ----
diff of stderr:

30    |                                                    - value moved here, in previous iteration of loop
32    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider cloning `y`
+    |
+    |
+ LL |         while true { while true { while true { x = y.clone(); x.clone(); } } }
33 
34 error: aborting due to previous error; 3 warnings emitted
35 

---
To only update this specific test, also pass `--test-args liveness/liveness-move-in-while.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-move-in-while.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-in-while" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-move-in-while/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |         ^^^^^^^^^^ help: use `loop`
   |
   = note: `#[warn(while_true)]` on by default

warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |                      ^^^^^^^^^^ help: use `loop`

warning: denote infinite loops with `loop { ... }`
   |
   |
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |                                   ^^^^^^^^^^ help: use `loop`

error[E0382]: borrow of moved value: `y`
   |
   |
LL |     let y: Box<isize> = 42.into();
   |         - move occurs because `y` has type `Box<isize>`, which does not implement the `Copy` trait
...
LL |         println!("{}", y); //~ ERROR borrow of moved value: `y`
   |                        ^ value borrowed here after move
LL |         while true { while true { while true { x = y; x.clone(); } } }
   |                                                    - value moved here, in previous iteration of loop
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning `y`
   |
   |
LL |         while true { while true { while true { x = y.clone(); x.clone(); } } }

error: aborting due to previous error; 3 warnings emitted

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/liveness/liveness-use-after-move.rs stdout ----
diff of stderr:

10    |                    ^^ value borrowed here after move
12    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider cloning `x`
+    |
+    |
+ LL |     let y = x.clone();
13 
14 error: aborting due to previous error
15 

---
To only update this specific test, also pass `--test-args liveness/liveness-use-after-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-use-after-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-move/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `x`
   |
   |
LL |     let x: Box<_> = 5.into();
   |         - move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
LL |     let y = x;
   |             - value moved here
LL | 
LL |     println!("{}", *x); //~ ERROR borrow of moved value: `x`
   |                    ^^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning `x`
   |
   |
LL |     let y = x.clone();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/liveness/liveness-use-after-send.rs stdout ----
diff of stderr:

9    |                    ^^^^^^^ value borrowed here after move
11    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider cloning `message`
+    |
+    |
+ LL |     send(ch, message.clone());
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args liveness/liveness-use-after-send.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-use-after-send.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-send" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-use-after-send/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `message`
   |
   |
LL | fn test00_start(ch: Chan<Box<isize>>, message: Box<isize>, _count: Box<isize>) {
   |                                       ------- move occurs because `message` has type `Box<isize>`, which does not implement the `Copy` trait
LL |     send(ch, message);
   |              ------- value moved here
LL |     println!("{}", message); //~ ERROR borrow of moved value: `message`
   |                    ^^^^^^^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning `message`
   |
   |
LL |     send(ch, message.clone());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/nll/closure-move-spans.rs stdout ----
diff of stderr:

9    |     value moved into closure here
10 LL |     let y = x;
11    |             ^ value used here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |     || x.clone();
12 
12 
13 error[E0382]: borrow of moved value: `x`

21    |     value moved into closure here
21    |     value moved into closure here
22 LL |     let y = &x;
23    |             ^^ value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |     || x.clone();
24 
24 
25 error[E0382]: borrow of moved value: `x`

33    |     value moved into closure here
33    |     value moved into closure here
34 LL |     let y = &mut x;
35    |             ^^^^^^ value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |     || x.clone();
36 
37 error: aborting due to 3 previous errors
38 

---
To only update this specific test, also pass `--test-args nll/closure-move-spans.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-move-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-move-spans" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-move-spans/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL | fn move_after_move(x: String) {
   |                    - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     || x;
   |     -- - variable moved due to use in closure
   |     value moved into closure here
   |     value moved into closure here
LL |     let y = x; //~ ERROR
   |             ^ value used here after move
help: consider cloning `x`
   |
   |
LL |     || x.clone();


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn borrow_after_move(x: String) {
   |                      - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     || x;
   |     -- - variable moved due to use in closure
   |     value moved into closure here
   |     value moved into closure here
LL |     let y = &x; //~ ERROR
   |             ^^ value borrowed here after move
help: consider cloning `x`
   |
   |
LL |     || x.clone();


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn borrow_mut_after_move(mut x: String) {
   |                          ----- move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     || x;
   |     -- - variable moved due to use in closure
   |     value moved into closure here
   |     value moved into closure here
LL |     let y = &mut x; //~ ERROR
   |             ^^^^^^ value borrowed here after move
help: consider cloning `x`
   |
   |
LL |     || x.clone();

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/nll/closure-access-spans.rs stdout ----
diff of stderr:

67    |     ^^ - borrow occurs due to use in closure
68    |     |
69    |     value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |     let r = x.clone();
70 
70 
71 error[E0382]: borrow of moved value: `x`


79    |     ^^ - borrow occurs due to use in closure
80    |     |
81    |     value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |     let r = x.clone();
82 
82 
83 error[E0382]: borrow of moved value: `x`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans/closure-access-spans.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans/closure-access-spans.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-access-spans.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-access-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-access-spans/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
LL |     let r = &mut x;
LL |     let r = &mut x;
   |             ------ mutable borrow occurs here
LL |     || x; //~ ERROR
   |     ^^ - second borrow occurs due to use of `x` in closure
   |     |
   |     immutable borrow occurs here
LL |     r.use_mut();
   |     ----------- mutable borrow later used here

error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
LL |     let r = &mut x;
LL |     let r = &mut x;
   |             ------ first mutable borrow occurs here
LL |     || x = 2; //~ ERROR
   |     ^^ - second borrow occurs due to use of `x` in closure
   |     |
   |     second mutable borrow occurs here
LL |     r.use_mut();


error[E0500]: closure requires unique access to `x` but it is already borrowed
   |
LL |     let r = &mut x;
LL |     let r = &mut x;
   |             ------ borrow occurs here
LL |     || *x = 2; //~ ERROR
   |     ^^ -- second borrow occurs due to use of `x` in closure
   |     closure construction occurs here
   |     closure construction occurs here
LL |     r.use_mut();


error[E0503]: cannot use `x` because it was mutably borrowed
   |
LL |     let r = &mut x;
LL |     let r = &mut x;
   |             ------ borrow of `x` occurs here
LL |     move || x; //~ ERROR
   |             ^ use of borrowed `x`
LL |     r.use_ref();


error[E0505]: cannot move out of `x` because it is borrowed
   |
   |
LL |     let r = &x;
   |             -- borrow of `x` occurs here
LL |     || x; //~ ERROR
   |     ^^ - move occurs due to use in closure
   |     |
   |     move out of `x` occurs here
LL |     r.use_ref();


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn closure_imm_capture_moved(mut x: String) {
   |                              ----- move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     let r = x;
   |             - value moved here
LL |     || x.len(); //~ ERROR
   |     ^^ - borrow occurs due to use in closure
   |     |
   |     value borrowed here after move
help: consider cloning `x`
   |
   |
LL |     let r = x.clone();


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn closure_mut_capture_moved(mut x: String) {
   |                              ----- move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     let r = x;
   |             - value moved here
LL |     || x = String::new(); //~ ERROR
   |     ^^ - borrow occurs due to use in closure
   |     |
   |     value borrowed here after move
help: consider cloning `x`
   |
   |
LL |     let r = x.clone();


error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn closure_unique_capture_moved(x: &mut String) {
   |                                 - move occurs because `x` has type `&mut String`, which does not implement the `Copy` trait
LL |     let r = x;
   |             - value moved here
LL |     || *x = String::new(); //~ ERROR
   |     ^^ -- borrow occurs due to use in closure
   |     |
   |     value borrowed here after move
error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/nll/closure-access-spans.rs:50:5
   |
   |
LL | fn closure_move_capture_moved(x: &mut String) {
   |                               - move occurs because `x` has type `&mut String`, which does not implement the `Copy` trait
LL |     let r = x;
   |             - value moved here
LL |     || x; //~ ERROR
   |     ^^ - use occurs due to use in closure
   |     value used here after move

error: aborting due to 9 previous errors

---

---- [ui] ui/nll/closures-in-loops.rs stdout ----
diff of stderr:

8    |         ^^ - use occurs due to use in closure
9    |         |
10    |         value moved into closure here, in previous iteration of loop
+ help: consider cloning `x`
+    |
+    |
+ LL |         || x.clone();
11 
11 
12 error[E0499]: cannot borrow `x` as mutable more than once at a time


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops/closures-in-loops.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops/closures-in-loops.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closures-in-loops.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closures-in-loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closures-in-loops/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL | fn repreated_move(x: String) {
   |                   - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     for i in 0..10 {
LL |         || x; //~ ERROR
   |         ^^ - use occurs due to use in closure
   |         |
   |         value moved into closure here, in previous iteration of loop
help: consider cloning `x`
   |
   |
LL |         || x.clone(); //~ ERROR


error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |         v.push(|| x = String::new()); //~ ERROR
   |                ^^ - borrows occur due to use of `x` in closure
   |                |
   |                `x` was mutably borrowed here in the previous iteration of the loop

error[E0524]: two closures require unique access to `x` at the same time
   |
   |
LL |         v.push(|| *x = String::new()); //~ ERROR
   |                ^^ -- borrows occur due to use of `x` in closure
   |                closures are constructed here in different iterations of loop

error: aborting due to 3 previous errors

---

---- [ui] ui/nll/issue-21232-partial-init-and-use.rs stdout ----
diff of stderr:

29    |         move occurs because `t` has type `(u32, Box<u32>)`, which does not implement the `Copy` trait
30 LL |     t.0 = 10; t.1 = Box::new(20);
31    |     ^^^^^^^^ value partially assigned here after move
+ help: consider cloning `t`
+    |
+    |
+ LL |     let mut t: T = (0, Box::new(0)); drop(t.clone());
32 
32 
33 error[E0381]: assign to part of possibly-uninitialized variable: `s`


61    |         move occurs because `t` has type `(u32, Box<u32>)`, which does not implement the `Copy` trait
62 LL |     t.0 = 10;
63    |     ^^^^^^^^ value partially assigned here after move
+ help: consider cloning `t`
+    |
+    |
+ LL |     let mut t: T = (0, Box::new(0)); drop(t.clone());
64 
64 
65 error[E0381]: assign to part of possibly-uninitialized variable: `s`

160    |         -- value moved here
161 LL |             c.0 = 2;
162    |             ^^^^^^^ value partially assigned here after move
162    |             ^^^^^^^ value partially assigned here after move
+    |
+ help: consider cloning `c`
+    |
+ LL |         c2.clone() => {
163 
164 error[E0382]: assign to part of moved value: `c`
165   --> $DIR/issue-21232-partial-init-and-use.rs:267:13


171    |         -- value moved here
172 LL |             (c.1).0 = 2;
173    |             ^^^^^^^^^^^ value partially assigned here after move
+ help: consider cloning `c`
+    |
+    |
+ LL |         c2.clone() => {
174 
174 
175 error[E0382]: assign to part of moved value: `c.1`

181    |             ^^^^^^^^^^^^^^^ value partially assigned here after move
182    |
182    |
183    = note: move occurs because `c.1` has type `(i32, (i32, String))`, which does not implement the `Copy` trait
+ help: consider cloning `c.1`
+    |
+ LL |         c2.clone() => {
184 
185 error: aborting due to 23 previous errors
186 

---
To only update this specific test, also pass `--test-args nll/issue-21232-partial-init-and-use.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-21232-partial-init-and-use" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-21232-partial-init-and-use/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0381]: assign to part of possibly-uninitialized variable: `s`
   |
   |
LL |     s.x = 10; s.y = Box::new(20);
   |     ^^^^^^^^ use of possibly-uninitialized `s`

error[E0381]: assign to part of possibly-uninitialized variable: `t`
   |
   |
LL |     t.0 = 10; t.1 = Box::new(20);
   |     ^^^^^^^^ use of possibly-uninitialized `t`
error[E0382]: assign to part of moved value: `s`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs:111:5
   |
   |
LL |     let mut s: S<B> = S::new(); drop(s);
   |         -----                        - value moved here
   |         |
   |         move occurs because `s` has type `S<Box<u32>>`, which does not implement the `Copy` trait
LL |     s.x = 10; s.y = Box::new(20);
   |     ^^^^^^^^ value partially assigned here after move
error[E0382]: assign to part of moved value: `t`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs:118:5
   |
   |
LL |     let mut t: T = (0, Box::new(0)); drop(t);
   |         -----                             - value moved here
   |         |
   |         move occurs because `t` has type `(u32, Box<u32>)`, which does not implement the `Copy` trait
LL |     t.0 = 10; t.1 = Box::new(20);
   |     ^^^^^^^^ value partially assigned here after move
help: consider cloning `t`
   |
   |
LL |     let mut t: T = (0, Box::new(0)); drop(t.clone());


error[E0381]: assign to part of possibly-uninitialized variable: `s`
   |
   |
LL |     s.x = 10;
   |     ^^^^^^^^ use of possibly-uninitialized `s`

error[E0381]: assign to part of possibly-uninitialized variable: `t`
   |
   |
LL |     t.0 = 10;
   |     ^^^^^^^^ use of possibly-uninitialized `t`
error[E0382]: assign to part of moved value: `s`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs:139:5
   |
   |
LL |     let mut s: S<B> = S::new(); drop(s);
   |         -----                        - value moved here
   |         |
   |         move occurs because `s` has type `S<Box<u32>>`, which does not implement the `Copy` trait
LL |     s.x = 10;
   |     ^^^^^^^^ value partially assigned here after move
error[E0382]: assign to part of moved value: `t`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs:146:5
   |
   |
LL |     let mut t: T = (0, Box::new(0)); drop(t);
   |         -----                             - value moved here
   |         |
   |         move occurs because `t` has type `(u32, Box<u32>)`, which does not implement the `Copy` trait
LL |     t.0 = 10;
   |     ^^^^^^^^ value partially assigned here after move
help: consider cloning `t`
   |
   |
LL |     let mut t: T = (0, Box::new(0)); drop(t.clone());


error[E0381]: assign to part of possibly-uninitialized variable: `s`
   |
   |
LL |     s.x = 10;
   |     ^^^^^^^^ use of possibly-uninitialized `s`

error[E0381]: assign to part of possibly-uninitialized variable: `t`
   |
   |
LL |     t.0 = 10;
   |     ^^^^^^^^ use of possibly-uninitialized `t`

error[E0381]: assign to part of possibly-uninitialized variable: `q`
   |
   |
LL |     q.r.f.x = 10; q.r.f.y = Box::new(20);
   |     ^^^^^^^^^^^^ use of possibly-uninitialized `q.r.f`

error[E0381]: assign to part of possibly-uninitialized variable: `q`
   |
   |
LL |     q.r.f.0 = 10; q.r.f.1 = Box::new(20);
   |     ^^^^^^^^^^^^ use of possibly-uninitialized `q.r.f`

error[E0382]: assign to part of moved value: `q.r`
   |
   |
LL |     let mut q: Q<S<B>> = Q::new(S::new()); drop(q.r);
   |                                                 --- value moved here
LL |     q.r.f.x = 10; q.r.f.y = Box::new(20);
   |     ^^^^^^^^^^^^ value partially assigned here after move
   |
   = note: move occurs because `q.r` has type `R<S<Box<u32>>>`, which does not implement the `Copy` trait

error[E0382]: assign to part of moved value: `q.r`
   |
   |
LL |     let mut q: Q<T> = Q::new((0, Box::new(0))); drop(q.r);
   |                                                      --- value moved here
LL |     q.r.f.0 = 10; q.r.f.1 = Box::new(20);
   |     ^^^^^^^^^^^^ value partially assigned here after move
   |
   = note: move occurs because `q.r` has type `R<(u32, Box<u32>)>`, which does not implement the `Copy` trait

error[E0381]: assign to part of possibly-uninitialized variable: `q`
   |
   |
LL |     q.r.f.x = 10;
   |     ^^^^^^^^^^^^ use of possibly-uninitialized `q.r.f`

error[E0381]: assign to part of possibly-uninitialized variable: `q`
   |
   |
LL |     q.r.f.0 = 10;
   |     ^^^^^^^^^^^^ use of possibly-uninitialized `q.r.f`

error[E0382]: assign to part of moved value: `q.r`
   |
   |
LL |     let mut q: Q<S<B>> = Q::new(S::new()); drop(q.r);
   |                                                 --- value moved here
LL |     q.r.f.x = 10;
   |     ^^^^^^^^^^^^ value partially assigned here after move
   |
   = note: move occurs because `q.r` has type `R<S<Box<u32>>>`, which does not implement the `Copy` trait

error[E0382]: assign to part of moved value: `q.r`
   |
   |
LL |     let mut q: Q<T> = Q::new((0, Box::new(0))); drop(q.r);
   |                                                      --- value moved here
LL |     q.r.f.0 = 10;
   |     ^^^^^^^^^^^^ value partially assigned here after move
   |
   = note: move occurs because `q.r` has type `R<(u32, Box<u32>)>`, which does not implement the `Copy` trait

error[E0381]: assign to part of possibly-uninitialized variable: `q`
   |
   |
LL |     q.r.f.x = 10;
   |     ^^^^^^^^^^^^ use of possibly-uninitialized `q.r.f`

error[E0381]: assign to part of possibly-uninitialized variable: `q`
   |
   |
LL |     q.r.f.0 = 10;
   |     ^^^^^^^^^^^^ use of possibly-uninitialized `q.r.f`
error[E0382]: assign to part of moved value: `c`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs:257:13
   |
   |
LL |     let mut c = (1, "".to_owned());
   |         ----- move occurs because `c` has type `(i32, String)`, which does not implement the `Copy` trait
LL |     match c {
LL |         c2 => {
   |         -- value moved here
LL |             c.0 = 2; //~ ERROR assign to part of moved value
   |             ^^^^^^^ value partially assigned here after move
help: consider cloning `c`
   |
LL |         c2.clone() => {
   |           ++++++++
   |           ++++++++

error[E0382]: assign to part of moved value: `c`
  --> /checkout/src/test/ui/nll/issue-21232-partial-init-and-use.rs:267:13
   |
LL |     let mut c = (1, (1, "".to_owned()));
   |         ----- move occurs because `c` has type `(i32, (i32, String))`, which does not implement the `Copy` trait
LL |     match c {
LL |         c2 => {
   |         -- value moved here
LL |             (c.1).0 = 2; //~ ERROR assign to part of moved value
   |             ^^^^^^^^^^^ value partially assigned here after move
help: consider cloning `c`
   |
LL |         c2.clone() => {
   |           ++++++++
   |           ++++++++

error[E0382]: assign to part of moved value: `c.1`
   |
LL |         c2 => {
   |         -- value moved here
   |         -- value moved here
LL |             ((c.1).1).0 = 3; //~ ERROR assign to part of moved value
   |             ^^^^^^^^^^^^^^^ value partially assigned here after move
   |
   = note: move occurs because `c.1` has type `(i32, (i32, String))`, which does not implement the `Copy` trait
help: consider cloning `c.1`
LL |         c2.clone() => {
   |           ++++++++

error: aborting due to 23 previous errors
---

---- [ui] ui/nll/issue-51512.rs stdout ----
diff of stderr:

7    |             ----- value moved here
8 LL |     let x = range.start;
9    |             ^^^^^^^^^^^ value used here after move
+ help: consider cloning `range`
+    |
+    |
+ LL |     let r = range.clone();
10 
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51512/issue-51512.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-51512.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-51512.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51512" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51512/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `range`
   |
LL |     let range = 0..1;
LL |     let range = 0..1;
   |         ----- move occurs because `range` has type `std::ops::Range<i32>`, which does not implement the `Copy` trait
LL |     let r = range;
   |             ----- value moved here
LL |     let x = range.start;
   |             ^^^^^^^^^^^ value used here after move
help: consider cloning `range`
   |
   |
LL |     let r = range.clone();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/nll/issue-53807.rs stdout ----
diff of stderr:

9    |
10 LL |         if let Some(ref thing) = maybe {
+ help: consider cloning value
+    |
+    |
+ LL |         if let Some(thing.clone()) = maybe {
12 
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53807/issue-53807.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-53807.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-53807.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53807" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53807/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value
   |
   |
LL |         if let Some(thing) = maybe {
   |                     ^^^^^ value moved here, in previous iteration of loop
   |
   = note: move occurs because value has type `Vec<bool>`, which does not implement the `Copy` trait
help: borrow this field in the pattern to avoid moving `maybe.0`
   |
LL |         if let Some(ref thing) = maybe {
help: consider cloning value
   |
   |
LL |         if let Some(thing.clone()) = maybe {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/nll/match-cfg-fake-edges.rs stdout ----
diff of stderr:

15 LL |         true => {
16 LL |             x;
17    |             ^ value used here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |         false if { drop(x.clone()); true } => 1,
18 
19 error: aborting due to 2 previous errors
20 

---
To only update this specific test, also pass `--test-args nll/match-cfg-fake-edges.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/match-cfg-fake-edges.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/match-cfg-fake-edges" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/match-cfg-fake-edges/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0381]: use of possibly-uninitialized variable: `x`
   |
   |
LL |             x; //~ ERROR use of possibly-uninitialized variable: `x`
   |             ^ use of possibly-uninitialized `x`
error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/nll/match-cfg-fake-edges.rs:35:13
   |
LL |     let x = String::new();
LL |     let x = String::new();
   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
...
LL |         false if { drop(x); true } => 1,
   |                         - value moved here
LL |         true => {
LL |             x; //~ ERROR use of moved value: `x`
   |             ^ value used here after move
help: consider cloning `x`
   |
   |
LL |         false if { drop(x.clone()); true } => 1,

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0381, E0382.
Some errors have detailed explanations: E0381, E0382.
For more information about an error, try `rustc --explain E0381`.
------------------------------------------


---- [ui] ui/nll/ref-suggestion.rs stdout ----
diff of stderr:

7    |             - value moved here
8 LL |     x;
9    |     ^ value used here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |     let y = x.clone();
10 
11 error[E0382]: use of moved value: `x`
12   --> $DIR/ref-suggestion.rs:8:5


17    |                 - value moved here
18 LL |     x;
19    |     ^ value used here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |     let mut y = x.clone();
20 
21 error[E0382]: use of partially moved value: `x`
22   --> $DIR/ref-suggestion.rs:16:5


32    |
33 LL |         (Some(ref y), ()) => {},
+ help: consider cloning value
+    |
+    |
+ LL |         (Some(y.clone()), ()) => {},
35 
36 error: aborting due to 3 previous errors
37 

---
To only update this specific test, also pass `--test-args nll/ref-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ref-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ref-suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ref-suggestion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL |     let x = vec![1];
   |         - move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
LL |     let y = x;
   |             - value moved here
LL |     x; //~ ERROR use of moved value
   |     ^ value used here after move
help: consider cloning `x`
   |
   |
LL |     let y = x.clone();

error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/nll/ref-suggestion.rs:8:5
   |
   |
LL |     let x = vec![1];
   |         - move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
LL |     let mut y = x;
   |                 - value moved here
LL |     x; //~ ERROR use of moved value
   |     ^ value used here after move
help: consider cloning `x`
   |
   |
LL |     let mut y = x.clone();

error[E0382]: use of partially moved value: `x`
  --> /checkout/src/test/ui/nll/ref-suggestion.rs:16:5
   |
   |
LL |         (Some(y), ()) => {},
   |               - value partially moved here
...
LL |     x; //~ ERROR use of partially moved value
   |     ^ value used here after partial move
   |
   = note: partial move occurs because value has type `Vec<i32>`, which does not implement the `Copy` trait
help: borrow this field in the pattern to avoid moving `x.0.0`
   |
LL |         (Some(ref y), ()) => {},
help: consider cloning value
   |
   |
LL |         (Some(y.clone()), ()) => {},

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0382`.
---

18    |
19 LL |     type Target = T;
20    |     ^^^^^^^^^^^^^^^^
+ help: consider cloning `arc_v`
+    |
+ LL |         assert_eq!((*arc_v.clone())[3], 4);
21 
22 error: aborting due to previous error
23 

---
To only update this specific test, also pass `--test-args no-capture-arc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-capture-arc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-capture-arc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-capture-arc/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `arc_v`
   |
   |
LL |     let arc_v = Arc::new(v);
   |         ----- move occurs because `arc_v` has type `Arc<Vec<i32>>`, which does not implement the `Copy` trait
LL | 
LL |     thread::spawn(move|| {
   |                   ------ value moved into closure here
LL |         assert_eq!((*arc_v)[3], 4);
   |                      ----- variable moved due to use in closure
...
LL |     assert_eq!((*arc_v)[2], 3);
   |                ^^^^^^^^ value borrowed here after move
   |
   = note: borrow occurs due to deref coercion to `Vec<i32>`
note: deref defined here
   |
LL |     type Target = T;
   |     ^^^^^^^^^^^^^^^^
help: consider cloning `arc_v`
help: consider cloning `arc_v`
   |
LL |         assert_eq!((*arc_v.clone())[3], 4);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
---

18    |
19 LL |     type Target = T;
20    |     ^^^^^^^^^^^^^^^^
+ help: consider cloning `arc_v`
+    |
+ LL |         assert_eq!((*arc_v.clone())[3], 4);
21 
22 error: aborting due to previous error
23 

---
To only update this specific test, also pass `--test-args no-reuse-move-arc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-reuse-move-arc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-reuse-move-arc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-reuse-move-arc/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `arc_v`
   |
   |
LL |     let arc_v = Arc::new(v);
   |         ----- move occurs because `arc_v` has type `Arc<Vec<i32>>`, which does not implement the `Copy` trait
LL | 
LL |     thread::spawn(move|| {
   |                   ------ value moved into closure here
LL |         assert_eq!((*arc_v)[3], 4);
   |                      ----- variable moved due to use in closure
...
LL |     assert_eq!((*arc_v)[2], 3); //~ ERROR borrow of moved value: `arc_v`
   |                ^^^^^^^^ value borrowed here after move
   |
   = note: borrow occurs due to deref coercion to `Vec<i32>`
note: deref defined here
   |
LL |     type Target = T;
   |     ^^^^^^^^^^^^^^^^
help: consider cloning `arc_v`
help: consider cloning `arc_v`
   |
LL |         assert_eq!((*arc_v.clone())[3], 4);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param.rs stdout ----
diff of stderr:

8    |      value used here after partial move
9    |
10    = note: partial move occurs because value has type `Box<u8>`, which does not implement the `Copy` trait
+ help: consider cloning value
+    |
+ LL | fn f(a @ A(u.clone()): A) -> Box<u8> {
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of partially moved value
   |
   |
LL | fn f(a @ A(u): A) -> Box<u8> {
   |      ^^^^^^-^
   |      |     value partially moved here
   |      |     value partially moved here
   |      value used here after partial move
   |
   = note: partial move occurs because value has type `Box<u8>`, which does not implement the `Copy` trait
   |
   |
LL | fn f(a @ A(u.clone()): A) -> Box<u8> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/suggestions/borrow-for-loop-head.rs stdout ----
diff of stderr:

24    |
25 LL |         for j in &a {
+ help: consider cloning `a`
+    |
+    |
+ LL |         for j in a.clone() {
27 
28 error: aborting due to 2 previous errors
29 

---
To only update this specific test, also pass `--test-args suggestions/borrow-for-loop-head.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/borrow-for-loop-head.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/borrow-for-loop-head/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0505]: cannot move out of `a` because it is borrowed
   |
LL |     for i in &a {
LL |     for i in &a {
   |              -- borrow of `a` occurs here
LL |         for j in a {
   |                  ^ move out of `a` occurs here
error[E0382]: use of moved value: `a`
  --> /checkout/src/test/ui/suggestions/borrow-for-loop-head.rs:4:18
   |
   |
LL |     let a = vec![1, 2, 3];
   |         - move occurs because `a` has type `Vec<i32>`, which does not implement the `Copy` trait
LL |     for i in &a {
LL |         for j in a {
   |                  ^ `a` moved due to this implicit call to `.into_iter()`, in previous iteration of loop
   |
note: this function takes ownership of the receiver `self`, which moves `a`
   |
LL |     fn into_iter(self) -> Self::IntoIter;
   |                  ^^^^
   |                  ^^^^
help: consider iterating over a slice of the `Vec<i32>`'s content to avoid moving into the `for` loop
   |
LL |         for j in &a {
help: consider cloning `a`
   |
   |
LL |         for j in a.clone() {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0382, E0505.
Some errors have detailed explanations: E0382, E0505.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/try-block/try-block-maybe-bad-lifetime.rs stdout ----
diff of stderr:

23    |                        ^ value borrowed here after move
25    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider cloning `x`
+    |
+ LL |             ::std::mem::drop(x.clone());
+ LL |             ::std::mem::drop(x.clone());
+    |                               ++++++++
26 
27 error[E0506]: cannot assign to `i` because it is borrowed
28   --> $DIR/try-block-maybe-bad-lifetime.rs:40:9

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-maybe-bad-lifetime/try-block-maybe-bad-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-block/try-block-maybe-bad-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-block/try-block-maybe-bad-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-maybe-bad-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition" "2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-block/try-block-maybe-bad-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `i` because it is borrowed
   |
LL |             &i
LL |             &i
   |             -- borrow of `i` occurs here
LL |         };
LL |         i = 0; //~ ERROR cannot assign to `i` because it is borrowed
   |         ^^^^^ assignment to borrowed `i` occurs here
LL |         let _ = i;
LL |         do_something_with(x);


error[E0382]: borrow of moved value: `x`
   |
LL |         let x = String::new();
LL |         let x = String::new();
   |             - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |             ::std::mem::drop(x);
   |                              - value moved here
LL |         };
LL |         };
LL |         println!("{}", x); //~ ERROR borrow of moved value: `x`
   |                        ^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning `x`
   |
LL |             ::std::mem::drop(x.clone());
LL |             ::std::mem::drop(x.clone());
   |                               ++++++++

error[E0506]: cannot assign to `i` because it is borrowed
   |
   |
LL |             j = &i;
   |                 -- borrow of `i` occurs here
LL |         };
LL |         i = 0; //~ ERROR cannot assign to `i` because it is borrowed
   |         ^^^^^ assignment to borrowed `i` occurs here
LL |         let _ = i;
LL |         do_something_with(j);

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0382, E0506.
---
16    |            ^^^^
- help: consider further restricting this bound
+ help: consider cloning `x`
18    |
- LL | fn move_then_borrow<T: Not<Output=T> + Clone + Copy>(x: T) {
-    |                                              ++++++
+ LL |     !x.clone();
21 
21 
22 error[E0505]: cannot move out of `x` because it is borrowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unop-move-semantics/unop-move-semantics.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unop-move-semantics/unop-move-semantics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unop-move-semantics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unop-move-semantics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unop-move-semantics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unop-move-semantics/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn move_then_borrow<T: Not<Output=T> + Clone>(x: T) {
   |                                               - move occurs because `x` has type `T`, which does not implement the `Copy` trait
LL |     !x;
   |     -- `x` moved due to usage in operator
LL | 
LL |     x.clone();  //~ ERROR: borrow of moved value
   |     ^^^^^^^^^ value borrowed here after move
note: calling this operator moves the left-hand side
  --> /checkout/library/core/src/ops/bit.rs:51:12
   |
LL |     fn not(self) -> Self::Output;
LL |     fn not(self) -> Self::Output;
   |            ^^^^
help: consider cloning `x`
   |
LL |     !x.clone();


error[E0505]: cannot move out of `x` because it is borrowed
   |
LL |     let m = &x;
LL |     let m = &x;
   |             -- borrow of `x` occurs here
...
LL |     !x;  //~ ERROR: cannot move out of `x` because it is borrowed
   |      ^ move out of `x` occurs here
...
LL |     use_mut(n); use_imm(m);


error[E0505]: cannot move out of `y` because it is borrowed
   |
LL |     let n = &mut y;
LL |     let n = &mut y;
   |             ------ borrow of `y` occurs here
...
LL |     !y;  //~ ERROR: cannot move out of `y` because it is borrowed
   |      ^ move out of `y` occurs here
LL |     use_mut(n); use_imm(m);


error[E0507]: cannot move out of `*m` which is behind a mutable reference
   |
   |
LL |     !*m;  //~ ERROR: cannot move out of `*m`
   |      ^^ move occurs because `*m` has type `T`, which does not implement the `Copy` trait

error[E0507]: cannot move out of `*n` which is behind a shared reference
   |
   |
LL |     !*n;  //~ ERROR: cannot move out of `*n`
   |      ^^ move occurs because `*n` has type `T`, which does not implement the `Copy` trait
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0382, E0505, E0507.
For more information about an error, try `rustc --explain E0382`.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/unsized-locals/borrow-after-move.rs stdout ----
diff of stderr:

66    |         - value moved here
67 LL |         println!("{}", &x);
68    |                        ^^ value borrowed here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |         x.clone().foo();
69 
70 error: aborting due to 5 previous errors; 1 warning emitted
71 

---
To only update this specific test, also pass `--test-args unsized-locals/borrow-after-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/borrow-after-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/borrow-after-move/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(unsized_locals, unsized_fn_params)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information

error[E0382]: borrow of moved value: `x`
   |
   |
LL |         let y = *x;
   |                 -- value moved here
LL |         drop_unsized(y);
LL |         println!("{}", &x);
   |                        ^^ value borrowed here after move
   |
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait

error[E0382]: borrow of moved value: `y`
   |
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         drop_unsized(y);
   |                      - value moved here
LL |         println!("{}", &y);
LL |         println!("{}", &y);
   |                        ^^ value borrowed here after move

error[E0382]: borrow of moved value: `x`
   |
   |
LL |         let y = *x;
   |                 -- value moved here
LL |         y.foo();
LL |         println!("{}", &x);
   |                        ^^ value borrowed here after move
   |
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait

error[E0382]: borrow of moved value: `y`
   |
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         y.foo();
   |           ----- `y` moved due to this method call
LL |         println!("{}", &y);
LL |         println!("{}", &y);
   |                        ^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `y`
   |
LL |     fn foo(self) -> String;
   |            ^^^^


error[E0382]: borrow of moved value: `x`
   |
   |
LL |         let x = "hello".to_owned().into_boxed_str();
   |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
LL |         x.foo();
   |         - value moved here
LL |         println!("{}", &x);
   |                        ^^ value borrowed here after move
help: consider cloning `x`
   |
   |
LL |         x.clone().foo();

error: aborting due to 5 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/unsized-locals/double-move.rs stdout ----
diff of stderr:

36    |                      - value moved here
37 LL |         let _y = *x;
38    |                  ^^ value used here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |         drop_unsized(x.clone());
39 
40 error[E0382]: use of moved value: `y`
41   --> $DIR/double-move.rs:40:9


72    |         - value moved here
73 LL |         let _y = *x;
74    |                  ^^ value used here after move
+ help: consider cloning `x`
+    |
+    |
+ LL |         x.clone().foo();
75 
76 error: aborting due to 6 previous errors; 1 warning emitted
77 

---
To only update this specific test, also pass `--test-args unsized-locals/double-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/double-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/double-move/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(unsized_locals, unsized_fn_params)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information

error[E0382]: use of moved value: `y`
  --> /checkout/src/test/ui/unsized-locals/double-move.rs:21:22
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         drop_unsized(y);
   |                      - value moved here
LL |         drop_unsized(y); //~ERROR use of moved value
   |                      ^ value used here after move
error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/unsized-locals/double-move.rs:27:22
   |
   |
LL |         let _y = *x;
   |                  -- value moved here
LL |         drop_unsized(x); //~ERROR use of moved value
   |                      ^ value used here after move
   |
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `*x`
   |
   |
LL |         let x = "hello".to_owned().into_boxed_str();
   |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
LL |         drop_unsized(x);
   |                      - value moved here
LL |         let _y = *x; //~ERROR use of moved value
   |                  ^^ value used here after move
help: consider cloning `x`
   |
LL |         drop_unsized(x.clone());
   |                       ++++++++
   |                       ++++++++

error[E0382]: use of moved value: `y`
  --> /checkout/src/test/ui/unsized-locals/double-move.rs:40:9
   |
LL |         let y = *x;
   |             - move occurs because `y` has type `str`, which does not implement the `Copy` trait
LL |         y.foo();
   |           ----- `y` moved due to this method call
LL |         y.foo(); //~ERROR use of moved value
   |         ^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `y`
   |
LL |     fn foo(self) -> String;
   |            ^^^^


error[E0382]: use of moved value: `x`
  --> /checkout/src/test/ui/unsized-locals/double-move.rs:46:9
   |
LL |         let _y = *x;
   |                  -- value moved here
LL |         x.foo(); //~ERROR use of moved value
   |         ^ value used here after move
   |
   = note: move occurs because `*x` has type `str`, which does not implement the `Copy` trait

error[E0382]: use of moved value: `*x`
   |
   |
LL |         let x = "hello".to_owned().into_boxed_str();
   |             - move occurs because `x` has type `Box<str>`, which does not implement the `Copy` trait
LL |         x.foo();
   |         - value moved here
LL |         let _y = *x; //~ERROR use of moved value
   |                  ^^ value used here after move
help: consider cloning `x`
   |
   |
LL |         x.clone().foo();

error: aborting due to 6 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] ui/use/use-after-move-based-on-type.rs stdout ----
diff of stderr:

9    |                    ^ value borrowed here after move
11    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider cloning `x`
+    |
+    |
+ LL |     let _y = x.clone();
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args use/use-after-move-based-on-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/use/use-after-move-based-on-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-based-on-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/use/use-after-move-based-on-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: borrow of moved value: `x`
   |
   |
LL |     let x = "Hello!".to_string();
   |         - move occurs because `x` has type `String`, which does not implement the `Copy` trait
LL |     let _y = x;
   |              - value moved here
LL |     println!("{}", x); //~ ERROR borrow of moved value
   |                    ^ value borrowed here after move
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning `x`
   |
   |
LL |     let _y = x.clone();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
