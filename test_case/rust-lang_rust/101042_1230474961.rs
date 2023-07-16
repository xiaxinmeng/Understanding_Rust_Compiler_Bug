plain
running 13442 tests
........................................................................................ 88/13442
..........................................................................iiiiiiiiiiiiii 176/13442
.....................i.................i................................................ 264/13442
.................................................F...................................... 352/13442
.........................................................F...................F.......... 440/13442
...............F.......FF............................................................... 528/13442
........................................................................................ 616/13442
...........................................................F...F........................ 704/13442
................................F..............................i........................ 792/13442
........................................................................................ 968/13442
........................................................................................ 1056/13442
........................................................................................ 1144/13442
...................F.................................................................... 1232/13442
...................F.................................................................... 1232/13442
...........F......................................i.............F....................... 1320/13442
.....................................................F.................................. 1408/13442
........................................................................................ 1584/13442
.......................................................F................................ 1672/13442
...........................................................i......ii.................... 1760/13442
........................................................................................ 1848/13442
---
........................................................................................ 4400/13442
........................................................................................ 4488/13442
........................................................................................ 4576/13442
........................................................................................ 4664/13442
........................F.F............................................................. 4752/13442
.........................................................F.............................. 4928/13442
........................................................................................ 5016/13442
........................................................................................ 5104/13442
.............................................................................i.......... 5192/13442
.............................................................................i.......... 5192/13442
........................................................i............................... 5280/13442
........................................................................................ 5368/13442
........................................................................................ 5456/13442
...F...................................................F................................ 5544/13442
........................................................................................ 5720/13442
........................................................................................ 5808/13442
........................................................................................ 5896/13442
........................................................................................ 5984/13442
---
.........................................i.............................................. 6600/13442
........................................................................................ 6688/13442
..................i.......................................................ii.ii........i 6776/13442
....i...............................................................i................... 6864/13442
.........................................................................F....F......... 6952/13442
..........FFF......FFFFFFFFFFFF..F.FFFF.i...Fi.........................................i 7040/13442
..i..................................................................................... 7216/13442
.....................i.................................................................. 7304/13442
........................................................................................ 7392/13442
..........................................F............................................. 7480/13442
..........................................F............................................. 7480/13442
..................ii......................................ii............................ 7568/13442
.....................................i.................................................. 7656/13442
........................................................................................ 7744/13442
.............................................ii...................................F.F... 7832/13442
........................................................................................ 8008/13442
...........................................................ii................i....i..ii. 8096/13442
........................................................................................ 8184/13442
........................................................................................ 8184/13442
...................................................................F.........FF....F...F 8272/13442
FF....F....................................................................FFF....F..... 8360/13442
............................F......F.....F.F..F...........F.............F.F..F.......... 8448/13442
...........................F...F....F....F....F......................F.................. 8536/13442
..F..F.F.......i..ii..............................................................ii.... 8624/13442
......................................F.......F......................................... 8712/13442
..........................................i........................................i.... 8888/13442
...............................................................i........................ 8976/13442
........................................................................................ 9064/13442
...........................................................i............................ 9152/13442
---
........................................................................................ 9856/13442
...............................................................................ii....... 9944/13442
........i............................................................................... 10032/13442
........................................................................................ 10120/13442
.....F......................F.F.....F....F........F......F.......F...F..FFFF........FF.. 10208/13442
....F.....F.........F.......F...FF.....F................................F.F..F......F..F 10296/13442
.FF.................FFFF..F............................................................. 10384/13442
........................................................................................ 10560/13442
........................................................................................ 10648/13442
..................i.iiii..i....i.i...................................................... 10736/13442
...........................................................................i............ 10824/13442
...........................................................................i............ 10824/13442
.....................................................................................iii 10912/13442
iii.i..iiiiii.i..........................F..F.......................F.F.........F.F.F.FF 11000/13442
FF..F................................................................................... 11088/13442
........................................................................................ 11264/13442
........................................................................................ 11352/13442
........................................................................................ 11440/13442
........................................................................................ 11528/13442
---
.............F.......................................................................... 13024/13442
........................................................................................ 13112/13442
........................................................................................ 13200/13442
........................................................................................ 13288/13442
....................FFFF.FF.FFF.F..............iii.FF.F................................. 13376/13442
...........F......................................................

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



