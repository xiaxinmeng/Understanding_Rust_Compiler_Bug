plain
running 13477 tests
........................................................................................ 88/13477
..........................................................................iiiiiiiiiiiiii 176/13477
.....................i..................i............................................... 264/13477
....................................................F................................... 352/13477
...........................................................F....................F....... 440/13477
...................F....FF.............................................................. 528/13477
........................................................................................ 616/13477
...........................................................FF.FF........................ 704/13477
.......................................................................i................ 880/13477
........................................................................................ 968/13477
........................................................................................ 1056/13477
..............................................................F......................... 1144/13477
..............................................................F......................... 1144/13477
........................................................................................ 1232/13477
........................F......................................i............F........... 1320/13477
.....................................................................F.................. 1408/13477
...................F.................................................................... 1584/13477
....................................................................F................... 1672/13477
..........................................................................i.....ii...... 1760/13477
........................................................................................ 1848/13477
---
..............F..................................i...................................... 4224/13477
............................................................................F........... 4312/13477
........................................................................................ 4400/13477
........................................................................................ 4488/13477
........................................F............................................... 4576/13477
....................................................................FF.................. 4664/13477
..........................................F..F.......................................... 4752/13477
........................................................................................ 4840/13477
............................................................................F.........F. 4928/13477
........................................................................................ 5104/13477
........................................................................................ 5192/13477
........i....................................................................i.......... 5280/13477
.F...................................................................................... 5368/13477
.F...................................................................................... 5368/13477
........................................................................................ 5456/13477
.......................F....F..................................................F........ 5544/13477
........................................................................................ 5720/13477
........................................................................................ 5808/13477
...............................................................................F........ 5896/13477
........................................................................................ 5984/13477
---
.........F....................................................i......................... 6600/13477
.....................................................................................F.. 6688/13477
.......................................i................................................ 6776/13477
........ii.ii........i....i............................................................. 6864/13477
..i...............................................................................F..... 6952/13477
F......F......F.F.......................FFF.FFFFFFFFFFF.FF.FFFiF...i.................... 7040/13477
........................i............................................................... 7216/13477
.............................................i.......................................... 7304/13477
........................................................................................ 7392/13477
.....................................................................F.................. 7480/13477
---
........................................................................................ 8008/13477
.....................................................................................ii. 8096/13477
...............i.......i.ii............................................................. 8184/13477
........................................................................................ 8272/13477
...........F.....FF.FF..F....FF....................F..............F............F......F. 8360/13477
.............F........F............F.......F......F.......FF..F.....F..F.....F....FF.... 8448/13477
...F...F.F..F.F..........................................FF....F...F...F.F..........FFF. 8536/13477
F.FF...FF..F.F.....................FF......i...ii....................................... 8624/13477
......................ii.............................................FFF................ 8712/13477
......................................................................i................. 8888/13477
.......................i................................................................ 8976/13477
...i.................................................................................... 9064/13477
.......................................................................................i 9152/13477
---
........................................................................................ 9856/13477
........................................................................................ 9944/13477
..................ii...............i.................................................... 10032/13477
........................................................................................ 10120/13477
.................................F.............................F...FF..F..F...F.F....... 10208/13477
F....F.....F............FF...F......F.....................FFFFFF..F..................F.. 10296/13477
............FFF........F...FF.....F..............FF.F.FF................................ 10384/13477
........................................................................................ 10560/13477
........................................................................................ 10648/13477
.............................................iiiii...i....i.i........................... 10736/13477
........................................................................................ 10824/13477
---
........................................................................................ 11352/13477
........................................................................................ 11440/13477
........................................................................................ 11528/13477
........................................................................................ 11616/13477
........................................................F............................... 11704/13477
......F..........................F...................................................... 11792/13477
...i........i.......i.....i.....................i....................................... 11968/13477
........................................................................................ 12056/13477
........................................................................................ 12144/13477
........................................................................................ 12232/13477
........................................................................................ 12232/13477
..............................................................................F......... 12320/13477
.......................................................F......F......................... 12408/13477
........................................................................................ 12584/13477
..............................i......................................................... 12672/13477
........................................................................................ 12760/13477
........................................................................................ 12848/13477
........................................................................................ 12848/13477
...................................................................F.........F.......... 12936/13477
............................................F...F....................................... 13024/13477
........................................................................................ 13200/13477
........................................................................................ 13288/13477
........................................................................................ 13288/13477
......................................................FFF.F.FFF.FF................iii... 13376/13477
............................................F........................................... 13464/13477
failures:

