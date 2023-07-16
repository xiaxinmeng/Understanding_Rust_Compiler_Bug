plain
.................................................................................................... 1100/11561
........................i........................................................................... 1200/11561
.................................................................................................... 1300/11561
.................................................................................................... 1400/11561
......................iiii.ii.i.............iF...........F.......................................... 1500/11561
......................................i............................................................. 1700/11561
.................................................................................................... 1800/11561
.......................................................i............................................ 1900/11561
.................................................................................................... 2000/11561
---
.................................................................................................... 2500/11561
.................................................................................................... 2600/11561
...........................................................i..i..................................... 2700/11561
.................................................................................................... 2800/11561
....................................F..............................iiiii............................ 2900/11561
.................................................................................................... 3100/11561
.................................................................................................... 3200/11561
.................................................................................................... 3300/11561
.................................................................................................... 3400/11561
---
.................................................................................................... 5700/11561
.......................................................................................i..........i. 5800/11561
.................................................................................................... 5900/11561
.................................................................................................... 6000/11561
....i.................F..................................................................F.......... 6100/11561
......................................ii.ii.......i...i............................................. 6300/11561
..........................................................................i....i.................... 6400/11561
..........i..........................i.............................................................. 6500/11561
...........................................................i........................................ 6600/11561
---
.................................................................................................... 9300/11561
.................................................................................................... 9400/11561
..........................................................................................i.....i... 9500/11561
.................................................................................................... 9600/11561
...................................iiiiiii..iiiiii.i................................................ 9700/11561
.................................................................................................... 9900/11561
.................................................................................................... 10000/11561
.................................................................................................... 10100/11561
.................................................................................................... 10200/11561
---
---- [ui] ui/codemap_tests/one_line.rs stdout ----
diff of stderr:

6    |     | |
7    |     | first borrow later used by call
8    |     first mutable borrow occurs here
+    |
+    = help: try adding a local storing the argument and then using the local in the call
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/one_line/one_line.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args codemap_tests/one_line.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codemap_tests/one_line.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/one_line" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codemap_tests/one_line/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `v` as mutable more than once at a time
   |
   |
LL |     v.push(v.pop().unwrap()); //~ ERROR cannot borrow
   |     - ---- ^ second mutable borrow occurs here
   |     | |
   |     | first borrow later used by call
   |     first mutable borrow occurs here
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/coercion/coerce-overloaded-autoderef-fail.rs stdout ----
diff of stderr:

29    |     |           |
30    |     |           first mutable borrow occurs here
31    |     first borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
32 
33 error[E0502]: cannot borrow `*x` as mutable because it is also borrowed as immutable
34   --> $DIR/coerce-overloaded-autoderef-fail.rs:28:5

39    |     |          immutable borrow occurs here
40    |     mutable borrow occurs here
41    |     immutable borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
43 error: aborting due to 4 previous errors
44 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-overloaded-autoderef-fail/coerce-overloaded-autoderef-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args coercion/coerce-overloaded-autoderef-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/coercion/coerce-overloaded-autoderef-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-overloaded-autoderef-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coercion/coerce-overloaded-autoderef-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `*x` as mutable more than once at a time
   |
   |
LL |     let y = borrow_mut(x);
   |                        - first mutable borrow occurs here
LL |     let z = borrow_mut(x);
   |                        ^ second mutable borrow occurs here
LL |     //~^ ERROR cannot borrow `*x` as mutable more than once at a time
LL |     drop((y, z));
   |           - first borrow later used here

error[E0506]: cannot assign to `**x` because it is borrowed
   |
   |
LL |     let y = borrow(x);
   |                    - borrow of `**x` occurs here
LL |     let z = borrow(x);
LL |     **x += 1;
   |     ^^^^^^^^ assignment to borrowed `**x` occurs here
LL |     //~^ ERROR cannot assign to `**x` because it is borrowed
LL |     drop((y, z));
   |           - borrow later used here

error[E0499]: cannot borrow `*x` as mutable more than once at a time
   |
LL |     borrow_mut2(x, x);
LL |     borrow_mut2(x, x);
   |     ----------- -  ^ second mutable borrow occurs here
   |     |           |
   |     |           first mutable borrow occurs here
   |     first borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call

error[E0502]: cannot borrow `*x` as mutable because it is also borrowed as immutable
   |
LL |     borrow2(x, x);
   |     -------^^^^-^
   |     |          |
   |     |          |
   |     |          immutable borrow occurs here
   |     mutable borrow occurs here
   |     immutable borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0499, E0502, E0506.
For more information about an error, try `rustc --explain E0499`.
---
---- [ui] ui/did_you_mean/issue-34126.rs stdout ----
diff of stderr:

15    |         |    |
16    |         |    immutable borrow later used by call
17    |         immutable borrow occurs here
+    |
+    = help: try adding a local storing the argument and then using the local in the call
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-34126/issue-34126.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/issue-34126.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-34126.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-34126" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-34126/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0596]: cannot borrow `self` as mutable, as it is not declared as mutable
  --> /checkout/src/test/ui/did_you_mean/issue-34126.rs:6:18
   |
LL |         self.run(&mut self); //~ ERROR cannot borrow
   |                  |
   |                  |
   |                  cannot borrow as mutable
   |                  try removing `&mut` here

error[E0502]: cannot borrow `self` as mutable because it is also borrowed as immutable
  --> /checkout/src/test/ui/did_you_mean/issue-34126.rs:6:18
   |
LL |         self.run(&mut self); //~ ERROR cannot borrow
   |         ---- --- ^^^^^^^^^ mutable borrow occurs here
   |         |    |
   |         |    immutable borrow later used by call
   |         immutable borrow occurs here
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0502, E0596.
For more information about an error, try `rustc --explain E0502`.
For more information about an error, try `rustc --explain E0502`.

------------------------------------------


---- [ui] ui/generator/retain-resume-ref.rs stdout ----
diff of stderr:

7    |                  ------ ^^^^^^^^^^ second mutable borrow occurs here
8    |                  |
9    |                  first borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/retain-resume-ref/retain-resume-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/retain-resume-ref.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/retain-resume-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/retain-resume-ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/retain-resume-ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `thing` as mutable more than once at a time
   |
   |
LL |     gen.as_mut().resume(&mut thing);
   |                         ---------- first mutable borrow occurs here
LL |     gen.as_mut().resume(&mut thing);
   |                  ------ ^^^^^^^^^^ second mutable borrow occurs here
   |                  |
   |                  first borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/issues/issue-11192.rs stdout ----
diff of stderr:

11    |     ---- ^^^^^ immutable borrow occurs here
12    |     |
13    |     mutable borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11192/issue-11192.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-11192.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-11192.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11192" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-11192/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0502]: cannot borrow `*ptr` as immutable because it is also borrowed as mutable
   |
   |
