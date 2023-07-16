plain
...........................F............................................................ 704/13057
..............................i......................................................... 792/13057
..........................i............................................................. 880/13057
........................................................................................ 968/13057
.........F.........F....F...F...F.......................F...F...F.................F...F. 1056/13057
..................................F........F...............F............................ 1144/13057
..............................................................F......F.....F...F......F. 1232/13057
..................i..........F...F........F...............................F............. 1320/13057
.........................F................................................i............. 1408/13057
...................................................F.................................... 1496/13057
....F................................................................................... 1584/13057
..............F....................F....F............................................... 1672/13057
........................................................................................ 1848/13057
.......................................................i................................ 1936/13057
........................................................................................ 2024/13057
........................................................................................ 2112/13057
---
........................................................................................ 4840/13057
........................................................................................ 4928/13057
........................................................................i............... 5016/13057
................................................i....................................... 5104/13057
.................................F.F.................................................... 5192/13057
.........................................................F.............................. 5368/13057
........................................................................................ 5456/13057
.........................................................F.............................. 5544/13057
........................................................................................ 5632/13057
........................................................................................ 5632/13057
.................................................................................i...... 5720/13057
........................................................................................ 5808/13057
........................................................................................ 5896/13057
........................................................................................ 5984/13057
........................................................................................ 6072/13057
...............................................................................F........ 6160/13057
...............................F..........F............................................i 6248/13057
........................................................................................ 6424/13057
i....................................................................................... 6512/13057
........................................................................i............... 6600/13057
......................................ii.ii........i...i................................ 6688/13057
---
........................i............................................................... 7656/13057
........................................................................................ 7744/13057
........................................................................................ 7832/13057
.........................ii................i....i..ii................................... 7920/13057
...........F............................................................................ 8008/13057
........................F...........................F............F...................... 8096/13057
...............................F........................................................ 8184/13057
.........................................................i..ii.......................... 8360/13057
....................................ii.................................................. 8448/13057
..........................................iiii.......................................... 8536/13057
............................................................................i........... 8624/13057
---
...........i............................................................................ 10648/13057
....................iiiiii.i..iiiiii.i.................................................. 10736/13057
........................................................................................ 10824/13057
........................................................................................ 10912/13057
.F...........F.......................................................................... 11000/13057
.............F.......................................................................... 11176/13057
........................................................................................ 11264/13057
........................................................................................ 11352/13057
.............F.......................................................................... 11440/13057
---
........................................................................................ 12144/13057
............................................................................i........... 12232/13057
........................................................................................ 12320/13057
........................................................................................ 12408/13057
...................................................FF................................... 12496/13057
..............F..........FF............................................................. 12584/13057
........................................................................................ 12760/13057
........................................................................................ 12848/13057
........................................................................................ 12936/13057
..............iii....................................................................... 13024/13057
..............iii....................................................................... 13024/13057
.................................
failures:

---- [ui] src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs stdout ----
diff of stderr:

10 LL |     y
11    |     - returning this value requires that `*x` is borrowed for `'1`
12 
- error[E0506]: cannot assign to `*x` because it is borrowed
+ error[E0506]: cannot assign to value because it is borrowed
15    |
15    |
16 LL |         let y = &*x;

-    |                 --- borrow of `*x` occurs here
+    |                 --- borrow of value occurs here
18 LL |         *x += 1;
-    |         ^^^^^^^ assignment to borrowed `*x` occurs here
+    |         ^^^^^^^ assignment to borrowed value occurs here
20 LL |         y
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that borrow lasts for `'1`
22 LL |     })()
23    |     - return type of async closure is &'1 i32


- error[E0506]: cannot assign to `*x` because it is borrowed
+ error[E0506]: cannot assign to value because it is borrowed
27    |
27    |
28 LL |     (async move || -> &i32 {

29    |                       - let's call the lifetime of this reference `'1`
30 LL |         let y = &*x;
-    |                 --- borrow of `*x` occurs here
+    |                 --- borrow of value occurs here
32 LL |         *x += 1;
-    |         ^^^^^^^ assignment to borrowed `*x` occurs here
+    |         ^^^^^^^ assignment to borrowed value occurs here
34 LL |         y
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that borrow lasts for `'1`
36 
- error[E0506]: cannot assign to `*x` because it is borrowed
+ error[E0506]: cannot assign to value because it is borrowed
39    |
39    |
40 LL |         let y = &*x;

-    |                 --- borrow of `*x` occurs here
+    |                 --- borrow of value occurs here
42 LL |         *x += 1;
-    |         ^^^^^^^ assignment to borrowed `*x` occurs here
+    |         ^^^^^^^ assignment to borrowed value occurs here
44 LL |         y
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that borrow lasts for `'1`
46 LL |     }
47    |     - return type of async block is &'1 i32


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations/issue-74072-lifetime-name-annotations.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations/issue-74072-lifetime-name-annotations.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-74072-lifetime-name-annotations.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74072-lifetime-name-annotations/auxiliary"
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
error[E0506]: cannot assign to value because it is borrowed
  --> /checkout/src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs:16:9
   |
   |
LL |         let y = &*x;
   |                 --- borrow of value occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed value occurs here
LL |         y
   |         - returning this value requires that borrow lasts for `'1`
LL |     })()
   |     - return type of async closure is &'1 i32
error[E0506]: cannot assign to value because it is borrowed
  --> /checkout/src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs:24:9
   |
   |