---- [ui] src/test/ui/associated-type-bounds/implied-region-constraints.rs stdout ----
diff of stderr:
diff of stderr:

7    |            lifetime `'a` defined here
8 ...
9 LL |     let _failure_proves_not_implied_outlives_region_b: &'b T = &x.f0;
-    |                                                        ^^^^^ type annotation requires that `'a` must outlive `'b`
+    |                                                        ^^^^^ type annotation requires that 'a must outlive `'b`
11    |
12    = help: consider adding the following bound: `'a: 'b`


20    |             lifetime `'a` defined here
21 ...
22 LL |             let _failure_proves_not_implied_outlives_region_b: &'b T = &x;
-    |                                                                ^^^^^ type annotation requires that `'a` must outlive `'b`
+    |                                                                ^^^^^ type annotation requires that 'a must outlive `'b`
24    |
25    = help: consider adding the following bound: `'a: 'b`


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
   |            --  -- lifetime `'b` defined here
   |            |
   |            lifetime `'a` defined here
...
LL |     let _failure_proves_not_implied_outlives_region_b: &'b T = &x.f0;
   |                                                        ^^^^^ type annotation requires that 'a must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-type-bounds/implied-region-constraints.rs:38:64
   |
   |
LL | fn _bad_en7<'a, 'b, T>(x: En7<'a, 'b, T>)
   |             --  -- lifetime `'b` defined here
   |             |
   |             lifetime `'a` defined here
...
LL |             let _failure_proves_not_implied_outlives_region_b: &'b T = &x;
   |                                                                ^^^^^ type annotation requires that 'a must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs stdout ----
diff of stderr:

7    |        lifetime `'a` defined here
8 ...
9 LL |     let z: I::A = if cond { x } else { y };
-    |                             ^ assignment requires that `'a` must outlive `'b`
+    |                             ^ assignment requires that 'a must outlive `'b`
11    |
12    = help: consider adding the following bound: `'a: 'b`


20    |        lifetime `'a` defined here
21 ...
22 LL |     let z: I::A = if cond { x } else { y };
-    |                                        ^ assignment requires that `'b` must outlive `'a`
+    |                                        ^ assignment requires that 'b must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/associated-types-project-from-hrtb-in-fn-body.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body/associated-types-project-from-hrtb-in-fn-body.stderr
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
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
...
LL |     let z: I::A = if cond { x } else { y };
   |                             ^ assignment requires that 'a must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-fn-body.rs:22:40
   |
   |