LL |     let mut test = |foo: &Foo| {
   |                    ----------- mutable borrow occurs here
LL |         println!("access {}", foo.x);
LL |         ptr = box Foo { x: ptr.x + 1 };
   |                            --- first borrow occurs due to use of `ptr` in closure
...
LL |     test(&*ptr);
   |     ---- ^^^^^ immutable borrow occurs here
   |     |
   |     mutable borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.


------------------------------------------


---- [ui] ui/issues/issue-18566.rs stdout ----
diff of stderr:

6    |           |  |
7    |           |  first borrow later used by call
8    |           first mutable borrow occurs here
+    |
+    = help: try adding a local storing the argument and then using the local in the call
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18566/issue-18566.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18566.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18566.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18566" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18566/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `*s` as mutable more than once at a time
   |
   |
LL |     MyPtr(s).poke(s);
   |           -  ---- ^ second mutable borrow occurs here
   |           |  |
   |           |  first borrow later used by call
   |           first mutable borrow occurs here
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/issues/issue-61623.rs stdout ----
diff of stderr:

15    |     |  |  first borrow occurs due to use of `x` in closure
16    |     |  immutable borrow occurs here
17    |     immutable borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61623/issue-61623.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-61623.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-61623.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61623" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61623/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0596]: cannot borrow `*x.1` as mutable, as it is behind a `&` reference
   |
   |
LL | fn f3<'a>(x: &'a ((), &'a mut ())) {
   |              -------------------- help: consider changing this to be a mutable reference: `&'a mut ((), &'a mut ())`
LL |     f2(|| x.0, f1(x.1))
   |                   ^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error[E0502]: cannot borrow `*x.1` as mutable because it is also borrowed as immutable
   |
   |
LL |     f2(|| x.0, f1(x.1))
   |     -- -- -       ^^^ mutable borrow occurs here
   |     |  |  |
   |     |  |  first borrow occurs due to use of `x` in closure
   |     |  immutable borrow occurs here
   |     immutable borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0502, E0596.
For more information about an error, try `rustc --explain E0502`.
---
---- [ui] ui/issues/issue-65131.rs stdout ----
diff of stderr:

6    |     |        |
7    |     |        first mutable borrow occurs here
8    |     first borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65131/issue-65131.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-65131.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65131.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65131" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65131/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |     get_pair(&mut x, &mut x);
   |     -------- ------  ^^^^^^ second mutable borrow occurs here
   |     |        |
   |     |        first mutable borrow occurs here
   |     first borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/moves/moves-based-on-type-no-recursive-stack-closure.rs stdout ----
diff of stderr:

6    |                     |
7    |                     first mutable borrow occurs here
8    |                     first borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
9 
10 error[E0382]: borrow of moved value: `f`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-no-recursive-stack-closure/moves-based-on-type-no-recursive-stack-closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-no-recursive-stack-closure/moves-based-on-type-no-recursive-stack-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args moves/moves-based-on-type-no-recursive-stack-closure.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/moves-based-on-type-no-recursive-stack-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-no-recursive-stack-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-no-recursive-stack-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `*f` as mutable more than once at a time
   |
   |
LL |                     (f.c)(f, true);
   |                     ----- ^ second mutable borrow occurs here
   |                     |
   |                     first mutable borrow occurs here
   |                     first borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call

error[E0382]: borrow of moved value: `f`
   |
   |
LL | fn conspirator<F>(mut f: F) where F: FnMut(&mut R, bool) {
   |                   ----- move occurs because `f` has type `F`, which does not implement the `Copy` trait
LL |     let mut r = R {c: Box::new(f)};
   |                                - value moved here
LL |     f(&mut r, false) //~ ERROR borrow of moved value
   |     ^ value borrowed here after move
help: consider further restricting this bound
   |
   |
LL | fn conspirator<F>(mut f: F) where F: FnMut(&mut R, bool) + Copy {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0382, E0499.
---

---- [ui] ui/nll/issue-51268.rs stdout ----
diff of stderr:

11    | |              ---- first borrow occurs due to use of `self` in closure
12 LL | |         });
13    | |__________^ mutable borrow occurs here
+    |
+    = help: try adding a local storing the argument and then using the local in the call
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51268/issue-51268.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-51268.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-51268.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51268" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-51268/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0502]: cannot borrow `self.thing` as mutable because it is also borrowed as immutable
   |
   |
LL |           self.thing.bar(|| {
   |           ^          --- -- immutable borrow occurs here
   |           |          |
   |  _________|          immutable borrow later used by call
   | |
LL | |         //~^ ERROR cannot borrow `self.thing` as mutable because it is also borrowed as immutable [E0502]
LL | |             &self.number;
   | |              ---- first borrow occurs due to use of `self` in closure
LL | |         });
   | |__________^ mutable borrow occurs here
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.


------------------------------------------


---- [ui] ui/regions/regions-adjusted-lvalue-op.rs stdout ----
diff of stderr:

6    |     |    |
7    |     |    mutable borrow later used by call
8    |     mutable borrow occurs here
+    |
+    = help: try adding a local storing the argument and then using the local in the call
9 
10 error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable

15    |       |  |
15    |       |  |
16    |       |  mutable borrow later used by call
17    |       mutable borrow occurs here
+    |
+    = help: try adding a local storing the argument and then using the local in the call
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-adjusted-lvalue-op/regions-adjusted-lvalue-op.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-adjusted-lvalue-op.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-adjusted-lvalue-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-adjusted-lvalue-op" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-adjusted-lvalue-op/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable
   |
   |
LL |     v[0].oh_no(&v); //~ ERROR cannot borrow `v` as immutable because
   |     -    ----- ^^ immutable borrow occurs here
   |     |    |
   |     |    mutable borrow later used by call
   |     mutable borrow occurs here
   |
   = help: try adding a local storing the argument and then using the local in the call

error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable
   |
   |
LL |     (*v).oh_no(&v); //~ ERROR cannot borrow `v` as immutable because
   |       -  ----- ^^ immutable borrow occurs here
   |       |  |
   |       |  mutable borrow later used by call
   |       mutable borrow occurs here
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0502`.


------------------------------------------


---- [ui] ui/unboxed-closures/unboxed-closures-recursive-fn-using-fn-mut.rs stdout ----
diff of stderr:

6    |         |
7    |         first mutable borrow occurs here
8    |         first borrow later used by call
+    |
+    = help: try adding a local storing the argument and then using the local in the call
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-recursive-fn-using-fn-mut/unboxed-closures-recursive-fn-using-fn-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-recursive-fn-using-fn-mut.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-recursive-fn-using-fn-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-recursive-fn-using-fn-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-recursive-fn-using-fn-mut/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `*self` as mutable more than once at a time
   |
   |
LL |         (self.func)(self, arg)
   |         ----------- ^^^^ second mutable borrow occurs here
   |         |
   |         first mutable borrow occurs here
   |         first borrow later used by call
   |
   = help: try adding a local storing the argument and then using the local in the call
error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.

---
test result: FAILED. 11454 passed; 12 failed; 95 ignored; 0 measured; 0 filtered out; finished in 133.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:19
