plain
............................................................i........................... 880/14141
........................................................................i............... 968/14141
........................................................................................ 1056/14141
........................................................................................ 1144/14141
................................F....................F....F........F.................... 1232/14141
...............F...........FF...F...................................................F... 1320/14141
......F..................F...................F...............................i.......... 1408/14141
.....................................................................................F.. 1496/14141
.........................................................................F.............. 1672/14141
........................................................................................ 1760/14141
........................................................................................ 1848/14141
.i...................i...........ii..................................................... 1936/14141
---

---- [ui] src/test/ui/borrowck/borrowck-field-sensitivity.rs stdout ----
diff of stderr:

134 LL |     let mut x: A;
135    |         ----- binding declared here but left uninitialized
136 LL |     x.b = Box::new(1);
-    |     ^^^ `x` partially assigned here but it isn't fully initialized
+    |     ^^^^^^^^^^^^^^^^^ `x` partially assigned here but it isn't fully initialized
138    |
139    = help: partial initialization isn't supported, fully initialize the binding with a default value and mutate it, or use `std::mem::MaybeUninit`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity/borrowck-field-sensitivity.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity/borrowck-field-sensitivity.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-field-sensitivity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-field-sensitivity/auxiliary"
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
error[E0382]: use of moved value: `x.b`
  --> /checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs:14:10
   |
   |
LL |     let y = A { a: 3, .. x };
   |             ---------------- value moved here
LL |     drop(*x.b); //~ ERROR use of moved value: `x.b`
   |          ^^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
error[E0382]: borrow of moved value: `x.b`
  --> /checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs:20:13
   |
LL |     drop(x.b);
LL |     drop(x.b);
   |          --- value moved here
LL |     let p = &x.b; //~ ERROR borrow of moved value: `x.b`
   |
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
error[E0382]: borrow of moved value: `x.b`
  --> /checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs:27:13
   |
   |
LL |     let _y = A { a: 3, .. x };
   |              ---------------- value moved here
LL |     let p = &x.b; //~ ERROR borrow of moved value: `x.b`
   |
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait

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
   |             -------- first mutable borrow occurs here
   |             -------- first mutable borrow occurs here
LL |     let q = &mut x.a; //~ ERROR cannot borrow `x.a` as mutable more than once at a time
   |             ^^^^^^^^ second mutable borrow occurs here
LL |     drop(*p);

error[E0382]: use of moved value: `x.b`
  --> /checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs:56:10
   |
   |
LL |     drop(x.b);
   |          --- value moved here
LL |     drop(x.b);  //~ ERROR use of moved value: `x.b`
   |          ^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `x.b`
  --> /checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs:62:10
   |
   |
LL |     let _y = A { a: 3, .. x };
   |              ---------------- value moved here
LL |     drop(x.b);  //~ ERROR use of moved value: `x.b`
   |          ^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `x.b`
  --> /checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs:68:14
   |
LL |     drop(x.b);
LL |     drop(x.b);
   |          --- value moved here
LL |     let _z = A { a: 3, .. x };  //~ ERROR use of moved value: `x.b`
   |              ^^^^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `x.b`
  --> /checkout/src/test/ui/borrowck/borrowck-field-sensitivity.rs:74:14
   |
   |
LL |     let _y = A { a: 3, .. x };
   |              ---------------- value moved here
LL |     let _z = A { a: 4, .. x };  //~ ERROR use of moved value: `x.b`
   |              ^^^^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `x.b` has type `Box<isize>`, which does not implement the `Copy` trait

error[E0381]: partially assigned binding `x` isn't fully initialized
   |
LL |     let mut x: A;
LL |     let mut x: A;
   |         ----- binding declared here but left uninitialized
LL |     x.a = 1; //~ ERROR E0381
   |     ^^^^^^^ `x` partially assigned here but it isn't fully initialized
   |
   = help: partial initialization isn't supported, fully initialize the binding with a default value and mutate it, or use `std::mem::MaybeUninit`

error[E0381]: partially assigned binding `x` isn't fully initialized
   |
LL |     let mut x: A;
LL |     let mut x: A;
   |         ----- binding declared here but left uninitialized
LL |     x.a = 1; //~ ERROR E0381
   |     ^^^^^^^ `x` partially assigned here but it isn't fully initialized
   |
   = help: partial initialization isn't supported, fully initialize the binding with a default value and mutate it, or use `std::mem::MaybeUninit`

error[E0381]: partially assigned binding `x` isn't fully initialized
   |
LL |     let mut x: A;
LL |     let mut x: A;
   |         ----- binding declared here but left uninitialized