LL | fn bar<'a, 'b, I : for<'x> Foo<&'x isize>>(
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
...
LL |     let z: I::A = if cond { x } else { y };
   |                                        ^ assignment requires that 'b must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/associated-types-subtyping-1.rs stdout ----
diff of stderr:

7    |            lifetime `'a` defined here
8 ...
9 LL |     let a: <T as Trait<'a>>::Type = make_any();
-    |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
+    |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that 'b must outlive `'a`
11    |
12    = help: consider adding the following bound: `'b: 'a`


20    |            lifetime `'a` defined here
21 ...
22 LL |     let _c: <T as Trait<'a>>::Type = b;
-    |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
+    |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that 'b must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`


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
   |            lifetime `'a` defined here
...
LL |     let a: <T as Trait<'a>>::Type = make_any();
   |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that 'b must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs:36:13
   |
   |
LL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'a` defined here
...
LL |     let _c: <T as Trait<'a>>::Type = b;
   |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that 'b must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs#transmute stdout ----
diff of stderr:

4 LL | fn baz<'a,'b>(x: &'a u32) -> &'static u32 {
5    |        -- lifetime `'a` defined here
6 LL |    bar(foo, x)
-    |    ^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
+    |    ^^^^^^^^^^^ returning this value requires that 'a must outlive `'static`
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.transmute/project-fn-ret-contravariant.transmute.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-contravariant.rs`

error in revision `transmute`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "transmute" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.transmute" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.transmute/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn baz<'a,'b>(x: &'a u32) -> &'static u32 {
   |        -- lifetime `'a` defined here
LL |    bar(foo, x) //[transmute]~ ERROR lifetime may not live long enough
   |    ^^^^^^^^^^^ returning this value requires that 'a must outlive `'static`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant.rs#oneuse stdout ----
diff of stderr:

7    |        lifetime `'a` defined here
8 LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
9 LL |     let a = bar(f, x);
-    |             ^^^^^^^^^ argument requires that `'a` must outlive `'b`
+    |             ^^^^^^^^^ argument requires that 'a must outlive `'b`
11    |
12    = help: consider adding the following bound: `'a: 'b`
13    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant

23    |        lifetime `'a` defined here
24 LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
25 LL |     let a = bar(f, x);
-    |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`
+    |             ^^^^^^^^^ argument requires that 'b must outlive `'a`
27    |
28    = help: consider adding the following bound: `'b: 'a`
29    = note: requirement occurs because of a function pointer to `foo`

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
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x);
   |             ^^^^^^^^^ argument requires that 'a must outlive `'b`
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
   |        lifetime `'a` defined here
LL |     let f = foo; // <-- No consistent type can be inferred for `f` here.
LL |     let a = bar(f, x);
   |             ^^^^^^^^^ argument requires that 'b must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a function pointer to `foo`
   = note: the function `foo` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/cache/project-fn-ret-invariant.rs#transmute stdout ----
diff of stderr:

5    |        -- lifetime `'a` defined here
6 ...
7 LL |     bar(foo, x)
-    |     ^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
+    |     ^^^^^^^^^^^ returning this value requires that 'a must outlive `'static`
9    |
10    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
11    = note: the struct `Type<'a>` is invariant over the parameter `'a`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.transmute/project-fn-ret-invariant.transmute.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`

error in revision `transmute`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "transmute" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.transmute" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.transmute/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>) -> Type<'static> {
   |        -- lifetime `'a` defined here
...
LL |     bar(foo, x) //[transmute]~ ERROR lifetime may not live long enough
   |     ^^^^^^^^^^^ returning this value requires that 'a must outlive `'static`
   |
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/issue-74072-lifetime-name-annotations.rs stdout ----
diff of stderr:

8 LL |     *x += 1;
9    |     ^^^^^^^ assignment to borrowed `*x` occurs here
10 LL |     y
-    |     - returning this value requires that `*x` is borrowed for `'1`
+    |     - returning this value requires that ``*x`` is borrowed for `'1`
12 
13 error[E0506]: cannot assign to `*x` because it is borrowed


18 LL |         *x += 1;
19    |         ^^^^^^^ assignment to borrowed `*x` occurs here
20 LL |         y
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that ``*x`` is borrowed for `'1`
22 LL |     })()
23    |     - return type of async closure is &'1 i32


32 LL |         *x += 1;
33    |         ^^^^^^^ assignment to borrowed `*x` occurs here
34 LL |         y
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that ``*x`` is borrowed for `'1`
36 
37 error[E0506]: cannot assign to `*x` because it is borrowed


42 LL |         *x += 1;
43    |         ^^^^^^^ assignment to borrowed `*x` occurs here
44 LL |         y
-    |         - returning this value requires that `*x` is borrowed for `'1`
+    |         - returning this value requires that ``*x`` is borrowed for `'1`
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
   |     - returning this value requires that ``*x`` is borrowed for `'1`

error[E0506]: cannot assign to `*x` because it is borrowed
   |
   |