---- [ui] src/test/ui/async-await/issue-74497-lifetime-in-opaque.rs stdout ----
diff of stderr:

5    |                  -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
6    |                  ||
7    |                  |return type of closure `impl Future<Output = ()>` contains a lifetime `'2`
-    |                  has type `&'1 u8`
+    |                  has type `impl Future<Output = ()>`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74497-lifetime-in-opaque/issue-74497-lifetime-in-opaque.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-74497-lifetime-in-opaque.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-74497-lifetime-in-opaque.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74497-lifetime-in-opaque" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-74497-lifetime-in-opaque/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let _ = foo(|x| bar(x)); //~ ERROR lifetime may not live long enough
   |                  -- ^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                  ||
   |                  |return type of closure `impl Future<Output = ()>` contains a lifetime `'2`
   |                  has type `impl Future<Output = ()>`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/issue-76547.rs stdout ----
diff of stderr:

2   --> $DIR/issue-76547.rs:20:13
3    |
4 LL | async fn fut(bufs: &mut [&mut [u8]]) {
-    |                    -     - let's call the lifetime of this reference `'2`
+    |                    -     - let's call the lifetime of this reference `'1`
6    |                    |
7    |                    let's call the lifetime of this reference `'1`
8 LL |     ListFut(bufs).await
17   --> $DIR/issue-76547.rs:34:14
18    |
18    |
19 LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
-    |                     -     - let's call the lifetime of this reference `'2`
+    |                     -     - let's call the lifetime of this reference `'1`
21    |                     |
22    |                     let's call the lifetime of this reference `'1`
23 LL |     ListFut2(bufs).await

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547/issue-76547.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-76547.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-76547.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-76547/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | async fn fut(bufs: &mut [&mut [u8]]) {
   |                    -     - let's call the lifetime of this reference `'1`
   |                    |
   |                    let's call the lifetime of this reference `'1`
LL |     ListFut(bufs).await
   |             ^^^^ this usage requires that `'1` must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | async fn fut<'a>(bufs: &'a mut [&'a mut [u8]]) {
   |             ++++        ++       ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/issue-76547.rs:34:14
   |
   |
LL | async fn fut2(bufs: &mut [&mut [u8]]) -> i32 {
   |                     -     - let's call the lifetime of this reference `'1`
   |                     |
   |                     let's call the lifetime of this reference `'1`
LL |     ListFut2(bufs).await
   |              ^^^^ this usage requires that `'1` must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | async fn fut2<'a>(bufs: &'a mut [&'a mut [u8]]) -> i32 {
   |              ++++        ++       ++
error: aborting due to 2 previous errors
------------------------------------------

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



---- [ui] src/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs stdout ----
diff of stderr:

4 LL |     let _action = move || {
6    |                   |     |
6    |                   |     |
-    |                   |     return type of closure `[closure@$DIR/issue-53432-nested-closure-outlives-borrowed-value.rs:4:9: 4:11]` contains a lifetime `'2`
+    |                   |     return type of closure `[closure@$DIR/issue-53432-nested-closure-outlives-borrowed-value.rs:4:9: 4:11]` contains a lifetime `'1`
8    |                   lifetime `'1` represents this closure's body
9 LL |         || f() // The `nested` closure
10    |         ^^^^^^ returning this value requires that `'1` must outlive `'2`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value/issue-53432-nested-closure-outlives-borrowed-value.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let _action = move || {
   |                   |     |
   |                   |     |
   |                   |     return type of closure `[closure@/checkout/src/test/ui/borrowck/issue-53432-nested-closure-outlives-borrowed-value.rs:4:9: 4:11]` contains a lifetime `'1`
   |                   lifetime `'1` represents this closure's body
LL |         || f() // The `nested` closure
   |         ^^^^^^ returning this value requires that `'1` must outlive `'2`
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
help: consider adding 'move' keyword before the nested closure
   |
LL |         move || f() // The `nested` closure

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs stdout ----
diff of stderr:

24 LL |     move |()| s.chars().map(|c| format!("{}{}", c, s))
25    |     --------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
26    |     |       |
-    |     |       return type of closure `Map<Chars<'_>, [closure@$DIR/issue-95079-missing-move-in-nested-closure.rs:9:29: 9:32]>` contains a lifetime `'2`
+    |     |       return type of closure `Map<Chars<'_>, [closure@$DIR/issue-95079-missing-move-in-nested-closure.rs:9:29: 9:32]>` contains a lifetime `'1`
28    |     lifetime `'1` represents this closure's body
29    |
30    = note: closure implements `Fn`, so references to captured variables can't escape the closure

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure/issue-95079-missing-move-in-nested-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-95079-missing-move-in-nested-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-95079-missing-move-in-nested-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error: captured variable cannot escape `FnMut` closure body
   |
   |
LL | fn foo1(s: &str) -> impl Iterator<Item = String> + '_ {
   |         - variable defined here
LL |     None.into_iter()
LL |         .flat_map(move |()| s.chars().map(|c| format!("{}{}", c, s)))
   |                           - -^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                           | |
   |                           | returns a reference to a captured variable which escapes the closure body
   |                           | variable captured here
   |                           inferred to be a `FnMut` closure
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
help: consider adding 'move' keyword before the nested closure
   |
LL |         .flat_map(move |()| s.chars().map(move |c| format!("{}{}", c, s)))

error: lifetime may not live long enough
  --> /checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs:9:15
   |
   |
LL |     move |()| s.chars().map(|c| format!("{}{}", c, s))
   |     --------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |     |       |
   |     |       return type of closure `Map<Chars<'_>, [closure@/checkout/src/test/ui/borrowck/issue-95079-missing-move-in-nested-closure.rs:9:29: 9:32]>` contains a lifetime `'1`
   |     lifetime `'1` represents this closure's body
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
help: consider adding 'move' keyword before the nested closure
   |
LL |     move |()| s.chars().map(move |c| format!("{}{}", c, s))

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/c-variadic/variadic-ffi-4.rs stdout ----
diff of stderr:

44 LL |     let _ = ap.with_copy(|ap| ap);
45    |                           --- ^^ returning this value requires that `'1` must outlive `'2`
46    |                           | |
-    |                           | return type of closure is VaList<'2, '_>
+    |                           | return type of closure is VaList<'1, '_>
48    |                           has type `VaList<'1, '_>`
50 error: lifetime may not live long enough

51   --> $DIR/variadic-ffi-4.rs:22:5
52    |
52    |
53 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ------- has type `&mut VaListImpl<'1>`
56    |                                               has type `&mut VaListImpl<'1>`
57 LL |     *ap0 = ap1;


67 LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
68    |                                               -------                   ------- has type `VaListImpl<'2>`
-    |                                               has type `&mut VaListImpl<'1>`
+    |                                               has type `VaListImpl<'2>`
71 LL |     *ap0 = ap1;
71 LL |     *ap0 = ap1;
72    |     ^^^^ assignment requires that `'2` must outlive `'1`

79   --> $DIR/variadic-ffi-4.rs:28:5
80    |
80    |
81 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ------- has type `&mut VaListImpl<'1>`
84    |                                               has type `&mut VaListImpl<'1>`
84    |                                               has type `&mut VaListImpl<'1>`
85 LL |     ap0 = &mut ap1;

95 LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
96    |                                               -------                   ------- has type `VaListImpl<'2>`
-    |                                               has type `&mut VaListImpl<'1>`
+    |                                               has type `VaListImpl<'2>`
+    |                                               has type `VaListImpl<'2>`
99 LL |     ap0 = &mut ap1;
100    |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`

121   --> $DIR/variadic-ffi-4.rs:35:12
122    |
122    |
123 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
-    |                                               -------                   ------- has type `VaListImpl<'2>`
+    |                                               -------                   ------- has type `&mut VaListImpl<'1>`
126    |                                               has type `&mut VaListImpl<'1>`
126    |                                               has type `&mut VaListImpl<'1>`
127 LL |     *ap0 = ap1.clone();

137 LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
138    |                                               -------                   ------- has type `VaListImpl<'2>`
-    |                                               has type `&mut VaListImpl<'1>`
+    |                                               has type `VaListImpl<'2>`
+    |                                               has type `VaListImpl<'2>`
141 LL |     *ap0 = ap1.clone();
142    |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`


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
   |                           | return type of closure is VaList<'1, '_>
   |                           has type `VaList<'1, '_>`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `&mut VaListImpl<'1>`
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
   |                                               has type `VaListImpl<'2>`
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
   |                                               -------                   ------- has type `&mut VaListImpl<'1>`
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
   |                                               has type `VaListImpl<'2>`
   |                                               has type `VaListImpl<'2>`
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
   |                                               -------                   ------- has type `&mut VaListImpl<'1>`
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
   |                                               has type `VaListImpl<'2>`
   |                                               has type `VaListImpl<'2>`
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


---- [ui] src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs stdout ----
diff of stderr:

2   --> $DIR/expect-region-supply-region-2.rs:14:30
3    |
4 LL | fn expect_bound_supply_named<'x>() {
-    |                              -- lifetime `'x` defined here
+    |                              -- lifetime `'1` defined here
6 ...
7 LL |     closure_expecting_bound(|x: &'x u32| {
8    |                              ^  - let's call the lifetime of this reference `'1`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2/expect-region-supply-region-2.stderr
To only update this specific test, also pass `--test-args closures/closure-expected-type/expect-region-supply-region-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected-type/expect-region-supply-region-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn expect_bound_supply_named<'x>() {
   |                              -- lifetime `'1` defined here
...
LL |     closure_expecting_bound(|x: &'x u32| {
   |                              ^  - let's call the lifetime of this reference `'1`
   |                              |
   |                              requires that `'1` must outlive `'x`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/closures/closure-expected-type/expect-region-supply-region-2.rs:14:30
   |
   |
LL | fn expect_bound_supply_named<'x>() {
   |                              -- lifetime `'x` defined here
...
LL |     closure_expecting_bound(|x: &'x u32| {
   |                              ^ requires that `'x` must outlive `'static`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/error-codes/E0621-does-not-trigger-for-closures.rs stdout ----
diff of stderr:

4 LL |     invoke(&x, |a, b| if a > b { a } else { b });
5    |                    --                       ^ returning this value requires that `'1` must outlive `'2`
6    |                    ||
-    |                    |return type of closure is &'2 i32
+    |                    |return type of closure is &'1 i32
8    |                    has type `&'1 i32`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures/E0621-does-not-trigger-for-closures.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0621-does-not-trigger-for-closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0621-does-not-trigger-for-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0621-does-not-trigger-for-closures/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     invoke(&x, |a, b| if a > b { a } else { b }); //~ ERROR lifetime may not live long enough
   |                    --                       ^ returning this value requires that `'1` must outlive `'2`
   |                    ||
   |                    |return type of closure is &'1 i32
   |                    has type `&'1 i32`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/fn/implied-bounds-unnorm-associated-type-2.rs stdout ----
diff of stderr:

4 LL | fn g<'a, 'b>() {
5    |      --  -- lifetime `'b` defined here
6    |      |
-    |      lifetime `'a` defined here
+    |      lifetime `'b` defined here
8 LL |     f::<'a, 'b>(());
9    |     ^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2/implied-bounds-unnorm-associated-type-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2/implied-bounds-unnorm-associated-type-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/implied-bounds-unnorm-associated-type-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/implied-bounds-unnorm-associated-type-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/implied-bounds-unnorm-associated-type-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn g<'a, 'b>() {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'b` defined here
LL |     f::<'a, 'b>(());
   |     ^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a function pointer to `f`
   = note: the function `f` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/hr-subtype/hr-subtype.rs#free_inv_x_vs_free_inv_y stdout ----
---
diff of stderr:

17   --> $DIR/propagate-approximated-fail-no-postdom.rs:46:13
18    |
19 LL |         |_outlives1, _outlives2, _outlives3, x, y| {
-    |          ----------              ---------- has type `Cell<&'2 &'_#3r u32>`
+    |          ----------              ---------- has type `Cell<&'_#1r &'1 u32>`
21    |          |
22    |          has type `Cell<&'_#1r &'1 u32>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/propagate-approximated-fail-no-postdom.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-fail-no-postdom.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |         |_outlives1, _outlives2, _outlives3, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&'_#2r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#3r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#5r
   = note: late-bound region is '_#6r


error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:46:13
   |
LL |         |_outlives1, _outlives2, _outlives3, x, y| {
   |          ----------              ---------- has type `Cell<&'_#1r &'1 u32>`
   |          |
   |          has type `Cell<&'_#1r &'1 u32>`
...
LL |             demand_y(x, y, p) //~ ERROR
   |             ^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs:38:1
   |
   |
LL | / fn supply<'a, 'b, 'c>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>, cell_c: Cell<&'c u32>) {
LL | |     establish_relationships(
LL | |         cell_a,
LL | |         cell_b,
LL | |     );
LL | | }
   | |_^
   |
---
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
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 5, kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:42:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs:45:9
   |
LL | fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |           --  -- lifetime `'a` defined here
   |           |
   |           lifetime `'a` defined here
...
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-val.rs stdout ----
diff of stderr:

32   --> $DIR/propagate-approximated-val.rs:38:9
33    |
34 LL | fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
36    |         |
37    |         lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/propagate-approximated-val.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-approximated-val.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-approximated-val/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:36:45
   |
   |
LL |     establish_relationships(cell_a, cell_b, |outlives1, outlives2, x, y| {
   |
   |
   = note: defining type: test::{closure#0} with closure substs [
               i16,
               for<'r, 's> extern "rust-call" fn((std::cell::Cell<&'_#1r &ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#2r u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) u32>, std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r
   = note: late-bound region is '_#4r
   = note: number of external vids: 5
   = note: where '_#1r: '_#2r
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:35:1
   |
   |
LL | / fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(cell_a, cell_b, |outlives1, outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(outlives1, outlives2, x.get())
LL | |         //~^ ERROR lifetime may not live long enough
LL | |     });
LL | | }
   |
   = note: defining type: test

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-approximated-val.rs:38:9
   |
LL | fn test<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |         demand_y(outlives1, outlives2, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs stdout ----
diff of stderr:

18 LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
19    |                                                ---------  - has type `&'_#7r Cell<&'1 u32>`
20    |                                                |
-    |                                                has type `&'_#5r Cell<&'2 &'_#1r u32>`
+    |                                                has type `&'_#7r Cell<&'1 u32>`
22 LL |         // Only works if 'x: 'y:
23 LL |         demand_y(x, y, x.get())
24    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>)),
               (),
   = note: late-bound region is '_#2r
   = note: late-bound region is '_#3r

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:37:9
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
   |                                                ---------  - has type `&'_#7r Cell<&'1 u32>`
   |                                                |
   |                                                has type `&'_#7r Cell<&'1 u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs:34:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
---
diff of stderr:

2   --> $DIR/region-lbr1-does-not-outlive-ebr2.rs:9:5
3    |
4 LL | fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
-    |        --  -- lifetime `'b` defined here
+    |        --  -- lifetime `'a` defined here
6    |        |
7    |        lifetime `'a` defined here
8 LL |     &*x

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr1-does-not-outlive-ebr2/region-lbr1-does-not-outlive-ebr2.stderr
To update references, rerun the tests and pass the `--bless` flag
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
   |        --  -- lifetime `'a` defined here
   |        |
   |        lifetime `'a` defined here
LL |     &*x
   |     ^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs stdout ----
diff of stderr:

16   --> $DIR/propagate-fail-to-approximate-longer-wrong-bounds.rs:41:9
17    |
18 LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
-    |                                                ----------  ---------- has type `&'_#8r Cell<&'2 &'_#2r u32>`
+    |                                                ----------  ---------- has type `&'_#6r Cell<&'1 &'_#1r u32>`
20    |                                                |
21    |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
22 LL |         // Only works if 'x: 'y:

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/propagate-fail-to-approximate-longer-wrong-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
note: no external requirements
   |
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |
   |
   = note: defining type: supply::{closure#0} with closure substs [
               i16,
               for<'r, 's, 't0, 't1, 't2, 't3> extern "rust-call" fn((&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) &'_#1r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 2, kind: BrNamed('t0) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) &'_#2r u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 4, kind: BrNamed('t2) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed('s) }) u32>, &ReLateBound(DebruijnIndex(0), BoundRegion { var: 5, kind: BrNamed('t3) }) std::cell::Cell<&ReLateBound(DebruijnIndex(0), BoundRegion { var: 3, kind: BrNamed('t1) }) u32>)),
               (),
   = note: late-bound region is '_#3r
   = note: late-bound region is '_#4r

error: lifetime may not live long enough
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:41:9
   |
LL |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
   |                                                ----------  ---------- has type `&'_#6r Cell<&'1 &'_#1r u32>`
   |                                                |
   |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
note: no external requirements
  --> /checkout/src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs:38:1
   |
   |
LL | / fn supply<'a, 'b>(cell_a: Cell<&'a u32>, cell_b: Cell<&'b u32>) {
LL | |     establish_relationships(&cell_a, &cell_b, |_outlives1, _outlives2, x, y| {
LL | |         // Only works if 'x: 'y:
LL | |         demand_y(x, y, x.get())
LL | |         //~^ ERROR
LL | |     });
LL | | }
   |
   = note: defining type: supply

error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/return-wrong-bound-region.rs stdout ----
diff of stderr:

17    |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
18    |                 |  |
19    |                 |  has type `&'1 i32`
-    |                 has type `&'2 i32`
+    |                 has type `&'1 i32`
22 note: no external requirements
23   --> $DIR/return-wrong-bound-region.rs:10:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/return-wrong-bound-region/return-wrong-bound-region.stderr
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
   |                 -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |                 |  |
   |                 |  has type `&'1 i32`
   |                 has type `&'1 i32`
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
---
diff of stderr:

2   --> $DIR/issue-52113.rs:30:9
3    |
4 LL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {
-    |                --  -- lifetime `'b` defined here
+    |                --  -- lifetime `'a` defined here
6    |                |
7    |                lifetime `'a` defined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113/issue-52113.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113/issue-52113.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52113.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52113.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52113/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn produce_err<'a, 'b: 'a>(data: &'b mut Vec<&'b u32>, value: &'a u32) -> impl Bazinga + 'b {
   |                --  -- lifetime `'a` defined here
   |                |
   |                lifetime `'a` defined here
...
LL |         data.push(value); //~ ERROR lifetime may not live long enough
   |         ^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52213.rs stdout ----
diff of stderr:

2   --> $DIR/issue-52213.rs:3:20
3    |
4 LL | fn transmute_lifetime<'a, 'b, T>(t: &'a (T,)) -> &'b T {
-    |                       --  -- lifetime `'b` defined here
+    |                       --  -- lifetime `'a` defined here
6    |                       |
7    |                       lifetime `'a` defined here
8 LL |     match (&t,) {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52213/issue-52213.stderr
To update references, rerun the tests and pass the `--bless` flag
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
   |                       --  -- lifetime `'a` defined here
   |                       |
   |                       lifetime `'a` defined here
LL |     match (&t,) {
LL |         ((u,),) => u,
   |                    ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52533-1.rs stdout ----
diff of stderr:

5    |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
6    |            |  |
7    |            |  has type `&Foo<'_, '1, u32>`
-    |            has type `&Foo<'_, '2, u32>`
+    |            has type `&Foo<'_, '1, u32>`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52533-1/issue-52533-1.stderr
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
   |            -  -  ^ closure was supposed to return data with lifetime `'2` but it is returning data with lifetime `'1`
   |            |  |
   |            |  has type `&Foo<'_, '1, u32>`
   |            has type `&Foo<'_, '1, u32>`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-52742.rs stdout ----
diff of stderr:

4 LL |     fn take_bar(&mut self, b: Bar<'_>) {
5    |                 ---------  - has type `Bar<'1>`
-    |                 has type `&mut Foo<'_, '2>`
+    |                 has type `Bar<'1>`
+    |                 has type `Bar<'1>`
8 LL |         self.y = b.z
9    |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/issue-52742.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-52742.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-52742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-52742/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     fn take_bar(&mut self, b: Bar<'_>) {
   |                 ---------  - has type `Bar<'1>`
   |                 has type `Bar<'1>`
   |                 has type `Bar<'1>`
LL |         self.y = b.z
   |         ^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-58053.rs stdout ----
diff of stderr:

4 LL |     let f = |x: &i32| -> &i32 { x };
5    |                 -        -      ^ returning this value requires that `'1` must outlive `'2`
6    |                 |        |
-    |                 |        let's call the lifetime of this reference `'2`
+    |                 |        let's call the lifetime of this reference `'1`
8    |                 let's call the lifetime of this reference `'1`
10 error: lifetime may not live long enough


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053/issue-58053.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-58053.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-58053.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-58053/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     let f = |x: &i32| -> &i32 { x };
   |                 -        -      ^ returning this value requires that `'1` must outlive `'2`
   |                 |        |
   |                 |        let's call the lifetime of this reference `'1`
   |                 let's call the lifetime of this reference `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-58053.rs:8:25
   |
   |
LL |     let g = |x: &i32| { x };
   |                 -   -   ^ returning this value requires that `'1` must outlive `'2`
   |                 |   |
   |                 |   return type of closure is &'2 i32
   |                 let's call the lifetime of this reference `'1`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/issue-67007-escaping-data.rs stdout ----
diff of stderr:

2   --> $DIR/issue-67007-escaping-data.rs:15:21
3    |
4 LL | impl<'tcx> Consumer<'tcx> {
-    |      ---- lifetime `'tcx` defined here
+    |      ---- lifetime `'a` defined here
6 LL |     fn bad_method<'a>(&self, fcx: &FnCtxt<'a, 'tcx>) {
7    |                   -- lifetime `'a` defined here
8 LL |         let other = self.use_fcx(fcx);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-67007-escaping-data/issue-67007-escaping-data.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-67007-escaping-data.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-67007-escaping-data.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-67007-escaping-data" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-67007-escaping-data/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-67007-escaping-data.rs:15:21
   |
LL | impl<'tcx> Consumer<'tcx> {
   |      ---- lifetime `'a` defined here
LL |     fn bad_method<'a>(&self, fcx: &FnCtxt<'a, 'tcx>) {
   |                   -- lifetime `'a` defined here
LL |         let other = self.use_fcx(fcx); //~ ERROR lifetime may not live long enough
   |                     ^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'tcx`
   |
   = help: consider adding the following bound: `'a: 'tcx`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-95272.rs stdout ----
diff of stderr:

2   --> $DIR/issue-95272.rs:10:13
3    |
4 LL | fn test<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>) {
-    |         --  -- lifetime `'b` defined here
+    |         --  -- lifetime `'a` defined here
6    |         |
7    |         lifetime `'a` defined here
8 LL |     let f = check;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-95272/issue-95272.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-95272.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-95272.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-95272" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-95272/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn test<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>) {
   |         --  -- lifetime `'a` defined here
   |         |
   |         lifetime `'a` defined here
LL |     let f = check;
   |             ^^^^^ assignment requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a function pointer to `check`
   = note: the function `check` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-98170.rs stdout ----
diff of stderr:

4 LL | impl MyStruct<'_> {
5    |               -- lifetime `'1` appears in the `impl`'s self type
6 LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
-    |                -- lifetime `'a` defined here
+    |                -- lifetime `'1` defined here
8 LL |         Self { field }
9    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`

12   --> $DIR/issue-98170.rs:7:16
13    |
13    |
14 LL | impl MyStruct<'_> {
-    |               -- lifetime `'1` appears in the `impl`'s self type
+    |               -- lifetime `'a` appears in the `impl`'s self type
16 LL |     pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
17    |                -- lifetime `'a` defined here
18 LL |         Self { field }

24 LL | impl<'a> Trait<'a> for MyStruct<'_> {
25    |      --                         -- lifetime `'1` appears in the `impl`'s self type
26    |      |
-    |      lifetime `'a` defined here
+    |      lifetime `'1` defined here
28 LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
29 LL |         Self { field }
30    |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
33   --> $DIR/issue-98170.rs:19:16
34    |
34    |
35 LL | impl<'a> Trait<'a> for MyStruct<'_> {
-    |      --                         -- lifetime `'1` appears in the `impl`'s self type
+    |      --                         -- lifetime `'a` appears in the `impl`'s self type
37    |      |
38    |      lifetime `'a` defined here
39 LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98170/issue-98170.stderr
To update references, rerun the tests and pass the `--bless` flag
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
   |                -- lifetime `'1` defined here
LL |         Self { field }
   |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:7:16
   |
   |
LL | impl MyStruct<'_> {
   |               -- lifetime `'a` appears in the `impl`'s self type
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
   |      lifetime `'1` defined here
LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
LL |         Self { field }
   |         ^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98170.rs:19:16
   |
   |
LL | impl<'a> Trait<'a> for MyStruct<'_> {
   |      --                         -- lifetime `'a` appears in the `impl`'s self type
   |      |
   |      lifetime `'a` defined here
LL |     fn new(field: &'a [u32]) -> MyStruct<'a> {
LL |         Self { field }
   |                ^^^^^ this usage requires that `'a` must outlive `'1`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/nll/issue-98589-closures-relate-named-regions.rs stdout ----
diff of stderr:

4 LL | fn test_early_early<'a: 'a, 'b: 'b>() {
5    |                     --      -- lifetime `'b` defined here
6    |                     |
-    |                     lifetime `'a` defined here
+    |                     lifetime `'b` defined here
8 LL |     || { None::<&'a &'b ()>; };
9    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


16 LL | fn test_early_late<'a: 'a, 'b>() {
17    |                    --      -- lifetime `'b` defined here
18    |                    |
-    |                    lifetime `'a` defined here
+    |                    lifetime `'b` defined here
20 LL |     || { None::<&'a &'b ()>; };
21    |          ^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


28 LL | fn test_late_late<'a, 'b>() {
29    |                   --  -- lifetime `'b` defined here
30    |                   |
-    |                   lifetime `'a` defined here
+    |                   lifetime `'b` defined here
32 LL |     || { None::<&'a &'b ()>; };
33    |          ^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98589-closures-relate-named-regions/issue-98589-closures-relate-named-regions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98589-closures-relate-named-regions/issue-98589-closures-relate-named-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-98589-closures-relate-named-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-98589-closures-relate-named-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98589-closures-relate-named-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-98589-closures-relate-named-regions/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn test_early_early<'a: 'a, 'b: 'b>() {
   |                     --      -- lifetime `'b` defined here
   |                     |
   |                     lifetime `'b` defined here
LL |     || { None::<&'a &'b ()>; };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98589-closures-relate-named-regions.rs:15:10
   |
   |
LL | fn test_early_late<'a: 'a, 'b>() {
   |                    --      -- lifetime `'b` defined here
   |                    |
   |                    lifetime `'b` defined here
LL |     || { None::<&'a &'b ()>; };
   |          ^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/issue-98589-closures-relate-named-regions.rs:21:10
   |
   |
LL | fn test_late_late<'a, 'b>() {
   |                   --  -- lifetime `'b` defined here
   |                   |
   |                   lifetime `'b` defined here
LL |     || { None::<&'a &'b ()>; };
   |          ^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/issue-98589-closures-relate-named-regions.rs:26:5
   |
   |
LL |     || { None::<&'a T>; };
   |     ^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test_early_type<'a: 'a, T: 'a>() {

error[E0309]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/nll/issue-98589-closures-relate-named-regions.rs:32:5
   |
   |
LL |     || { None::<&'a T>; };
   |     ^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test_late_type<'a, T: 'a>() {

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0309`.
For more information about this error, try `rustc --explain E0309`.
------------------------------------------


---- [ui] src/test/ui/nll/mir_check_cast_closure.rs stdout ----
diff of stderr:

4 LL | fn bar<'a, 'b>() -> fn(&'a u32, &'b u32) -> &'a u32 {
5    |        --  -- lifetime `'b` defined here
6    |        |
-    |        lifetime `'a` defined here
+    |        lifetime `'b` defined here
8 LL |     let g: fn(_, _) -> _ = |_x, y| y;
9 LL |     g
10    |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure/mir_check_cast_closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/mir_check_cast_closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/mir_check_cast_closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/mir_check_cast_closure/auxiliary"