LL |     (async move || -> &i32 {
   |                       - let's call the lifetime of this reference `'1`
LL |         let y = &*x;
   |                 --- borrow of value occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed value occurs here
LL |         y
   |         - returning this value requires that borrow lasts for `'1`
error[E0506]: cannot assign to value because it is borrowed
  --> /checkout/src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs:32:9
   |
   |
LL |         let y = &*x;
   |                 --- borrow of value occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed value occurs here
LL |         y
   |         - returning this value requires that borrow lasts for `'1`
LL |     }
   |     - return type of async block is &'1 i32
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
3    |
4 LL |   fn bar() -> impl Fn() -> usize {



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait/borrow-immutable-upvar-mutation-impl-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrow-immutable-upvar-mutation-impl-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation-impl-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
LL |   fn bar() -> impl Fn() -> usize {
LL |   fn bar() -> impl Fn() -> usize {
   |      ---      ------------------ change this to return `FnMut` instead of `Fn`
LL |       let mut x = 0;
LL | /     move || {
LL | |         x += 1; //~ ERROR cannot assign
LL | |         x
LL | |     }
   | |_____- in this closure

---

---- [ui] src/test/ui/borrowck/borrow-immutable-upvar-mutation.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
3    |
3    |
4 LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
9    |                  |
9    |                  |
10    |                  expects `Fn` instead of `FnMut`
11 
- error[E0596]: cannot borrow `y` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
14    |
14    |
15 LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
20    |                  |
20    |                  |
21    |                  expects `Fn` instead of `FnMut`
22 
- error[E0594]: cannot assign to `z`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
25    |
25    |
26 LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
31    |             |
31    |             |
32    |             expects `Fn` instead of `FnMut`
33 
- error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
36    |
36    |
37 LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
42    |                  |
42    |                  |
43    |                  expects `Fn` instead of `FnMut`
44 
- error[E0596]: cannot borrow `y` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
47    |
47    |
48 LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
53    |                  |
53    |                  |
54    |                  expects `Fn` instead of `FnMut`
55 
- error[E0594]: cannot assign to `z`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
58    |
58    |
59 LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
64    |             |
64    |             |
65    |             expects `Fn` instead of `FnMut`
66 
- error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
69    |
70 LL |   fn foo() -> Box<dyn Fn() -> usize> {



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation/borrow-immutable-upvar-mutation.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrow-immutable-upvar-mutation.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-immutable-upvar-mutation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-immutable-upvar-mutation/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
   |
LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |         let _f = to_fn(|| x = 42); //~ ERROR cannot assign
   |                  |
   |                  |
   |                  expects `Fn` instead of `FnMut`

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |         let _g = to_fn(|| set(&mut y)); //~ ERROR cannot borrow
   |                  -----        ^^^^^^ cannot borrow as mutable
   |                  |
   |                  expects `Fn` instead of `FnMut`

error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
   |
LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |             to_fn(|| z = 42); //~ ERROR cannot assign
   |             |
   |             |
   |             expects `Fn` instead of `FnMut`

error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
   |
LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |         let _f = to_fn(move || x = 42); //~ ERROR cannot assign
   |                  |
   |                  |
   |                  expects `Fn` instead of `FnMut`

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |         let _g = to_fn(move || set(&mut y)); //~ ERROR cannot borrow
   |                  -----             ^^^^^^ cannot borrow as mutable
   |                  |
   |                  expects `Fn` instead of `FnMut`

error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
   |
LL | fn to_fn<A, F: Fn<A>>(f: F) -> F {
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |             to_fn(move || z = 42);
   |             |
   |             |
   |             expects `Fn` instead of `FnMut`

error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
LL |   fn foo() -> Box<dyn Fn() -> usize> {
LL |   fn foo() -> Box<dyn Fn() -> usize> {
   |      ---      ---------------------- change this to return `FnMut` instead of `Fn`
LL |       let mut x = 0;
LL |       Box::new(move || {
   |  ______________-
LL | |         x += 1; //~ ERROR cannot assign
LL | |         x
LL | |     })
   | |_____- in this closure

---

---- [ui] src/test/ui/borrowck/borrowck-access-permissions.rs stdout ----
diff of stderr:

7 LL |         let _y1 = &mut x;
8    |                   ^^^^^^ cannot borrow as mutable
9 
- error[E0596]: cannot borrow immutable static item `static_x` as mutable
+ error[E0596]: cannot borrow immutable static item value as mutable
12    |
12    |
13 LL |         let _y1 = &mut static_x;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-access-permissions/borrowck-access-permissions.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-access-permissions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-access-permissions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-access-permissions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-access-permissions/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
LL |     let x = 1;
LL |     let x = 1;
   |         - help: consider changing this to be mutable: `mut x`
...
LL |         let _y1 = &mut x; //~ ERROR [E0596]
   |                   ^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow immutable static item value as mutable
   |
   |
LL |         let _y1 = &mut static_x; //~ ERROR [E0596]
   |                   ^^^^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `*box_x` as mutable, as `box_x` is not declared as mutable
   |
   |
LL |         let box_x = Box::new(1);
   |             ----- help: consider changing this to be mutable: `mut box_x`
...
LL |         let _y1 = &mut *box_x; //~ ERROR [E0596]
   |                   ^^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `*ref_x` as mutable, as it is behind a `&` reference
   |
   |
LL |         let ref_x = &x;
   |                     -- help: consider changing this to be a mutable reference: `&mut x`
...
LL |         let _y1 = &mut *ref_x; //~ ERROR [E0596]
   |                   ^^^^^^^^^^^ `ref_x` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error[E0596]: cannot borrow `*ptr_x` as mutable, as it is behind a `*const` pointer
   |
   |
LL |         let ptr_x : *const _ = &x;
   |                                -- help: consider changing this to be a mutable pointer: `&mut x`
...
LL |             let _y1 = &mut *ptr_x; //~ ERROR [E0596]
   |                       ^^^^^^^^^^^ `ptr_x` is a `*const` pointer, so the data it refers to cannot be borrowed as mutable

error[E0596]: cannot borrow `*foo_ref.f` as mutable, as it is behind a `&` reference
   |
   |
LL |         let foo_ref = &foo;
   |                       ---- help: consider changing this to be a mutable reference: `&mut foo`
LL |         let _y = &mut *foo_ref.f; //~ ERROR [E0596]
   |                  ^^^^^^^^^^^^^^^ `foo_ref` is a `&` reference, so the data it refers to cannot be borrowed as mutable
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0596`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrow-raw-address-of-mutability.rs stdout ----
diff of stderr:

6 LL |     let y = &raw mut x;
7    |             ^^^^^^^^^^ cannot borrow as mutable
8 
- error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
11    |
12 LL |     let x = 0;

26 LL |     f();
26 LL |     f();
27    |     ^ cannot borrow as mutable
28 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
31    |
31    |
32 LL |   fn make_fn<F: Fn()>(f: F) -> F { f }
41 LL | |     });
42    | |_____- in this closure
43 
43 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
46    |
46    |
47 LL |   fn make_fn<F: Fn()>(f: F) -> F { f }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability/borrow-raw-address-of-mutability.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrow-raw-address-of-mutability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrow-raw-address-of-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrow-raw-address-of-mutability/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   |
LL |     let x = 0;
LL |     let x = 0;
   |         - help: consider changing this to be mutable: `mut x`
LL |     let y = &raw mut x;                 //~ ERROR cannot borrow
   |             ^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
   |
LL |     let x = 0;
LL |     let x = 0;
   |         - help: consider changing this to be mutable: `mut x`
LL |     let mut f = || {
LL |         let y = &raw mut x;             //~ ERROR cannot borrow
   |                 ^^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow `f` as mutable, as it is not declared as mutable
   |
LL |     let f = || {
LL |     let f = || {
   |         - help: consider changing this to be mutable: `mut f`
LL |         let y = &raw mut x;
   |                          - calling `f` requires mutable binding due to mutable borrow of `x`
LL |     };
LL |     f();                                //~ ERROR cannot borrow
   |     ^ cannot borrow as mutable

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn make_fn<F: Fn()>(f: F) -> F { f }
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |       let f = make_fn(|| {
   |  _____________-------_-
   | |             |
   | |             expects `Fn` instead of `FnMut`
LL | |         let y = &raw mut x;             //~ ERROR cannot borrow
   | |                 ^^^^^^^^^^ cannot borrow as mutable
LL | |     });
   | |_____- in this closure

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn make_fn<F: Fn()>(f: F) -> F { f }
   |                          - change this to accept `FnMut` instead of `Fn`
...
LL |       let f = make_fn(move || {
   |  _____________-------_-
   | |             |
   | |             expects `Fn` instead of `FnMut`
LL | |         let y = &raw mut x;             //~ ERROR cannot borrow
   | |                 ^^^^^^^^^^ cannot borrow as mutable
LL | |     });
   | |_____- in this closure
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0596`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-assign-to-constants.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to immutable static item `foo`
+ error[E0594]: cannot assign to immutable static item value
3    |
4 LL |     foo = 6;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-constants/borrowck-assign-to-constants.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-assign-to-constants.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-assign-to-constants.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-constants" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-assign-to-constants/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to immutable static item value
   |
   |
LL |     foo = 6; //~ ERROR cannot assign to immutable static item `foo`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
For more information about this error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-closures-mut-of-imm.rs stdout ----
diff of stderr:

- error[E0596]: cannot borrow `*x` as mutable, as it is behind a `&` reference
+ error[E0596]: cannot borrow data in a `&` reference as mutable
3    |
3    |
4 LL |     let mut c1 = || set(&mut *x);
5    |                         ^^^^^^^ cannot borrow as mutable
6 
6 
- error[E0596]: cannot borrow `*x` as mutable, as it is behind a `&` reference
+ error[E0596]: cannot borrow data in a `&` reference as mutable
9    |
9    |
10 LL |     let mut c2 = || set(&mut *x);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-mut-of-imm/borrowck-closures-mut-of-imm.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-closures-mut-of-imm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-mut-of-imm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-mut-of-imm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-mut-of-imm/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow data in a `&` reference as mutable
   |
   |
LL |     let mut c1 = || set(&mut *x);
   |                         ^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow data in a `&` reference as mutable
   |
   |
LL |     let mut c2 = || set(&mut *x);
   |                         ^^^^^^^ cannot borrow as mutable

error[E0524]: two closures require unique access to `x` at the same time
   |
   |
LL |     let mut c1 = || set(&mut *x);
   |                  --          -- first borrow occurs due to use of `x` in closure
   |                  first closure is constructed here
   |                  first closure is constructed here
LL |     //~^ ERROR cannot borrow
LL |     let mut c2 = || set(&mut *x);
   |                  ^^          -- second borrow occurs due to use of `x` in closure
   |                  second closure is constructed here
...
...
LL |     c2(); c1();

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0524, E0596.
Some errors have detailed explanations: E0524, E0596.
For more information about an error, try `rustc --explain E0524`.
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-closures-unique-imm.rs stdout ----
diff of stderr:

- error[E0502]: cannot borrow `this.x` as mutable because it is also borrowed as immutable
+ error[E0502]: cannot borrow value as mutable because it is also borrowed as immutable
3    |
3    |
4 LL |         let p = &this.x;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique-imm/borrowck-closures-unique-imm.stderr
To only update this specific test, also pass `--test-args borrowck/borrowck-closures-unique-imm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-unique-imm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique-imm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique-imm/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0502]: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |         let p = &this.x;
   |                 ------- immutable borrow occurs here
LL |         &mut this.x; //~ ERROR cannot borrow
   |         ^^^^^^^^^^^ mutable borrow occurs here
LL |         p.use_ref();
   |         ----------- immutable borrow later used here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
------------------------------------------
---

40 LL |     c1;
41    |     -- first borrow later used here
42 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
45    |
45    |
46 LL | fn e(x: &'static mut isize) {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique/borrowck-closures-unique.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-closures-unique.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-closures-unique.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-closures-unique/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0500]: closure requires unique access to `x` but it is already borrowed
   |
   |
LL |     let c1 = || get(x);
   |              --     - first borrow occurs due to use of `x` in closure
   |              |
   |              borrow occurs here
LL |     let c2 = || set(x); //~ ERROR closure requires unique access to `x`
   |              ^^     - second borrow occurs due to use of `x` in closure
   |              closure construction occurs here
LL |     c1;
   |     -- first borrow later used here


error[E0500]: closure requires unique access to `x` but it is already borrowed
   |
   |
LL |     let c1 = || get(x);
   |              --     - first borrow occurs due to use of `x` in closure
   |              |
   |              borrow occurs here
LL |     let c2 = || { get(x); set(x); }; //~ ERROR closure requires unique access to `x`
   |              ^^               - second borrow occurs due to use of `x` in closure
   |              closure construction occurs here
LL |     c1;
   |     -- first borrow later used here


error[E0524]: two closures require unique access to `x` at the same time
   |
   |
LL |     let c1 = || set(x);
   |              --     - first borrow occurs due to use of `x` in closure
   |              first closure is constructed here
   |              first closure is constructed here
LL |     let c2 = || set(x); //~ ERROR two closures require unique access to `x` at the same time
   |              ^^     - second borrow occurs due to use of `x` in closure
   |              second closure is constructed here
LL |     c1;
   |     -- first borrow later used here


error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/borrowck/borrowck-closures-unique.rs:43:38
   |
LL | fn e(x: &'static mut isize) {
   |      - help: consider changing this to be mutable: `mut x`
LL |     let c1 = |y: &'static mut isize| x = y;

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0500, E0524, E0594.
Some errors have detailed explanations: E0500, E0524, E0594.
For more information about an error, try `rustc --explain E0500`.
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-describe-lvalue.rs stdout ----
diff of stderr:

- error[E0499]: cannot borrow `x` as mutable more than once at a time
+ error[E0499]: cannot borrow value as mutable more than once at a time
3    |
3    |
4 LL |             let y = &mut x;

8 LL |             *y = 1;
10 
10 
- error[E0499]: cannot borrow `x` as mutable more than once at a time
+ error[E0499]: cannot borrow value as mutable more than once at a time
13    |
13    |
14 LL |                    let y = &mut x;
352 LL |             drop(x);
353    |                  - mutable borrow later used here
354 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error[E0382]: use of moved value: `x`
+ error[E0382]: use of moved value
356   --> $DIR/borrowck-describe-lvalue.rs:274:22
357    |
358 LL |                 drop(x);

360 LL |                 drop(x);
361    |                      ^ value used here after move
362    |
-    = note: move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
+    = note: move occurs because value has type `Vec<i32>`, which does not implement the `Copy` trait
365 error: aborting due to 32 previous errors
366 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/borrowck-describe-lvalue.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-describe-lvalue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0499]: cannot borrow value as mutable more than once at a time
   |
   |
LL |             let y = &mut x;
   |                     ------ first mutable borrow occurs here
LL |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |             ^^^^^^ second mutable borrow occurs here
LL |             *y = 1;


error[E0499]: cannot borrow value as mutable more than once at a time
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

error[E0382]: use of moved value
error[E0382]: use of moved value
  --> /checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs:274:22
   |
LL |                 drop(x);
   |                      - value moved here
LL |                 drop(x); //~ ERROR use of moved value: `x`
   |                      ^ value used here after move
   |
   = note: move occurs because value has type `Vec<i32>`, which does not implement the `Copy` trait
error: aborting due to 32 previous errors

Some errors have detailed explanations: E0382, E0499, E0502, E0503.
For more information about an error, try `rustc --explain E0382`.
For more information about an error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] src/test/ui/borrowck/borrowck-in-static.rs stdout ----
diff of stderr:

---

---- [ui] src/test/ui/borrowck/issue-42344.rs stdout ----
diff of stderr:

- error[E0596]: cannot borrow `*TAB[_]` as mutable, as `TAB` is an immutable static item
+ error[E0596]: cannot borrow value as mutable, as `TAB` is an immutable static item
3    |
3    |
4 LL |     TAB[0].iter_mut();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-42344/issue-42344.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-42344.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-42344.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-42344" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-42344/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow value as mutable, as `TAB` is an immutable static item
   |
   |
LL |     TAB[0].iter_mut();
   |     ^^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-47215-ice-from-drop-elab.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of static item `X`
+ error[E0507]: cannot move out of static item value
3    |
4 LL |         let mut x = X;

5    |                     ^
5    |                     ^
6    |                     |
-    |                     move occurs because `X` has type `AtomicUsize`, which does not implement the `Copy` trait
+    |                     move occurs because value has type `AtomicUsize`, which does not implement the `Copy` trait
9 
10 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-47215-ice-from-drop-elab/issue-47215-ice-from-drop-elab.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-47215-ice-from-drop-elab.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-47215-ice-from-drop-elab.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-47215-ice-from-drop-elab" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-47215-ice-from-drop-elab/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of static item value
   |
   |
LL |         let mut x = X; //~ ERROR cannot move out of static item `X` [E0507]
   |                     |
   |                     |
   |                     move occurs because value has type `AtomicUsize`, which does not implement the `Copy` trait

error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
For more information about this error, try `rustc --explain E0507`.
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs#nll stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
3    |
4 LL |     pub fn e(x: &'static mut isize) {

7 LL |         let mut c1 = |y: &'static mut isize| x = y;
9 
9 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
12    |
12    |
13 LL |     pub fn ee(x: &'static mut isize) {

16 LL |             let mut c2 = |y: &'static mut isize| x = y;
18 
18 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
21    |
21    |
22 LL |     pub fn capture_assign_whole(x: (i32,)) {

24 LL |         || { x = (1,); };
26 
26 
- error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
+ error[E0594]: cannot assign to value, as `x` is not declared as mutable
29    |
29    |
30 LL |     pub fn capture_assign_part(x: (i32,)) {

32 LL |         || { x.0 = 1; };
34 
34 
- error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
37    |
37    |
38 LL |     pub fn capture_reborrow_whole(x: (i32,)) {

40 LL |         || { &mut x; };
41    |              ^^^^^^ cannot borrow as mutable
42 
- error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as `x` is not declared as mutable
45    |
45    |
46 LL |     pub fn capture_reborrow_part(x: (i32,)) {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.nll/issue-55492-borrowck-migrate-scans-parents.nll.stderr
To only update this specific test, also pass `--test-args borrowck/issue-55492-borrowck-migrate-scans-parents.rs`


error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
   |
LL |     pub fn e(x: &'static mut isize) {
   |              - help: consider changing this to be mutable: `mut x`
LL |         static mut Y: isize = 3;
LL |         let mut c1 = |y: &'static mut isize| x = y;

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:28:50
   |
   |
LL |     pub fn ee(x: &'static mut isize) {
   |               - help: consider changing this to be mutable: `mut x`
...
LL |             let mut c2 = |y: &'static mut isize| x = y;

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:40:14
   |
   |
LL |     pub fn capture_assign_whole(x: (i32,)) {
   |                                 - help: consider changing this to be mutable: `mut x`
LL |         || { x = (1,); };


error[E0594]: cannot assign to value, as `x` is not declared as mutable
   |
   |
LL |     pub fn capture_assign_part(x: (i32,)) {
   |                                - help: consider changing this to be mutable: `mut x`
LL |         || { x.0 = 1; };


error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
   |
   |
LL |     pub fn capture_reborrow_whole(x: (i32,)) {
   |                                   - help: consider changing this to be mutable: `mut x`
LL |         || { &mut x; };
   |              ^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow value as mutable, as `x` is not declared as mutable
   |
   |
LL |     pub fn capture_reborrow_part(x: (i32,)) {
   |                                  - help: consider changing this to be mutable: `mut x`
LL |         || { &mut x.0; };
   |              ^^^^^^^^ cannot borrow as mutable
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0594, E0596.
For more information about an error, try `rustc --explain E0594`.
For more information about an error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs#migrate stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
3    |
4 LL |     pub fn e(x: &'static mut isize) {

7 LL |         let mut c1 = |y: &'static mut isize| x = y;
9 
9 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
12    |
12    |
13 LL |     pub fn ee(x: &'static mut isize) {

16 LL |             let mut c2 = |y: &'static mut isize| x = y;
18 
18 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
21    |
21    |
22 LL |     pub fn capture_assign_whole(x: (i32,)) {

24 LL |         || { x = (1,); };
26 
26 
- error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
+ error[E0594]: cannot assign to value, as `x` is not declared as mutable
29    |
29    |
30 LL |     pub fn capture_assign_part(x: (i32,)) {

32 LL |         || { x.0 = 1; };
34 
34 
- error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
37    |
37    |
38 LL |     pub fn capture_reborrow_whole(x: (i32,)) {

40 LL |         || { &mut x; };
41    |              ^^^^^^ cannot borrow as mutable
42 
- error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as `x` is not declared as mutable
45    |
45    |
46 LL |     pub fn capture_reborrow_part(x: (i32,)) {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.migrate/issue-55492-borrowck-migrate-scans-parents.migrate.stderr
To only update this specific test, also pass `--test-args borrowck/issue-55492-borrowck-migrate-scans-parents.rs`


error in revision `migrate`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.migrate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.migrate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
   |
LL |     pub fn e(x: &'static mut isize) {
   |              - help: consider changing this to be mutable: `mut x`
LL |         static mut Y: isize = 3;
LL |         let mut c1 = |y: &'static mut isize| x = y;

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:28:50
   |
   |
LL |     pub fn ee(x: &'static mut isize) {
   |               - help: consider changing this to be mutable: `mut x`
...
LL |             let mut c2 = |y: &'static mut isize| x = y;

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/borrowck/issue-55492-borrowck-migrate-scans-parents.rs:40:14
   |
   |
LL |     pub fn capture_assign_whole(x: (i32,)) {
   |                                 - help: consider changing this to be mutable: `mut x`
LL |         || { x = (1,); };


error[E0594]: cannot assign to value, as `x` is not declared as mutable
   |
   |
LL |     pub fn capture_assign_part(x: (i32,)) {
   |                                - help: consider changing this to be mutable: `mut x`
LL |         || { x.0 = 1; };


error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
   |
   |
LL |     pub fn capture_reborrow_whole(x: (i32,)) {
   |                                   - help: consider changing this to be mutable: `mut x`
LL |         || { &mut x; };
   |              ^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow value as mutable, as `x` is not declared as mutable
   |
   |
LL |     pub fn capture_reborrow_part(x: (i32,)) {
   |                                  - help: consider changing this to be mutable: `mut x`
LL |         || { &mut x.0; };
   |              ^^^^^^^^ cannot borrow as mutable
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0594, E0596.
For more information about an error, try `rustc --explain E0594`.
---

16    = note: calls in statics are limited to constant functions, tuple structs and tuple variants
17    = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
18 
- error[E0507]: cannot move out of static item `settings_dir`
+ error[E0507]: cannot move out of static item value
21    |
21    |
22 LL |     let settings_data = from_string(settings_dir);

-    |                                     ^^^^^^^^^^^^ move occurs because `settings_dir` has type `String`, which does not implement the `Copy` trait
+    |                                     ^^^^^^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
25 error: aborting due to 3 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453/issue-64453.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-64453.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-64453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-64453/auxiliary"
stdout: none
--- stderr -------------------------------
error: `Arguments::<'a>::new_v1` is not yet stable as a const fn
   |
   |
LL | static settings_dir: String = format!("");
   |
   = help: add `#![feature(const_fmt_arguments_new)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::__export::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0015]: cannot call non-const fn `format` in statics
  --> /checkout/src/test/ui/borrowck/issue-64453.rs:4:31
   |
LL | static settings_dir: String = format!("");
   |
   = note: calls in statics are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0507]: cannot move out of static item value
   |
   |
LL |     let settings_data = from_string(settings_dir);
   |                                     ^^^^^^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0015, E0507.
For more information about an error, try `rustc --explain E0015`.
For more information about an error, try `rustc --explain E0015`.
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-87456-point-to-closure.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of `val`, a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, a captured variable in an `FnMut` closure
3    |
4 LL |       let val = String::new();


10 LL | |         let _foo: String = val;
12    | |                            |
12    | |                            |
-    | |                            move occurs because `val` has type `String`, which does not implement the `Copy` trait
+    | |                            move occurs because value has type `String`, which does not implement the `Copy` trait
15 LL | |
16 LL | |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-87456-point-to-closure/issue-87456-point-to-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-87456-point-to-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-87456-point-to-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-87456-point-to-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-87456-point-to-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of value, a captured variable in an `FnMut` closure
   |
LL |       let val = String::new();
   |           --- captured outer variable
   |           --- captured outer variable
LL |       //~^ NOTE: captured outer variable
LL |       take_mut(|| {
   |  ______________-
LL | |     //~^ NOTE: captured by this `FnMut` closure
LL | |         let _foo: String = val;
   | |                            |
   | |                            |
   | |                            move occurs because value has type `String`, which does not implement the `Copy` trait
   | |                            help: consider borrowing here: `&val`
LL | |         //~^ ERROR: cannot move out of `val`, a captured variable in an `FnMut` closure [E0507]
LL | |         //~| NOTE: move occurs because
LL | |     })
   | |_____- captured by this `FnMut` closure
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/move-error-snippets.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of static item `D`
+ error[E0507]: cannot move out of static item value
3    |
4 LL |         let a = $c;

5    |                 ^^
5    |                 ^^
6    |                 |
-    |                 move occurs because `D` has type `A`, which does not implement the `Copy` trait
+    |                 move occurs because value has type `A`, which does not implement the `Copy` trait
8    |                 help: consider borrowing here: `&$c`
10   ::: $DIR/move-error-snippets.rs:21:1


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/move-error-snippets.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/move-error-snippets.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-snippets.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of static item value
   |
LL |         let a = $c;
   |                 ^^
   |                 |
   |                 |
   |                 move occurs because value has type `A`, which does not implement the `Copy` trait
   |                 help: consider borrowing here: `&$c`
  ::: /checkout/src/test/ui/borrowck/move-error-snippets.rs:21:1
   |
   |
LL | sss!();
   |
   |
   = note: this error originates in the macro `aaa` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/mutability-errors.rs stdout ----
diff of stderr:

116 LL |     &mut (*f()).0;
117    |     ^^^^^^^^^^^^^ cannot borrow as mutable
118 
- error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
121    |
121    |
122 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
134 LL | |     });
135    | |_____- in this closure
136 
136 
- error[E0594]: cannot assign to `x.0`, as `Fn` closures cannot mutate their captured variables
+ error[E0594]: cannot assign to value, as `Fn` closures cannot mutate their captured variables
139    |
139    |
140 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
152 LL | |     });
153    | |_____- in this closure
154 
154 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
157    |
157    |
158 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
170 LL | |     });
171    | |_____- in this closure
172 
172 
- error[E0596]: cannot borrow `x.0` as mutable, as `Fn` closures cannot mutate their captured variables
+ error[E0596]: cannot borrow value as mutable, as `Fn` closures cannot mutate their captured variables
175    |
175    |
176 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
188 LL | |     });
189    | |_____- in this closure
190 
190 
- error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
193    |
193    |
194 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
206 LL | |     });
207    | |_____- in this closure
208 
208 
- error[E0594]: cannot assign to `x.0`, as `Fn` closures cannot mutate their captured variables
+ error[E0594]: cannot assign to value, as `Fn` closures cannot mutate their captured variables
211    |
211    |
212 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
224 LL | |     });
225    | |_____- in this closure
226 
226 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
229    |
229    |
230 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
242 LL | |     });
243    | |_____- in this closure
244 
244 
- error[E0596]: cannot borrow `x.0` as mutable, as `Fn` closures cannot mutate their captured variables
+ error[E0596]: cannot borrow value as mutable, as `Fn` closures cannot mutate their captured variables
247    |
247    |
248 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
277 LL |     &mut x.0;
278    |     ^^^^^^^^ cannot borrow as mutable
279 
279 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
282    |
282    |
283 LL | fn imm_capture(x: (i32,)) {
286 LL |         x = (1,);
287    |         ^^^^^^^^ cannot assign
288 
288 
- error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
+ error[E0594]: cannot assign to value, as `x` is not declared as mutable
291    |
291    |
292 LL | fn imm_capture(x: (i32,)) {
295 LL |         x.0 = 1;
296    |         ^^^^^^^ cannot assign
297 
297 
- error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
300    |
300    |
301 LL | fn imm_capture(x: (i32,)) {
304 LL |         &mut x;
305    |         ^^^^^^ cannot borrow as mutable
306 
306 
- error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as `x` is not declared as mutable
309    |
309    |
310 LL | fn imm_capture(x: (i32,)) {
313 LL |         &mut x.0;
314    |         ^^^^^^^^ cannot borrow as mutable
315 
315 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
318    |
318    |
319 LL | fn imm_capture(x: (i32,)) {
322 LL |         x = (1,);
323    |         ^^^^^^^^ cannot assign
324 
324 
- error[E0594]: cannot assign to `x.0`, as `x` is not declared as mutable
+ error[E0594]: cannot assign to value, as `x` is not declared as mutable
327    |
327    |
328 LL | fn imm_capture(x: (i32,)) {
331 LL |         x.0 = 1;
332    |         ^^^^^^^ cannot assign
333 
333 
- error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
336    |
336    |
337 LL | fn imm_capture(x: (i32,)) {
340 LL |         &mut x;
341    |         ^^^^^^ cannot borrow as mutable
342 
342 
- error[E0596]: cannot borrow `x.0` as mutable, as `x` is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as `x` is not declared as mutable
345    |
345    |
346 LL | fn imm_capture(x: (i32,)) {
349 LL |         &mut x.0;
350    |         ^^^^^^^^ cannot borrow as mutable
351 
351 
- error[E0594]: cannot assign to immutable static item `X`
+ error[E0594]: cannot assign to immutable static item value
354    |
355 LL |     X = (1,);

356    |     ^^^^^^^^ cannot assign
356    |     ^^^^^^^^ cannot assign
357 
- error[E0594]: cannot assign to `X.0`, as `X` is an immutable static item
+ error[E0594]: cannot assign to value, as `X` is an immutable static item
360    |
361 LL |     X.0 = 1;

362    |     ^^^^^^^ cannot assign
362    |     ^^^^^^^ cannot assign
363 
- error[E0596]: cannot borrow immutable static item `X` as mutable
+ error[E0596]: cannot borrow immutable static item value as mutable
366    |
367 LL |     &mut X;

368    |     ^^^^^^ cannot borrow as mutable
368    |     ^^^^^^ cannot borrow as mutable
369 
- error[E0596]: cannot borrow `X.0` as mutable, as `X` is an immutable static item
+ error[E0596]: cannot borrow value as mutable, as `X` is an immutable static item
372    |
373 LL |     &mut X.0;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mutability-errors/mutability-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/mutability-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mutability-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mutability-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mutability-errors/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to `*x`, which is behind a `&` reference
   |
   |
LL | fn named_ref(x: &(i32,)) {
   |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
LL |     *x = (1,); //~ ERROR
   |     ^^^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written

error[E0594]: cannot assign to `x.0`, which is behind a `&` reference
   |
   |
LL | fn named_ref(x: &(i32,)) {
   |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
LL |     *x = (1,); //~ ERROR
LL |     x.0 = 1; //~ ERROR
   |     ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be written

error[E0596]: cannot borrow `*x` as mutable, as it is behind a `&` reference
   |
   |
LL | fn named_ref(x: &(i32,)) {
   |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
...
LL |     &mut *x; //~ ERROR
   |     ^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error[E0596]: cannot borrow `x.0` as mutable, as it is behind a `&` reference
   |
   |
LL | fn named_ref(x: &(i32,)) {
   |                 ------- help: consider changing this to be a mutable reference: `&mut (i32,)`
...
LL |     &mut x.0; //~ ERROR
   |     ^^^^^^^^ `x` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error[E0594]: cannot assign to data in a `&` reference
   |
   |
LL |     *f() = (1,); //~ ERROR


error[E0594]: cannot assign to data in a `&` reference
   |
   |
LL |     f().0 = 1; //~ ERROR


error[E0596]: cannot borrow data in a `&` reference as mutable
   |
   |
LL |     &mut *f(); //~ ERROR
   |     ^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow data in a `&` reference as mutable
   |
   |
LL |     &mut f().0; //~ ERROR
   |     ^^^^^^^^^^ cannot borrow as mutable

error[E0594]: cannot assign to `*x`, which is behind a `*const` pointer
   |
   |
LL | unsafe fn named_ptr(x: *const (i32,)) {
   |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
LL |     *x = (1,); //~ ERROR
   |     ^^^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be written

error[E0594]: cannot assign to `x.0`, which is behind a `*const` pointer
   |
   |
LL | unsafe fn named_ptr(x: *const (i32,)) {
   |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
LL |     *x = (1,); //~ ERROR
LL |     (*x).0 = 1; //~ ERROR
   |     ^^^^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be written

error[E0596]: cannot borrow `*x` as mutable, as it is behind a `*const` pointer
   |
   |
LL | unsafe fn named_ptr(x: *const (i32,)) {
   |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
...
LL |     &mut *x; //~ ERROR
   |     ^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be borrowed as mutable

error[E0596]: cannot borrow `x.0` as mutable, as it is behind a `*const` pointer
   |
   |
LL | unsafe fn named_ptr(x: *const (i32,)) {
   |                        ------------- help: consider changing this to be a mutable pointer: `*mut (i32,)`
...
LL |     &mut (*x).0; //~ ERROR
   |     ^^^^^^^^^^^ `x` is a `*const` pointer, so the data it refers to cannot be borrowed as mutable

error[E0594]: cannot assign to data in a `*const` pointer
   |
   |
LL |     *f() = (1,); //~ ERROR


error[E0594]: cannot assign to data in a `*const` pointer
   |
   |
LL |     (*f()).0 = 1; //~ ERROR


error[E0596]: cannot borrow data in a `*const` pointer as mutable
   |
   |
LL |     &mut *f(); //~ ERROR
   |     ^^^^^^^^^ cannot borrow as mutable

error[E0596]: cannot borrow data in a `*const` pointer as mutable
   |
   |
LL |     &mut (*f()).0; //~ ERROR
   |     ^^^^^^^^^^^^^ cannot borrow as mutable

error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(|| {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         x = (1,); //~ ERROR
   | |         ^^^^^^^^ cannot assign
LL | |         x.0 = 1; //~ ERROR
LL | |         &mut x; //~ ERROR
LL | |         &mut x.0; //~ ERROR
LL | |     });
   | |_____- in this closure

error[E0594]: cannot assign to value, as `Fn` closures cannot mutate their captured variables
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(|| {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         x = (1,); //~ ERROR
---

---- [ui] src/test/ui/cannot-mutate-captured-non-mut-var.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
4 LL |     let x = 1;


6 LL |     to_fn_once(move|| { x = 2; });
8 
8 
- error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
+ error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
11    |
12 LL |     let s = std::io::stdin();



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cannot-mutate-captured-non-mut-var/cannot-mutate-captured-non-mut-var.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cannot-mutate-captured-non-mut-var.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cannot-mutate-captured-non-mut-var.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cannot-mutate-captured-non-mut-var" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cannot-mutate-captured-non-mut-var/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
LL |     let x = 1;
LL |     let x = 1;
   |         - help: consider changing this to be mutable: `mut x`
LL |     to_fn_once(move|| { x = 2; });


error[E0596]: cannot borrow value as mutable, as it is not declared as mutable
   |
LL |     let s = std::io::stdin();
LL |     let s = std::io::stdin();
   |         - help: consider changing this to be mutable: `mut s`
LL |     to_fn_once(move|| { s.read_to_end(&mut Vec::new()); });
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot borrow as mutable
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0594, E0596.
For more information about an error, try `rustc --explain E0594`.
---

53 LL |     box 3;
54    |     ^^^^^ allocation not allowed in statics
55 
- error[E0507]: cannot move out of static item `x`
+ error[E0507]: cannot move out of static item value
58    |
58    |
59 LL |     let y = { static x: Box<isize> = box 3; x };
60    |                                             ^
61    |                                             |
61    |                                             |
-    |                                             move occurs because `x` has type `Box<isize>`, which does not implement the `Copy` trait
+    |                                             move occurs because value has type `Box<isize>`, which does not implement the `Copy` trait
64 
65 error[E0010]: allocations are not allowed in statics



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/check-static-values-constraints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-static-values-constraints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-values-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
   |
   |
LL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),
   |  ___________________________________________^
LL | | //~^ ERROR destructors cannot be evaluated at compile-time
LL | |                                                      field2: SafeEnum::Variant1}};
   | |                                                                                ^- value is dropped here
   |                                                                                  statics cannot evaluate destructors

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;
   |                                 ^^^^^^^^^^^ allocation not allowed in statics

error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in statics
   |
   |
LL |     field2: SafeEnum::Variant4("str".to_string())
   |
   = note: calls in statics are limited to constant functions, tuple structs and tuple variants

error[E0010]: allocations are not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:94:5
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:95:5
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics
   |     ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:99:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:100:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics
   |      ^^^^^^^^^^^ allocation not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:106:5
   |
LL |     box 3;
LL |     box 3;
   |     ^^^^^ allocation not allowed in statics

error[E0507]: cannot move out of static item value
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                             |
   |                                             |
   |                                             move occurs because value has type `Box<isize>`, which does not implement the `Copy` trait

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:110:38
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                      ^^^^^ allocation not allowed in statics
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0010, E0015, E0493, E0507.
For more information about an error, try `rustc --explain E0010`.
For more information about an error, try `rustc --explain E0010`.
------------------------------------------


---- [ui] src/test/ui/closures/2229_closure_analysis/diagnostics/cant-mutate-imm.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `z.0.0.0`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
3    |
4 LL |     let z = (y, 10);

7 LL |         z.0.0.0 = 20;
9 
9 
- error[E0594]: cannot assign to `*bx.0`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
12    |
12    |
13 LL |     let bx = Box::new(x);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/cant-mutate-imm/cant-mutate-imm.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/diagnostics/cant-mutate-imm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/cant-mutate-imm.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/cant-mutate-imm" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/diagnostics/cant-mutate-imm/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
   |
LL |     let z = (y, 10);
   |         - help: consider changing this to be mutable: `mut z`
...
LL |         z.0.0.0 = 20;

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/closures/2229_closure_analysis/diagnostics/cant-mutate-imm.rs:24:9
   |
   |
LL |     let bx = Box::new(x);
   |         -- help: consider changing this to be mutable: `mut bx`
...
LL |         bx.0 = 20;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0594`.
For more information about this error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/closures/closure-immutable-outer-variable.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `y`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
4 LL |     let y = true;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-immutable-outer-variable/closure-immutable-outer-variable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/closure-immutable-outer-variable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-immutable-outer-variable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-immutable-outer-variable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-immutable-outer-variable/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
LL |     let y = true;
LL |     let y = true;
   |         - help: consider changing this to be mutable: `mut y`
LL |     foo(Box::new(move || y = !y) as Box<_>);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
---
- error[E0382]: use of moved value: `x`
+ error[E0382]: use of moved value
2   --> $DIR/issue-10398.rs:7:14
3    |
4 LL |         let _a = x;
6 LL |         drop(x);
6 LL |         drop(x);
7    |              ^ value used here after move
8    |
-    = note: move occurs because `x` has type `Box<i32>`, which does not implement the `Copy` trait
+    = note: move occurs because value has type `Box<i32>`, which does not implement the `Copy` trait
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-10398/issue-10398.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/issue-10398.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/issue-10398.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-10398" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-10398/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value
   |
   |
LL |         let _a = x;
   |                  - value moved here
LL |         drop(x);
   |              ^ value used here after move
   |
   = note: move occurs because value has type `Box<i32>`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
------------------------------------------
---
- error[E0382]: use of moved value: `t`
+ error[E0382]: use of moved value
2   --> $DIR/issue-67123.rs:2:13
3    |
4 LL |     || { t; t; };
6    |          |
7    |          value moved here
8    |
8    |
-    = note: move occurs because `t` has type `T`, which does not implement the `Copy` trait
+    = note: move occurs because value has type `T`, which does not implement the `Copy` trait
10 help: consider restricting type parameter `T`
11    |
12 LL | fn foo<T: Copy>(t: T) {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-67123/issue-67123.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/issue-67123.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/issue-67123.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-67123" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-67123/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value
   |
   |
LL |     || { t; t; }; //~ ERROR: use of moved value
   |          -  ^ value used here after move
   |          value moved here
   |
   |
   = note: move occurs because value has type `T`, which does not implement the `Copy` trait
help: consider restricting type parameter `T`
   |
LL | fn foo<T: Copy>(t: T) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
For more information about this error, try `rustc --explain E0382`.
------------------------------------------


---- [ui] src/test/ui/consts/miri_unleashed/mutable_references.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item
+ error[E0594]: cannot assign to value, as `OH_YES` is an immutable static item
3    |
3    |
4 LL |     *OH_YES = 99;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/mutable_references.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_references.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_references/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as `OH_YES` is an immutable static item
   |
   |
LL |     *OH_YES = 99; //~ ERROR cannot assign to `*OH_YES`, as `OH_YES` is an immutable static item

warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:9:26
   |
LL | static FOO: &&mut u32 = &&mut 42;
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:13:23
   |
   |
LL | static BAR: &mut () = &mut ();
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:18:28
   |
   |
LL | static BOO: &mut Foo<()> = &mut Foo(());
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:26:8
   |
   |
LL |     x: &UnsafeCell::new(42),
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/mutable_references.rs:30:27
   |
   |
LL | static OH_YES: &mut i32 = &mut 42;

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0594`.
For more information about this error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/consts/write_to_static_via_mut_ref.rs stdout ----
diff of stderr:

4 LL | static OH_NO: &mut i32 = &mut 42;
6 
6 
- error[E0594]: cannot assign to `*OH_NO`, as `OH_NO` is an immutable static item
+ error[E0594]: cannot assign to value, as `OH_NO` is an immutable static item
9    |
9    |
10 LL |     *OH_NO = 43;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write_to_static_via_mut_ref/write_to_static_via_mut_ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/write_to_static_via_mut_ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/write_to_static_via_mut_ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write_to_static_via_mut_ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/write_to_static_via_mut_ref/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: mutable references are not allowed in the final value of statics
   |
   |
LL | static OH_NO: &mut i32 = &mut 42; //~ ERROR mutable references are not allowed


error[E0594]: cannot assign to value, as `OH_NO` is an immutable static item
   |
   |
LL |     *OH_NO = 43; //~ ERROR cannot assign to `*OH_NO`, as `OH_NO` is an immutable static

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0594, E0764.
Some errors have detailed explanations: E0594, E0764.
For more information about an error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/error-codes/E0017.rs stdout ----
diff of stderr:

34 LL | static STATIC_REF: &'static mut i32 = &mut X;
36 
36 
- error[E0596]: cannot borrow immutable static item `X` as mutable
+ error[E0596]: cannot borrow immutable static item value as mutable
38   --> $DIR/E0017.rs:7:39
39    |
40 LL | static STATIC_REF: &'static mut i32 = &mut X;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/E0017.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0017.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/auxiliary"
stdout: none
--- stderr -------------------------------
warning: taking a mutable reference to a `const` item
   |
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   = note: `#[warn(const_item_mutation)]` on by default
   = note: `#[warn(const_item_mutation)]` on by default
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
   |
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^^^^^^


error[E0764]: mutable references are not allowed in the final value of constants
  --> /checkout/src/test/ui/error-codes/E0017.rs:5:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0658]: mutation through a reference is not allowed in statics
  --> /checkout/src/test/ui/error-codes/E0017.rs:7:39
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0764]: mutable references are not allowed in the final value of statics
  --> /checkout/src/test/ui/error-codes/E0017.rs:7:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658


error[E0596]: cannot borrow immutable static item value as mutable
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0658
   |                                       ^^^^^^ cannot borrow as mutable

warning: taking a mutable reference to a `const` item
   |
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   |
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
   |
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^^^^^^


error[E0764]: mutable references are not allowed in the final value of statics
  --> /checkout/src/test/ui/error-codes/E0017.rs:11:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0764]: mutable references are not allowed in the final value of statics
  --> /checkout/src/test/ui/error-codes/E0017.rs:13:52
   |
   |
LL | static STATIC_MUT_REF: &'static mut i32 = unsafe { &mut M }; //~ ERROR mutable references are not

error: aborting due to 6 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0596, E0658, E0764.
Some errors have detailed explanations: E0596, E0658, E0764.
For more information about an error, try `rustc --explain E0596`.
------------------------------------------


---- [ui] src/test/ui/error-codes/E0388.rs stdout ----
diff of stderr:

34 LL | static STATIC_REF: &'static mut i32 = &mut X;
36 
36 
- error[E0596]: cannot borrow immutable static item `X` as mutable
+ error[E0596]: cannot borrow immutable static item value as mutable
38   --> $DIR/E0388.rs:6:39
39    |
40 LL | static STATIC_REF: &'static mut i32 = &mut X;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/E0388.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0388.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0388.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/auxiliary"
stdout: none
--- stderr -------------------------------
warning: taking a mutable reference to a `const` item
   |
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   = note: `#[warn(const_item_mutation)]` on by default
   = note: `#[warn(const_item_mutation)]` on by default
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
   |
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^^^^^^


error[E0764]: mutable references are not allowed in the final value of constants
  --> /checkout/src/test/ui/error-codes/E0388.rs:4:30
   |
LL | const CR: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error[E0658]: mutation through a reference is not allowed in statics
  --> /checkout/src/test/ui/error-codes/E0388.rs:6:39
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0764]: mutable references are not allowed in the final value of statics
  --> /checkout/src/test/ui/error-codes/E0388.rs:6:39
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow


error[E0596]: cannot borrow immutable static item value as mutable
   |
   |
LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR cannot borrow
   |                                       ^^^^^^ cannot borrow as mutable

warning: taking a mutable reference to a `const` item
   |
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed
   |
   |
   = note: each usage of a `const` item creates a new temporary
   = note: the mutable reference will refer to this temporary, not the original `const` item
note: `const` item defined here
   |
LL | const C: i32 = 2;
   | ^^^^^^^^^^^^^^^^^


error[E0764]: mutable references are not allowed in the final value of statics
  --> /checkout/src/test/ui/error-codes/E0388.rs:10:38
   |
LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR mutable references are not allowed

error: aborting due to 5 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0596, E0658, E0764.
Some errors have detailed explanations: E0596, E0658, E0764.
For more information about an error, try `rustc --explain E0596`.
------------------------------------------


---- [ui] src/test/ui/error-codes/E0594.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to immutable static item `NUM`
+ error[E0594]: cannot assign to immutable static item value
2   --> $DIR/E0594.rs:4:5
4 LL |     NUM = 20;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0594/E0594.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0594.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0594.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0594" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0594/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to immutable static item value
   |
   |
LL |     NUM = 20; //~ ERROR cannot assign to immutable static item `NUM`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
For more information about this error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/fn/fn-closure-mutable-capture.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
2   --> $DIR/fn-closure-mutable-capture.rs:5:17
3    |
4 LL | pub fn bar<F: Fn()>(_f: F) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-closure-mutable-capture/fn-closure-mutable-capture.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/fn-closure-mutable-capture.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/fn-closure-mutable-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-closure-mutable-capture" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-closure-mutable-capture/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
   |
LL | pub fn bar<F: Fn()>(_f: F) {} //~ NOTE change this to accept `FnMut` instead of `Fn`
   |                         - change this to accept `FnMut` instead of `Fn`
...
LL |     bar(move || x = 1);
   |     |
   |     |
   |     expects `Fn` instead of `FnMut`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
------------------------------------------
---
2   --> $DIR/issue-12127.rs:11:9
3    |
4 LL |         f();

-    |         --- `f` moved due to this call
+    |         --- value moved due to this call
6 LL |         f();
7    |         ^ value used here after move

11    |
12 LL |         f();
13    |         ^
13    |         ^
-    = note: move occurs because `f` has type `[closure@$DIR/issue-12127.rs:8:24: 8:41]`, which does not implement the `Copy` trait
+    = note: move occurs because value has type `[closure@$DIR/issue-12127.rs:8:24: 8:41]`, which does not implement the `Copy` trait
16 error: aborting due to previous error
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12127/issue-12127.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-12127.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12127.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12127" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12127/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value
   |
LL |         f();
   |         --- value moved due to this call
LL |         f();
LL |         f();
   |         ^ value used here after move
   |
note: this value implements `FnOnce`, which causes it to be moved when called
   |
LL |         f();
   |         ^
   |         ^
   = note: move occurs because value has type `[closure@/checkout/src/test/ui/issues/issue-12127.rs:8:24: 8:41]`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
------------------------------------------
---
- error[E0382]: use of moved value: `tx`
+ error[E0382]: use of moved value
2   --> $DIR/issue-12041.rs:8:22
3    |
4 LL |             let tx = tx;

5    |                      ^^ value moved here, in previous iteration of loop
6    |
-    = note: move occurs because `tx` has type `Sender<i32>`, which does not implement the `Copy` trait
+    = note: move occurs because value has type `Sender<i32>`, which does not implement the `Copy` trait
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12041/issue-12041.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-12041.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12041/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value
   |
   |
LL |             let tx = tx;
   |                      ^^ value moved here, in previous iteration of loop
   |
   = note: move occurs because value has type `Sender<i32>`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-17718-static-move.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of static item `FOO`
+ error[E0507]: cannot move out of static item value
3    |
3    |
4 LL |     let _a = FOO;
5    |              ^^^
6    |              |
6    |              |
-    |              move occurs because `FOO` has type `Foo`, which does not implement the `Copy` trait
+    |              move occurs because value has type `Foo`, which does not implement the `Copy` trait
8    |              help: consider borrowing here: `&FOO`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-move/issue-17718-static-move.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17718-static-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17718-static-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-static-move/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of static item value
   |
   |
LL |     let _a = FOO; //~ ERROR: cannot move out of static item
   |              |
   |              |
   |              move occurs because value has type `Foo`, which does not implement the `Copy` trait
   |              help: consider borrowing here: `&FOO`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-21600.rs stdout ----
diff of stderr:

- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
3    |
3    |
4 LL | fn call_it<F>(f: F) where F: Fn() { f(); }
9    |         |
9    |         |
10    |         expects `Fn` instead of `FnMut`
11 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
14    |
14    |
15 LL |   fn call_it<F>(f: F) where F: Fn() { f(); }

21    | |     expects `Fn` instead of `FnMut`
22 LL | |         call_it(|| x.gen());
23 LL | |         call_it(|| x.gen_mut());
-    | |                 ^^ - mutable borrow occurs due to use of `x` in closure
+    | |                 ^^ - mutable borrow occurs due to use of value in closure
25    | |                 |
26    | |                 cannot borrow as mutable
27 LL | |

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21600/issue-21600.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-21600.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21600.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21600" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21600/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL | fn call_it<F>(f: F) where F: Fn() { f(); }
   |                  - change this to accept `FnMut` instead of `Fn`
...
LL |         call_it(|| x.gen_mut());
   |         -------    ^^^^^^^^^^^ cannot borrow as mutable
   |         |
   |         expects `Fn` instead of `FnMut`

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn call_it<F>(f: F) where F: Fn() { f(); }
   |                    - change this to accept `FnMut` instead of `Fn`
...
LL |       call_it(|| {
   |  _____-------_-
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         call_it(|| x.gen());
LL | |         call_it(|| x.gen_mut());
   | |                 ^^ - mutable borrow occurs due to use of value in closure
   | |                 |
   | |                 cannot borrow as mutable
LL | |         //~^ ERROR cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
LL | |         //~| ERROR cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
LL | |     });
   | |_____- in this closure
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0596`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-4335.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of `*v`, as `v` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `v` is a captured variable in an `FnMut` closure
3    |
3    |
4 LL | fn f<'r, T>(v: &'r T) -> Box<dyn FnMut() -> T + 'r> {

6 LL |     id(Box::new(|| *v))
7    |                 ---^^
8    |                 |  |
-    |                 |  move occurs because `*v` has type `T`, which does not implement the `Copy` trait
+    |                 |  move occurs because value has type `T`, which does not implement the `Copy` trait
10    |                 captured by this `FnMut` closure
12 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335/issue-4335.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-4335.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-4335.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-4335/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of value, as `v` is a captured variable in an `FnMut` closure
   |
   |
LL | fn f<'r, T>(v: &'r T) -> Box<dyn FnMut() -> T + 'r> {
   |             - captured outer variable
LL |     id(Box::new(|| *v))
   |                 ---^^
   |                 |  |
   |                 |  move occurs because value has type `T`, which does not implement the `Copy` trait
   |                 captured by this `FnMut` closure
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-46023.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
4 LL |     let x = 0;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46023/issue-46023.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-46023.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46023.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46023" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46023/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
LL |     let x = 0;
LL |     let x = 0;
   |         - help: consider changing this to be mutable: `mut x`
LL |         x = 1;
   |         ^^^^^ cannot assign

error: aborting due to previous error
---

---- [ui] src/test/ui/issues/issue-46604.rs stdout ----
diff of stderr:

4 LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];
6 
6 
- error[E0594]: cannot assign to `buf[_]`, as `buf` is an immutable static item
+ error[E0594]: cannot assign to value, as `buf` is an immutable static item
9    |
10 LL |     buf[0]=2;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/issue-46604.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-46604.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46604.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: mutable references are not allowed in the final value of statics
   |
   |
LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //~ ERROR mutable references are not allowed


error[E0594]: cannot assign to value, as `buf` is an immutable static item
   |
   |
LL |     buf[0]=2;                                   //~ ERROR E0594

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0594, E0764.
Some errors have detailed explanations: E0594, E0764.
For more information about an error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/moves/moves-based-on-type-move-out-of-closure-env-issue-1965.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of `i`, a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, a captured variable in an `Fn` closure
3    |
4 LL |     let i = Box::new(3);


6 LL |     let _f = to_fn(|| test(i));
7    |                    --------^-
8    |                    |       |
-    |                    |       move occurs because `i` has type `Box<usize>`, which does not implement the `Copy` trait
+    |                    |       move occurs because value has type `Box<usize>`, which does not implement the `Copy` trait
10    |                    captured by this `Fn` closure
12 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-move-out-of-closure-env-issue-1965/moves-based-on-type-move-out-of-closure-env-issue-1965.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args moves/moves-based-on-type-move-out-of-closure-env-issue-1965.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/moves-based-on-type-move-out-of-closure-env-issue-1965.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-move-out-of-closure-env-issue-1965" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/moves-based-on-type-move-out-of-closure-env-issue-1965/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of value, a captured variable in an `Fn` closure
   |
LL |     let i = Box::new(3);
   |         - captured outer variable
   |         - captured outer variable
LL |     let _f = to_fn(|| test(i)); //~ ERROR cannot move out
   |                    --------^-
   |                    |       |
   |                    |       move occurs because value has type `Box<usize>`, which does not implement the `Copy` trait
   |                    captured by this `Fn` closure
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/closure-captures.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
3    |
4 LL | fn one_closure(x: i32) {
7 LL |     x = 1;
8    |     ^^^^^ cannot assign
9 
9 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
12    |
12    |
13 LL | fn one_closure(x: i32) {
16 LL |     x = 1;
17    |     ^^^^^ cannot assign
18 
18 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
21    |
21    |
22 LL | fn two_closures(x: i32) {
25 LL |         x = 1;
26    |         ^^^^^ cannot assign
27 
27 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
30    |
30    |
31 LL | fn two_closures(x: i32) {
34 LL |         x = 1;
35    |         ^^^^^ cannot assign
36 
36 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
39    |
39    |
40 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
49 LL | |          x = 1;}
50    | |__________-_____- in this closure
51    |            |
51    |            |
-    |            mutable borrow occurs due to use of `x` in closure
+    |            mutable borrow occurs due to use of value in closure
53 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
56    |
56    |
57 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }

66 LL | |     x = 1;});
67    | |_____-_____- in this closure
68    |       |
-    |       mutable borrow occurs due to use of `x` in closure
+    |       mutable borrow occurs due to use of value in closure
70 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
73    |
73    |
74 LL | fn two_closures_ref(x: i32) {
77 LL |          x = 1;}
78    |          ^^^^^ cannot assign
79 
79 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
82    |
82    |
83 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
92 LL | |          x = 1;}
93    | |__________-_____- in this closure
94    |            |
94    |            |
-    |            mutable borrow occurs due to use of `x` in closure
+    |            mutable borrow occurs due to use of value in closure
96 
- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
99    |
99    |
100 LL | fn two_closures_ref(x: i32) {
103 LL |     x = 1;});
104    |     ^^^^^ cannot assign
105 
105 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
108    |
108    |
109 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }

118 LL | |     x = 1;});
119    | |_____-_____- in this closure
120    |       |
-    |       mutable borrow occurs due to use of `x` in closure
+    |       mutable borrow occurs due to use of value in closure
122 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
125    |
125    |
126 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }

135 LL | |         *x = 1;});
136    | |_________--_____- in this closure
137    |           |
-    |           mutable borrow occurs due to use of `x` in closure
+    |           mutable borrow occurs due to use of value in closure
139 
- error[E0596]: cannot borrow `x` as mutable, as it is a captured variable in a `Fn` closure
+ error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
142    |
142    |
143 LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }

152 LL | |         *x = 1;});
153    | |_________--_____- in this closure
154    |           |
-    |           mutable borrow occurs due to use of `x` in closure
+    |           mutable borrow occurs due to use of value in closure
157 error: aborting due to 12 previous errors
158 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-captures/closure-captures.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-captures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-captures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-captures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-captures/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
   |
LL | fn one_closure(x: i32) {
   |                - help: consider changing this to be mutable: `mut x`
LL |     ||
LL |     x = 1; //~ ERROR

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/nll/closure-captures.rs:9:5
   |
   |
LL | fn one_closure(x: i32) {
   |                - help: consider changing this to be mutable: `mut x`
...
LL |     x = 1; //~ ERROR

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/nll/closure-captures.rs:15:9
   |
   |
LL | fn two_closures(x: i32) {
   |                 - help: consider changing this to be mutable: `mut x`
...
LL |         x = 1; //~ ERROR

error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/nll/closure-captures.rs:19:9
   |
   |
LL | fn two_closures(x: i32) {
   |                 - help: consider changing this to be mutable: `mut x`
...
LL |         x = 1; //~ ERROR


error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(|| {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         || //~ ERROR
   | |         ^^ cannot borrow as mutable
LL | |          x = 1;}
   | |__________-_____- in this closure
   |            |
   |            mutable borrow occurs due to use of value in closure

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(move || {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         ||  //~ ERROR
   | |         ^^ cannot borrow as mutable
LL | |     x = 1;});
   | |_____-_____- in this closure
   |       |
   |       mutable borrow occurs due to use of value in closure
error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/nll/closure-captures.rs:39:10
   |
   |
LL | fn two_closures_ref(x: i32) {
   |                     - help: consider changing this to be mutable: `mut x`
...
LL |          x = 1;} //~ ERROR


error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(|| {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         || //~ ERROR
   | |         ^^ cannot borrow as mutable
LL | |          x = 1;} //~ ERROR
   | |__________-_____- in this closure
   |            |
   |            mutable borrow occurs due to use of value in closure
error[E0594]: cannot assign to value, as it is not declared as mutable
  --> /checkout/src/test/ui/nll/closure-captures.rs:43:5
   |
   |
LL | fn two_closures_ref(x: i32) {
   |                     - help: consider changing this to be mutable: `mut x`
...
LL |     x = 1;}); //~ ERROR


error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(move || {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         ||  //~ ERROR
   | |         ^^ cannot borrow as mutable
LL | |     x = 1;}); //~ ERROR
   | |_____-_____- in this closure
   |       |
   |       mutable borrow occurs due to use of value in closure

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(|| {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         || //~ ERROR
   | |         ^^ cannot borrow as mutable
LL | |         *x = 1;});
   | |_________--_____- in this closure
   |           |
   |           mutable borrow occurs due to use of value in closure

error[E0596]: cannot borrow value as mutable, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn fn_ref<F: Fn()>(f: F) -> F { f }
   |                         - change this to accept `FnMut` instead of `Fn`
LL |       fn_ref(move || {
   |  _____------_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         || //~ ERROR
   | |         ^^ cannot borrow as mutable
LL | |         *x = 1;});
   | |_________--_____- in this closure
   |           |
   |           mutable borrow occurs due to use of value in closure
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0594, E0596.
For more information about an error, try `rustc --explain E0594`.
For more information about an error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/nll/constant-thread-locals-issue-47053.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to immutable static item `FOO`
+ error[E0594]: cannot assign to immutable static item value
3    |
4 LL |     FOO = 6;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/constant-thread-locals-issue-47053/constant-thread-locals-issue-47053.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/constant-thread-locals-issue-47053.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/constant-thread-locals-issue-47053.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/constant-thread-locals-issue-47053" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/constant-thread-locals-issue-47053/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to immutable static item value
   |
   |
LL |     FOO = 6; //~ ERROR cannot assign to immutable static item `FOO` [E0594]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
For more information about this error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/nll/generator-upvar-mutability.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `x`, as it is not declared as mutable
+ error[E0594]: cannot assign to value, as it is not declared as mutable
3    |
4 LL |     let x = 0;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-upvar-mutability/generator-upvar-mutability.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/generator-upvar-mutability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/generator-upvar-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-upvar-mutability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/generator-upvar-mutability/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is not declared as mutable
   |
LL |     let x = 0;
LL |     let x = 0;
   |         - help: consider changing this to be mutable: `mut x`
LL |     move || {
LL |         x = 1;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
For more information about this error, try `rustc --explain E0594`.
------------------------------------------


---- [ui] src/test/ui/nll/issue-52663-span-decl-captured-variable.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `x` is a captured variable in an `Fn` closure
3    |
3    |
4 LL |        let x = (vec![22], vec![44]);

6 LL |        expect_fn(|| drop(x.0));
7    |                  --------^^^-
8    |                  |       |
-    |                  |       move occurs because `x.0` has type `Vec<i32>`, which does not implement the `Copy` trait
+    |                  |       move occurs because value has type `Vec<i32>`, which does not implement the `Copy` trait
10    |                  captured by this `Fn` closure
12 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52663-span-decl-captured-variable/issue-52663-span-decl-captured-variable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52663-span-decl-captured-variable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52663-span-decl-captured-variable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52663-span-decl-captured-variable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52663-span-decl-captured-variable/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of value, as `x` is a captured variable in an `Fn` closure
   |
   |
LL |        let x = (vec![22], vec![44]);
   |            - captured outer variable
LL |        expect_fn(|| drop(x.0));
   |                  --------^^^-
   |                  |       |
   |                  |       move occurs because value has type `Vec<i32>`, which does not implement the `Copy` trait
---
To only update this specific test, also pass `--test-args span/issue-11925.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-11925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-11925" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-11925/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/issue-11925.rs:8:35
   |
   |
LL |         let f = to_fn_once(move|| &x); //~ ERROR cannot return reference to local data `x`
   |                            |      |
   |                            |      returns a reference to data owned by the current function
   |                            function parameter borrowed here

---

---- [ui] src/test/ui/static/static-items-cant-move.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of static item `BAR`
+ error[E0507]: cannot move out of static item value
3    |
4 LL |     test(BAR);


-    |          ^^^ move occurs because `BAR` has type `Foo`, which does not implement the `Copy` trait
+    |          ^^^ move occurs because value has type `Foo`, which does not implement the `Copy` trait
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-items-cant-move/static-items-cant-move.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-items-cant-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-items-cant-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-items-cant-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-items-cant-move/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of static item value
   |
   |
LL |     test(BAR); //~ ERROR cannot move out of static item
   |          ^^^ move occurs because value has type `Foo`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/dont-suggest-ref/move-into-closure.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `x` is a captured variable in an `Fn` closure
3    |
3    |
4 LL |       let x = X(Y);
18 LL | |     });
18 LL | |     });
19    | |_____- captured by this `Fn` closure
20 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
23    |
23    |
24 LL |       let e = Either::One(X(Y));
40 LL | |     });
40 LL | |     });
41    | |_____- captured by this `Fn` closure
42 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
45    |
45    |
46 LL |       let e = Either::One(X(Y));
62 LL | |     });
62 LL | |     });
63    | |_____- captured by this `Fn` closure
64 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
67    |
67    |
68 LL |       let e = Either::One(X(Y));
87 LL | |     });
87 LL | |     });
88    | |_____- captured by this `Fn` closure
89 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
92    |
92    |
93 LL |       let e = Either::One(X(Y));
112 LL | |     });
112 LL | |     });
113    | |_____- captured by this `Fn` closure
114 
- error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `x` is a captured variable in an `Fn` closure
117    |
117    |
118 LL |       let x = X(Y);
134 LL | |     });
134 LL | |     });
135    | |_____- captured by this `Fn` closure
136 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
139    |
139    |
140 LL |       let mut em = Either::One(X(Y));
156 LL | |     });
156 LL | |     });
157    | |_____- captured by this `Fn` closure
158 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
161    |
161    |
162 LL |       let mut em = Either::One(X(Y));
178 LL | |     });
178 LL | |     });
179    | |_____- captured by this `Fn` closure
180 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
183    |
183    |
184 LL |       let mut em = Either::One(X(Y));
203 LL | |     });
203 LL | |     });
204    | |_____- captured by this `Fn` closure
205 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `Fn` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
208    |
208    |
209 LL |       let mut em = Either::One(X(Y));
228 LL | |     });
228 LL | |     });
229    | |_____- captured by this `Fn` closure
230 
- error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `x` is a captured variable in an `FnMut` closure
233    |
233    |
234 LL |       let x = X(Y);
248 LL | |     });
248 LL | |     });
249    | |_____- captured by this `FnMut` closure
250 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `FnMut` closure
253    |
253    |
254 LL |       let e = Either::One(X(Y));
270 LL | |     });
270 LL | |     });
271    | |_____- captured by this `FnMut` closure
272 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `FnMut` closure
275    |
275    |
276 LL |       let e = Either::One(X(Y));
292 LL | |     });
292 LL | |     });
293    | |_____- captured by this `FnMut` closure
294 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `FnMut` closure
297    |
297    |
298 LL |       let e = Either::One(X(Y));
317 LL | |     });
317 LL | |     });
318    | |_____- captured by this `FnMut` closure
319 
- error[E0507]: cannot move out of `e.0`, as `e` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `e` is a captured variable in an `FnMut` closure
322    |
322    |
323 LL |       let e = Either::One(X(Y));
342 LL | |     });
342 LL | |     });
343    | |_____- captured by this `FnMut` closure
344 
- error[E0507]: cannot move out of `x.0`, as `x` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `x` is a captured variable in an `FnMut` closure
347    |
347    |
348 LL |       let x = X(Y);
364 LL | |     });
364 LL | |     });
365    | |_____- captured by this `FnMut` closure
366 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `FnMut` closure
369    |
369    |
370 LL |       let mut em = Either::One(X(Y));
386 LL | |     });
386 LL | |     });
387    | |_____- captured by this `FnMut` closure
388 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `FnMut` closure
391    |
391    |
392 LL |       let mut em = Either::One(X(Y));
408 LL | |     });
408 LL | |     });
409    | |_____- captured by this `FnMut` closure
410 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `FnMut` closure
413    |
413    |
414 LL |       let mut em = Either::One(X(Y));
433 LL | |     });
433 LL | |     });
434    | |_____- captured by this `FnMut` closure
435 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `FnMut` closure
438    |
438    |
439 LL |       let mut em = Either::One(X(Y));
458 LL | |     });
458 LL | |     });
459    | |_____- captured by this `FnMut` closure
460 
- error[E0507]: cannot move out of `em.0`, as `em` is a captured variable in an `FnMut` closure
+ error[E0507]: cannot move out of value, as `em` is a captured variable in an `FnMut` closure
463    |
463    |
464 LL |       let mut em = Either::One(X(Y));

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/move-into-closure/move-into-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/dont-suggest-ref/move-into-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-suggest-ref/move-into-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/move-into-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-ref/move-into-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of value, as `x` is a captured variable in an `Fn` closure
   |
   |