LL |         let y = &*x;
   |                 --- borrow of `*x` occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed `*x` occurs here
LL |         y
   |         - returning this value requires that ``*x`` is borrowed for `'1`
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
   |         - returning this value requires that ``*x`` is borrowed for `'1`

error[E0506]: cannot assign to `*x` because it is borrowed
   |
   |
LL |         let y = &*x;
   |                 --- borrow of `*x` occurs here
LL |         *x += 1; //~ ERROR cannot assign to `*x` because it is borrowed
   |         ^^^^^^^ assignment to borrowed `*x` occurs here
LL |         y
   |         - returning this value requires that ``*x`` is borrowed for `'1`
LL |     }
   |     - return type of async block is &'1 i32
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/issue-75785-confusing-named-region.rs stdout ----
diff of stderr:

8 LL |     *x += 1;
9    |     ^^^^^^^ assignment to borrowed `*x` occurs here
10 LL |     (&32, y)
-    |     -------- returning this value requires that `*x` is borrowed for `'1`
+    |     -------- returning this value requires that ``*x`` is borrowed for `'1`
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-75785-confusing-named-region/issue-75785-confusing-named-region.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-75785-confusing-named-region.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-75785-confusing-named-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-75785-confusing-named-region" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-75785-confusing-named-region/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `*x` because it is borrowed
   |
   |
LL | pub async fn async_fn(x: &mut i32) -> (&i32, &i32) {
   |                          - let's call the lifetime of this reference `'1`
LL |     let y = &*x;
   |             --- borrow of `*x` occurs here
LL |     *x += 1; //~ ERROR cannot assign to
   |     ^^^^^^^ assignment to borrowed `*x` occurs here
LL |     (&32, y)
   |     -------- returning this value requires that ``*x`` is borrowed for `'1`
error: aborting due to previous error
---
diff of stderr:

2   --> $DIR/issue-90170-elision-mismatch.rs:3:40
3    |
4 LL | pub fn foo(x: &mut Vec<&u8>, y: &u8) { x.push(y); }
-    |                        -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                        -        -      ^^^^^^^^^ argument requires that '1 must outlive `'2`
6    |                        |        |
7    |                        |        let's call the lifetime of this reference `'1`
8    |                        let's call the lifetime of this reference `'2`
16   --> $DIR/issue-90170-elision-mismatch.rs:5:44
17    |
17    |
18 LL | pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); }
-    |                         -           -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                         -           -      ^^^^^^^^^ argument requires that '1 must outlive `'2`
20    |                         |           |
21    |                         |           let's call the lifetime of this reference `'1`
22    |                         let's call the lifetime of this reference `'2`
30   --> $DIR/issue-90170-elision-mismatch.rs:7:63
31    |
31    |
32 LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&u8>, y: &u8) { x.push(y); }
-    |                                               -        -      ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |                                               -        -      ^^^^^^^^^ argument requires that '1 must outlive `'2`
34    |                                               |        |
35    |                                               |        let's call the lifetime of this reference `'1`
36    |                                               let's call the lifetime of this reference `'2`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch/issue-90170-elision-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/issue-90170-elision-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90170-elision-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | pub fn foo(x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                        -        -      ^^^^^^^^^ argument requires that '1 must outlive `'2`
   |                        |        |
   |                        |        let's call the lifetime of this reference `'1`
   |                        let's call the lifetime of this reference `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |           ++++              ++          ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:5:44
   |
   |