LL |     x.b = Box::new(1); //~ ERROR E0381
   |     ^^^^^^^^^^^^^^^^^ `x` partially assigned here but it isn't fully initialized
   |
   = help: partial initialization isn't supported, fully initialize the binding with a default value and mutate it, or use `std::mem::MaybeUninit`
error: aborting due to 14 previous errors

Some errors have detailed explanations: E0381, E0382, E0499, E0505.
For more information about an error, try `rustc --explain E0381`.
---
To only update this specific test, also pass `--test-args borrowck/borrowck-issue-48962.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-issue-48962.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-48962" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-issue-48962/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `src`
   |
LL |     let mut src = &mut node;
LL |     let mut src = &mut node;
   |         ------- move occurs because `src` has type `&mut Node`, which does not implement the `Copy` trait
LL |     {src};
   |      --- value moved here
LL |     src.next = None; //~ ERROR use of moved value: `src` [E0382]
   |     ^^^^^^^^^^^^^^^ value used here after move
error[E0382]: use of moved value: `src`
  --> /checkout/src/test/ui/borrowck/borrowck-issue-48962.rs:20:5
   |
   |
LL |     let mut src = &mut (22, 44);
   |         ------- move occurs because `src` has type `&mut (i32, i32)`, which does not implement the `Copy` trait
LL |     {src};
   |      --- value moved here
LL |     src.0 = 66; //~ ERROR use of moved value: `src` [E0382]
   |     ^^^^^^^^^^ value used here after move
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0382`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-lend-flow-loop.rs stdout ----
diff of stderr:

8    |                ^^^ immutable borrow occurs here
9 LL |     }
10 LL |     *x = Box::new(5);
-    |     -- mutable borrow later used here
+    |     ---------------- mutable borrow later used here
12 
13 error[E0502]: cannot borrow `*v` as immutable because it is also borrowed as mutable


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-lend-flow-loop/borrowck-lend-flow-loop.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-lend-flow-loop/borrowck-lend-flow-loop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-lend-flow-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-lend-flow-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-lend-flow-loop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-lend-flow-loop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0502]: cannot borrow `*v` as immutable because it is also borrowed as mutable
   |
LL |     let mut x = &mut v;
LL |     let mut x = &mut v;
   |                 ------ mutable borrow occurs here
LL |     for _ in 0..3 {
LL |         borrow(&*v); //~ ERROR cannot borrow
   |                ^^^ immutable borrow occurs here
LL |     }
LL |     *x = Box::new(5);
   |     ---------------- mutable borrow later used here

error[E0502]: cannot borrow `*v` as immutable because it is also borrowed as mutable
   |
   |
LL |         **x += 1;
   |         -------- mutable borrow later used here
LL |         borrow(&*v); //~ ERROR cannot borrow
   |                ^^^ immutable borrow occurs here
