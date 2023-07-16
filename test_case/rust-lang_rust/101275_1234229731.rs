plain
running 13445 tests
........................................................................................ 88/13445
..........................................................................iiiiiiiiiiiiii 176/13445
.....................i.................i................................................ 264/13445
..................................................F..................................... 352/13445
......................................................F...........................F..... 440/13445
..............F......F..F............................................................... 528/13445
........................................................................................ 704/13445
................................F..............................i........................ 792/13445
......................................................i................................. 880/13445
........................................................................................ 968/13445
........................................................................................ 968/13445
........................................................................................ 1056/13445
........................................................................................ 1144/13445
...............F...........................................................F............ 1232/13445
........................................................................................ 1408/13445
.................i...................................................................... 1496/13445
........................................................................................ 1584/13445
........................................................................................ 1672/13445
---
........................................................................................ 2200/13445
........................................................................................ 2288/13445
........................................................................................ 2376/13445
........................................................................................ 2464/13445
................................F..................................F..................F. 2552/13445
..................................FF.F..........F...............F....................... 2640/13445
.....................F.FF.F..F.....................F..................FF................ 2728/13445
...............................................................F........................ 2816/13445
................................................F....................................... 2904/13445
..........................................F................................FF..F........ 2992/13445
..F...................i.....................................................i........... 3080/13445
........................................................................................ 3256/13445
....................................................iiiii............................... 3344/13445
........................................................................................ 3432/13445
........................................................................................ 3520/13445
---
........................................................................................ 4400/13445
........................................................................................ 4488/13445
........................................................................................ 4576/13445
........................................................................................ 4664/13445
.........................F.F............................................................ 4752/13445
......................................................F................................. 4928/13445
........................................................................................ 5016/13445
........................................................................................ 5104/13445
...........................................................................i............ 5192/13445
---
........................................................................................ 6688/13445
................i.......................................................ii.ii........i.. 6776/13445
..i...............................................................i..................... 6864/13445
........................................................................................ 6952/13445
...............F....FF..F.FF..........i....i.........................................i.. 7040/13445
i....................................................................................... 7216/13445
...................i.................................................................... 7304/13445
........................................................................................ 7392/13445
.........................................F.............................................. 7480/13445
.........................................F.............................................. 7480/13445
................ii.......................................ii............................. 7568/13445
...................................i.................................................... 7656/13445
........................................................................................ 7744/13445
...........................................ii....................................F..F... 7832/13445
........................................................................................ 8008/13445
..........................................................ii................i....i..ii.. 8096/13445
........................................................................................ 8184/13445
........................................................................................ 8184/13445
.............................................................................F.......F.F 8272/13445
.......................................................................F..F............. 8360/13445
...............................F...F........F....F.............F........F.F.F........... 8448/13445
........................F.....F...F......F..F........................................... 8536/13445
..FF..FF......i..ii..............................................................ii..... 8624/13445
.......................................F......F........................................i 8712/13445
.........................................i........................................i..... 8888/13445
..............................................................i......................... 8976/13445
........................................................................................ 9064/13445
..........................................................i............................. 9152/13445
---
........................................................................................ 9856/13445
............................................................................ii.......... 9944/13445
.....i.................................................................................. 10032/13445
........................................................................................ 10120/13445
.F.........................F.....F..F...F..........F....F........F..FFFFF.............F. 10208/13445
.......FF...F................FF..F....................................FF..F.....F.F..... 10296/13445
.............F..F..FF...F............................................................... 10384/13445
........................................................................................ 10560/13445
........................................................................................ 10648/13445
...............iiiii...i....i.i......................................................... 10736/13445
........................................................................i............... 10824/13445
........................................................................i............... 10824/13445
..................................................................................iiiiii 10912/13445
..i..iiiiiii............................................................................ 11000/13445
........................................................................................ 11088/13445
........................................................................................ 11176/13445
........................................................................................ 11264/13445
................................................................................F....... 11352/13445
.............F.....F.F.................................................................. 11440/13445
........................................................................................ 11616/13445
........................................................................................ 11704/13445
........................................................................................ 11792/13445
............................................................i.......i........i.....i.... 11880/13445
---
........................................................................................ 13024/13445
........................................................................................ 13112/13445
........................................................................................ 13200/13445
........................................................................................ 13288/13445
.......................FFFFFF.F..F.FF.............iii.FF.F.............................. 13376/13445
.........F...........................................................

---- [ui] src/test/ui/associated-type-bounds/implied-region-constraints.rs stdout ----
diff of stderr:


2   --> $DIR/implied-region-constraints.rs:17:56
3    |
4 LL | fn _bad_st<'a, 'b, T>(x: St<'a, 'b, T>)
-    |            --  -- lifetime `'b` defined here
+    |            --  -- lifetime `'a` defined here
6    |            |
7    |            lifetime `'a` defined here

15   --> $DIR/implied-region-constraints.rs:38:64
16    |
16    |
17 LL | fn _bad_en7<'a, 'b, T>(x: En7<'a, 'b, T>)
-    |             --  -- lifetime `'b` defined here
+    |             --  -- lifetime `'a` defined here
19    |             |
20    |             lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints/implied-region-constraints.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints/implied-region-constraints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/implied-region-constraints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/implied-region-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/implied-region-constraints/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn _bad_st<'a, 'b, T>(x: St<'a, 'b, T>)
   |            --  -- lifetime `'a` defined here
   |            |
   |            lifetime `'a` defined here
...
LL |     let _failure_proves_not_implied_outlives_region_b: &'b T = &x.f0;
   |                                                        ^^^^^ type annotation requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-type-bounds/implied-region-constraints.rs:38:64
   |
   |
LL | fn _bad_en7<'a, 'b, T>(x: En7<'a, 'b, T>)
   |             --  -- lifetime `'a` defined here
   |             |
   |             lifetime `'a` defined here
...
LL |             let _failure_proves_not_implied_outlives_region_b: &'b T = &x;
   |                                                                ^^^^^ type annotation requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs stdout ----
diff of stderr:

2   --> $DIR/associated-types-project-from-hrtb-in-fn-body.rs:22:29
3    |
4 LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
-    |        --  -- lifetime `'b` defined here
+    |        --  -- lifetime `'a` defined here
6    |        |
7    |        lifetime `'a` defined here


17 LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
18    |        --  -- lifetime `'b` defined here
19    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
21 ...
22 LL |     let z: I::A = if cond { x } else { y };
23    |                                        ^ assignment requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/associated-types-project-from-hrtb-in-fn-body.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-project-from-hrtb-in-fn-body.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
   |        --  -- lifetime `'a` defined here
   |        |
   |        lifetime `'a` defined here
...
LL |     let z: I::A = if cond { x } else { y };
   |                             ^ assignment requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs:22:40
   |
   |
LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'b` defined here
...
LL |     let z: I::A = if cond { x } else { y };
   |                                        ^ assignment requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/associated-types-subtyping-1.rs stdout ----
diff of stderr:

4 LL | fn method2<'a,'b,T>(x: &'a T, y: &'b T)
5    |            -- -- lifetime `'b` defined here
6    |            |
-    |            lifetime `'a` defined here
+    |            lifetime `'b` defined here
8 ...
9 LL |     let a: <T as Trait<'a>>::Type = make_any();
10    |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

17 LL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)
18    |            -- -- lifetime `'b` defined here
19    |            |
-    |            lifetime `'a` defined here
+    |            lifetime `'b` defined here
21 ...
22 LL |     let _c: <T as Trait<'a>>::Type = b;
23    |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/associated-types-subtyping-1.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-subtyping-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn method2<'a,'b,T>(x: &'a T, y: &'b T)
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'b` defined here
...
LL |     let a: <T as Trait<'a>>::Type = make_any();
   |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs:36:13
   |
   |
LL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'b` defined here
...
LL |     let _c: <T as Trait<'a>>::Type = b;
   |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs#krisskross stdout ----
diff of stderr:

2   --> $DIR/project-fn-ret-contravariant.rs:46:4
3    |
4 LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
-    |              -- -- lifetime `'b` defined here
+    |              -- -- lifetime `'a` defined here
6    |              |
7    |              lifetime `'a` defined here


17 LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
18    |              -- -- lifetime `'b` defined here
19    |              |
-    |              lifetime `'a` defined here
+    |              lifetime `'b` defined here
22 LL |    (a, b)
22 LL |    (a, b)
23    |    ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`

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
   |              -- -- lifetime `'a` defined here
   |              |
   |              lifetime `'a` defined here
...
LL |    (a, b) //[krisskross]~ ERROR lifetime may not live long enough
   |    ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs:46:4
   |
   |
LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |              -- -- lifetime `'b` defined here
   |              |
   |              lifetime `'b` defined here
...
LL |    (a, b) //[krisskross]~ ERROR lifetime may not live long enough
   |    ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant.rs#krisskross stdout ----
diff of stderr:

2   --> $DIR/project-fn-ret-invariant.rs:59:5
3    |
4 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |              --  -- lifetime `'b` defined here
+    |              --  -- lifetime `'a` defined here
6    |              |
7    |              lifetime `'a` defined here


20 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
21    |              --  -- lifetime `'b` defined here
22    |              |
-    |              lifetime `'a` defined here
+    |              lifetime `'b` defined here
25 LL |     (a, b)
25 LL |     (a, b)
26    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`

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
   |              --  -- lifetime `'a` defined here
   |              |
   |              lifetime `'a` defined here
LL |     (a, b)
LL |     (a, b)
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
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
   |              lifetime `'b` defined here
LL |     (a, b)
LL |     (a, b)
   |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant.rs#oneuse stdout ----
diff of stderr:

2   --> $DIR/project-fn-ret-invariant.rs:40:13
3    |
4 LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |        --  -- lifetime `'b` defined here
+    |        --  -- lifetime `'a` defined here
6    |        |
7    |        lifetime `'a` defined here
8 LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.

20 LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
21    |        --  -- lifetime `'b` defined here
22    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
24 LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
25 LL |     let a = bar(f, x);
26    |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/project-fn-ret-invariant.oneuse.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`

error in revision `oneuse`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "oneuse" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |        --  -- lifetime `'a` defined here
   |        |
   |        lifetime `'a` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x);
   |             ^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:40:13
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'b` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x);
   |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a function pointer to `foo`
   = note: the function `foo` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs stdout ----
diff of stderr:

4 LL |   async fn async_ret_impl_trait3<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {
5    |  ________________________________--__--_______________________________________________^
6    | |                                |   |
-    | |                                |   lifetime `'b` defined here
+    | |                                |   lifetime `'a` defined here
8    | |                                lifetime `'a` defined here
9 LL | |
10 LL | |     (a, b)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/ret-impl-trait-one.stderr
To update references, rerun the tests and pass the `--bless` flag
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
   | |                                |   lifetime `'a` defined here
   | |                                lifetime `'a` defined here
LL | |     //~^ ERROR lifetime may not live long enough
LL | |     (a, b)
LL | | }
   | |_^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
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
---
diff of stderr:

2   --> $DIR/borrowck-reborrow-from-shorter-lived-andmut.rs:9:5
3    |
4 LL | fn copy_borrowed_ptr<'a,'b>(p: &'a mut S<'b>) -> S<'b> {
-    |                      -- -- lifetime `'b` defined here
+    |                      -- -- lifetime `'a` defined here
6    |                      |
7    |                      lifetime `'a` defined here
8 LL |     S { pointer: &mut *p.pointer }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-reborrow-from-shorter-lived-andmut/borrowck-reborrow-from-shorter-lived-andmut.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-reborrow-from-shorter-lived-andmut.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-reborrow-from-shorter-lived-andmut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-reborrow-from-shorter-lived-andmut" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-reborrow-from-shorter-lived-andmut/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn copy_borrowed_ptr<'a,'b>(p: &'a mut S<'b>) -> S<'b> {
   |                      -- -- lifetime `'a` defined here
   |                      |
   |                      lifetime `'a` defined here
LL |     S { pointer: &mut *p.pointer }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/borrowck/issue-17545.rs stdout ----
diff of stderr:

9 LL | |     ));
10    | |      -- temporary value is freed at the end of this statement
11    | |______|
-    |        argument requires that borrow lasts for `'a`
+    |        argument requires that `` lasts for `'a`
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-17545/issue-17545.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-17545.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-17545.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-17545" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-17545/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-17545.rs:7:10
   |
   |
LL |   pub fn foo<'a, F: Fn(&'a ())>(bar: F) {
   |              -- lifetime `'a` defined here
LL | /     bar.call((
LL | |         &id(()), //~ ERROR temporary value dropped while borrowed
   | |          ^^^^^^ creates a temporary which is freed while still in use
LL | |     ));
   | |______|
   | |______|
   |        argument requires that `` lasts for `'a`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/const-eval-intrinsic-promotion.rs stdout ----
diff of stderr:

2   --> $DIR/const-eval-intrinsic-promotion.rs:5:10
3    |
4 LL |     let x: &'static usize =
-    |            -------------- type annotation requires that borrow lasts for `'static`
+    |            -------------- type annotation requires that `` lasts for `'static`
6 LL |         &std::intrinsics::size_of::<i32>();
7    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
8 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-intrinsic-promotion/const-eval-intrinsic-promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-intrinsic-promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-intrinsic-promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-intrinsic-promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-intrinsic-promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-intrinsic-promotion.rs:5:10
   |
LL |     let x: &'static usize =
LL |     let x: &'static usize =
   |            -------------- type annotation requires that `` lasts for `'static`
LL |         &std::intrinsics::size_of::<i32>(); //~ ERROR temporary value dropped while borrowed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs stdout ----
diff of stderr:

12 LL |     let _: &'static u32 = &foo();
13    |            ------------    ^^^^^ creates a temporary which is freed while still in use
14    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
16 LL | }
17    | - temporary value is freed at the end of this statement


22 LL |     let _: &'static u32 = &meh();
23    |            ------------    ^^^^^ creates a temporary which is freed while still in use
24    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
27 LL | }
28    | - temporary value is freed at the end of this statement


33 LL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();
34    |            ----------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
35    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
38 LL | }
39    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn/dont_promote_unstable_const_fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/dont_promote_unstable_const_fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn/auxiliary"
stdout: none
--- stderr -------------------------------
error: `foo` is not yet stable as a const fn
   |
   |
LL | const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn
   |
   |
   = help: add `#![feature(foo)]` to the crate attributes to enable
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:17:28
   |
   |
LL |     let _: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:21:28
   |
   |
LL |     let _: &'static u32 = &meh(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:22:26
   |
LL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();
   |            ----------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs stdout ----
diff of stderr:

4 LL |     let _: &'static u32 = &foo();
5    |            ------------    ^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
8 LL |     let _x: &'static u32 = &foo();
9 LL | }
10    | - temporary value is freed at the end of this statement

15 LL |     let _x: &'static u32 = &foo();
16    |             ------------    ^^^^^ creates a temporary which is freed while still in use
17    |             |
-    |             type annotation requires that borrow lasts for `'static`
+    |             type annotation requires that `` lasts for `'static`
19 LL | }
20    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate/dont_promote_unstable_const_fn_cross_crate.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate/dont_promote_unstable_const_fn_cross_crate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs:8:28
   |
   |
LL |     let _: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     let _x: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs:9:29
   |
   |
LL |     let _x: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^ creates a temporary which is freed while still in use
   |             |
   |             type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/promoted_const_fn_fail.rs stdout ----
diff of stderr:

4 LL |     let x: &'static u8 = &(bar() + 1);
5    |            -----------    ^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail/promoted_const_fn_fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_const_fn_fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail.rs:19:27
   |
   |
LL |     let x: &'static u8 = &(bar() + 1); //~ ERROR temporary value dropped while borrowed
   |            -----------    ^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error: aborting due to previous error
---

---- [ui] src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs stdout ----
diff of stderr:

4 LL |     let x: &'static u8 = &(bar() + 1);
5    |            -----------    ^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err/promoted_const_fn_fail_deny_const_err.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_const_fn_fail_deny_const_err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs:20:27
   |
   |
LL |     let x: &'static u8 = &(bar() + 1);
   |            -----------    ^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error: aborting due to previous error
---

---- [ui] src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs stdout ----
diff of stderr:

4 LL |     let x: &'static bool = &(42 as *const i32 == 43 as *const i32);
5    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let y: &'static usize = &(&1 as *const i32 as usize + 1);
16    |            --------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
20 LL | }
21    | - temporary value is freed at the end of this statement


26 LL |     let z: &'static i32 = &(unsafe { *(42 as *const i32) });
27    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
28    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
31 LL | }
32    | - temporary value is freed at the end of this statement


37 LL |     let a: &'static bool = &(main as fn() == main as fn());
38    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
39    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
42 LL | }
43    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_raw_ptr_ops/promoted_raw_ptr_ops.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_raw_ptr_ops.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_raw_ptr_ops" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_raw_ptr_ops/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:2:29
   |
   |
LL |     let x: &'static bool = &(42 as *const i32 == 43 as *const i32);
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:4:30
   |
LL |     let y: &'static usize = &(&1 as *const i32 as usize + 1);
   |            --------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:6:28
   |
LL |     let z: &'static i32 = &(unsafe { *(42 as *const i32) });
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:8:29
   |
LL |     let a: &'static bool = &(main as fn() == main as fn());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/transmute-const-promotion.rs stdout ----
diff of stderr:

4 LL |     let x: &'static u32 = unsafe { &mem::transmute(3.0f32) };
5    |            ------------             ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const-promotion/transmute-const-promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/transmute-const-promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/transmute-const-promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const-promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const-promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/transmute-const-promotion.rs:4:37
   |
   |
LL |     let x: &'static u32 = unsafe { &mem::transmute(3.0f32) };
   |            ------------             ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/union_promotion.rs stdout ----
diff of stderr:

4 LL |       let x: &'static bool = &unsafe {
5    |  ____________-------------____^
6    | |            |
-    | |            type annotation requires that borrow lasts for `'static`
+    | |            type annotation requires that `` lasts for `'static`
8 LL | |         Foo { a: &1 }.b == Foo { a: &2 }.b
9 LL | |     };
10    | |_____^ creates a temporary which is freed while still in use

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union_promotion/union_promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/union_promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union_promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union_promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union_promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/union_promotion.rs:10:29
   |
   |
LL |       let x: &'static bool = &unsafe { //~ temporary value dropped while borrowed
   |  ____________-------------____^
   | |            |
   | |            type annotation requires that `` lasts for `'static`
LL | |         Foo { a: &1 }.b == Foo { a: &2 }.b
LL | |     };
   | |_____^ creates a temporary which is freed while still in use
LL |   }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
For more information about this error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/consts/const-int-conversion.rs stdout ----
diff of stderr:

4 LL |     let x: &'static i32 = &(5_i32.reverse_bits());
5    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let y: &'static i32 = &(i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]));
16    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
20 LL | }
21    | - temporary value is freed at the end of this statement


26 LL |     let z: &'static i32 = &(i32::from_le_bytes([0x12, 0x34, 0x56, 0x78]));
27    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
28    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
31 LL | }
32    | - temporary value is freed at the end of this statement


37 LL |     let a: &'static i32 = &(i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0])));
38    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
39    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
42 LL | }
43    | - temporary value is freed at the end of this statement


48 LL |     let b: &'static [u8] = &(0x12_34_56_78_i32.to_be_bytes());
49    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
50    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
53 LL | }
54    | - temporary value is freed at the end of this statement


59 LL |     let c: &'static [u8] = &(0x12_34_56_78_i32.to_le_bytes());
60    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
61    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
64 LL | }
65    | - temporary value is freed at the end of this statement


70 LL |     let d: &'static [u8] = &(i32::MIN.to_be().to_ne_bytes());
71    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
72    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
75 LL | }
76    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion/const-int-conversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-conversion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-conversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:2:28
   |
   |
LL |     let x: &'static i32 = &(5_i32.reverse_bits());
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:4:28
   |
LL |     let y: &'static i32 = &(i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:6:28
   |
LL |     let z: &'static i32 = &(i32::from_le_bytes([0x12, 0x34, 0x56, 0x78]));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:8:28
   |
LL |     let a: &'static i32 = &(i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0])));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:10:29
   |
LL |     let b: &'static [u8] = &(0x12_34_56_78_i32.to_be_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:12:29
   |
LL |     let c: &'static [u8] = &(0x12_34_56_78_i32.to_le_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:14:29
   |
LL |     let d: &'static [u8] = &(i32::MIN.to_be().to_ne_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |         //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-int-overflowing.rs stdout ----
diff of stderr:

4 LL |     let x: &'static (i32, bool) = &(5_i32.overflowing_add(3));
5    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let y: &'static (i32, bool) = &(5_i32.overflowing_sub(3));
16    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
20 LL | }
21    | - temporary value is freed at the end of this statement