LL | pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                         -           -      ^^^^^^^^^ argument requires that '1 must outlive `'2`
   |                         |           |
   |                         |           let's call the lifetime of this reference `'1`
   |                         let's call the lifetime of this reference `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo2<'a>(x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough

error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/issue-90170-elision-mismatch.rs:7:63
   |
   |
LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                                               -        -      ^^^^^^^^^ argument requires that '1 must outlive `'2`
   |                                               |        |
   |                                               |        let's call the lifetime of this reference `'1`
   |                                               let's call the lifetime of this reference `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&'a u8>, y: &'a u8) { x.push(y); } //~ ERROR lifetime may not live long enough
   |                                                ++          ++
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/lifetimes/issue-90600-expected-return-static-indirect.rs stdout ----
diff of stderr:

17    |                   - let's call the lifetime of this reference `'1`
18 ...
19 LL |     let read = &refcell as &RefCell<dyn Read>;
-    |                ^^^^^^^^ cast requires that `'1` must outlive `'static`
+    |                ^^^^^^^^ cast requires that '1 must outlive `'static`
22 error: aborting due to 2 previous errors
23 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect/issue-90600-expected-return-static-indirect.stderr
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
   |                -------- cast requires that `foo` is borrowed for `'static`
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
   |                ^^^^^^^^ cast requires that '1 must outlive `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lifetimes/lifetime-errors/ex2d-push-inference-variable-2.rs stdout ----
diff of stderr:

7    |            lifetime `'b` defined here
8 ...
9 LL |     a.push(b);
-    |     ^^^^^^^^^ argument requires that `'c` must outlive `'b`
+    |     ^^^^^^^^^ argument requires that 'c must outlive `'b`
11    |
12    = help: consider adding the following bound: `'c: 'b`


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
   |            lifetime `'b` defined here
...
LL |     a.push(b);
   |     ^^^^^^^^^ argument requires that 'c must outlive `'b`
   |
   = help: consider adding the following bound: `'c: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable.rs stdout ----
diff of stderr:

7    |            lifetime `'b` defined here
8 LL |     let z = Ref { data: y.data };
9 LL |     x.push(z);
-    |     ^^^^^^^^^ argument requires that `'c` must outlive `'b`
+    |     ^^^^^^^^^ argument requires that 'c must outlive `'b`
11    |
12    = help: consider adding the following bound: `'c: 'b`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable/ex2c-push-inference-variable.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2c-push-inference-variable/ex2c-push-inference-variable.stderr
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
   |            lifetime `'b` defined here
LL |     let z = Ref { data: y.data };
LL |     x.push(z);
   |     ^^^^^^^^^ argument requires that 'c must outlive `'b`
   |
   = help: consider adding the following bound: `'c: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex2b-push-no-existing-names.rs stdout ----
diff of stderr:

6    |        |
7    |        has type `&mut Vec<Ref<'2, i32>>`
8 LL |     x.push(y);
-    |     ^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |     ^^^^^^^^^ argument requires that '1 must outlive `'2`
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2b-push-no-existing-names/ex2b-push-no-existing-names.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex2b-push-no-existing-names.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex2b-push-no-existing-names.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2b-push-no-existing-names" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex2b-push-no-existing-names/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(x: &mut Vec<Ref<i32>>, y: Ref<i32>) {
   |        -                      - has type `Ref<'1, i32>`
   |        |
   |        has type `&mut Vec<Ref<'2, i32>>`
LL |     x.push(y);
   |     ^^^^^^^^^ argument requires that '1 must outlive `'2`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2.rs stdout ----
diff of stderr:

6    |                                   |
7    |                                   let's call the lifetime of this reference `'2`
8 LL |     *v = x;
-    |     ^^^^^^ assignment requires that `'1` must outlive `'2`
+    |     ^^^^^^ assignment requires that '1 must outlive `'2`
11 help: consider introducing a named lifetime parameter
12    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2/ex3-both-anon-regions-2.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(&mut (ref mut v, w): &mut (&u8, &u8), x: &u8) {
   |                                   -             - let's call the lifetime of this reference `'1`
   |                                   |
   |                                   let's call the lifetime of this reference `'2`
LL |     *v = x;
   |     ^^^^^^ assignment requires that '1 must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | fn foo<'a>(&mut (ref mut v, w): &mut (&'a u8, &u8), x: &'a u8) {
   |       ++++                             ++               ++
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex2e-push-inference-variable-3.rs stdout ----
diff of stderr:

7    |            lifetime `'b` defined here
9 LL |     Vec::push(a, b);
9 LL |     Vec::push(a, b);
-    |     ^^^^^^^^^^^^^^^ argument requires that `'c` must outlive `'b`
+    |     ^^^^^^^^^^^^^^^ argument requires that 'c must outlive `'b`
11    |
12    = help: consider adding the following bound: `'c: 'b`


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
   |            lifetime `'b` defined here
LL |     Vec::push(a, b);
LL |     Vec::push(a, b);
   |     ^^^^^^^^^^^^^^^ argument requires that 'c must outlive `'b`
   |
   = help: consider adding the following bound: `'c: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs stdout ----
diff of stderr:

6    |                     |
7    |                     let's call the lifetime of this reference `'2`
8 LL |     z.push((x,y));
-    |     ^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |     ^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
11 help: consider introducing a named lifetime parameter
12    |

21    |                         |
21    |                         |
22    |                         let's call the lifetime of this reference `'4`
23 LL |     z.push((x,y));
-    |     ^^^^^^^^^^^^^ argument requires that `'3` must outlive `'4`
+    |     ^^^^^^^^^^^^^ argument requires that '3 must outlive `'4`
26 help: consider introducing a named lifetime parameter
27    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3/ex3-both-anon-regions-3.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
   |                     -                   - let's call the lifetime of this reference `'1`
   |                     |
   |                     let's call the lifetime of this reference `'2`
LL |     z.push((x,y));
   |     ^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
help: consider introducing a named lifetime parameter
   |
   |
LL | fn foo<'a>(z: &mut Vec<(&'a u8,&u8)>, (x, y): (&'a u8, &u8)) {
   |       ++++               ++                     ++
error: lifetime may not live long enough
  --> /checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-3.rs:2:5
   |
   |
LL | fn foo(z: &mut Vec<(&u8,&u8)>, (x, y): (&u8, &u8)) {
   |                         -                    - let's call the lifetime of this reference `'3`
   |                         |
   |                         let's call the lifetime of this reference `'4`
LL |     z.push((x,y));
   |     ^^^^^^^^^^^^^ argument requires that '3 must outlive `'4`
help: consider introducing a named lifetime parameter
   |
   |
LL | fn foo<'a>(z: &mut Vec<(&u8,&'a u8)>, (x, y): (&u8, &'a u8)) {
   |       ++++                   ++                      ++
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-3.rs stdout ----
diff of stderr:

7    |        has type `Ref<'_, '1>`
8    |        has type `Ref<'2, '_>`
9 LL |     x.a = x.b;
-    |     ^^^^^^^^^ assignment requires that `'1` must outlive `'2`
+    |     ^^^^^^^^^ assignment requires that '1 must outlive `'2`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-3/ex3-both-anon-regions-both-are-structs-3.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-3/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(mut x: Ref) {
   |        |
   |        |
   |        has type `Ref<'_, '1>`
   |        has type `Ref<'2, '_>`
LL |     x.a = x.b;
   |     ^^^^^^^^^ assignment requires that '1 must outlive `'2`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-2.rs stdout ----
diff of stderr:

6    |        |
7    |        has type `Ref<'_, '2>`
8 LL |     x.b = y.b;
-    |     ^^^^^^^^^ assignment requires that `'1` must outlive `'2`
+    |     ^^^^^^^^^ assignment requires that '1 must outlive `'2`
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-2/ex3-both-anon-regions-both-are-structs-2.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(mut x: Ref, y: Ref) {
   |        -----       - has type `Ref<'_, '1>`
   |        |
   |        has type `Ref<'_, '2>`
LL |     x.b = y.b;
   |     ^^^^^^^^^ assignment requires that '1 must outlive `'2`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions.rs stdout ----
diff of stderr:

7    |        lifetime `'a` defined here
8 ...
9 LL |     x.push(y);
-    |     ^^^^^^^^^ argument requires that `'b` must outlive `'a`
+    |     ^^^^^^^^^ argument requires that 'b must outlive `'a`
11    |
12    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions/ex3-both-anon-regions-both-are-structs-earlybound-regions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions/ex3-both-anon-regions-both-are-structs-earlybound-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-both-are-structs-earlybound-regions/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
---

---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-fail-no-postdom.rs stdout ----
diff of stderr:

22    |          has type `Cell<&'_#1r &'1 u32>`
23 ...
24 LL |             demand_y(x, y, p)
-    |             ^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |             ^^^^^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
27 note: no external requirements
28   --> $DIR/propagate-approximated-fail-no-postdom.rs:38:1



The actual stderr differed from the expected stderr.
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
   |          ----------              ---------- has type `Cell<&'2 &'_#3r u32>`
   |          |
   |          has type `Cell<&'_#1r &'1 u32>`
...
LL |             demand_y(x, y, p) //~ ERROR
   |             ^^^^^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
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

---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-ref.rs stdout ----
diff of stderr:

37    |           lifetime `'a` defined here
38 ...
39 LL |         demand_y(x, y, x.get())
-    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that 'a must outlive `'b`
41    |
42    = help: consider adding the following bound: `'a: 'b`


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
   |           --  -- lifetime `'b` defined here
   |           |
   |           lifetime `'a` defined here
...
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that 'a must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs stdout ----
diff of stderr:

4 LL | fn foo<'a>(x: &'a u32) -> &'static u32 {
5    |        -- lifetime `'a` defined here
6 LL |     &*x
-    |     ^^^ returning this value requires that `'a` must outlive `'static`
+    |     ^^^ returning this value requires that 'a must outlive `'static`
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/region-lbr-named-does-not-outlive-static.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-named-does-not-outlive-static/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a>(x: &'a u32) -> &'static u32 {
   |        -- lifetime `'a` defined here