LL |       let x = X(Y);
   |           - captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
   | |               |
   | |               data moved here
   | |               data moved here
   | |               move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
   |
   |
LL |       let e = Either::One(X(Y));
   |           - captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
LL | |         //~| SUGGESTION &x
LL | |         if let Either::One(_t) = e { }
   | |                            --    ^ help: consider borrowing here: `&e`
   | |                            data moved here
   | |                            data moved here
   | |                            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
   |
   |
LL |       let e = Either::One(X(Y));
   |           - captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
...  |
LL | |         while let Either::One(_t) = e { }
   | |                               --    ^ help: consider borrowing here: `&e`
   | |                               data moved here
   | |                               data moved here
   | |                               move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
   |
   |
LL |       let e = Either::One(X(Y));
   |           - captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
LL | |         match e {
   | |               ^ help: consider borrowing here: `&e`
...  |
...  |
LL | |             Either::One(_t)
   | |                         |
   | |                         data moved here
   | |                         data moved here
   | |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `e` is a captured variable in an `Fn` closure
   |
   |
LL |       let e = Either::One(X(Y));
   |           - captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
LL | |         match e {
   | |               ^ help: consider borrowing here: `&e`
...  |
...  |
LL | |             Either::One(_t) => (),
   | |                         |
   | |                         data moved here
   | |                         data moved here
   | |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `x` is a captured variable in an `Fn` closure
   |
   |
LL |       let x = X(Y);
   |           - captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
...  |
LL | |         let X(mut _t) = x;
   | |               |
   | |               data moved here
   | |               data moved here
   | |               move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
   |
   |
LL |       let mut em = Either::One(X(Y));
   |           ------ captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
...  |
LL | |         if let Either::One(mut _t) = em { }
   | |                            ------    ^^ help: consider borrowing here: `&em`
   | |                            data moved here
   | |                            data moved here
   | |                            move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
   |
   |
LL |       let mut em = Either::One(X(Y));
   |           ------ captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
...  |
LL | |         while let Either::One(mut _t) = em { }
   | |                               ------    ^^ help: consider borrowing here: `&em`
   | |                               data moved here
   | |                               data moved here
   | |                               move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
   |
   |
LL |       let mut em = Either::One(X(Y));
   |           ------ captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
LL | |         match em {
   | |               ^^ help: consider borrowing here: `&em`
...  |
...  |
LL | |             Either::One(mut _t)
   | |                         |
   | |                         data moved here
   | |                         data moved here
   | |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `em` is a captured variable in an `Fn` closure
   |
   |
LL |       let mut em = Either::One(X(Y));
   |           ------ captured outer variable
...
LL |       consume_fn(|| {
   |  ________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
LL | |         match em {
   | |               ^^ help: consider borrowing here: `&em`
...  |
...  |
LL | |             Either::One(mut _t) => (),
   | |                         |
   | |                         data moved here
   | |                         data moved here
   | |                         move occurs because `_t` has type `X`, which does not implement the `Copy` trait
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `Fn` closure

error[E0507]: cannot move out of value, as `x` is a captured variable in an `FnMut` closure
   |
   |
LL |       let x = X(Y);
   |           - captured outer variable
...
LL |       consume_fnmut(|| {
   |  ___________________-
LL | |         let X(_t) = x;
   | |               |
   | |               data moved here
   | |               data moved here
   | |               move occurs because `_t` has type `Y`, which does not implement the `Copy` trait
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
LL | |         }
LL | |     });
LL | |     });
   | |_____- captured by this `FnMut` closure

error[E0507]: cannot move out of value, as `e` is a captured variable in an `FnMut` closure
   |
   |
LL |       let e = Either::One(X(Y));
   |           - captured outer variable
...
LL |       consume_fnmut(|| {
   |  ___________________-
LL | |         let X(_t) = x;
LL | |         //~^ ERROR cannot move
LL | |         //~| HELP consider borrowing here
---

---- [ui] src/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure.rs stdout ----
diff of stderr:

- error[E0594]: cannot assign to `counter`, as it is a captured variable in a `Fn` closure
+ error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
3    |
3    |
4 LL |   fn call<F>(f: F) where F : Fn() {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure/unboxed-closures-mutated-upvar-from-fn-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-mutated-upvar-from-fn-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0594]: cannot assign to value, as it is a captured variable in a `Fn` closure
   |
   |
LL |   fn call<F>(f: F) where F : Fn() {
   |                 - change this to accept `FnMut` instead of `Fn`
LL |       call(|| {
   |  _____----_-
   | |     |
   | |     |
   | |     expects `Fn` instead of `FnMut`
LL | |         counter += 1;
   | |         ^^^^^^^^^^^^ cannot assign
LL | |         //~^ ERROR cannot assign to `counter`
LL | |     });
   | |_____- in this closure
error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
------------------------------------------