26 LL |     let z: &'static (i32, bool) = &(5_i32.overflowing_mul(3));
27    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
28    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
31 LL | }
32    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/const-int-overflowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-overflowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-overflowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-overflowing.rs:2:36
   |
   |
LL |     let x: &'static (i32, bool) = &(5_i32.overflowing_add(3));
   |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-overflowing.rs:4:36
   |
LL |     let y: &'static (i32, bool) = &(5_i32.overflowing_sub(3));
   |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-overflowing.rs:6:36
   |
LL |     let z: &'static (i32, bool) = &(5_i32.overflowing_mul(3));
   |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-int-rotate.rs stdout ----
diff of stderr:

4 LL |     let x: &'static i32 = &(5_i32.rotate_left(3));
5    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let y: &'static i32 = &(5_i32.rotate_right(3));
16    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
20 LL | }
21    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/const-int-rotate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-rotate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-rotate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-rotate.rs:2:28
   |
   |
LL |     let x: &'static i32 = &(5_i32.rotate_left(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-rotate.rs:4:28
   |
LL |     let y: &'static i32 = &(5_i32.rotate_right(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-int-sign.rs stdout ----
diff of stderr:

4 LL |     let x: &'static bool = &(5_i32.is_negative());
5    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let y: &'static bool = &(5_i32.is_positive());
16    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
---

12    |                              |         | |
13    |                              |         | temporary value is freed at the end of this statement
14    |                              |         creates a temporary which is freed while still in use
-    |                              using this value as a constant requires that borrow lasts for `'static`
+    |                              using this value as a constant requires that `` lasts for `'static`
17 error[E0716]: temporary value dropped while borrowed
18   --> $DIR/mut_ref_in_final.rs:19:42

22    |                              |           | |
22    |                              |           | |
23    |                              |           | temporary value is freed at the end of this statement
24    |                              |           creates a temporary which is freed while still in use
-    |                              using this value as a constant requires that borrow lasts for `'static`
+    |                              using this value as a constant requires that `` lasts for `'static`
27 error[E0716]: temporary value dropped while borrowed
28   --> $DIR/mut_ref_in_final.rs:34:65

32    |                                  |                              |  |
32    |                                  |                              |  |
33    |                                  |                              |  temporary value is freed at the end of this statement
34    |                                  |                              creates a temporary which is freed while still in use
-    |                                  using this value as a constant requires that borrow lasts for `'static`
+    |                                  using this value as a constant requires that `` lasts for `'static`
37 error[E0716]: temporary value dropped while borrowed
38   --> $DIR/mut_ref_in_final.rs:37:67

42    |                                    |                              |  |
42    |                                    |                              |  |
43    |                                    |                              |  temporary value is freed at the end of this statement
44    |                                    |                              creates a temporary which is freed while still in use
-    |                                    using this value as a static requires that borrow lasts for `'static`
+    |                                    using this value as a static requires that `` lasts for `'static`
47 error[E0716]: temporary value dropped while borrowed
48   --> $DIR/mut_ref_in_final.rs:40:71

52    |                                        |                              |  |
52    |                                        |                              |  |
53    |                                        |                              |  temporary value is freed at the end of this statement
54    |                                        |                              creates a temporary which is freed while still in use
-    |                                        using this value as a static requires that borrow lasts for `'static`
+    |                                        using this value as a static requires that `` lasts for `'static`
57 error: aborting due to 6 previous errors
58 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/mut_ref_in_final.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-mut-refs/mut_ref_in_final.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: mutable references are not allowed in the final value of constants
   |
   |
LL | const B: *mut i32 = &mut 4; //~ ERROR mutable references are not allowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:16:40
   |
   |
LL | const B3: Option<&mut i32> = Some(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                              ----------^^-
   |                              |         | temporary value is freed at the end of this statement
   |                              |         creates a temporary which is freed while still in use
   |                              |         creates a temporary which is freed while still in use
   |                              using this value as a constant requires that `` lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:19:42
   |
   |
LL | const B4: Option<&mut i32> = helper(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                              ------------^^-
   |                              |           | temporary value is freed at the end of this statement
   |                              |           creates a temporary which is freed while still in use
   |                              |           creates a temporary which is freed while still in use
   |                              using this value as a constant requires that `` lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:34:65
   |
   |
LL | const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                  -------------------------------^^--
   |                                  |                              |  temporary value is freed at the end of this statement
   |                                  |                              creates a temporary which is freed while still in use
   |                                  |                              creates a temporary which is freed while still in use
   |                                  using this value as a constant requires that `` lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:37:67
   |
   |
LL | static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                    -------------------------------^^--
   |                                    |                              |  temporary value is freed at the end of this statement
   |                                    |                              creates a temporary which is freed while still in use
   |                                    |                              creates a temporary which is freed while still in use
   |                                    using this value as a static requires that `` lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:40:71
   |
   |
LL | static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                        -------------------------------^^--
   |                                        |                              |  temporary value is freed at the end of this statement
   |                                        |                              creates a temporary which is freed while still in use
   |                                        |                              creates a temporary which is freed while still in use
   |                                        using this value as a static requires that `` lasts for `'static`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0716, E0764.
For more information about an error, try `rustc --explain E0716`.
For more information about an error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/consts/const-ptr-unique.rs stdout ----
diff of stderr:

4 LL |     let x: &'static *mut u32 = &(unique.as_ptr());
5    |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique/const-ptr-unique.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-ptr-unique.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-unique.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-ptr-unique.rs:8:33
   |
   |
LL |     let x: &'static *mut u32 = &(unique.as_ptr());
   |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-ptr-nonnull.rs stdout ----
diff of stderr:

4 LL |     let x: &'static NonNull<u32> = &(NonNull::dangling());
5    |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let x: &'static NonNull<u32> = &(non_null.cast());
16    |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
20 LL | }
21    | - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull/const-ptr-nonnull.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-ptr-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-ptr-nonnull.rs:4:37
   |
   |
LL |     let x: &'static NonNull<u32> = &(NonNull::dangling());
   |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-ptr-nonnull.rs:9:37
   |
LL |     let x: &'static NonNull<u32> = &(non_null.cast());
   |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/control-flow/interior-mutability.rs stdout ----
diff of stderr:

4 LL |     let x: &'static _ = &X;
5    |            ----------    ^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let y: &'static _ = &Y;
16    |            ----------    ^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
19 LL |     let z: &'static _ = &Z;
20 LL | }
21    | - temporary value is freed at the end of this statement

26 LL |     let z: &'static _ = &Z;
27    |            ----------    ^ creates a temporary which is freed while still in use
28    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
30 LL | }
31    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/interior-mutability.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/interior-mutability.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/control-flow/interior-mutability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/interior-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:40:26
   |
   |
LL |     let x: &'static _ = &X; //~ ERROR temporary value dropped while borrowed
   |            ----------    ^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:41:26
   |
LL |     let y: &'static _ = &Y; //~ ERROR temporary value dropped while borrowed
   |            ----------    ^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     let z: &'static _ = &Z; //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:42:26
   |
   |
LL |     let z: &'static _ = &Z; //~ ERROR temporary value dropped while borrowed
   |            ----------    ^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
---

6    |                                 |     |        |
7    |                                 |     |        temporary value is freed at the end of this statement
8    |                                 |     creates a temporary which is freed while still in use
-    |                                 using this value as a constant requires that borrow lasts for `'static`
+    |                                 using this value as a constant requires that `` lasts for `'static`
11 error[E0716]: temporary value dropped while borrowed
12   --> $DIR/issue-54224.rs:9:57

16    |                                          |              |        |
16    |                                          |              |        |
17    |                                          |              |        temporary value is freed at the end of this statement
18    |                                          |              creates a temporary which is freed while still in use
-    |                                          using this value as a constant requires that borrow lasts for `'static`
+    |                                          using this value as a constant requires that `` lasts for `'static`
21 error: aborting due to 2 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/issue-54224.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-54224.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-54224.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/issue-54224.rs:1:39
   |
   |
LL | const FOO: Option<&[[u8; 3]]> = Some(&[*b"foo"]); //~ ERROR temporary value dropped while borrowed
   |                                 |     |        |
   |                                 |     |        temporary value is freed at the end of this statement
   |                                 |     creates a temporary which is freed while still in use
   |                                 |     creates a temporary which is freed while still in use
   |                                 using this value as a constant requires that `` lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/issue-54224.rs:9:57
   |
   |
LL | pub const Z: Cow<'static, [ [u8; 3] ]> = Cow::Borrowed(&[*b"ABC"]);
   |                                          |              |        |
   |                                          |              |        temporary value is freed at the end of this statement
   |                                          |              creates a temporary which is freed while still in use
   |                                          |              creates a temporary which is freed while still in use
   |                                          using this value as a constant requires that `` lasts for `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/min_const_fn/promotion.rs stdout ----
diff of stderr:

4 LL |     let x: &'static () = &foo1();
5    |            -----------    ^^^^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let y: &'static i32 = &foo2(42);
16    |            ------------    ^^^^^^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
20 LL | }
21    | - temporary value is freed at the end of this statement


26 LL |     let z: &'static i32 = &foo3();
27    |            ------------    ^^^^^^ creates a temporary which is freed while still in use
28    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
31 LL | }
32    | - temporary value is freed at the end of this statement


37 LL |     let a: &'static Cell<i32> = &foo4();
38    |            ------------------    ^^^^^^ creates a temporary which is freed while still in use
39    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
42 LL | }
43    | - temporary value is freed at the end of this statement


48 LL |     let a: &'static Option<Cell<i32>> = &foo5();
49    |            --------------------------    ^^^^^^ creates a temporary which is freed while still in use
50    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
52 LL |     let a: &'static Option<Cell<i32>> = &foo6();
53 LL | }
54    | - temporary value is freed at the end of this statement

59 LL |     let a: &'static Option<Cell<i32>> = &foo6();
60    |            --------------------------    ^^^^^^ creates a temporary which is freed while still in use
61    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
63 LL | }
64    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion/promotion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion/promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/min_const_fn/promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:11:27
   |
   |
LL |     let x: &'static () = &foo1(); //~ ERROR temporary value dropped while borrowed
   |            -----------    ^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:12:28
   |
LL |     let y: &'static i32 = &foo2(42); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:13:28
   |
LL |     let z: &'static i32 = &foo3(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:14:34
   |
LL |     let a: &'static Cell<i32> = &foo4();  //~ ERROR temporary value dropped while borrowed
   |            ------------------    ^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:15:42
   |
LL |     let a: &'static Option<Cell<i32>> = &foo5(); //~ ERROR temporary value dropped while borrowed
   |            --------------------------    ^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     let a: &'static Option<Cell<i32>> = &foo6(); //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:16:42
   |
   |
LL |     let a: &'static Option<Cell<i32>> = &foo6(); //~ ERROR temporary value dropped while borrowed
   |            --------------------------    ^^^^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/promote_const_let.rs stdout ----
diff of stderr:

15 LL |       let x: &'static u32 = &{
16    |  ____________------------____^
17    | |            |
-    | |            type annotation requires that borrow lasts for `'static`
+    | |            type annotation requires that `` lasts for `'static`
19 LL | |         let y = 42;
20 LL | |         y
21 LL | |     };

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
   |            ------------ type annotation requires that `y` is borrowed for `'static`
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
   | |            type annotation requires that `` lasts for `'static`