LL |     &*x
   |     ^^^ returning this value requires that 'a must outlive `'static`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/region-lbr-anon-does-not-outlive-static.rs stdout ----
diff of stderr:

4 LL | fn foo(x: &u32) -> &'static u32 {
5    |           - let's call the lifetime of this reference `'1`
6 LL |     &*x
-    |     ^^^ returning this value requires that `'1` must outlive `'static`
+    |     ^^^ returning this value requires that '1 must outlive `'static`
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-anon-does-not-outlive-static/region-lbr-anon-does-not-outlive-static.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/closure-requirements/region-lbr-anon-does-not-outlive-static.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-requirements/region-lbr-anon-does-not-outlive-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-anon-does-not-outlive-static" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/region-lbr-anon-does-not-outlive-static/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo(x: &u32) -> &'static u32 {
   |           - let's call the lifetime of this reference `'1`
LL |     &*x
   |     ^^^ returning this value requires that '1 must outlive `'static`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-approximated-val.rs stdout ----
diff of stderr:

37    |         lifetime `'a` defined here
38 ...
39 LL |         demand_y(outlives1, outlives2, x.get())
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'a` must outlive `'b`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that 'a must outlive `'b`
41    |
42    = help: consider adding the following bound: `'a: 'b`


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
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
...
LL |         demand_y(outlives1, outlives2, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that 'a must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds.rs stdout ----
diff of stderr:

21    |                                                has type `&'_#5r Cell<&'2 &'_#1r u32>`
22 LL |         // Only works if 'x: 'y:
23 LL |         demand_y(x, y, x.get())
-    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
26 note: no external requirements
27   --> $DIR/propagate-fail-to-approximate-longer-no-bounds.rs:34:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-no-bounds/propagate-fail-to-approximate-longer-no-bounds.stderr
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
   |                                                has type `&'_#5r Cell<&'2 &'_#1r u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
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
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds.rs stdout ----
diff of stderr:

21    |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
22 LL |         // Only works if 'x: 'y:
23 LL |         demand_y(x, y, x.get())
-    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
26 note: no external requirements
27   --> $DIR/propagate-fail-to-approximate-longer-wrong-bounds.rs:38:1



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-requirements/propagate-fail-to-approximate-longer-wrong-bounds/propagate-fail-to-approximate-longer-wrong-bounds.stderr
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
   |                                                ----------  ---------- has type `&'_#8r Cell<&'2 &'_#2r u32>`
   |                                                |
   |                                                has type `&'_#6r Cell<&'1 &'_#1r u32>`
LL |         // Only works if 'x: 'y:
LL |         demand_y(x, y, x.get())
   |         ^^^^^^^^^^^^^^^^^^^^^^^ argument requires that '1 must outlive `'2`
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


---- [ui] src/test/ui/nll/guarantor-issue-46974.rs stdout ----
diff of stderr:

16    |           - let's call the lifetime of this reference `'1`
17 LL |     // FIXME(#46983): error message should be better
18 LL |     &s.0
-    |     ^^^^ returning this value requires that `'1` must outlive `'static`
+    |     ^^^^ returning this value requires that '1 must outlive `'static`
21 error: aborting due to 2 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/guarantor-issue-46974/guarantor-issue-46974.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/guarantor-issue-46974.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/guarantor-issue-46974.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/guarantor-issue-46974" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/guarantor-issue-46974/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0506]: cannot assign to `*s` because it is borrowed
   |
   |
LL |     let t = &mut *s; // this borrow should last for the entire function
   |             ------- borrow of `*s` occurs here
LL |     let x = &t.0;
LL |     *s = (2,); //~ ERROR cannot assign to `*s`
   |     ^^^^^^^^^ assignment to borrowed `*s` occurs here
LL |     *x

error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/guarantor-issue-46974.rs:13:5
   |
   |
LL | fn bar(s: &Box<(i32,)>) -> &'static i32 {
   |           - let's call the lifetime of this reference `'1`
LL |     // FIXME(#46983): error message should be better
LL |     &s.0 //~ ERROR lifetime may not live long enough
   |     ^^^^ returning this value requires that '1 must outlive `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0506`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-42574-diagnostic-in-nested-closure.rs stdout ----
diff of stderr:

2   --> $DIR/issue-42574-diagnostic-in-nested-closure.rs:6:8
3    |
4 LL |     || doit(data);
-    |     -- ^^^^^^^^^^ argument requires that `'1` must outlive `'static`
+    |     -- ^^^^^^^^^^ argument requires that '1 must outlive `'static`
6    |     |
7    |     lifetime `'1` represents this closure's body


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-42574-diagnostic-in-nested-closure/issue-42574-diagnostic-in-nested-closure.stderr
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
   |     -- ^^^^^^^^^^ argument requires that '1 must outlive `'static`
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
   |     |  argument requires that `data` is borrowed for `'static`
...
LL | }
LL | }
   |  - `data` dropped here while still borrowed
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/issue-48238.rs stdout ----
diff of stderr:

2   --> $DIR/issue-48238.rs:9:13
3    |
4 LL |     move || use_val(&orig);
-    |     ------- ^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'2`
+    |     ------- ^^^^^^^^^^^^^^ returning this value requires that '1 must outlive `'2`
6    |     |     |
7    |     |     return type of closure is &'2 u8
8    |     lifetime `'1` represents this closure's body

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/issue-48238.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-48238.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-48238.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-48238/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |     move || use_val(&orig); //~ ERROR
   |     ------- ^^^^^^^^^^^^^^ returning this value requires that '1 must outlive `'2`
   |     |     |
   |     |     return type of closure is &'2 u8
   |     lifetime `'1` represents this closure's body
   |
   = note: closure implements `Fn`, so references to captured variables can't escape the closure
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/issue-50716.rs stdout ----
diff of stderr:

5    |        -- lifetime `'a` defined here
7 LL |     let _x = *s;
7 LL |     let _x = *s;
-    |              ^^ proving this value is `Sized` requires that `'a` must outlive `'static`
+    |              ^^ proving this value is `Sized` requires that 'a must outlive `'static`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-50716/issue-50716.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-50716.rs`
error: 1 errors occurred comparing output.
