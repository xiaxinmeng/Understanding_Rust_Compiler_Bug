plain
..........................................................................iiiiiiiiiiiiii 176/13422
.......................i...............i................................................ 264/13422
........................................................................................ 352/13422
........................................................................................ 440/13422
...............F.....F.................................................................. 528/13422
..........................................F............................................. 704/13422
................................F..............................i........................ 792/13422
...........................................................i............................ 880/13422
........................................................................................ 968/13422
---
........................................................................................ 5896/13422
........................................................................................ 5984/13422
........................................................................................ 6072/13422
........................................................................................ 6160/13422
.................................................................F..F................... 6248/13422
......................................i.............F................................... 6424/13422
.................................F...................................................... 6512/13422
..................................i..................................................... 6600/13422
........................................................................................ 6688/13422
........................................................................................ 6688/13422
...........i.......................................................ii.ii........i....i.. 6776/13422
.............................................................i.......................... 6864/13422
........................................................................................ 6952/13422
.....F..FF.................F.....i....iF........................................i....... 7040/13422
........................................................................................ 7216/13422
........................................................................................ 7216/13422
..............i................................................F......F................. 7304/13422
F..................F.........F....F......................F.............................. 7392/13422
...........ii......................................ii................................... 7568/13422
..............................i......................................................... 7656/13422
........................................................................................ 7744/13422
......................................ii..................................FF............ 7832/13422
......................................ii..................................FF............ 7832/13422
........................................................................................ 7920/13422
........................................................................................ 8008/13422
...................................................ii................i....i..ii......... 8096/13422
........................................................................................ 8184/13422
....................................................F.......................F........... 8272/13422
F................................................................F...F...........F...... 8360/13422
...........F...........F...............F............F...........F....F...F.............. 8448/13422
.....................................F.FF.........................................F.F... 8536/13422
.......i..ii...............................................................ii........... 8624/13422
.................................F....F.........................................iiii.... 8712/13422
..................................i........................................i............ 8888/13422
.......................................................i................................ 8976/13422
........................................................................................ 9064/13422
...................................................i.................................... 9152/13422
---
........................................................................................ 9856/13422
....................................................................ii...............i.. 9944/13422
........................................................................................ 10032/13422
........................................................................................ 10120/13422
....................FF.F.......................F.......F...F.FFF..............F......... 10208/13422
F........................F..............................................F.F.....F....... 10296/13422
....F................................................................................... 10384/13422
........................................................................................ 10560/13422
........................................................................................ 10648/13422
.........i....i.i....................................................................... 10736/13422
..........................................................i............................. 10824/13422
..........................................................i............................. 10824/13422
....................................................................iiiiii.i..iiiiii..i. 10912/13422
.........................F..F....................F.....F.......F.F.FFFFF.F.............. 11000/13422
........................................................................................ 11176/13422
........................................................................................ 11264/13422
........................................................................................ 11352/13422
........................................................................................ 11440/13422
---
........................................................................................ 13024/13422
........................................................................................ 13112/13422
........................................................................................ 13200/13422
........................................................................................ 13288/13422
F...F...F..................iiiFF..F.........................................F........... 13376/13422
failures:

---- [ui] src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs#krisskross stdout ----
diff of stderr:
diff of stderr:

7    |              lifetime `'a` defined here
9 LL |    (a, b)
9 LL |    (a, b)
-    |    ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |    ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`


20    |              lifetime `'a` defined here
22 LL |    (a, b)
22 LL |    (a, b)
-    |    ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |    ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
24    |
25    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross/project-fn-ret-contravariant.krisskross.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-contravariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |              -- -- lifetime `'b` defined here
   |              |
   |              lifetime `'a` defined here
...
LL |    (a, b) //[krisskross]~ ERROR lifetime may not live long enough
   |    ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs:46:4
   |
   |
LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |              -- -- lifetime `'b` defined here
   |              |
   |              lifetime `'a` defined here
...
LL |    (a, b) //[krisskross]~ ERROR lifetime may not live long enough
   |    ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant.rs#krisskross stdout ----
diff of stderr:

7    |              lifetime `'a` defined here
9 LL |     (a, b)
9 LL |     (a, b)
-    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`
13    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant

23    |              lifetime `'a` defined here
25 LL |     (a, b)
25 LL |     (a, b)
-    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
27    |
28    = help: consider adding the following bound: `'b: 'a`
29    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/project-fn-ret-invariant.krisskross.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |              --  -- lifetime `'b` defined here
   |              |
   |              lifetime `'a` defined here
LL |     (a, b)
LL |     (a, b)
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:59:5
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |              --  -- lifetime `'b` defined here
   |              |
   |              lifetime `'a` defined here
LL |     (a, b)
LL |     (a, b)
   |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/async-await/issue-69446-fnmut-capture.rs stdout ----
diff of stderr:

10 LL | |         x.foo();
11    | |         - variable captured here
12 LL | |     });
-    | |_____^ returns an `async` block that contains a reference to a captured variable, which then escapes the closure body
+    | |_____^    returns an `async` block that contains a reference to a captured variable, which then
+ escapes the closure body
14    |
-    = note: `FnMut` closures only have access to their captured variables while they are executing...
+    = note: `FnMut` closures only have access to their captured variables while they are
+            executing...
16    = note: ...therefore, they cannot allow references to captured variables to escape
18 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/issue-69446-fnmut-capture.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-69446-fnmut-capture.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-69446-fnmut-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/auxiliary"
stdout: none
--- stderr -------------------------------
error: captured variable cannot escape `FnMut` closure body
   |
LL |       let mut x = Foo;
   |           ----- variable defined here
   |           ----- variable defined here
LL |       bar(move || async { //~ ERROR captured
   |  _______________-_^
   | |               |
   | |               inferred to be a `FnMut` closure
LL | |         x.foo();
   | |         - variable captured here
LL | |     });
   | |_____^    returns an `async` block that contains a reference to a captured variable, which then
escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are
           executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs stdout ----
diff of stderr:

9 LL | |
10 LL | |     (a, b)
11 LL | | }
-    | |_^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    | |_^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
13    |
14    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/ret-impl-trait-one.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/ret-impl-trait-one.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |   async fn async_ret_impl_trait3<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {
   |  ________________________________--__--_______________________________________________^
   | |                                |   |
   | |                                |   lifetime `'b` defined here
   | |                                lifetime `'a` defined here
LL | |     //~^ ERROR lifetime may not live long enough
LL | |     (a, b)
LL | | }
   | |_^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`

error[E0700]: hidden type for `impl Trait<'a>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |  ____________________________________--__________________________________________^
   | |                                    |
   | |                                    hidden type `(&'a u8, &'b u8)` captures the lifetime `'b` as defined here
LL | |     //~^ ERROR captures lifetime that does not appear in bounds
LL | |     (a, b)
LL | | }
   |
   |
help: to declare that the `impl Trait` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
For more information about this error, try `rustc --explain E0700`.
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-describe-lvalue.rs stdout ----
diff of stderr:

32 LL | |                    *y = 1;
33 LL | |                    drop(y);
34 LL | |                 }
-    | |_________________^ returns a closure that contains a reference to a captured variable, which then escapes the closure body
+    | |_________________^    returns a closure that contains a reference to a captured variable, which then
+ escapes the closure body
36    |
-    = note: `FnMut` closures only have access to their captured variables while they are executing...
+    = note: `FnMut` closures only have access to their captured variables while they are
+            executing...
38    = note: ...therefore, they cannot allow references to captured variables to escape
39 help: consider adding 'move' keyword before the nested closure


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/borrowck-describe-lvalue.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/borrowck-describe-lvalue.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/auxiliary"
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
   | |_________________^    returns a closure that contains a reference to a captured variable, which then
escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are
           executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
help: consider adding 'move' keyword before the nested closure
   |
LL |                move || { //~ ERROR captured variable cannot escape `FnMut` closure body


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

---
diff of stderr:

2   --> $DIR/issue-52533.rs:5:16
3    |
4 LL |     foo(|a, b| b)
-    |          -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |          -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning
+ data with lifetime `'1`
6    |          |  |
7    |          |  has type `&'1 u32`
8    |          has type `&'2 u32`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533/issue-52533.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-52533.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52533.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52533/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     foo(|a, b| b)
   |          -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning
data with lifetime `'1`
   |          |  |
   |          |  has type `&'1 u32`
   |          has type `&'2 u32`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-if-else-using-impl.rs stdout ----
diff of stderr:

7    |            lifetime `'a` defined here
8 LL |
9 LL |         if x > y { x } else { y }
-    |                    ^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
+    |                    ^ associated function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'1`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-if-else-using-impl/ex1-return-one-existing-name-if-else-using-impl.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex1-return-one-existing-name-if-else-using-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-if-else-using-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-if-else-using-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-if-else-using-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn foo<'a>(x: &i32, y: &'a i32) -> &'a i32 {
   |            --     - let's call the lifetime of this reference `'1`
   |            |
   |            lifetime `'a` defined here
LL |
LL |         if x > y { x } else { y }
   |                    ^ associated function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'1`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-return-type-is-anon.rs stdout ----
diff of stderr:

7    |          lifetime `'a` defined here
9 LL |     x
9 LL |     x
-    |     ^ associated function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'a`
+    |     ^ associated function was supposed to return data with lifetime `'1` but it is returning
+ data with lifetime `'a`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-return-type-is-anon/ex1-return-one-existing-name-return-type-is-anon.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex1-return-one-existing-name-return-type-is-anon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-return-type-is-anon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-return-type-is-anon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-return-type-is-anon/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |   fn foo<'a>(&self, x: &'a i32) -> &i32 {
   |          --  - let's call the lifetime of this reference `'1`
   |          |
   |          lifetime `'a` defined here
LL |     x
LL |     x
   |     ^ associated function was supposed to return data with lifetime `'1` but it is returning
data with lifetime `'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-self-is-anon.rs stdout ----
diff of stderr:

7    |            lifetime `'a` defined here
8 LL |
9 LL |         if true { x } else { self }
-    |                              ^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
+    |                              ^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'1`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-self-is-anon/ex1-return-one-existing-name-self-is-anon.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex1-return-one-existing-name-self-is-anon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-self-is-anon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-self-is-anon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex1-return-one-existing-name-self-is-anon/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn foo<'a>(&self, x: &'a Foo) -> &'a Foo {
   |            --  - let's call the lifetime of this reference `'1`
   |            |
   |            lifetime `'a` defined here
LL |
LL |         if true { x } else { self }
   |                              ^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'1`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon.rs stdout ----
diff of stderr:

6    |              |
7    |              let's call the lifetime of this reference `'2`
8 LL |     x
-    |     ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |     ^ associated function was supposed to return data with lifetime `'2` but it is returning
+ data with lifetime `'1`
11 help: consider introducing a named lifetime parameter and update trait if needed
12    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon/ex3-both-anon-regions-return-type-is-anon.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-return-type-is-anon/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |   fn foo<'a>(&self, x: &i32) -> &i32 {
   |              -         - let's call the lifetime of this reference `'1`
   |              |
   |              let's call the lifetime of this reference `'2`
LL |     x
   |     ^ associated function was supposed to return data with lifetime `'2` but it is returning
data with lifetime `'1`
help: consider introducing a named lifetime parameter and update trait if needed
   |
   |
LL |   fn foo<'a>(&'a self, x: &'a i32) -> &i32 {
   |               ++           ++
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-self-is-anon.rs stdout ----
diff of stderr:

6    |                |
7    |                let's call the lifetime of this reference `'2`
8 LL |         if true { x } else { self }
-    |                   ^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |                   ^ associated function was supposed to return data with lifetime `'2` but it is returning
+ data with lifetime `'1`
11 help: consider introducing a named lifetime parameter and update trait if needed
12    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-self-is-anon/ex3-both-anon-regions-self-is-anon.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-self-is-anon.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-self-is-anon.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-self-is-anon" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-self-is-anon/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn foo<'a>(&self, x: &Foo) -> &Foo {
   |                -         - let's call the lifetime of this reference `'1`
   |                |
   |                let's call the lifetime of this reference `'2`
LL |         if true { x } else { self }
   |                   ^ associated function was supposed to return data with lifetime `'2` but it is returning
data with lifetime `'1`
help: consider introducing a named lifetime parameter and update trait if needed
   |
   |
LL |     fn foo<'a>(&'a self, x: &'a Foo) -> &Foo {
   |                 ++           ++
error: aborting due to previous error
------------------------------------------


---
-    |             |
-    |             help: remove this `mut`
+    |             ^^^^^ help: remove this `mut`
20    |
21    = note: this overrides the previous `expect` lint level and warns about the `unused_mut` lint here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels/expect_nested_lint_levels.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels/expect_nested_lint_levels.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels/auxiliary"
stdout: none
--- stderr -------------------------------
error: unused variable: `this_is_my_function`
   |
   |
LL |     let this_is_my_function = 3;
   |         ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_this_is_my_function`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:45:10
   |
   |
LL | #[forbid(unused_variables)]

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:36:13
   |
   |
LL |         let mut v = 0;
   |             ^^^^^ help: remove this `mut`
   |
   = note: this overrides the previous `expect` lint level and warns about the `unused_mut` lint here
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:31:9
   |
LL |         unused_mut,
   |         ^^^^^^^^^^
   |         ^^^^^^^^^^

warning: this lint expectation is unfulfilled
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:7:5
   |
LL |     unused_mut,
   |     ^^^^^^^^^^
   |
   = note: `#[warn(unfulfilled_lint_expectations)]` on by default
   = note: this `expect` is overridden by a `allow` attribute before the `unused_mut` lint is triggered
warning: this lint expectation is unfulfilled
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:24:5
   |
LL |     unused_mut,
LL |     unused_mut,
   |     ^^^^^^^^^^
   |
   = note: this `expect` is overridden by a `warn` attribute before the `unused_mut` lint is triggered
warning: this lint expectation is unfulfilled
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/expect_nested_lint_levels.rs:43:10
   |
   |
LL | #[expect(unused_variables)]

error: aborting due to previous error; 4 warnings emitted
------------------------------------------



---- [ui] src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs stdout ----
diff of stderr:

30   --> $DIR/force_warn_expected_lints_fulfilled.rs:32:9
31    |
32 LL |     let mut what_does_the_fox_say = "*ding* *deng* *dung*";
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove this `mut`
36    |
36    |
37    = note: requested on the command line with `--force-warn unused-mut`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled/force_warn_expected_lints_fulfilled.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled/force_warn_expected_lints_fulfilled.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warn" "while_true" "--force-warn" "unused_variables" "--force-warn" "unused_mut" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
LL |     while true {
   |     ^^^^^^^^^^ help: use `loop`
   |
   |
   = note: requested on the command line with `--force-warn while-true`
warning: unused variable: `x`
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs:20:9
   |
LL |     let x = 2;
LL |     let x = 2;
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
   = note: requested on the command line with `--force-warn unused-variables`

warning: unused variable: `fox_name`
   |
   |
LL |     let fox_name = "Sir Nibbles";
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_fox_name`
warning: unused variable: `this_should_fulfill_the_expectation`
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs:43:9
   |
   |
LL |     let this_should_fulfill_the_expectation = "The `#[allow]` has no power here";
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_this_should_fulfill_the_expectation`
warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/rfc-2383-lint-reason/force_warn_expected_lints_fulfilled.rs:32:9
   |
   |
LL |     let mut what_does_the_fox_say = "*ding* *deng* *dung*";
   |
   |
   = note: requested on the command line with `--force-warn unused-mut`
warning: 5 warnings emitted
------------------------------------------



---- [ui] src/test/ui/lint/suggestions.rs stdout ----
diff of stderr:

27   --> $DIR/suggestions.rs:48:13
28    |
29 LL |         let mut registry_no = (format!("NX-{}", 74205));
-    |             |
-    |             help: remove this `mut`
+    |             ^^^^^^^^^^^^^^^ help: remove this `mut`
33    |
---
+    |  _____________^
+ LL | |             b = 1;
+    | |_____________^ help: remove this `mut`
51 
52 error: const items should never be `#[no_mangle]`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/suggestions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/suggestions.stderr
diff of fixed:

45     loop {
46     //~^ WARN denote infinite loops
47     //~| HELP use `loop`
-         let registry_no = format!("NX-{}", 74205);
+         let  = format!("NX-{}", 74205);
49         //~^ WARN does not need to be mutable
50         //~| HELP remove this `mut`
51         //~| WARN unnecessary parentheses

52         //~| HELP remove these parentheses
53         // the line after `mut` has a `\t` at the beginning, this is on purpose
-         let b = 1;
+         let  = 1;
55         //~^^ WARN does not need to be mutable
56         //~| HELP remove this `mut`
57         let d = Equinox { warp_factor: 9.975 };

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/suggestions.fixed
To only update this specific test, also pass `--test-args lint/suggestions.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/suggestions/auxiliary"
stdout: none
--- stderr -------------------------------
warning: denote infinite loops with `loop { ... }`
   |
LL |     while true {
   |     ^^^^^^^^^^ help: use `loop`
   |
   |
   = note: `#[warn(while_true)]` on by default

warning: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/suggestions.rs:48:31
   |
LL |         let mut registry_no = (format!("NX-{}", 74205));
   |                               ^                       ^
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/suggestions.rs:4:21
   |
   |
LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896
help: remove these parentheses
   |
   |
LL -         let mut registry_no = (format!("NX-{}", 74205));
LL +         let mut registry_no = format!("NX-{}", 74205);

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/suggestions.rs:48:13
   |
   |
LL |         let mut registry_no = (format!("NX-{}", 74205));
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/suggestions.rs:4:9
   |
   |
LL | #![warn(unused_mut, unused_parens)] // UI tests pass `-A unused`—see Issue #43896

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/suggestions.rs:54:13
   |
   |
LL |           let mut
   |  _____________^
LL | |             b = 1;
   | |_____________^ help: remove this `mut`

error: const items should never be `#[no_mangle]`
   |
   |
LL | #[no_mangle] const DISCOVERY: usize = 1;
   |              |
   |              |
   |              help: try a static value: `pub static`
   |
   = note: `#[deny(no_mangle_const_items)]` on by default
warning: functions generic over types or consts must be mangled
  --> /checkout/src/test/ui/lint/suggestions.rs:12:1
   |
   |
LL | #[no_mangle]
   | ------------ help: remove this attribute
LL | //~^ HELP remove this attribute
LL | pub fn defiant<T>(_t: T) {}
   |
   = note: `#[warn(no_mangle_generic_items)]` on by default


warning: the `warp_factor:` in this pattern is redundant
   |
   |
LL |             Equinox { warp_factor: warp_factor } => {}
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^ help: use shorthand field pattern: `warp_factor`
   = note: `#[warn(non_shorthand_field_patterns)]` on by default


error: const items should never be `#[no_mangle]`
   |
   |
LL |     #[no_mangle] pub const DAUNTLESS: bool = true;
   |                  |
   |                  |
   |                  help: try a static value: `pub static`
warning: functions generic over types or consts must be mangled
  --> /checkout/src/test/ui/lint/suggestions.rs:26:18
   |
   |
LL |     #[no_mangle] pub fn val_jean<T>() {}
   |     |
   |     help: remove this attribute


error: const items should never be `#[no_mangle]`
   |
   |
LL |     #[no_mangle] pub(crate) const VETAR: bool = true;
   |                  |
   |                  |
   |                  help: try a static value: `pub static`
warning: functions generic over types or consts must be mangled
  --> /checkout/src/test/ui/lint/suggestions.rs:35:18
   |
   |
LL |     #[no_mangle] pub(crate) fn crossfield<T>() {}
   |     |
   |     help: remove this attribute

error: aborting due to 3 previous errors; 8 warnings emitted
---
106 

108   --> $DIR/issue-47390-unused-variable-in-struct-pattern.rs:37:10
109    |
110 LL |     let (mut var, unused_var) = (1, 2);
-    |          |
-    |          help: remove this `mut`
+    |          ^^^^^^^ help: remove this `mut`
114 
---
To only update this specific test, also pass `--test-args lint/unused/issue-47390-unused-variable-in-struct-pattern.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `i_think_continually`
   |
   |
LL |     let i_think_continually = 2; //~ WARNING unused variable: `i_think_continually`
   |         ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_i_think_continually`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:5:9
   |
   |
LL | #![warn(unused)] // UI tests pass `-A unused` (#43896)
   = note: `#[warn(unused_variables)]` implied by `#[warn(unused)]`

warning: unused variable: `mut_unused_var`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:13
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:13
   |
LL |     let mut mut_unused_var = 1;
   |             ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_mut_unused_var`

warning: unused variable: `var`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:14
   |
LL |     let (mut var, unused_var) = (1, 2);
   |              ^^^ help: if this is intentional, prefix it with an underscore: `_var`
warning: unused variable: `unused_var`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:19
   |
   |
LL |     let (mut var, unused_var) = (1, 2);
   |                   ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused_var`

warning: unused variable: `corridors_of_light`
   |
   |
LL |     if let SoulHistory { corridors_of_light, //~ WARNING unused variable: `corridors_of_light`
   |                          ^^^^^^^^^^^^^^^^^^ help: try ignoring the field: `corridors_of_light: _`

warning: variable `hours_are_suns` is assigned to, but never used
   |
   |
LL |                          mut hours_are_suns, //~ WARNING `hours_are_suns` is assigned to, but
   |
   |
   = note: consider using `_hours_are_suns` instead

warning: value assigned to `hours_are_suns` is never read
   |
   |
LL |         hours_are_suns = false; //~ WARNING unused_assignments
   |
   = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`
   = note: `#[warn(unused_assignments)]` implied by `#[warn(unused)]`
   = help: maybe it is overwritten before being read?
warning: unused variable: `fire`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:52:32
   |
   |
LL |     let LovelyAmbition { lips, fire } = the_spirit; //~ WARNING unused variable: `fire`
   |                                ^^^^ help: try ignoring the field: `fire: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:61:23
   |
   |
LL |         Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                       ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:66:24
   |
   |
LL |         &Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:71:27
   |
   |
LL |         box Large::Suit { case } => {} //~ WARNING unused variable: `case`
   |                           ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:76:24
   |
   |
LL |         (Large::Suit { case },) => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:81:24
   |
   |
LL |         [Large::Suit { case }] => {} //~ WARNING unused variable: `case`
   |                        ^^^^ help: try ignoring the field: `case: _`
warning: unused variable: `case`
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:86:29
   |
   |
LL |         Tuple(Large::Suit { case }, ()) => {} //~ WARNING unused variable: `case`
   |                             ^^^^ help: try ignoring the field: `case: _`
warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:33:9
   |
LL |     let mut mut_unused_var = 1;
LL |     let mut mut_unused_var = 1;
   |         ^^^^^^^^^^^^^^^^^^ help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` implied by `#[warn(unused)]`

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/issue-47390-unused-variable-in-struct-pattern.rs:37:10
   |
LL |     let (mut var, unused_var) = (1, 2);

warning: 16 warnings emitted
------------------------------------------

---
10   --> $DIR/lint-unused-mut-self.rs:6:9

16   --> $DIR/lint-unused-mut-self.rs:11:12
17    |
18 LL |     fn bar(mut self: Box<Foo>) {}
-    |            |
-    |            help: remove this `mut`
+    |            ^^^^^^^^ help: remove this `mut`
22 
---

7 
8 struct Foo;
9 impl Foo {
-     fn foo(self) {} //~ ERROR: variable does not need to be mutable
-     fn bar(self: Box<Foo>) {} //~ ERROR: variable does not need to be mutable
+     fn foo() {} //~ ERROR: variable does not need to be mutable
+     fn bar(: Box<Foo>) {} //~ ERROR: variable does not need to be mutable
13 
14 fn main() {}



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-self/lint-unused-mut-self.fixed
To only update this specific test, also pass `--test-args lint/unused/lint-unused-mut-self.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/lint-unused-mut-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-self" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-self/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
   |
LL |     fn foo(mut self) {} //~ ERROR: variable does not need to be mutable
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-self.rs:6:9
   |
   |
LL | #![deny(unused_mut)]
   |         ^^^^^^^^^^

error: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-self.rs:11:12
   |
LL |     fn bar(mut self: Box<Foo>) {} //~ ERROR: variable does not need to be mutable

error: aborting due to 2 previous errors
------------------------------------------

---
71 warning: variable does not need to be mutable
72   --> $DIR/lint-unused-mut-variables.rs:107:14

73    |
74 LL |     let x = |mut y: isize| 10;
-    |              |
-    |              help: remove this `mut`
+    |              ^^^^^ help: remove this `mut`
78 
---
103 warning: variable does not need to be mutable
104   --> $DIR/lint-unused-mut-variables.rs:75:9

105    |
106 LL |     let mut a = vec![3];
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
110 
110 
111 warning: variable does not need to be mutable
112   --> $DIR/lint-unused-mut-variables.rs:77:10

113    |
114 LL |     let (mut a, b) = (1, 2);
-    |          |
-    |          help: remove this `mut`
+    |          ^^^^^ help: remove this `mut`
118 
---
143 warning: variable does not need to be mutable
144   --> $DIR/lint-unused-mut-variables.rs:99:10

145    |
146 LL |         (mut x, 1) |
-    |          |
-    |          help: remove this `mut`
+    |          ^^^^^ help: remove this `mut`
150 
---
159 warning: variable does not need to be mutable
160   --> $DIR/lint-unused-mut-variables.rs:117:9

161    |
162 LL |     let mut b = (&mut a,);
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
166 
---
175 warning: variable does not need to be mutable
176   --> $DIR/lint-unused-mut-variables.rs:132:9

177    |
178 LL |     let mut v : &mut Vec<()> = &mut vec![];
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
182 
---
191 warning: variable does not need to be mutable
192   --> $DIR/lint-unused-mut-variables.rs:109:13

193    |
194 LL |     fn what(mut foo: isize) {}
-    |             |
-    |             help: remove this `mut`
+    |             ^^^^^^^ help: remove this `mut`
198 
198 
199 warning: variable does not need to be mutable
200   --> $DIR/lint-unused-mut-variables.rs:127:20

201    |
202 LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
-    |                    |
-    |                    help: remove this `mut`
+    |                    ^^^^^^^ help: remove this `mut`
206 
206 
207 error: variable does not need to be mutable
208   --> $DIR/lint-unused-mut-variables.rs:205:9

209    |
210 LL |     let mut b = vec![2];
-    |         |
-    |         help: remove this `mut`
+    |         ^^^^^ help: remove this `mut`
214    |
---
To only update this specific test, also pass `--test-args lint/unused/lint-unused-mut-variables.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-variables" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/lint-unused-mut-variables/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:9:5
   |
LL |     mut a: i32,
   |     ^^^^^ help: remove this `mut`
---

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:107:14
   |
LL |     let x = |mut y: isize| 10; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:69:9
   |
   |
LL |     let mut a = 3; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:71:9
   |
   |
LL |     let mut a = 2; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:73:9
   |
   |
LL |     let mut b = 3; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:75:9
   |
   |
LL |     let mut a = vec![3]; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:77:10
   |
   |
LL |     let (mut a, b) = (1, 2); //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:79:9
   |
   |
LL |     let mut a; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:83:9
   |
   |
LL |     let mut b; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:92:9
   |
   |
LL |         mut x => {} //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:99:10
   |
   |
LL |         (mut x, 1) | //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:112:9
   |
   |
LL |     let mut a = &mut 5; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:117:9
   |
   |
LL |     let mut b = (&mut a,); //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:120:9
   |
   |
LL |     let mut x = &mut 1; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:132:9
   |
   |
LL |     let mut v : &mut Vec<()> = &mut vec![]; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:187:9
   |
   |
LL |     let mut raw_address_of_const = 1; //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:109:13
   |
   |
LL |     fn what(mut foo: isize) {} //~ WARN: variable does not need to be mutable

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:127:20
   |
   |
LL |     fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {

error: variable does not need to be mutable
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:205:9
   |
   |
LL |     let mut b = vec![2]; //~ ERROR: variable does not need to be mutable
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/lint-unused-mut-variables.rs:201:8
   |
---
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var/unused-mut-warning-captured-var.stderr
diff of fixed:

3 #![forbid(unused_mut)]
5 fn main() {
-     let x = 1;
+     let  = 1;
+     let  = 1;
7     //~^ ERROR: variable does not need to be mutable
8     (move|| { println!("{}", x); })();


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var/unused-mut-warning-captured-var.fixed
To only update this specific test, also pass `--test-args lint/unused/unused-mut-warning-captured-var.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-mut-warning-captured-var.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-mut-warning-captured-var/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
LL |     let mut x = 1;
   |         ^^^^^ help: remove this `mut`
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-mut-warning-captured-var.rs:3:11
   |
LL | #![forbid(unused_mut)]

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/match/match-ref-mut-invariance.rs stdout ----
diff of stderr:

6 LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
7    |            -- lifetime `'a` defined here
8 LL |         match self.0 { ref mut x => x }
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`
12    = note: requirement occurs because of a mutable reference to `&i32`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance/match-ref-mut-invariance.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args match/match-ref-mut-invariance.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-ref-mut-invariance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl<'b> S<'b> {
   |      -- lifetime `'b` defined here
LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
   |            -- lifetime `'a` defined here
LL |         match self.0 { ref mut x => x }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to `&i32`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/match/match-ref-mut-let-invariance.rs stdout ----
diff of stderr:

7    |            -- lifetime `'a` defined here
8 LL |         let ref mut x = self.0;
9 LL |         x
-    |         ^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |         ^ associated function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`
13    = note: requirement occurs because of a mutable reference to `&i32`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance/match-ref-mut-let-invariance.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args match/match-ref-mut-let-invariance.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-ref-mut-let-invariance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl<'b> S<'b> {
   |      -- lifetime `'b` defined here
LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
   |            -- lifetime `'a` defined here
LL |         let ref mut x = self.0;
LL |         x
   |         ^ associated function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to `&i32`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------


---
diff of fixed:

6 #![deny(unused_mut)]
7 
8 pub fn mutable_upvar() {
-     let x = &mut 0;
+     let  = &mut 0;
10     //~^ ERROR
11     let _ = move || {
12         *x = 1;

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/capture-mut-ref.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/capture-mut-ref.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/capture-mut-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/capture-mut-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
LL |     let mut x = &mut 0;
   |         ^^^^^ help: remove this `mut`
   |
---
---- [ui] src/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs stdout ----
diff of stderr:

6    |        |
7    |        lifetime `'a` defined here
8 LL |     &*x
-    |     ^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2/region-lbr1-does-not-outlive-ebr2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2/region-lbr1-does-not-outlive-ebr2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
LL |     &*x
   |     ^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs stdout ----
diff of stderr:

14   --> $DIR/return-wrong-bound-region.rs:11:23
15    |
16 LL |     expect_sig(|a, b| b); // ought to return `a`
-    |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning
+ data with lifetime `'1`
18    |                 |  |
19    |                 |  has type `&'1 i32`
20    |                 has type `&'2 i32`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/return-wrong-bound-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/return-wrong-bound-region.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) i32, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) i32)) -> &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) i32,
               (),

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:11:23
   |
   |
LL |     expect_sig(|a, b| b); // ought to return `a`
   |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning
data with lifetime `'1`
   |                 |  |
   |                 |  has type `&'1 i32`
   |                 has type `&'2 i32`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs:10:1
   |
LL | / fn test() {
LL | / fn test() {
LL | |     expect_sig(|a, b| b); // ought to return `a`
LL | |     //~^ ERROR
LL | | }
   |
   = note: defining type: test

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/issue-52213.rs stdout ----
diff of stderr:

7    |                       lifetime `'a` defined here
8 LL |     match (&t,) {
9 LL |         ((u,),) => u,
-    |                    ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |                    ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213/issue-52213.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213/issue-52213.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52213.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52213.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn transmute_lifetime<'a, 'b, T>(t: &'a (T,)) -> &'b T {
   |                       --  -- lifetime `'b` defined here
   |                       |
   |                       lifetime `'a` defined here
LL |     match (&t,) {
LL |         ((u,),) => u,
   |                    ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52533-1.rs stdout ----
diff of stderr:

2   --> $DIR/issue-52533-1.rs:9:18
3    |
4 LL |     gimme(|x, y| y)
-    |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning
+ data with lifetime `'1`
6    |            |  |
7    |            |  has type `&Foo<'_, '1, u32>`
8    |            has type `&Foo<'_, '2, u32>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1/issue-52533-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52533-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52533-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     gimme(|x, y| y)
   |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning
data with lifetime `'1`
   |            |  |
   |            |  has type `&Foo<'_, '1, u32>`
   |            has type `&Foo<'_, '2, u32>`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-53040.rs stdout ----
diff of stderr:

10    |      | returns a reference to a captured variable which escapes the closure body
11    |      inferred to be a `FnMut` closure
12    |
-    = note: `FnMut` closures only have access to their captured variables while they are executing...
+    = note: `FnMut` closures only have access to their captured variables while they are
+            executing...
14    = note: ...therefore, they cannot allow references to captured variables to escape
16 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53040/issue-53040.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-53040.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-53040.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53040" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53040/auxiliary"
stdout: none
--- stderr -------------------------------
error: captured variable cannot escape `FnMut` closure body
   |
   |
LL |     let mut v: Vec<()> = Vec::new();
   |         ----- variable defined here
LL |     || &mut v;
   |      - ^^^^^-
   |      | |    variable captured here
   |      | |    variable captured here
   |      | returns a reference to a captured variable which escapes the closure body
   |      inferred to be a `FnMut` closure
   |
   = note: `FnMut` closures only have access to their captured variables while they are
           executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-55394.rs stdout ----
diff of stderr:

6    |                 |
7    |                 let's call the lifetime of this reference `'1`
8 LL |         Foo { bar }
-    |         ^^^^^^^^^^^ associated function was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
+    |         ^^^^^^^^^^^ associated function was supposed to return data with lifetime `'2` but it is returning
+ data with lifetime `'1`
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/issue-55394.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-55394.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-55394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-55394/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn new(bar: &mut Bar) -> Self {
   |                 -            ---- return type is Foo<'2>
   |                 |
   |                 let's call the lifetime of this reference `'1`
LL |         Foo { bar } //~ERROR
   |         ^^^^^^^^^^^ associated function was supposed to return data with lifetime `'2` but it is returning
data with lifetime `'1`
error: aborting due to previous error
------------------------------------------


---

3 #![deny(unused_mut)]
4 
5 fn main() {
-     let x; //~ ERROR: variable does not need to be mutable
+     let ; //~ ERROR: variable does not need to be mutable
7     x = String::new();
8     dbg!(x);


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/issue-61424.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-61424.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-61424.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-61424/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
   |
LL |     let mut x; //~ ERROR: variable does not need to be mutable
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/nll/issue-61424.rs:3:9
   |
---

---- [ui] src/test/ui/nll/issue-98170.rs stdout ----
diff of stderr:

6 LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
7    |                -- lifetime `'a` defined here
8 LL |         Self { field }
-    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
+    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'1`
11 error: lifetime may not live long enough
12   --> $DIR/issue-98170.rs:7:16


27    |      lifetime `'a` defined here
28 LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
29 LL |         Self { field }
-    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
+    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'1`
32 error: lifetime may not live long enough
33   --> $DIR/issue-98170.rs:19:16



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98170/issue-98170.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-98170.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-98170.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98170" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98170/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl MyStruct<'_> {
   |               -- lifetime `'1` appears in the `impl`'s self type
LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
   |                -- lifetime `'a` defined here
LL |         Self { field }
   |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:7:16
   |
   |
LL | impl MyStruct<'_> {
   |               -- lifetime `'1` appears in the `impl`'s self type
LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
   |                -- lifetime `'a` defined here
LL |         Self { field }
   |                ^^^^^ this usage requires that `'a` must outlive `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:19:9
   |
   |
LL | impl<'a> Trait<'a> for MyStruct<'_> {
   |      --                         -- lifetime `'1` appears in the `impl`'s self type
   |      |
   |      lifetime `'a` defined here
LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
LL |         Self { field }
   |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:19:16
   |
   |
LL | impl<'a> Trait<'a> for MyStruct<'_> {
   |      --                         -- lifetime `'1` appears in the `impl`'s self type
   |      |
   |      lifetime `'a` defined here
LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
LL |         Self { field }
   |                ^^^^^ this usage requires that `'a` must outlive `'1`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/mir_check_cast_closure.rs stdout ----
diff of stderr:

7    |        lifetime `'a` defined here
8 LL |     let g: fn(_, _) -> _ = |_x, y| y;
9 LL |     g
-    |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
11    |
12    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure/mir_check_cast_closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure/mir_check_cast_closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/mir_check_cast_closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/mir_check_cast_closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn bar<'a, 'b>() -> fn(&'a u32, &'b u32) -> &'a u32 {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
LL |     let g: fn(_, _) -> _ = |_x, y| y;
LL |     g
   |     ^ function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/outlives-suggestion-more.rs stdout ----
diff of stderr:

6    |         |
7    |         lifetime `'a` defined here
8 LL |     (x, y)
-    |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning data with lifetime `'a`
+    |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'c`

18    |             |
18    |             |
19    |             lifetime `'b` defined here
20 LL |     (x, y)
-    |     ^^^^^^ function was supposed to return data with lifetime `'d` but it is returning data with lifetime `'b`
+    |     ^^^^^^ function was supposed to return data with lifetime `'d` but it is returning
+ data with lifetime `'b`
22    |
23    = help: consider adding the following bound: `'b: 'd`

35    |         |
35    |         |
36    |         lifetime `'a` defined here
37 LL |     (x, y)
-    |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning data with lifetime `'a`
+    |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning
+ data with lifetime `'a`
39    |
40    = help: consider adding the following bound: `'a: 'c`


61    |         lifetime `'a` defined here
63 LL |     (x, y, z)
63 LL |     (x, y, z)
-    |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
65    |
66    = help: consider adding the following bound: `'a: 'b`


74    |         lifetime `'a` defined here
76 LL |     (x, y, z)
76 LL |     (x, y, z)
-    |     ^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
78    |
79    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-more/outlives-suggestion-more.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-more/outlives-suggestion-more.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/outlives-suggestion-more.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-more.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-more" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-more/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo1<'a, 'b, 'c, 'd>(x: &'a usize, y: &'b usize) -> (&'c usize, &'d usize) {
   |         --      -- lifetime `'c` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'c`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:5:5
   |
   |
LL | fn foo1<'a, 'b, 'c, 'd>(x: &'a usize, y: &'b usize) -> (&'c usize, &'d usize) {
   |             --      -- lifetime `'d` defined here
   |             |
   |             lifetime `'b` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'d` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'd`
help: the following changes may resolve your lifetime errors
   |
   |
   = help: add bound `'a: 'c`
   = help: add bound `'b: 'd`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:11:5
   |
   |
LL | fn foo2<'a, 'b, 'c>(x: &'a usize, y: &'b usize) -> (&'c usize, &'static usize) {
   |         --      -- lifetime `'c` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'c` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'c`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:11:5
   |
   |
LL | fn foo2<'a, 'b, 'c>(x: &'a usize, y: &'b usize) -> (&'c usize, &'static usize) {
   |             -- lifetime `'b` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ returning this value requires that `'b` must outlive `'static`
help: the following changes may resolve your lifetime errors
   |
   |
   = help: add bound `'a: 'c`
   = help: replace `'b` with `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:21:5
   |
   |
LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |     (x, y, z) //~ERROR lifetime may not live long enough
   |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:21:5
   |
   |
LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |     (x, y, z) //~ERROR lifetime may not live long enough
   |     ^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-more.rs:21:5
   |
   |
LL | fn foo3<'a, 'b, 'c, 'd, 'e>(
   |                 -- lifetime `'c` defined here
...
LL |     (x, y, z) //~ERROR lifetime may not live long enough
   |     ^^^^^^^^^ returning this value requires that `'c` must outlive `'static`
help: the following changes may resolve your lifetime errors
   |
   |
   = help: `'a` and `'b` must be the same: replace one with the other
   = help: replace `'c` with `'static`
error: aborting due to 7 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/outlives-suggestion-simple.rs stdout ----
diff of stderr:

6    |         |
7    |         lifetime `'a` defined here
8 LL |     x
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`

26    |         |
26    |         |
27    |         lifetime `'a` defined here
28 LL |     (x, y)
-    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
30    |
31    = help: consider adding the following bound: `'a: 'b`

38    |         |
38    |         |
39    |         lifetime `'a` defined here
40 LL |     (x, y)
-    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
42    |
43    = help: consider adding the following bound: `'b: 'a`


53    |         lifetime `'a` defined here
55 LL |     (x, x)
55 LL |     (x, x)
-    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
57    |
58    = help: consider adding the following bound: `'a: 'b`


73 LL |     pub fn get<'b>(&self) -> &'b usize {
74    |                -- lifetime `'b` defined here
75 LL |         self.x
-    |         ^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |         ^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
77    |
78    = help: consider adding the following bound: `'a: 'b`


85 LL |     fn get<'b>(&'b self) -> &'a i32 {
86    |            -- lifetime `'b` defined here
87 LL |         self.x
-    |         ^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |         ^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
89    |
90    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/outlives-suggestion-simple.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/outlives-suggestion-simple.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/outlives-suggestion-simple.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/outlives-suggestion-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/outlives-suggestion-simple/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo1<'a, 'b>(x: &'a usize) -> &'b usize {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     x //~ERROR lifetime may not live long enough
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:8:5
   |
   |
LL | fn foo2<'a>(x: &'a usize) -> &'static usize {
   |         -- lifetime `'a` defined here
LL |     x //~ERROR lifetime may not live long enough
   |     ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:12:5
   |
   |
LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:12:5
   |
   |
LL | fn foo3<'a, 'b>(x: &'a usize, y: &'b usize) -> (&'b usize, &'a usize) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     (x, y) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:20:5
   |
   |
LL | fn foo4<'a, 'b, 'c>(x: &'a usize) -> (&'b usize, &'c usize) {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |     (x, x) //~ERROR lifetime may not live long enough
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:29:9
   |
   |
LL |     pub fn foo<'a>(x: &'a usize) -> Self {
   |                -- lifetime `'a` defined here
LL |         Foo { x } //~ERROR lifetime may not live long enough
   |         ^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:39:9
   |
   |
LL | impl<'a> Bar<'a> {
   |      -- lifetime `'a` defined here
LL |     pub fn get<'b>(&self) -> &'b usize {
   |                -- lifetime `'b` defined here
LL |         self.x //~ERROR lifetime may not live long enough
   |         ^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:50:9
   |
   |
LL | impl<'a> Baz<'a> {
   |      -- lifetime `'a` defined here
LL |     fn get<'b>(&'b self) -> &'a i32 {
   |            -- lifetime `'b` defined here
LL |         self.x //~ERROR lifetime may not live long enough
   |         ^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/outlives-suggestion-simple.rs:71:9
   |
   |
LL | impl<'a> Foo2<'a> {
   |      -- lifetime `'a` defined here
LL |     // should not produce outlives suggestions to name 'self
LL |     fn get_bar(&self) -> Bar2 {
   |                - let's call the lifetime of this reference `'1`
LL |         Bar2::new(&self) //~ERROR lifetime may not live long enough
   |         ^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'a`
error: aborting due to 9 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/polonius/subset-relations.rs stdout ----
diff of stderr:

6    |                   |
7    |                   lifetime `'a` defined here
8 LL |     y
-    |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
10    |
11    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations/subset-relations.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations/subset-relations.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/polonius/subset-relations.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/polonius/subset-relations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "polonius" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/polonius/subset-relations/auxiliary"
---

4 #![allow(unused_variables)] // for rustfix
5 
6 fn main() {
-     vec![(42, 22)].iter().map(|(x, _y)| ()).count();
+     vec![(42, 22)].iter().map(|(, _y)| ()).count();
8     //~^ ERROR: variable does not need to be mutable
10 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343/unused-mut-issue-50343.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/unused-mut-issue-50343.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/unused-mut-issue-50343.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/unused-mut-issue-50343/auxiliary"
stdout: none
--- stderr -------------------------------
error: variable does not need to be mutable
   |
   |
LL |     vec![(42, 22)].iter().map(|(mut x, _y)| ()).count();
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/nll/unused-mut-issue-50343.rs:3:9
   |
---
---- [ui] src/test/ui/nll/type-check-pointer-coercions.rs stdout ----
diff of stderr:

6    |                    |
7    |                    lifetime `'a` defined here
8 LL |     x
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`

18    |                    |
18    |                    |
19    |                    lifetime `'a` defined here
20 LL |     x
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
22    |
23    = help: consider adding the following bound: `'a: 'b`


31    |                  lifetime `'a` defined here
32 LL |     // Two errors because *mut is invariant
33 LL |     x
-    |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
35    |
36    = help: consider adding the following bound: `'b: 'a`
37    = note: requirement occurs because of a mutable pointer to `&i32`

47    |                  lifetime `'a` defined here
48 LL |     // Two errors because *mut is invariant
49 LL |     x
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
51    |
52    = help: consider adding the following bound: `'a: 'b`
53    = note: requirement occurs because of a mutable pointer to `&i32`
64    |                 |
64    |                 |
65    |                 lifetime `'a` defined here
66 LL |     x
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
68    |
69    = help: consider adding the following bound: `'a: 'b`


77    |               lifetime `'a` defined here
79 LL |     y
79 LL |     y
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
81    |
82    = help: consider adding the following bound: `'a: 'b`


90    |                 lifetime `'a` defined here
92 LL |     y
92 LL |     y
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
94    |
95    = help: consider adding the following bound: `'a: 'b`


103    |                 lifetime `'a` defined here
105 LL |     y
105 LL |     y
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
107    |
108    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions/type-check-pointer-coercions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions/type-check-pointer-coercions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/type-check-pointer-coercions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/type-check-pointer-coercions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn shared_to_const<'a, 'b>(x: &&'a i32) -> *const &'b i32 {
   |                    --  -- lifetime `'b` defined here
   |                    |
   |                    lifetime `'a` defined here
LL |     x   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:6:5
   |
   |
LL | fn unique_to_const<'a, 'b>(x: &mut &'a i32) -> *const &'b i32 {
   |                    --  -- lifetime `'b` defined here
   |                    |
   |                    lifetime `'a` defined here
LL |     x   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:11:5
   |
   |
LL | fn unique_to_mut<'a, 'b>(x: &mut &'a i32) -> *mut &'b i32 {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     // Two errors because *mut is invariant
LL |     x   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable pointer to `&i32`
   = note: mutable pointers are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:11:5
   |
   |
LL | fn unique_to_mut<'a, 'b>(x: &mut &'a i32) -> *mut &'b i32 {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     // Two errors because *mut is invariant
LL |     x   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable pointer to `&i32`
   = note: mutable pointers are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'b` and `'a` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:16:5
   |
   |
LL | fn mut_to_const<'a, 'b>(x: *mut &'a i32) -> *const &'b i32 {
   |                 --  -- lifetime `'b` defined here
   |                 |
   |                 lifetime `'a` defined here
LL |     x   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:22:5
   |
   |
LL | fn array_elem<'a, 'b>(x: &'a i32) -> *const &'b i32 {
   |               --  -- lifetime `'b` defined here
   |               |
   |               lifetime `'a` defined here
...
LL |     y   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:28:5
   |
   |
LL | fn array_coerce<'a, 'b>(x: &'a i32) -> *const [&'b i32; 3] {
   |                 --  -- lifetime `'b` defined here
   |                 |
   |                 lifetime `'a` defined here
...
LL |     y   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:34:5
   |
   |
LL | fn nested_array<'a, 'b>(x: &'a i32) -> *const [&'b i32; 2] {
   |                 --  -- lifetime `'b` defined here
   |                 |
   |                 lifetime `'a` defined here
...
LL |     y   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to 8 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/where_clauses_in_functions.rs stdout ----
diff of stderr:

6    |        |
7    |        lifetime `'a` defined here
8 LL |     foo(x, y)
-    |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions/where_clauses_in_functions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions/where_clauses_in_functions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/where_clauses_in_functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/where_clauses_in_functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_functions/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn bar<'a, 'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
LL |     foo(x, y)
   |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/user-annotations/wf-self-type.rs stdout ----
diff of stderr:

6    |            |
7    |            lifetime `'a` defined here
8 LL |     Foo::xmute(u)
-    |     ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
10    |
11    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/wf-self-type/wf-self-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/wf-self-type/wf-self-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/wf-self-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/wf-self-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/wf-self-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/wf-self-type/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {
   |            --  -- lifetime `'b` defined here
   |            |
   |            lifetime `'a` defined here
LL |     Foo::xmute(u) //~ ERROR lifetime may not live long enough
   |     ^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/object-lifetime/object-lifetime-default-elision.rs stdout ----
diff of stderr:

7    |          lifetime `'a` defined here
9 LL |     ss
9 LL |     ss
-    |     ^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-elision/object-lifetime-default-elision.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-elision/object-lifetime-default-elision.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-elision.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-elision.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-elision" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-elision/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn load3<'a,'b>(ss: &'a dyn SomeTrait) -> &'b dyn SomeTrait {
   |          -- -- lifetime `'b` defined here
   |          |
   |          lifetime `'a` defined here
LL |     ss
LL |     ss
   |     ^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/object-lifetime/object-lifetime-default-mybox.rs stdout ----
diff of stderr:

7    |          lifetime `'a` defined here
9 LL |     a
9 LL |     a
-    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox/object-lifetime-default-mybox.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox/object-lifetime-default-mybox.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-mybox.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn load1<'a,'b>(a: &'a MyBox<dyn SomeTrait>,
   |          -- -- lifetime `'b` defined here
   |          |
   |          lifetime `'a` defined here
LL |     a
LL |     a
   |     ^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error[E0521]: borrowed data escapes outside of function
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs:32:5
   |
   |
LL | fn load2<'a>(ss: &MyBox<dyn SomeTrait + 'a>) -> MyBox<dyn SomeTrait + 'a> {
   |          --  -- `ss` is a reference that is only valid in the function body
   |          |
   |          lifetime `'a` defined here
LL |     load0(ss)
   |     |
   |     |
   |     `ss` escapes the function body here
   |     argument requires that `'a` must outlive `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0521`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/region-object-lifetime-2.rs stdout ----
diff of stderr:

6    |                                          |
7    |                                          lifetime `'a` defined here
8 LL |     x.borrowed()
-    |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2/region-object-lifetime-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2/region-object-lifetime-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-object-lifetime-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn borrowed_receiver_different_lifetimes<'a,'b>(x: &'a dyn Foo) -> &'b () {
   |                                          -- -- lifetime `'b` defined here
   |                                          |
   |                                          lifetime `'a` defined here
LL |     x.borrowed()
   |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/region-object-lifetime-4.rs stdout ----
diff of stderr:

6    |                                         |
7    |                                         lifetime `'a` defined here
8 LL |     x.borrowed()
-    |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4/region-object-lifetime-4.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4/region-object-lifetime-4.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-object-lifetime-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn borrowed_receiver_related_lifetimes2<'a,'b>(x: &'a (dyn Foo + 'b)) -> &'b () {
   |                                         -- -- lifetime `'b` defined here
   |                                         |
   |                                         lifetime `'a` defined here
LL |     x.borrowed()
   |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/region-object-lifetime-in-coercion.rs stdout ----
diff of stderr:

54    |      |
55    |      lifetime `'a` defined here
56 LL |     Box::new(v)
-    |     ^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |     ^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning
+ data with lifetime `'a`
58    |
59    = help: consider adding the following bound: `'a: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion/region-object-lifetime-in-coercion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion/region-object-lifetime-in-coercion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-object-lifetime-in-coercion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion/auxiliary"
---
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref/issue-73592-borrow_mut-through-deref.stderr
diff of fixed:

39     let _ = &mut *s[0].borrow_mut();
41 
41 
- fn test_mut_pin(s: Pin<&S>) {
+ fn test_mut_pin(: Pin<&S>) {
43     //~^ WARN variable does not need to be mutable
44     let _ = &mut *s.0.borrow_mut();

46 
46 
- fn test_mut_pin_mut(s: Pin<&mut S>) {
+ fn test_mut_pin_mut(: Pin<&mut S>) {
48     //~^ WARN variable does not need to be mutable
49     let _ = &mut *s.0.borrow_mut();


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref/issue-73592-borrow_mut-through-deref.fixed
To only update this specific test, also pass `--test-args typeck/issue-73592-borrow_mut-through-deref.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-73592-borrow_mut-through-deref/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs:42:17
   |
   |
LL | fn test_mut_pin(mut s: Pin<&S>) {
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs:20:9
   |
   |
LL | #![warn(unused_mut)]
   |         ^^^^^^^^^^

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/typeck/issue-73592-borrow_mut-through-deref.rs:47:21
   |
LL | fn test_mut_pin_mut(mut s: Pin<&mut S>) {

warning: 2 warnings emitted
------------------------------------------



---- [ui] src/test/ui/variance/variance-contravariant-arg-object.rs stdout ----
diff of stderr:

7    |                     lifetime `'min` defined here
9 LL |     v
9 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
11    |
12    = help: consider adding the following bound: `'min: 'max`


20    |                     lifetime `'min` defined here
22 LL |     v
22 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
24    |
25    = help: consider adding the following bound: `'min: 'max`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-object/variance-contravariant-arg-object.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-object/variance-contravariant-arg-object.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-contravariant-arg-object.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-contravariant-arg-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-object" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-object/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
   |                     ----  ---- lifetime `'max` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-contravariant-arg-object.rs:23:5
   |
   |
LL | fn get_max_from_min<'min, 'max>(v: Box<dyn Get<&'min i32>>)
   |                     ----  ---- lifetime `'max` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-covariant-arg-object.rs stdout ----
diff of stderr:

7    |                     lifetime `'min` defined here
9 LL |     v
9 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
11    |
12    = help: consider adding the following bound: `'min: 'max`


20    |                     lifetime `'min` defined here
22 LL |     v
22 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
24    |
25    = help: consider adding the following bound: `'min: 'max`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-object/variance-covariant-arg-object.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-object/variance-covariant-arg-object.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-covariant-arg-object.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-covariant-arg-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-object" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-object/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
   |                     ----  ---- lifetime `'max` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-covariant-arg-object.rs:23:5
   |
   |
LL | fn get_max_from_min<'min, 'max>(v: Box<dyn Get<&'min i32>>)
   |                     ----  ---- lifetime `'max` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-invariant-arg-object.rs stdout ----
diff of stderr:

7    |                     lifetime `'min` defined here
9 LL |     v
9 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
11    |
12    = help: consider adding the following bound: `'min: 'max`


20    |                     lifetime `'min` defined here
22 LL |     v
22 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
24    |
25    = help: consider adding the following bound: `'min: 'max`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-invariant-arg-object/variance-invariant-arg-object.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-invariant-arg-object/variance-invariant-arg-object.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-invariant-arg-object.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-invariant-arg-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-invariant-arg-object" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-invariant-arg-object/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
   |                     ----  ---- lifetime `'max` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-invariant-arg-object.rs:19:5
   |
   |
LL | fn get_max_from_min<'min, 'max>(v: Box<dyn Get<&'min i32>>)
   |                     ----  ---- lifetime `'max` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-use-covariant-struct-1.rs stdout ----
diff of stderr:

7    |        lifetime `'min` defined here
9 LL |     v
9 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
11    |
12    = help: consider adding the following bound: `'min: 'max`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-covariant-struct-1/variance-use-covariant-struct-1.stderr
To only update this specific test, also pass `--test-args variance/variance-use-covariant-struct-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-use-covariant-struct-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-covariant-struct-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-covariant-struct-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'min,'max>(v: SomeStruct<&'min ()>)
   |        ---- ---- lifetime `'max` defined here
   |        |
   |        lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/variance/variance-use-contravariant-struct-1.rs stdout ----
diff of stderr:

7    |        lifetime `'min` defined here
9 LL |     v
9 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
11    |
12    = help: consider adding the following bound: `'min: 'max`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-contravariant-struct-1/variance-use-contravariant-struct-1.stderr
To only update this specific test, also pass `--test-args variance/variance-use-contravariant-struct-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-use-contravariant-struct-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-contravariant-struct-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-contravariant-struct-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'min,'max>(v: SomeStruct<&'max ()>)
   |        ---- ---- lifetime `'max` defined here
   |        |
   |        lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/variance/variance-use-invariant-struct-1.rs stdout ----
diff of stderr:

7    |        lifetime `'min` defined here
9 LL |     v
9 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
11    |
12    = help: consider adding the following bound: `'min: 'max`
13    = note: requirement occurs because of the type `SomeStruct<&()>`, which makes the generic argument `&()` invariant

23    |        lifetime `'min` defined here
25 LL |     v
25 LL |     v
-    |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
+    |     ^ function was supposed to return data with lifetime `'max` but it is returning
+ data with lifetime `'min`
27    |
28    = help: consider adding the following bound: `'min: 'max`
29    = note: requirement occurs because of the type `SomeStruct<&()>`, which makes the generic argument `&()` invariant

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-invariant-struct-1/variance-use-invariant-struct-1.stderr
To only update this specific test, also pass `--test-args variance/variance-use-invariant-struct-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-use-invariant-struct-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-invariant-struct-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-use-invariant-struct-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'min,'max>(v: SomeStruct<&'max ()>)
   |        ---- ---- lifetime `'max` defined here
   |        |
   |        lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
   = note: requirement occurs because of the type `SomeStruct<&()>`, which makes the generic argument `&()` invariant
   = note: the struct `SomeStruct<T>` is invariant over the parameter `T`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-use-invariant-struct-1.rs:18:5
   |
   |
LL | fn bar<'min,'max>(v: SomeStruct<&'min ()>)
   |        ---- ---- lifetime `'max` defined here
   |        |
   |        lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning
data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
   = note: requirement occurs because of the type `SomeStruct<&()>`, which makes the generic argument `&()` invariant
   = note: the struct `SomeStruct<T>` is invariant over the parameter `T`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/wf/wf-static-method.rs stdout ----
diff of stderr:

7    |      lifetime `'a` defined here
9 LL |         u
9 LL |         u
-    |         ^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |         ^ associated function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
11    |
12    = help: consider adding the following bound: `'b: 'a`


33    |      lifetime `'a` defined here
34 LL |     fn inherent_evil(u: &'b u32) -> &'a u32 {
35 LL |         u
-    |         ^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |         ^ associated function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
37    |
38    = help: consider adding the following bound: `'b: 'a`

45    |         |
45    |         |
46    |         lifetime `'a` defined here
47 LL |     <()>::static_evil(b)
-    |     ^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
49    |
50    = help: consider adding the following bound: `'b: 'a`

57    |                  |
57    |                  |
58    |                  lifetime `'a` defined here
59 LL |     <IndirectEvil>::static_evil(b)
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
61    |
62    = help: consider adding the following bound: `'b: 'a`

69    |                  |
69    |                  |
70    |                  lifetime `'a` defined here
71 LL |     <Evil>::inherent_evil(b)
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning
+ data with lifetime `'b`
73    |
74    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/wf-static-method.stderr
To only update this specific test, also pass `--test-args wf/wf-static-method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-static-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl<'a, 'b> Foo<'a, 'b, Evil<'a, 'b>> for () {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
LL |         u
LL |         u
   |         ^ associated function was supposed to return data with lifetime `'a` but it is returning
data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/wf/wf-static-method.rs:27:18
   |
   |
LL | impl<'a, 'b> Foo<'a, 'b, ()> for IndirectEvil<'a, 'b> {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
...
LL |         let me = Self::make_me();
   |                  ^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/wf/wf-static-method.rs:35:9
   |