LL |         if cond2 {
LL |             x = &mut v; // OK
   |                 ------ mutable borrow occurs here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0502`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616.rs stdout ----
diff of stderr:

7    |                type annotation requires that `*s` is borrowed for `'static`
9 LL |     *s = String::new();
9 LL |     *s = String::new();
-    |     ^^ assignment to borrowed `*s` occurs here
+    |     ^^^^^^^^^^^^^^^^^^ assignment to borrowed `*s` occurs here
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616/borrowck-loan-of-static-data-issue-27616.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-loan-of-static-data-issue-27616.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-loan-of-static-data-issue-27616/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `*s` because it is borrowed
   |
LL |     let alias: &'static mut String = s;
LL |     let alias: &'static mut String = s;
   |                -------------------   - borrow of `*s` occurs here
   |                |
   |                type annotation requires that `*s` is borrowed for `'static`
...
LL |     *s = String::new(); //~ ERROR cannot assign
   |     ^^^^^^^^^^^^^^^^^^ assignment to borrowed `*s` occurs here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [ui] src/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs stdout ----
diff of stderr:

4 LL |       Cycle::Node(ref mut y) => {
5    |                   --------- borrow of `x.0` occurs here
6 LL |         y.a = x;
-    |         ---   ^ move out of `x` occurs here
+    |         ------^
+    |         |     |
+    |         |     |
+    |         |     move out of `x` occurs here
10 
11 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap/borrowck-no-cycle-in-exchange-heap.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-no-cycle-in-exchange-heap.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-no-cycle-in-exchange-heap/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0505]: cannot move out of `x` because it is borrowed
   |
   |
LL |       Cycle::Node(ref mut y) => {
   |                   --------- borrow of `x.0` occurs here
LL |         y.a = x; //~ ERROR cannot move out of
   |         |     |
   |         |     |
   |         |     move out of `x` occurs here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0505`.
For more information about this error, try `rustc --explain E0505`.
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-partial-reinit-2.rs stdout ----
diff of stderr:

6 LL |     let mut u = Test { a: 2, b: Some(Box::new(t))};
7    |                                               - value moved here
8 LL |     t.b = Some(Box::new(u));
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ value assigned here after move
10 
11 error: aborting due to previous error
12 
12 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-2/borrowck-partial-reinit-2.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-partial-reinit-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-partial-reinit-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: assign of moved value: `t`
   |
   |
LL |     let mut t = Test { a: 1, b: None};
   |         ----- move occurs because `t` has type `Test`, which does not implement the `Copy` trait
LL |     let mut u = Test { a: 2, b: Some(Box::new(t))};
   |                                               - value moved here
LL |     t.b = Some(Box::new(u));

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
---
diff of stderr:

7 LL |     drop(t);
8    |          - value moved here
9 LL |     t.b = Some(u);
+    |     ^^^^^^^^^^^^^ value assigned here after move
11 
12 error[E0382]: assign of moved value: `t`
13   --> $DIR/borrowck-partial-reinit-1.rs:33:5
13   --> $DIR/borrowck-partial-reinit-1.rs:33:5

18 LL |     drop(t);
19    |          - value moved here
20 LL |     t.0 = Some(u);
+    |     ^^^^^^^^^^^^^ value assigned here after move
22 
23 error: aborting due to 2 previous errors
24 
24 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-1/borrowck-partial-reinit-1.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-partial-reinit-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-partial-reinit-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: assign of moved value: `t`
   |
   |
LL |     let mut t = Test2 { b: None };
   |         ----- move occurs because `t` has type `Test2`, which does not implement the `Copy` trait
LL |     let u = Test;
LL |     drop(t);
   |          - value moved here
LL |     t.b = Some(u);

error[E0382]: assign of moved value: `t`
  --> /checkout/src/test/ui/borrowck/borrowck-partial-reinit-1.rs:33:5
   |
   |
LL |     let mut t = Test3(None);
   |         ----- move occurs because `t` has type `Test3`, which does not implement the `Copy` trait
LL |     let u = Test;
LL |     drop(t);
   |          - value moved here
LL |     t.0 = Some(u);

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-partial-reinit-4.rs stdout ----
diff of stderr:

4 LL |     let mut x : (Test2, Test2);
5    |         ----- binding declared here but left uninitialized
6 LL |     (x.0).0 = Some(Test);
-    |     ^^^^^^^ `x.0` assigned here but it isn't fully initialized
+    |     ^^^^^^^^^^^^^^^^^^^^ `x.0` assigned here but it isn't fully initialized
8    |
9    = help: partial initialization isn't supported, fully initialize the binding with a default value and mutate it, or use `std::mem::MaybeUninit`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-4/borrowck-partial-reinit-4.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-partial-reinit-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-partial-reinit-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-partial-reinit-4/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0381]: assigned binding `x.0` isn't fully initialized
   |
   |
LL |     let mut x : (Test2, Test2);
   |         ----- binding declared here but left uninitialized
LL |     (x.0).0 = Some(Test); //~ ERROR E0381
   |     ^^^^^^^^^^^^^^^^^^^^ `x.0` assigned here but it isn't fully initialized
   |
   = help: partial initialization isn't supported, fully initialize the binding with a default value and mutate it, or use `std::mem::MaybeUninit`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0381`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-vec-pattern-nesting.rs stdout ----
diff of stderr:

5    |              ------ borrow of `vec[_]` occurs here
6 LL |
7 LL |             vec[0] = Box::new(4);
-    |             ^^^^^^ assignment to borrowed `vec[_]` occurs here
+    |             ^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `vec[_]` occurs here
9 LL |
10 LL |             _a.use_ref();


17    |               ------ borrow of `vec[_]` occurs here
18 LL |
19 LL |             vec[0] = Box::new(4);
-    |             ^^^^^^ assignment to borrowed `vec[_]` occurs here
+    |             ^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `vec[_]` occurs here
21 LL |
22 LL |             _b.use_ref();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting/borrowck-vec-pattern-nesting.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting/borrowck-vec-pattern-nesting.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-vec-pattern-nesting.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-vec-pattern-nesting.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-vec-pattern-nesting/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `vec[_]` because it is borrowed
   |
   |
LL |         [box ref _a, _, _] => {
   |              ------ borrow of `vec[_]` occurs here
LL |         //~^ NOTE borrow of `vec[_]` occurs here
LL |             vec[0] = Box::new(4); //~ ERROR cannot assign
   |             ^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `vec[_]` occurs here
LL |             //~^ NOTE assignment to borrowed `vec[_]` occurs here
LL |             _a.use_ref();


error[E0506]: cannot assign to `vec[_]` because it is borrowed
   |
   |
LL |         &mut [ref _b @ ..] => {
   |               ------ borrow of `vec[_]` occurs here
LL |         //~^ borrow of `vec[_]` occurs here
LL |             vec[0] = Box::new(4); //~ ERROR cannot assign
   |             ^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `vec[_]` occurs here
LL |             //~^ NOTE assignment to borrowed `vec[_]` occurs here
LL |             _b.use_ref();


error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
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
help: consider removing the mutable borrow
   |
LL -         &mut [_a,
LL +         [_a,
LL +         [_a,
   |

error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
   |
   |
LL |     let a = vec[0]; //~ ERROR cannot move out
   |             |
   |             cannot move out of here
   |             cannot move out of here
   |             move occurs because `vec[_]` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider borrowing here
   |
   |
LL |     let a = &vec[0]; //~ ERROR cannot move out


error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
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
help: consider removing the mutable borrow
   |
LL -         &mut [
LL +         [
LL +         [
   |

error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
   |
   |
LL |     let a = vec[0]; //~ ERROR cannot move out
   |             |
   |             cannot move out of here
   |             cannot move out of here
   |             move occurs because `vec[_]` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider borrowing here
   |
   |
LL |     let a = &vec[0]; //~ ERROR cannot move out


error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
   |
LL |     match vec {
   |           ^^^ cannot move out of here
...
...
LL |         &mut [_a, _b, _c] => {}
   |               --  --  -- ...and here
   |               |   |
   |               |   ...and here
   |               data moved here
   = note: move occurs because these variables have types that don't implement the `Copy` trait
help: consider removing the mutable borrow
   |
   |
LL -         &mut [_a, _b, _c] => {}
LL +         [_a, _b, _c] => {}


error[E0508]: cannot move out of type `[Box<isize>]`, a non-copy slice
   |
   |
LL |     let a = vec[0]; //~ ERROR cannot move out
   |             |
   |             cannot move out of here
   |             cannot move out of here
   |             move occurs because `vec[_]` has type `Box<isize>`, which does not implement the `Copy` trait
help: consider borrowing here
   |
   |
LL |     let a = &vec[0]; //~ ERROR cannot move out

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0506, E0508.
---
diff of stderr:

14   --> $DIR/index-mut-help.rs:11:5
15    |
16 LL |     map["peter"] = "0".to_string();
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot assign
18    |
18    |
19    = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `HashMap<&str, String>`
- help: to modify a `HashMap<&str, String>`, use `.get_mut()`, `.insert()` or the entry API
-    |
- LL |     map.insert("peter", "0".to_string());
-    |        ~~~~~~~~       ~                +
- LL |     map.get_mut("peter").map(|val| { *val = "0".to_string(); });
-    |        ~~~~~~~~~       ~~~~~~~~~~~~~~~~~~                  ++++
- LL |     let val = map.entry("peter").or_insert("0".to_string());
-    |     +++++++++    ~~~~~~~       ~~~~~~~~~~~~               +
+    = help: to modify a `HashMap<&str, String>`, use `.get_mut()`, `.insert()` or the entry API
28 
29 error[E0596]: cannot borrow data in an index of `HashMap<&str, String>` as mutable


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/index-mut-help/index-mut-help.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/index-mut-help/index-mut-help.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/index-mut-help.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/index-mut-help.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/index-mut-help" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/index-mut-help/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow data in an index of `HashMap<&str, String>` as mutable
   |
   |
LL |     map["peter"].clear();           //~ ERROR
   |     ^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
   |
   = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `HashMap<&str, String>`
help: to modify a `HashMap<&str, String>` use `.get_mut()`
   |
LL |     map.get_mut("peter").map(|val| val.clear());           //~ ERROR


error[E0594]: cannot assign to data in an index of `HashMap<&str, String>`
   |
   |
LL |     map["peter"] = "0".to_string(); //~ ERROR
   |
   |
   = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `HashMap<&str, String>`
   = help: to modify a `HashMap<&str, String>`, use `.get_mut()`, `.insert()` or the entry API

error[E0596]: cannot borrow data in an index of `HashMap<&str, String>` as mutable
   |
   |
LL |     let _ = &mut map["peter"];      //~ ERROR
   |             ^^^^^^^^^^^^^^^^^ cannot borrow as mutable
   |
   = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `HashMap<&str, String>`
   = help: to modify a `HashMap<&str, String>`, use `.get_mut()`, `.insert()` or the entry API
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0594, E0596.
For more information about an error, try `rustc --explain E0594`.
---
diff of stderr:

5    |         - help: consider making this binding mutable: `mut b`
6 ...
7 LL |     b = Box::new(1);
-    |     - first assignment to `b`
+    |     --------------- first assignment to `b`
9 LL |     b = Box::new(2);
-    |     ^ cannot assign twice to immutable variable
+    |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
12 error[E0384]: cannot assign twice to immutable variable `b`
13   --> $DIR/issue-45199.rs:14:5

19    |         help: consider making this binding mutable: `mut b`
19    |         help: consider making this binding mutable: `mut b`
20 ...
21 LL |     b = Box::new(2);
-    |     ^ cannot assign twice to immutable variable
+    |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
24 error[E0384]: cannot assign to immutable argument `b`
25   --> $DIR/issue-45199.rs:20:5

28    |              - help: consider making this binding mutable: `mut b`
28    |              - help: consider making this binding mutable: `mut b`
29 LL |
30 LL |     b = Box::new(2);
-    |     ^ cannot assign to immutable argument
+    |     ^^^^^^^^^^^^^^^ cannot assign to immutable argument
33 error: aborting due to 3 previous errors
34 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45199/issue-45199.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-45199.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-45199.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45199" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-45199/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0384]: cannot assign twice to immutable variable `b`
   |
   |
LL |     let b: Box<isize>;
   |         - help: consider making this binding mutable: `mut b`
...
LL |     b = Box::new(1);    //~ NOTE first assignment
   |     --------------- first assignment to `b`
LL |     b = Box::new(2);    //~ ERROR cannot assign twice to immutable variable `b`
   |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
error[E0384]: cannot assign twice to immutable variable `b`
  --> /checkout/src/test/ui/borrowck/issue-45199.rs:14:5
   |
   |
LL |     let b = Box::new(1);    //~ NOTE first assignment
   |         |
   |         first assignment to `b`
   |         help: consider making this binding mutable: `mut b`
...
...
LL |     b = Box::new(2);        //~ ERROR cannot assign twice to immutable variable `b`
   |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
error[E0384]: cannot assign to immutable argument `b`
  --> /checkout/src/test/ui/borrowck/issue-45199.rs:20:5
   |
   |
LL | fn test_args(b: Box<i32>) {  //~ HELP consider making this binding mutable
   |              - help: consider making this binding mutable: `mut b`
LL |                                 //~| SUGGESTION mut b
LL |     b = Box::new(2);            //~ ERROR cannot assign to immutable argument `b`
   |     ^^^^^^^^^^^^^^^ cannot assign to immutable argument
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0384`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-58776-borrowck-scans-children.rs stdout ----
diff of stderr:

7    |                borrow of `greeting` occurs here
8 LL |
9 LL |     greeting = "DEALLOCATED".to_string();
-    |     ^^^^^^^^ assignment to borrowed `greeting` occurs here
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `greeting` occurs here
11 ...
12 LL |     println!("thread result: {:?}", res);

14 
14 
- error[E0505]: cannot move out of `greeting` because it is borrowed
-   --> $DIR/issue-58776-borrowck-scans-children.rs:7:10
-    |
- LL |     let res = (|| (|| &greeting)())();
-    |                --      -------- borrow occurs due to use in closure
-    |                |
-    |                borrow of `greeting` occurs here
- ...
- LL |     drop(greeting);
-    |          ^^^^^^^^ move out of `greeting` occurs here
- ...
- LL |     println!("thread result: {:?}", res);
+ error: aborting due to previous error
28 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args borrowck/issue-58776-borrowck-scans-children.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-58776-borrowck-scans-children.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-58776-borrowck-scans-children" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-58776-borrowck-scans-children/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `greeting` because it is borrowed
   |
   |
LL |     let res = (|| (|| &greeting)())();
   |                --      -------- borrow occurs due to use in closure
   |                |
   |                borrow of `greeting` occurs here
LL |
LL |     greeting = "DEALLOCATED".to_string();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `greeting` occurs here
...
LL |     println!("thread result: {:?}", res);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
For more information about this error, try `rustc --explain E0506`.
------------------------------------------


---- [ui] src/test/ui/c-variadic/variadic-ffi-4.rs stdout ----
diff of stderr:

55    |                                               |
56    |                                               has type `&mut VaListImpl<'1>`
57 LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'1` must outlive `'2`
+    |     ^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
59    |
60    = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
61    = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`
69    |                                               |
70    |                                               has type `&mut VaListImpl<'1>`
71 LL |     *ap0 = ap1;
71 LL |     *ap0 = ap1;
-    |     ^^^^ assignment requires that `'2` must outlive `'1`
+    |     ^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
73    |
74    = note: requirement occurs because of the type `VaListImpl<'_>`, which makes the generic argument `'_` invariant
75    = note: the struct `VaListImpl<'f>` is invariant over the parameter `'f`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/variadic-ffi-4.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/auxiliary"
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
   |     ^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
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
   |     ^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
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
   |     assignment requires that `ap1` is borrowed for `'3`
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


---- [ui] src/test/ui/closures/2229_closure_analysis/diagnostics/box.rs stdout ----
diff of stderr:

7    |         --------- borrow occurs due to use in closure
8 ...
9 LL |     e.0.0.m.x = format!("not-x");
-    |     ^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
12 LL |     c();
13    |     - borrow later used here

37    |                        --------- borrow occurs due to use in closure
37    |                        --------- borrow occurs due to use in closure
38 ...
39 LL |     e.0.0.m.x = format!("not-x");
-    |     ^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
42 LL |     c();
43    |     - borrow later used here



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/box/box.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/box.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/box.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/box" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/box/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `e.0.0.m.x` because it is borrowed
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- borrow of `e.0.0.m.x` occurs here
LL |         e.0.0.m.x = format!("not-x");
   |         --------- borrow occurs due to use in closure
...
LL |     e.0.0.m.x = format!("not-x");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
LL |     //~^ ERROR: cannot assign to `e.0.0.m.x` because it is borrowed
LL |     c();


error[E0502]: cannot borrow `e.0.0.m.x` as immutable because it is also borrowed as mutable
   |
LL |     let mut c = || {
LL |     let mut c = || {
   |                 -- mutable borrow occurs here
LL |         e.0.0.m.x = format!("not-x");
   |         --------- first borrow occurs due to use of `e.0.0.m.x` in closure
...
LL |     println!("{}", e.0.0.m.x);
   |                    ^^^^^^^^^ immutable borrow occurs here
LL |     //~^ ERROR: cannot borrow `e.0.0.m.x` as immutable because it is also borrowed as mutable
LL |     c();
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0506]: cannot assign to `e.0.0.m.x` because it is borrowed
   |
LL |     let c = || {
LL |     let c = || {
   |             -- borrow of `e.0.0.m.x` occurs here
LL |         println!("{}", e.0.0.m.x);
   |                        --------- borrow occurs due to use in closure
...
LL |     e.0.0.m.x = format!("not-x");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `e.0.0.m.x` occurs here
LL |     //~^ ERROR: cannot assign to `e.0.0.m.x` because it is borrowed
LL |     c();

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0502, E0506.
---
diff of stderr:

2   --> $DIR/const_let.rs:16:32
3    |
4 LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
-    |                                ^^^^^                  - value is dropped here
+    |                                ^^^^^                  ----------------- value is dropped here
7    |                                the destructor for this type cannot be evaluated in constants
8 

10   --> $DIR/const_let.rs:20:33
10   --> $DIR/const_let.rs:20:33
11    |
12 LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
-    |                                 ^^^^^                     - value is dropped here
+    |                                 ^^^^^                     ----------------- value is dropped here
15    |                                 the destructor for this type cannot be evaluated in constants
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/const_let.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_let/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0493]: destructor of `FakeNeedsDrop` cannot be evaluated at compile-time
   |
   |
LL | const Y: FakeNeedsDrop = { let mut x = FakeNeedsDrop; x = FakeNeedsDrop; x };
   |                                ^^^^^                  ----------------- value is dropped here
   |                                the destructor for this type cannot be evaluated in constants


error[E0493]: destructor of `FakeNeedsDrop` cannot be evaluated at compile-time
   |
   |
LL | const Y2: FakeNeedsDrop = { let mut x; x = FakeNeedsDrop; x = FakeNeedsDrop; x };
   |                                 ^^^^^                     ----------------- value is dropped here
   |                                 the destructor for this type cannot be evaluated in constants


error[E0493]: destructor of `Option<FakeNeedsDrop>` cannot be evaluated at compile-time
   |
   |
LL | const Z: () = { let mut x = None; x = Some(FakeNeedsDrop); };
   |                     ^^^^^                                  - value is dropped here
   |                     the destructor for this type cannot be evaluated in constants


error[E0493]: destructor of `Option<FakeNeedsDrop>` cannot be evaluated at compile-time
   |
   |
LL | const Z2: () = { let mut x; x = None; x = Some(FakeNeedsDrop); };
   |                      ^^^^^                                     - value is dropped here
   |                      the destructor for this type cannot be evaluated in constants

error: aborting due to 4 previous errors

---
diff of stderr:

5    |         ^^^^^^^^^^^^^^^^^^^ the destructor for this type cannot be evaluated in constants
6 ...
7 LL |         always_returned = never_returned;
-    |         --------------- value is dropped here
+    |         -------------------------------- value is dropped here
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/livedrop/livedrop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/livedrop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/livedrop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/livedrop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/livedrop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0493]: destructor of `Option<Vec<i32>>` cannot be evaluated at compile-time
   |
   |
LL |     let mut always_returned = None; //~ ERROR destructor of
   |         ^^^^^^^^^^^^^^^^^^^ the destructor for this type cannot be evaluated in constants
LL |         always_returned = never_returned;
LL |         always_returned = never_returned;
   |         -------------------------------- value is dropped here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0493`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/liveness/liveness-assign/liveness-assign-imm-local-with-drop.rs stdout ----
diff of stderr:

8    |         help: consider making this binding mutable: `mut b`
9 ...
10 LL |     b = Box::new(2);
-    |     ^ cannot assign twice to immutable variable
+    |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-assign/liveness-assign-imm-local-with-drop/liveness-assign-imm-local-with-drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args liveness/liveness-assign/liveness-assign-imm-local-with-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/liveness/liveness-assign/liveness-assign-imm-local-with-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-assign/liveness-assign-imm-local-with-drop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/liveness/liveness-assign/liveness-assign-imm-local-with-drop/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0384]: cannot assign twice to immutable variable `b`
   |
   |
LL |     let b = Box::new(1); //~ NOTE first assignment
   |         |
   |         first assignment to `b`
   |         help: consider making this binding mutable: `mut b`
...
...
LL |     b = Box::new(2); //~ ERROR cannot assign twice to immutable variable `b`
   |     ^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0384`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/mir/drop-elaboration-after-borrowck-error.rs stdout ----
diff of stderr:

2   --> $DIR/drop-elaboration-after-borrowck-error.rs:7:5
3    |
4 LL |     a[0] = String::new();
+    |     ^^^^^^^^^^^^^^^^^^^^
6    |     |
7    |     the destructor for this type cannot be evaluated in statics
8    |     value is dropped here
8    |     value is dropped here

34   --> $DIR/drop-elaboration-after-borrowck-error.rs:18:9
35    |
36 LL |         self.0[0] = other;
+    |         ^^^^^^^^^^^^^^^^^
38    |         |
39    |         the destructor for this type cannot be evaluated in constant functions
40    |         value is dropped here
---
To only update this specific test, also pass `--test-args mir/drop-elaboration-after-borrowck-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/drop-elaboration-after-borrowck-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/drop-elaboration-after-borrowck-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/drop-elaboration-after-borrowck-error/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0493]: destructor of `String` cannot be evaluated at compile-time
   |
   |
LL |     a[0] = String::new();
   |     |
   |     the destructor for this type cannot be evaluated in statics
   |     value is dropped here


error[E0493]: destructor of `[String; 1]` cannot be evaluated at compile-time
   |
   |
LL |     let a: [String; 1];
   |         ^ the destructor for this type cannot be evaluated in statics
LL | };
LL | };
   | - value is dropped here
error[E0381]: used binding `a` isn't initialized
  --> /checkout/src/test/ui/mir/drop-elaboration-after-borrowck-error.rs:7:5
   |
   |
LL |     let a: [String; 1];
   |         - binding declared here but left uninitialized
LL |     //~^ ERROR destructor of
LL |     a[0] = String::new();
   |     ^^^^ `a` used here but it isn't initialized
help: consider assigning a value
   |
   |
LL |     let a: [String; 1] = todo!();


error[E0493]: destructor of `T` cannot be evaluated at compile-time
   |
   |
LL |         self.0[0] = other;
   |         |
   |         the destructor for this type cannot be evaluated in constant functions
   |         value is dropped here


error[E0493]: destructor of `B<T>` cannot be evaluated at compile-time
   |
LL |         let _this = self;
   |             ^^^^^ the destructor for this type cannot be evaluated in constant functions
...
...
LL |     }
   |     - value is dropped here

error[E0382]: use of moved value: `self.0`
  --> /checkout/src/test/ui/mir/drop-elaboration-after-borrowck-error.rs:18:9
   |
LL |     pub const fn f(mut self, other: T) -> Self {
   |                    -------- move occurs because `self` has type `B<T>`, which does not implement the `Copy` trait
LL |         let _this = self;
   |                     ---- value moved here
LL |         //~^ ERROR destructor of
LL |         self.0[0] = other;
   |         ^^^^^^^^^ value used here after move
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0381, E0382, E0493.
For more information about an error, try `rustc --explain E0381`.
For more information about an error, try `rustc --explain E0381`.
------------------------------------------


---- [ui] src/test/ui/nll/issue-27868.rs stdout ----
diff of stderr:

7    |  _____borrow of `vecvec` occurs here
8    | |
9 LL | |         vecvec = vec![];
-    | |         ^^^^^^ assignment to borrowed `vecvec` occurs here
+    | |         ^^^^^^^^^^^^^^^ assignment to borrowed `vecvec` occurs here
11 LL | |
12 LL | |         0
13 LL | |     };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27868/issue-27868.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-27868.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-27868.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27868" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-27868/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `vecvec` because it is borrowed
   |
   |
LL |       vecvec[0] += {
   |       |
   |       |
   |  _____borrow of `vecvec` occurs here
   | |
LL | |         vecvec = vec![];
   | |         ^^^^^^^^^^^^^^^ assignment to borrowed `vecvec` occurs here
LL | |         //~^ ERROR cannot assign to `vecvec` because it is borrowed [E0506]
LL | |         0
LL | |     };
   | |_____- borrow later used here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs stdout ----
diff of stderr:

25    |                   --------------- help: add explicit lifetime `'b` to the type of `ss`: `&mut SomeStruct<'b>`
26 ...
27 LL |     ss.r = b;
-    |     ^^^^ lifetime `'b` required
+    |     ^^^^^^^^ lifetime `'b` required
30 error: aborting due to 3 previous errors
31 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error/object-lifetime-default-from-box-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-from-box-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait> {
   |         -- has type `&mut SomeStruct<'1>`
LL |     ss.r
LL |     ss.r
   |     ^^^^ returning this value requires that `'1` must outlive `'static`
   |
help: to declare that the trait object captures data from argument `ss`, you can add an explicit `'_` lifetime bound
   |
LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait + '_> {


error[E0507]: cannot move out of `ss.r` which is behind a mutable reference
   |
LL |     ss.r
LL |     ss.r
   |     ^^^^ move occurs because `ss.r` has type `Box<dyn SomeTrait>`, which does not implement the `Copy` trait

error[E0621]: explicit lifetime required in the type of `ss`
   |
   |
LL | fn store1<'b>(ss: &mut SomeStruct, b: Box<dyn SomeTrait+'b>) {
   |                   --------------- help: add explicit lifetime `'b` to the type of `ss`: `&mut SomeStruct<'b>`
...
LL |     ss.r = b; //~ ERROR explicit lifetime required in the type of `ss` [E0621]
   |     ^^^^^^^^ lifetime `'b` required
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0507, E0621.
For more information about an error, try `rustc --explain E0507`.
For more information about an error, try `rustc --explain E0507`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-infer-paramd-indirect.rs stdout ----
diff of stderr:

7 LL |     fn set_f_bad(&mut self, b: Box<B>) {
8    |                             - has type `Box<Box<&'1 isize>>`
9 LL |         self.f = b;
-    |         ^^^^^^ assignment requires that `'1` must outlive `'a`
+    |         ^^^^^^^^^^ assignment requires that `'1` must outlive `'a`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect/regions-infer-paramd-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-paramd-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-paramd-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-paramd-indirect/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl<'a> SetF<'a> for C<'a> {
   |      -- lifetime `'a` defined here
...
LL |     fn set_f_bad(&mut self, b: Box<B>) {
   |                             - has type `Box<Box<&'1 isize>>`
LL |         self.f = b;
   |         ^^^^^^^^^^ assignment requires that `'1` must outlive `'a`
error: aborting due to previous error
------------------------------------------


---
23 LL |     factorial = Some(Box::new(f));
-    |     ^^^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
25    |     |
26    |     assignment to borrowed `factorial` occurs here

52    |                 --------- borrow occurs due to use in closure
53 ...
54 LL |     factorial = Some(Box::new(f));
54 LL |     factorial = Some(Box::new(f));
-    |     ^^^^^^^^^ assignment to borrowed `factorial` occurs here
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `factorial` occurs here
57 error: aborting due to 4 previous errors
58 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1/unboxed-closures-failed-recursive-fn-1.stderr
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-failed-recursive-fn-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-failed-recursive-fn-1/auxiliary"
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
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     assignment to borrowed `factorial` occurs here


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
   |                        ----------------------------------------- type annotation requires that `factorial` is borrowed for `'static`
LL |
LL |     let f = |x: u32| -> u32 {
   |             --------------- borrow of `factorial` occurs here
LL |         let g = factorial.as_ref().unwrap();
   |                 --------- borrow occurs due to use in closure
LL |     factorial = Some(Box::new(f));
LL |     factorial = Some(Box::new(f));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assignment to borrowed `factorial` occurs here
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0506, E0597.
For more information about an error, try `rustc --explain E0506`.