LL | |         let y = 42;
LL | |         y
LL | |     };
   | |_____^ creates a temporary which is freed while still in use
LL |   }

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0716.
---

6    |                                        |         |        |
7    |                                        |         |        temporary value is freed at the end of this statement
8    |                                        |         creates a temporary which is freed while still in use
-    |                                        using this value as a static requires that borrow lasts for `'static`
+    |                                        using this value as a static requires that `` lasts for `'static`
11 error[E0716]: temporary value dropped while borrowed
12   --> $DIR/promote-not.rs:11:18


14 LL |     let x = &mut [1,2,3];
15    |                  ^^^^^^^ creates a temporary which is freed while still in use
16 LL |     x
-    |     - using this value as a static requires that borrow lasts for `'static`
+    |     - using this value as a static requires that `` lasts for `'static`
18 LL | };
19    | - temporary value is freed at the end of this statement


24 LL |         let _x: &'static () = &foo();
25    |                 -----------    ^^^^^ creates a temporary which is freed while still in use
26    |                 |
-    |                 type annotation requires that borrow lasts for `'static`
+    |                 type annotation requires that `` lasts for `'static`
29    |     - temporary value is freed at the end of this statement
30 


34 LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x };
35    |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
36    |             |
-    |             type annotation requires that borrow lasts for `'static`
+    |             type annotation requires that `` lasts for `'static`
38 LL | }
39    | - temporary value is freed at the end of this statement


44 LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x };
45    |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
46    |             |
-    |             type annotation requires that borrow lasts for `'static`
+    |             type annotation requires that `` lasts for `'static`
48 LL | };
49    | - temporary value is freed at the end of this statement


54 LL |     let _val: &'static _ = &(Cell::new(1), 2).1;
55    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
56    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
58 LL | };
59    | - temporary value is freed at the end of this statement


64 LL |     let _val: &'static _ = &(Cell::new(1), 2).0;
65    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
66    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
69 LL | }
70    | - temporary value is freed at the end of this statement


75 LL |     let _val: &'static _ = &(Cell::new(1), 2).1;
76    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
77    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
80 LL | }
81    | - temporary value is freed at the end of this statement


86 LL |     let _val: &'static _ = &(1/0);
87    |               ----------    ^^^^^ creates a temporary which is freed while still in use
88    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
91 LL | }
92    | - temporary value is freed at the end of this statement


97 LL |     let _val: &'static _ = &(1/(1-1));
98    |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
99    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
102 LL | }
103    | - temporary value is freed at the end of this statement


108 LL |     let _val: &'static _ = &(1%0);
109    |               ----------    ^^^^^ creates a temporary which is freed while still in use
110    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
113 LL | }
114    | - temporary value is freed at the end of this statement


119 LL |     let _val: &'static _ = &(1%(1-1));
120    |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
121    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
124 LL | }
125    | - temporary value is freed at the end of this statement


130 LL |     let _val: &'static _ = &([1,2,3][4]+1);
131    |               ----------    ^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
132    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
135 LL | }
136    | - temporary value is freed at the end of this statement


141 LL |     let _val: &'static _ = &TEST_DROP;
142    |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
143    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
146 LL | }
147    | - temporary value is freed at the end of this statement


152 LL |     let _val: &'static _ = &&TEST_DROP;
153    |               ----------    ^^^^^^^^^^ creates a temporary which is freed while still in use
154    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
157 LL | }
158    | - temporary value is freed at the end of this statement


163 LL |     let _val: &'static _ = &&TEST_DROP;
164    |               ----------     ^^^^^^^^^ creates a temporary which is freed while still in use
165    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
168 LL | }
169    | - temporary value is freed at the end of this statement


174 LL |     let _val: &'static _ = &(&TEST_DROP,);
175    |               ----------    ^^^^^^^^^^^^^ creates a temporary which is freed while still in use
176    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
179 LL | }
180    | - temporary value is freed at the end of this statement


185 LL |     let _val: &'static _ = &(&TEST_DROP,);
186    |               ----------      ^^^^^^^^^ creates a temporary which is freed while still in use
187    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
190 LL | }
191    | - temporary value is freed at the end of this statement


196 LL |     let _val: &'static _ = &[&TEST_DROP; 1];
197    |               ----------    ^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
198    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
201 LL | }
202    | - temporary value is freed at the end of this statement

208    |               ----------      ^^^^^^^^^    - temporary value is freed at the end of this statement
208    |               ----------      ^^^^^^^^^    - temporary value is freed at the end of this statement
209    |               |               |
210    |               |               creates a temporary which is freed while still in use
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
213 error: aborting due to 20 previous errors
214 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not/promote-not.stderr
To only update this specific test, also pass `--test-args consts/promote-not.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote-not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/promote-not.rs:8:50
   |
   |
LL | static mut TEST1: Option<&mut [i32]> = Some(&mut [1, 2, 3]); //~ ERROR temporary value dropped while borrowed
   |                                        |         |        |
   |                                        |         |        temporary value is freed at the end of this statement
   |                                        |         creates a temporary which is freed while still in use
   |                                        |         creates a temporary which is freed while still in use
   |                                        using this value as a static requires that `` lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:11:18
   |
   |
LL |     let x = &mut [1,2,3]; //~ ERROR temporary value dropped while borrowed
   |                  ^^^^^^^ creates a temporary which is freed while still in use
LL |     x
   |     - using this value as a static requires that `` lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:20:32
   |
   |
LL |         let _x: &'static () = &foo(); //~ ERROR temporary value dropped while borrowed
   |                 -----------    ^^^^^ creates a temporary which is freed while still in use
   |                 |
   |                 type annotation requires that `` lasts for `'static`
   |     - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:28:29
  --> /checkout/src/test/ui/consts/promote-not.rs:28:29
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |             |
   |             type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:33:29
   |
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |             |
   |             type annotation requires that `` lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:39:29
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:46:29
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).0; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:47:29
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:50:29
   |
LL |     let _val: &'static _ = &(1/0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:51:29
   |
LL |     let _val: &'static _ = &(1/(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:52:29
   |
LL |     let _val: &'static _ = &(1%0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:53:29
   |
LL |     let _val: &'static _ = &(1%(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:54:29
   |
LL |     let _val: &'static _ = &([1,2,3][4]+1); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:57:29
   |
LL |     let _val: &'static _ = &TEST_DROP;
   |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:59:29
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------    ^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:59:30
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------     ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:62:29
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------    ^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:62:31
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------      ^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:65:29
   |
LL |     let _val: &'static _ = &[&TEST_DROP; 1];
   |               ----------    ^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
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
   |               type annotation requires that `` lasts for `'static`
error: aborting due to 20 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/promoted-const-drop.rs stdout ----
diff of stderr:

4 LL |     let _: &'static A = &A();
5    |            ----------    ^^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
8 LL |     let _: &'static [A] = &[C];
9 LL | }
10    | - temporary value is freed at the end of this statement

15 LL |     let _: &'static [A] = &[C];
16    |            ------------    ^^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
19 LL | }
20    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop/promoted-const-drop.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop/promoted-const-drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/promoted-const-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promoted-const-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/promoted-const-drop.rs:13:26
   |
   |
LL |     let _: &'static A = &A(); //~ ERROR temporary value dropped while borrowed
   |            ----------    ^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL |     let _: &'static [A] = &[C]; //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promoted-const-drop.rs:14:28
   |
   |
LL |     let _: &'static [A] = &[C]; //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^ creates a temporary which is freed while still in use
   |            |
   |            type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/qualif-union.rs stdout ----
diff of stderr:

4 LL |     let _: &'static _ = &C1;
5    |            ----------    ^^ creates a temporary which is freed while still in use
6    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


15 LL |     let _: &'static _ = &C2;
16    |            ----------    ^^ creates a temporary which is freed while still in use
17    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
20 LL | }
21    | - temporary value is freed at the end of this statement


26 LL |     let _: &'static _ = &C3;
27    |            ----------    ^^ creates a temporary which is freed while still in use
28    |            |
-    |            type annotation requires that borrow lasts for `'static`
+    |            type annotation requires that `` lasts for `'static`
31 LL | }
32    | - temporary value is freed at the end of this statement


37 LL |     let _: &'static _ = &C4;
---

5    |     -----^^^^^^^^^^^^^^^^^^-
6    |     |    |
7    |     |    creates a temporary which is freed while still in use
-    |     argument requires that borrow lasts for `'static`
+    |     argument requires that `` lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/issue-52049.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/issue-52049.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-52049.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52049.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-52049.rs:6:10
   |
   |
LL |     foo(&unpromotable(5u32));
   |     |    |
   |     |    creates a temporary which is freed while still in use
   |     |    creates a temporary which is freed while still in use
   |     argument requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable.rs stdout ----
diff of stderr:

4 LL | fn foo<'a, 'b, 'c>(x: &'a mut Vec<Ref<'b, i32>>, y: Ref<'c, i32>) {
5    |            --  -- lifetime `'c` defined here
6    |            |
-    |            lifetime `'b` defined here
+    |            lifetime `'c` defined here
8 LL |     let z = Ref { data: y.data };
9 LL |     x.push(z);
10    |     ^^^^^^^^^ argument requires that `'c` must outlive `'b`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable/ex2c-push-inference-variable.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex2c-push-inference-variable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b, 'c>(x: &'a mut Vec<Ref<'b, i32>>, y: Ref<'c, i32>) {
   |            --  -- lifetime `'c` defined here
   |            |
   |            lifetime `'c` defined here
LL |     let z = Ref { data: y.data };
LL |     x.push(z);
   |     ^^^^^^^^^ argument requires that `'c` must outlive `'b`
   |
   = help: consider adding the following bound: `'c: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex2d-push-inference-variable-2.rs stdout ----
diff of stderr:

4 LL | fn foo<'a, 'b, 'c>(x: &'a mut Vec<Ref<'b, i32>>, y: Ref<'c, i32>) {
5    |            --  -- lifetime `'c` defined here
6    |            |
-    |            lifetime `'b` defined here
+    |            lifetime `'c` defined here
8 ...
9 LL |     a.push(b);
10    |     ^^^^^^^^^ argument requires that `'c` must outlive `'b`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2d-push-inference-variable-2/ex2d-push-inference-variable-2.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex2d-push-inference-variable-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex2d-push-inference-variable-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2d-push-inference-variable-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2d-push-inference-variable-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b, 'c>(x: &'a mut Vec<Ref<'b, i32>>, y: Ref<'c, i32>) {
   |            --  -- lifetime `'c` defined here
   |            |
   |            lifetime `'c` defined here
...
LL |     a.push(b);
   |     ^^^^^^^^^ argument requires that `'c` must outlive `'b`
   |
   = help: consider adding the following bound: `'c: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex2e-push-inference-variable-3.rs stdout ----
diff of stderr:

4 LL | fn foo<'a, 'b, 'c>(x: &'a mut Vec<Ref<'b, i32>>, y: Ref<'c, i32>) {
5    |            --  -- lifetime `'c` defined here
6    |            |
-    |            lifetime `'b` defined here
+    |            lifetime `'c` defined here
9 LL |     Vec::push(a, b);
9 LL |     Vec::push(a, b);
10    |     ^^^^^^^^^^^^^^^ argument requires that `'c` must outlive `'b`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2e-push-inference-variable-3/ex2e-push-inference-variable-3.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex2e-push-inference-variable-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex2e-push-inference-variable-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2e-push-inference-variable-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2e-push-inference-variable-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b, 'c>(x: &'a mut Vec<Ref<'b, i32>>, y: Ref<'c, i32>) {
   |            --  -- lifetime `'c` defined here
   |            |
   |            lifetime `'c` defined here
LL |     Vec::push(a, b);
LL |     Vec::push(a, b);
   |     ^^^^^^^^^^^^^^^ argument requires that `'c` must outlive `'b`
   |
   = help: consider adding the following bound: `'c: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions.rs stdout ----
diff of stderr:

4 LL | fn foo<'a, 'b>(mut x: Vec<Ref<'a>>, y: Ref<'b>)
5    |        --  -- lifetime `'b` defined here
6    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
8 ...
9 LL |     x.push(y);
10    |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions/ex3-both-anon-regions-both-are-structs-earlybound-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b>(mut x: Vec<Ref<'a>>, y: Ref<'b>)
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'b` defined here
...
LL |     x.push(y);
   |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-latebound-regions.rs stdout ----
diff of stderr:

4 LL | fn foo<'a,'b>(x: &mut Vec<&'a u8>, y: &'b u8) {
5    |        -- -- lifetime `'b` defined here
6    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
8 LL |     x.push(y);
9    |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-latebound-regions/ex3-both-anon-regions-latebound-regions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-latebound-regions/ex3-both-anon-regions-latebound-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-latebound-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-latebound-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-latebound-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-latebound-regions/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a,'b>(x: &mut Vec<&'a u8>, y: &'b u8) {
   |        -- -- lifetime `'b` defined here
   |        |
   |        lifetime `'b` defined here
LL |     x.push(y);
   |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-latebound-regions.rs stdout ----
diff of stderr:

4 LL | fn foo<'a, 'b>(mut x: Vec<Ref<'a>>, y: Ref<'b>) {
5    |        --  -- lifetime `'b` defined here
6    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
8 LL |     x.push(y);
9    |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-latebound-regions/ex3-both-anon-regions-both-are-structs-latebound-regions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-latebound-regions/ex3-both-anon-regions-both-are-structs-latebound-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-latebound-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-latebound-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-latebound-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-latebound-regions/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b>(mut x: Vec<Ref<'a>>, y: Ref<'b>) {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'b` defined here
LL |     x.push(y);
   |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lub-glb/empty-binders-err.rs stdout ----
diff of stderr:

2   --> $DIR/empty-binders-err.rs:20:12
3    |
4 LL | fn covariance<'a, 'b, 'upper>(v: bool)
-    |               --      ------ lifetime `'upper` defined here
+    |               --      ------ lifetime `'a` defined here
6    |               |
7    |               lifetime `'a` defined here

15   --> $DIR/empty-binders-err.rs:20:12
16    |
16    |
17 LL | fn covariance<'a, 'b, 'upper>(v: bool)
-    |                   --  ------ lifetime `'upper` defined here
+    |                   --  ------ lifetime `'b` defined here
19    |                   |
20    |                   lifetime `'b` defined here


35 LL | fn contra_fn<'a, 'b, 'lower>(v: bool)
36    |              --      ------ lifetime `'lower` defined here
37    |              |
-    |              lifetime `'a` defined here
+    |              lifetime `'lower` defined here
39 ...
40 LL |     let _: fn(&'lower ()) = match v {
41    |            ^^^^^^^^^^^^^^ type annotation requires that `'lower` must outlive `'a`

48 LL | fn contra_struct<'a, 'b, 'lower>(v: bool)
49    |                  --      ------ lifetime `'lower` defined here
50    |                  |
-    |                  lifetime `'a` defined here
+    |                  lifetime `'lower` defined here
52 ...
53 LL |     let _: Contra<'lower> = match v {
54    |            ^^^^^^^^^^^^^^ type annotation requires that `'lower` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/empty-binders-err/empty-binders-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lub-glb/empty-binders-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/empty-binders-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/empty-binders-err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/empty-binders-err/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn covariance<'a, 'b, 'upper>(v: bool)
   |               --      ------ lifetime `'a` defined here
   |               |
   |               lifetime `'a` defined here
...
LL |     let _: &'upper () = match v {
   |            ^^^^^^^^^^ type annotation requires that `'a` must outlive `'upper`
   |
   = help: consider adding the following bound: `'a: 'upper`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lub-glb/empty-binders-err.rs:20:12
   |
   |
LL | fn covariance<'a, 'b, 'upper>(v: bool)
   |                   --  ------ lifetime `'b` defined here
   |                   |
   |                   lifetime `'b` defined here
...
LL |     let _: &'upper () = match v {
   |            ^^^^^^^^^^ type annotation requires that `'b` must outlive `'upper`
   |
   = help: consider adding the following bound: `'b: 'upper`
help: the following changes may resolve your lifetime errors
   |
   |
   = help: add bound `'a: 'upper`
   = help: add bound `'b: 'upper`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lub-glb/empty-binders-err.rs:35:12
   |
   |
LL | fn contra_fn<'a, 'b, 'lower>(v: bool)
   |              --      ------ lifetime `'lower` defined here
   |              |
   |              lifetime `'lower` defined here
...
LL |     let _: fn(&'lower ()) = match v {
   |            ^^^^^^^^^^^^^^ type annotation requires that `'lower` must outlive `'a`
   |
   = help: consider adding the following bound: `'lower: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lub-glb/empty-binders-err.rs:48:12
   |
   |
LL | fn contra_struct<'a, 'b, 'lower>(v: bool)
   |                  --      ------ lifetime `'lower` defined here
   |                  |
   |                  lifetime `'lower` defined here
...
LL |     let _: Contra<'lower> = match v {
   |            ^^^^^^^^^^^^^^ type annotation requires that `'lower` must outlive `'a`
   |
   = help: consider adding the following bound: `'lower: 'a`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/match/match-ref-mut-let-invariance.rs stdout ----
diff of stderr:

2   --> $DIR/match-ref-mut-let-invariance.rs:11:9
3    |
4 LL | impl<'b> S<'b> {
-    |      -- lifetime `'b` defined here
+    |      -- lifetime `'a` defined here
6 LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
7    |            -- lifetime `'a` defined here
8 LL |         let ref mut x = self.0;

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
   |      -- lifetime `'a` defined here
LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
   |            -- lifetime `'a` defined here
LL |         let ref mut x = self.0;
LL |         x
   |         ^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to `&i32`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/match/match-ref-mut-invariance.rs stdout ----
diff of stderr:

2   --> $DIR/match-ref-mut-invariance.rs:10:9
3    |
4 LL | impl<'b> S<'b> {
-    |      -- lifetime `'b` defined here
+    |      -- lifetime `'a` defined here
6 LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
7    |            -- lifetime `'a` defined here
8 LL |         match self.0 { ref mut x => x }

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
   |      -- lifetime `'a` defined here
LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
   |            -- lifetime `'a` defined here
LL |         match self.0 { ref mut x => x }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to `&i32`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs stdout ----
diff of stderr:

32   --> $DIR/propagate-approximated-ref.rs:45:9
33    |
34 LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
-    |           --  -- lifetime `'b` defined here
+    |           --  -- lifetime `'a` defined here
36    |           |
37    |           lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/propagate-approximated-ref.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/propagate-approximated-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-ref.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-ref/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:43:47
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |
---
diff of stderr:

2   --> $DIR/where_clauses_in_structs.rs:11:11
3    |
4 LL | fn bar<'a, 'b>(x: Cell<&'a u32>, y: Cell<&'b u32>) {
-    |        --  -- lifetime `'b` defined here
+    |        --  -- lifetime `'a` defined here
6    |        |
7    |        lifetime `'a` defined here
8 LL |     Foo { x, y };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_structs/where_clauses_in_structs.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/where_clauses_in_structs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/where_clauses_in_structs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_structs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/where_clauses_in_structs/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn bar<'a, 'b>(x: Cell<&'a u32>, y: Cell<&'b u32>) {
   |        --  -- lifetime `'a` defined here
   |        |
   |        lifetime `'a` defined here
LL |     Foo { x, y };
   |           ^ this usage requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Cell<&u32>`, which makes the generic argument `&u32` invariant
   = note: the struct `Cell<T>` is invariant over the parameter `T`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/object-lifetime/object-lifetime-default-elision.rs stdout ----
diff of stderr:

2   --> $DIR/object-lifetime-default-elision.rs:71:5
3    |
4 LL | fn load3<'a,'b>(ss: &'a dyn SomeTrait) -> &'b dyn SomeTrait {
-    |          -- -- lifetime `'b` defined here
+    |          -- -- lifetime `'a` defined here
6    |          |
7    |          lifetime `'a` defined here


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
   |          -- -- lifetime `'a` defined here
   |          |
   |          lifetime `'a` defined here
LL |     ss
LL |     ss
   |     ^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/object-lifetime/object-lifetime-default-mybox.rs stdout ----
diff of stderr:

2   --> $DIR/object-lifetime-default-mybox.rs:27:5
3    |
4 LL | fn load1<'a,'b>(a: &'a MyBox<dyn SomeTrait>,
-    |          -- -- lifetime `'b` defined here
+    |          -- -- lifetime `'a` defined here
6    |          |
7    |          lifetime `'a` defined here


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
   |          -- -- lifetime `'a` defined here
   |          |
   |          lifetime `'a` defined here
LL |     a
LL |     a
   |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
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


---- [ui] src/test/ui/regions/issue-28848.rs stdout ----
diff of stderr:

4 LL | pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {
5    |            --  -- lifetime `'b` defined here
6    |            |
-    |            lifetime `'a` defined here
+    |            lifetime `'b` defined here
8 LL |     Foo::<'a, 'b>::xmute(u)
9    |     ^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-28848/issue-28848.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-28848/issue-28848.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/issue-28848.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-28848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-28848" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-28848/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub fn foo<'a, 'b>(u: &'b ()) -> &'a () {
   |            --  -- lifetime `'b` defined here
   |            |
   |            lifetime `'b` defined here
LL |     Foo::<'a, 'b>::xmute(u)
   |     ^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/region-object-lifetime-2.rs stdout ----
diff of stderr:

2   --> $DIR/region-object-lifetime-2.rs:10:5
3    |
4 LL | fn borrowed_receiver_different_lifetimes<'a,'b>(x: &'a dyn Foo) -> &'b () {
-    |                                          -- -- lifetime `'b` defined here
+    |                                          -- -- lifetime `'a` defined here
6    |                                          |
7    |                                          lifetime `'a` defined here
8 LL |     x.borrowed()

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2/region-object-lifetime-2.stderr
To update references, rerun the tests and pass the `--bless` flag
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
   |                                          -- -- lifetime `'a` defined here
   |                                          |
   |                                          lifetime `'a` defined here
LL |     x.borrowed()
   |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/region-object-lifetime-4.rs stdout ----
diff of stderr:

2   --> $DIR/region-object-lifetime-4.rs:12:5
3    |
4 LL | fn borrowed_receiver_related_lifetimes2<'a,'b>(x: &'a (dyn Foo + 'b)) -> &'b () {
-    |                                         -- -- lifetime `'b` defined here
+    |                                         -- -- lifetime `'a` defined here
6    |                                         |
7    |                                         lifetime `'a` defined here
8 LL |     x.borrowed()

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4/region-object-lifetime-4.stderr
To update references, rerun the tests and pass the `--bless` flag
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
   |                                         -- -- lifetime `'a` defined here
   |                                         |
   |                                         lifetime `'a` defined here
LL |     x.borrowed()
   |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/region-object-lifetime-in-coercion.rs stdout ----
diff of stderr:

50   --> $DIR/region-object-lifetime-in-coercion.rs:26:5
51    |
52 LL | fn d<'a,'b>(v: &'a [u8]) -> Box<dyn Foo+'b> {
-    |      -- -- lifetime `'b` defined here
+    |      -- -- lifetime `'a` defined here
54    |      |
55    |      lifetime `'a` defined here
56 LL |     Box::new(v)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion/region-object-lifetime-in-coercion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-object-lifetime-in-coercion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn a(v: &[u8]) -> Box<dyn Foo + 'static> {
   |         - let's call the lifetime of this reference `'1`
LL |     let x: Box<dyn Foo + 'static> = Box::new(v);
   |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'1` must outlive `'static`
   |
help: consider changing the trait object's explicit `'static` bound to the lifetime of argument `v`
   |
LL | fn a(v: &[u8]) -> Box<dyn Foo + '_> {
   |                                 ~~
help: alternatively, add an explicit `'static` bound to this reference
   |
LL | fn a(v: &'static [u8]) -> Box<dyn Foo + 'static> {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:14:5
   |
   |
LL | fn b(v: &[u8]) -> Box<dyn Foo + 'static> {
   |         - let's call the lifetime of this reference `'1`
LL |     Box::new(v)
   |     ^^^^^^^^^^^ returning this value requires that `'1` must outlive `'static`
   |
help: consider changing the trait object's explicit `'static` bound to the lifetime of argument `v`
   |
LL | fn b(v: &[u8]) -> Box<dyn Foo + '_> {
   |                                 ~~
help: alternatively, add an explicit `'static` bound to this reference
   |
LL | fn b(v: &'static [u8]) -> Box<dyn Foo + 'static> {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:21:5
   |
   |
LL | fn c(v: &[u8]) -> Box<dyn Foo> {
   |         - let's call the lifetime of this reference `'1`
LL |     Box::new(v)
LL |     Box::new(v)
   |     ^^^^^^^^^^^ returning this value requires that `'1` must outlive `'static`
   |
help: to declare that the trait object captures data from argument `v`, you can add an explicit `'_` lifetime bound
   |
LL | fn c(v: &[u8]) -> Box<dyn Foo + '_> {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:26:5
   |
   |
LL | fn d<'a,'b>(v: &'a [u8]) -> Box<dyn Foo+'b> {
   |      -- -- lifetime `'a` defined here
   |      |
   |      lifetime `'a` defined here
LL |     Box::new(v)
   |     ^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs stdout ----
diff of stderr:

4 LL | fn with_assoc<'a,'b>() {
5    |               -- -- lifetime `'b` defined here
6    |               |
-    |               lifetime `'a` defined here
+    |               lifetime `'b` defined here
8 ...
9 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container/regions-assoc-type-in-supertrait-outlives-container.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-assoc-type-in-supertrait-outlives-container.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-assoc-type-in-supertrait-outlives-container/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn with_assoc<'a,'b>() {
   |               -- -- lifetime `'b` defined here
   |               |
   |               lifetime `'b` defined here
...
LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.rs stdout ----
diff of stderr:

4 LL | fn caller2<'a,'b,F:Foo<'a>>(a: Inv<'a>, b: Inv<'b>, f: F) {
5    |            -- -- lifetime `'b` defined here
6    |            |
-    |            lifetime `'a` defined here
+    |            lifetime `'b` defined here
8 LL |     // Here the value provided for 'y is 'b, and hence 'b:'a does not hold.
9 LL |     f.method(b);
10    |     ^^^^^^^^^^^ argument requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound/regions-bounded-method-type-parameters-trait-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-bounded-method-type-parameters-trait-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn caller2<'a,'b,F:Foo<'a>>(a: Inv<'a>, b: Inv<'b>, f: F) {
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'b` defined here
LL |     // Here the value provided for 'y is 'b, and hence 'b:'a does not hold.
LL |     f.method(b);
   |     ^^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type `Inv<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Inv<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-bounds.rs stdout ----
diff of stderr:

2   --> $DIR/regions-bounds.rs:9:12
3    |
4 LL | fn a_fn1<'a,'b>(e: TupleStruct<'a>) -> TupleStruct<'b> {
-    |          -- -- lifetime `'b` defined here
+    |          -- -- lifetime `'a` defined here
6    |          |
7    |          lifetime `'a` defined here
8 LL |     return e;
14   --> $DIR/regions-bounds.rs:14:12
15    |
15    |
16 LL | fn a_fn3<'a,'b>(e: Struct<'a>) -> Struct<'b> {
-    |          -- -- lifetime `'b` defined here
+    |          -- -- lifetime `'a` defined here
18    |          |
19    |          lifetime `'a` defined here
20 LL |     return e;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounds/regions-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn a_fn1<'a,'b>(e: TupleStruct<'a>) -> TupleStruct<'b> {
   |          -- -- lifetime `'a` defined here
   |          |
   |          lifetime `'a` defined here
LL |     return e;
   |            ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-bounds.rs:14:12
   |
   |
LL | fn a_fn3<'a,'b>(e: Struct<'a>) -> Struct<'b> {
   |          -- -- lifetime `'a` defined here
   |          |
   |          lifetime `'a` defined here
LL |     return e;
   |            ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/regions/regions-close-over-type-parameter-multiple.rs stdout ----
diff of stderr:

2   --> $DIR/regions-close-over-type-parameter-multiple.rs:19:5
3    |
4 LL | fn make_object_bad<'a,'b,'c,A:SomeTrait+'a+'b>(v: A) -> Box<dyn SomeTrait + 'c> {
-    |                    --    -- lifetime `'c` defined here
+    |                    --    -- lifetime `'a` defined here
6    |                    |
7    |                    lifetime `'a` defined here
8 LL |     // A outlives 'a AND 'b...but not 'c.

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-multiple/regions-close-over-type-parameter-multiple.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-close-over-type-parameter-multiple.rs`
---
diff of stderr:

2   --> $DIR/regions-free-region-ordering-incorrect.rs:15:9
3    |
4 LL |   impl<'b, T> Node<'b, T> {
-    |        -- lifetime `'b` defined here
+    |        -- lifetime `'a` defined here
6 LL |       fn get<'a>(&'a self) -> &'b T {
7    |              -- lifetime `'a` defined here
8 LL | /         match self.next {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-incorrect/regions-free-region-ordering-incorrect.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-free-region-ordering-incorrect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-free-region-ordering-incorrect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-incorrect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-incorrect/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |   impl<'b, T> Node<'b, T> {
   |        -- lifetime `'a` defined here
LL |       fn get<'a>(&'a self) -> &'b T {
   |              -- lifetime `'a` defined here
LL | /         match self.next { //~ ERROR lifetime may not live long enough
LL | |             Some(ref next) => next.get(),
LL | |             None => &self.val
LL | |         }
   | |_________^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-infer-contravariance-due-to-decl.rs stdout ----
diff of stderr:

2   --> $DIR/regions-infer-contravariance-due-to-decl.rs:25:12
3    |
4 LL | fn use_<'short,'long>(c: Contravariant<'short>,
-    |         ------ ----- lifetime `'long` defined here
+    |         ------ ----- lifetime `'short` defined here
6    |         |
7    |         lifetime `'short` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-contravariance-due-to-decl/regions-infer-contravariance-due-to-decl.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-contravariance-due-to-decl/regions-infer-contravariance-due-to-decl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-contravariance-due-to-decl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-contravariance-due-to-decl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-contravariance-due-to-decl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-contravariance-due-to-decl/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn use_<'short,'long>(c: Contravariant<'short>,
   |         ------ ----- lifetime `'short` defined here
   |         |
   |         lifetime `'short` defined here
...
LL |     let _: Contravariant<'long> = c;
   |            ^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'short` must outlive `'long`
   |
   = help: consider adding the following bound: `'short: 'long`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-infer-covariance-due-to-decl.rs stdout ----
diff of stderr:

2   --> $DIR/regions-infer-covariance-due-to-decl.rs:22:12
3    |
4 LL | fn use_<'short,'long>(c: Covariant<'long>,
-    |         ------ ----- lifetime `'long` defined here
+    |         ------ ----- lifetime `'short` defined here
6    |         |
7    |         lifetime `'short` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-covariance-due-to-decl/regions-infer-covariance-due-to-decl.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-covariance-due-to-decl/regions-infer-covariance-due-to-decl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-covariance-due-to-decl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-covariance-due-to-decl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-covariance-due-to-decl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-covariance-due-to-decl/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn use_<'short,'long>(c: Covariant<'long>,
   |         ------ ----- lifetime `'short` defined here
   |         |
   |         lifetime `'short` defined here
...
LL |     let _: Covariant<'short> = c;
   |            ^^^^^^^^^^^^^^^^^ type annotation requires that `'short` must outlive `'long`
   |
   = help: consider adding the following bound: `'short: 'long`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-infer-not-param.rs stdout ----
diff of stderr:

2   --> $DIR/regions-infer-not-param.rs:15:54
3    |
4 LL | fn take_direct<'a,'b>(p: Direct<'a>) -> Direct<'b> { p }
-    |                -- -- lifetime `'b` defined here      ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |                -- -- lifetime `'a` defined here      ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
6    |                |
7    |                lifetime `'a` defined here


14 LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
15    |                   -- -- lifetime `'b` defined here            ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
16    |                   |
-    |                   lifetime `'a` defined here
+    |                   lifetime `'b` defined here
18    |
19    = help: consider adding the following bound: `'b: 'a`
20    = note: requirement occurs because of the type `Indirect2<'_>`, which makes the generic argument `'_` invariant
25   --> $DIR/regions-infer-not-param.rs:20:63
26    |
26    |
27 LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
-    |                   -- -- lifetime `'b` defined here            ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
+    |                   -- -- lifetime `'a` defined here            ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
29    |                   |
30    |                   lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-not-param/regions-infer-not-param.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-not-param/regions-infer-not-param.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-infer-not-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-infer-not-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-not-param" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-infer-not-param/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn take_direct<'a,'b>(p: Direct<'a>) -> Direct<'b> { p }
   |                -- -- lifetime `'a` defined here      ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |                |
   |                lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-infer-not-param.rs:20:63
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
   |                   -- -- lifetime `'b` defined here            ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |                   |
   |                   lifetime `'b` defined here
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type `Indirect2<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Indirect2<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-infer-not-param.rs:20:63
   |
   |
LL | fn take_indirect2<'a,'b>(p: Indirect2<'a>) -> Indirect2<'b> { p }
   |                   -- -- lifetime `'a` defined here            ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |                   |
   |                   lifetime `'a` defined here
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Indirect2<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Indirect2<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'b` and `'a` must be the same: replace one with the other
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/regions/regions-outlives-projection-container-wc.rs stdout ----
diff of stderr:

4 LL | fn with_assoc<'a,'b>() {
5    |               -- -- lifetime `'b` defined here
6    |               |
-    |               lifetime `'a` defined here
+    |               lifetime `'b` defined here
8 ...
9 LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc/regions-outlives-projection-container-wc.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-wc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-wc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-wc/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn with_assoc<'a,'b>() {
   |               -- -- lifetime `'b` defined here
   |               |
   |               lifetime `'b` defined here
...
LL |     let _: &'a WithAssoc<TheType<'b>> = loop { };
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-outlives-projection-container-hrtb.rs stdout ----
diff of stderr:

4 LL | fn with_assoc<'a,'b>() {
5    |               -- -- lifetime `'b` defined here
6    |               |
-    |               lifetime `'a` defined here
+    |               lifetime `'b` defined here
8 ...
9 LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
10    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

17 LL | fn with_assoc_sub<'a,'b>() {
18    |                   -- -- lifetime `'b` defined here
19    |                   |
-    |                   lifetime `'a` defined here
+    |                   lifetime `'b` defined here
21 ...
22 LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
23    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb/regions-outlives-projection-container-hrtb.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container-hrtb.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container-hrtb/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn with_assoc<'a,'b>() {
   |               -- -- lifetime `'b` defined here
   |               |
   |               lifetime `'b` defined here
...
LL |     let _: &'a WithHrAssoc<TheType<'b>> = loop { };
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-outlives-projection-container-hrtb.rs:46:12
   |
   |
LL | fn with_assoc_sub<'a,'b>() {
   |                   -- -- lifetime `'b` defined here
   |                   |
   |                   lifetime `'b` defined here
...
LL |     let _: &'a WithHrAssocSub<TheType<'b>> = loop { };
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/regions/regions-outlives-projection-container.rs stdout ----
diff of stderr:

4 LL | fn with_assoc<'a,'b>() {
5    |               -- -- lifetime `'b` defined here
6    |               |
-    |               lifetime `'a` defined here
+    |               lifetime `'b` defined here
8 ...
9 LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
10    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

17 LL | fn without_assoc<'a,'b>() {
18    |                  -- -- lifetime `'b` defined here
19    |                  |
-    |                  lifetime `'a` defined here
+    |                  lifetime `'b` defined here
21 ...
22 LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
23    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`

30 LL | fn call_with_assoc<'a,'b>() {
31    |                    -- -- lifetime `'b` defined here
32    |                    |
-    |                    lifetime `'a` defined here
+    |                    lifetime `'b` defined here
34 ...
35 LL |     call::<&'a WithAssoc<TheType<'b>>>();
36    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`

43 LL | fn call_without_assoc<'a,'b>() {
44    |                       -- -- lifetime `'b` defined here
45    |                       |
-    |                       lifetime `'a` defined here
+    |                       lifetime `'b` defined here
47 ...
48 LL |     call::<&'a WithoutAssoc<TheType<'b>>>();
49    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/regions-outlives-projection-container.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-outlives-projection-container.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-outlives-projection-container.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-outlives-projection-container/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn with_assoc<'a,'b>() {
   |               -- -- lifetime `'b` defined here
   |               |
   |               lifetime `'b` defined here
...
LL |     let _x: &'a WithAssoc<TheType<'b>> = loop { };
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-outlives-projection-container.rs:54:13
   |
   |
LL | fn without_assoc<'a,'b>() {
   |                  -- -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'b` defined here
...
LL |     let _x: &'a WithoutAssoc<TheType<'b>> = loop { };
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-outlives-projection-container.rs:63:5
   |
   |
LL | fn call_with_assoc<'a,'b>() {
   |                    -- -- lifetime `'b` defined here
   |                    |
   |                    lifetime `'b` defined here
...
LL |     call::<&'a WithAssoc<TheType<'b>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-outlives-projection-container.rs:70:5
   |
   |
LL | fn call_without_assoc<'a,'b>() {
   |                       -- -- lifetime `'b` defined here
   |                       |
   |                       lifetime `'b` defined here
...
LL |     call::<&'a WithoutAssoc<TheType<'b>>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/regions/regions-reborrow-from-shorter-mut-ref-mut-ref.rs stdout ----
diff of stderr:

2   --> $DIR/regions-reborrow-from-shorter-mut-ref-mut-ref.rs:4:5
3    |
4 LL | fn copy_borrowed_ptr<'a, 'b, 'c>(p: &'a mut &'b mut &'c mut isize) -> &'b mut isize {
-    |                      --  -- lifetime `'b` defined here
+    |                      --  -- lifetime `'a` defined here
6    |                      |
7    |                      lifetime `'a` defined here
8 LL |     &mut ***p

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-reborrow-from-shorter-mut-ref-mut-ref/regions-reborrow-from-shorter-mut-ref-mut-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-reborrow-from-shorter-mut-ref-mut-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-reborrow-from-shorter-mut-ref-mut-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-reborrow-from-shorter-mut-ref-mut-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-reborrow-from-shorter-mut-ref-mut-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn copy_borrowed_ptr<'a, 'b, 'c>(p: &'a mut &'b mut &'c mut isize) -> &'b mut isize {
   |                      --  -- lifetime `'a` defined here
   |                      |
   |                      lifetime `'a` defined here
LL |     &mut ***p
   |     ^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-reborrow-from-shorter-mut-ref.rs stdout ----
diff of stderr:

2   --> $DIR/regions-reborrow-from-shorter-mut-ref.rs:6:5
3    |
4 LL | fn copy_borrowed_ptr<'a, 'b>(p: &'a mut &'b mut isize) -> &'b mut isize {
-    |                      --  -- lifetime `'b` defined here
+    |                      --  -- lifetime `'a` defined here
6    |                      |
7    |                      lifetime `'a` defined here
8 LL |     &mut **p

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-reborrow-from-shorter-mut-ref/regions-reborrow-from-shorter-mut-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-reborrow-from-shorter-mut-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-reborrow-from-shorter-mut-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-reborrow-from-shorter-mut-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-reborrow-from-shorter-mut-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn copy_borrowed_ptr<'a, 'b>(p: &'a mut &'b mut isize) -> &'b mut isize {
   |                      --  -- lifetime `'a` defined here
   |                      |
   |                      lifetime `'a` defined here
LL |     &mut **p
   |     ^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/regions/regions-trait-object-subtyping.rs stdout ----
diff of stderr:

2   --> $DIR/regions-trait-object-subtyping.rs:15:5
3    |
4 LL | fn foo3<'a,'b>(x: &'a mut dyn Dummy) -> &'b mut dyn Dummy {
-    |         -- -- lifetime `'b` defined here
+    |         -- -- lifetime `'a` defined here
6    |         |
7    |         lifetime `'a` defined here
---

---- [ui] src/test/ui/static/static-region-bound.rs stdout ----
diff of stderr:

4 LL |     let x = &id(3);
5    |              ^^^^^ creates a temporary which is freed while still in use
6 LL |     f(x);
-    |     ---- argument requires that borrow lasts for `'static`
+    |     ---- argument requires that `` lasts for `'static`
8 LL | }
9    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/static-region-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/static-region-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-region-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-region-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/static/static-region-bound.rs:10:14
   |
   |
LL |     let x = &id(3); //~ ERROR temporary value dropped while borrowed
   |              ^^^^^ creates a temporary which is freed while still in use
LL |     f(x);
   |     ---- argument requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
---

7    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
8    |     |                |
9    |     |                creates a temporary which is freed while still in use
-    |     assignment requires that borrow lasts for `'1`
+    |     assignment requires that `` lasts for `'1`
12 error[E0716]: temporary value dropped while borrowed
13   --> $DIR/static-reference-to-fn-2.rs:24:22

18    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
18    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
19    |     |                |
20    |     |                creates a temporary which is freed while still in use
-    |     assignment requires that borrow lasts for `'1`
+    |     assignment requires that `` lasts for `'1`
23 error[E0716]: temporary value dropped while borrowed
24   --> $DIR/static-reference-to-fn-2.rs:30:22

29    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
29    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
30    |     |                |
31    |     |                creates a temporary which is freed while still in use
-    |     assignment requires that borrow lasts for `'1`
+    |     assignment requires that `` lasts for `'1`
34 error[E0515]: cannot return value referencing temporary value
35   --> $DIR/static-reference-to-fn-2.rs:40:5



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-reference-to-fn-2/static-reference-to-fn-2.stderr
To only update this specific test, also pass `--test-args static/static-reference-to-fn-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-reference-to-fn-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-reference-to-fn-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-reference-to-fn-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:18:22
   |
   |
LL | fn state1(self_: &mut StateMachineIter) -> Option<&'static str> {
   |           ----- has type `&mut StateMachineIter<'1>`
LL |     self_.statefn = &id(state2 as StateMachineFunc);
   |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
   |     |                creates a temporary which is freed while still in use
   |     |                creates a temporary which is freed while still in use
   |     assignment requires that `` lasts for `'1`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:24:22
   |
   |
LL | fn state2(self_: &mut StateMachineIter) -> Option<(&'static str)> {
   |           ----- has type `&mut StateMachineIter<'1>`
LL |     self_.statefn = &id(state3 as StateMachineFunc);
   |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
   |     |                creates a temporary which is freed while still in use
   |     |                creates a temporary which is freed while still in use
   |     assignment requires that `` lasts for `'1`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:30:22
   |
   |
LL | fn state3(self_: &mut StateMachineIter) -> Option<(&'static str)> {
   |           ----- has type `&mut StateMachineIter<'1>`
LL |     self_.statefn = &id(finished as StateMachineFunc);
   |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
   |     |                creates a temporary which is freed while still in use
   |     |                creates a temporary which is freed while still in use
   |     assignment requires that `` lasts for `'1`
error[E0515]: cannot return value referencing temporary value
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:40:5
   |
LL | /     StateMachineIter {
LL | /     StateMachineIter {
LL | |     //~^ ERROR cannot return value referencing temporary value
LL | |         statefn: &id(state1 as StateMachineFunc)
LL | |     }
LL | |     }
   | |_____^ returns a value referencing data owned by the current function
   |
   = help: use `.collect()` to allocate the iterator
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0515, E0716.
For more information about an error, try `rustc --explain E0515`.
For more information about an error, try `rustc --explain E0515`.
------------------------------------------


---- [ui] src/test/ui/statics/issue-44373.rs stdout ----
diff of stderr:

4 LL |     let _val: &'static [&'static u32] = &[&FOO];
5    |               -----------------------    ^^^^^^ creates a temporary which is freed while still in use
6    |               |
-    |               type annotation requires that borrow lasts for `'static`
+    |               type annotation requires that `` lasts for `'static`
8 LL | }
9    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-44373/issue-44373.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-44373/issue-44373.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args statics/issue-44373.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/statics/issue-44373.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-44373" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-44373/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/statics/issue-44373.rs:4:42
   |
   |
LL |     let _val: &'static [&'static u32] = &[&FOO]; //~ ERROR temporary value dropped while borrowed
   |               -----------------------    ^^^^^^ creates a temporary which is freed while still in use
   |               |
   |               type annotation requires that `` lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/variance/variance-contravariant-arg-trait-match.rs stdout ----
diff of stderr:

2   --> $DIR/variance-contravariant-arg-trait-match.rs:13:5
3    |
4 LL | fn get_min_from_max<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
6    |                     |
7    |                     lifetime `'min` defined here

15   --> $DIR/variance-contravariant-arg-trait-match.rs:22:5
16    |
16    |
17 LL | fn get_max_from_min<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
19    |                     |
20    |                     lifetime `'min` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-trait-match/variance-contravariant-arg-trait-match.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-trait-match/variance-contravariant-arg-trait-match.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-contravariant-arg-trait-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-contravariant-arg-trait-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-trait-match" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-trait-match/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn get_min_from_max<'min, 'max, G>()
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
...
LL |     impls_get::<G,&'min i32>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'min` must outlive `'max`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-contravariant-arg-trait-match.rs:22:5
   |
   |
LL | fn get_max_from_min<'min, 'max, G>()
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
...
LL |     impls_get::<G,&'max i32>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'min` must outlive `'max`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-cell-is-invariant.rs stdout ----
diff of stderr:

2   --> $DIR/variance-cell-is-invariant.rs:14:12
3    |
4 LL | fn use_<'short,'long>(c: Foo<'short>,
-    |         ------ ----- lifetime `'long` defined here
+    |         ------ ----- lifetime `'short` defined here
6    |         |
7    |         lifetime `'short` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-cell-is-invariant/variance-cell-is-invariant.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-cell-is-invariant/variance-cell-is-invariant.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-cell-is-invariant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-cell-is-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-cell-is-invariant" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-cell-is-invariant/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn use_<'short,'long>(c: Foo<'short>,
   |         ------ ----- lifetime `'short` defined here
   |         |
   |         lifetime `'short` defined here
...
LL |     let _: Foo<'long> = c;
   |            ^^^^^^^^^^ type annotation requires that `'short` must outlive `'long`
   |
   = help: consider adding the following bound: `'short: 'long`
   = note: requirement occurs because of the type `Foo<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Foo<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/variance/variance-covariant-arg-trait-match.rs stdout ----
diff of stderr:

2   --> $DIR/variance-covariant-arg-trait-match.rs:14:5
3    |
4 LL | fn get_min_from_max<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
6    |                     |
7    |                     lifetime `'min` defined here

15   --> $DIR/variance-covariant-arg-trait-match.rs:21:5
16    |
16    |
17 LL | fn get_max_from_min<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
19    |                     |
20    |                     lifetime `'min` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-trait-match/variance-covariant-arg-trait-match.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-trait-match/variance-covariant-arg-trait-match.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-covariant-arg-trait-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-covariant-arg-trait-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-trait-match" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-trait-match/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn get_min_from_max<'min, 'max, G>()
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
...
LL |     impls_get::<G,&'min i32>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'min` must outlive `'max`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-covariant-arg-trait-match.rs:21:5
   |
   |
LL | fn get_max_from_min<'min, 'max, G>()
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
...
LL |     impls_get::<G,&'max i32>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'min` must outlive `'max`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-covariant-arg-object.rs stdout ----
diff of stderr:

2   --> $DIR/variance-covariant-arg-object.rs:15:5
3    |
4 LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
6    |                     |
7    |                     lifetime `'min` defined here

15   --> $DIR/variance-covariant-arg-object.rs:23:5
16    |
16    |
17 LL | fn get_max_from_min<'min, 'max>(v: Box<dyn Get<&'min i32>>)
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
19    |                     |
20    |                     lifetime `'min` defined here


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
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-covariant-arg-object.rs:23:5
   |
   |
LL | fn get_max_from_min<'min, 'max>(v: Box<dyn Get<&'min i32>>)
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-contravariant-arg-object.rs stdout ----
diff of stderr:

2   --> $DIR/variance-contravariant-arg-object.rs:14:5
3    |
4 LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
6    |                     |
7    |                     lifetime `'min` defined here

15   --> $DIR/variance-contravariant-arg-object.rs:23:5
16    |
16    |
17 LL | fn get_max_from_min<'min, 'max>(v: Box<dyn Get<&'min i32>>)
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
19    |                     |
20    |                     lifetime `'min` defined here


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
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-contravariant-arg-object.rs:23:5
   |
   |
LL | fn get_max_from_min<'min, 'max>(v: Box<dyn Get<&'min i32>>)
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
LL |     v
LL |     v
   |     ^ function was supposed to return data with lifetime `'max` but it is returning data with lifetime `'min`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-contravariant-self-trait-match.rs stdout ----
diff of stderr:

2   --> $DIR/variance-contravariant-self-trait-match.rs:13:5
3    |
4 LL | fn get_min_from_max<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
6    |                     |
7    |                     lifetime `'min` defined here

15   --> $DIR/variance-contravariant-self-trait-match.rs:23:5
16    |
16    |
17 LL | fn get_max_from_min<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
19    |                     |
20    |                     lifetime `'min` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-self-trait-match/variance-contravariant-self-trait-match.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-self-trait-match/variance-contravariant-self-trait-match.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-contravariant-self-trait-match.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-contravariant-self-trait-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-self-trait-match" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-self-trait-match/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn get_min_from_max<'min, 'max, G>()
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
...
LL |     impls_get::<&'min G>();
   |     ^^^^^^^^^^^^^^^^^^^^ requires that `'min` must outlive `'max`
   |
   = help: consider adding the following bound: `'min: 'max`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/variance/variance-contravariant-self-trait-match.rs:23:5
   |
   |
LL | fn get_max_from_min<'min, 'max, G>()
   |                     ----  ---- lifetime `'min` defined here
   |                     |
   |                     lifetime `'min` defined here
...
LL |     impls_get::<&'max G>();
   |     ^^^^^^^^^^^^^^^^^^^^ requires that `'min` must outlive `'max`
   |
   = help: consider adding the following bound: `'min: 'max`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/variance/variance-covariant-self-trait-match.rs stdout ----
diff of stderr:

2   --> $DIR/variance-covariant-self-trait-match.rs:14:5
3    |
4 LL | fn get_min_from_max<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
6    |                     |
7    |                     lifetime `'min` defined here

15   --> $DIR/variance-covariant-self-trait-match.rs:21:5
16    |
16    |
17 LL | fn get_max_from_min<'min, 'max, G>()
-    |                     ----  ---- lifetime `'max` defined here
+    |                     ----  ---- lifetime `'min` defined here
