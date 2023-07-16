plain

---- [ui (nll)] ui/associated-types/cache/project-fn-ret-invariant.rs#krisskross stdout ----
diff of stderr:

10    |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`
-    = note: requirement occurs because of the type Type<'_>, which makes the generic argument '_ invariant
-    = note: the struct Type<'a> is invariant over the parameter 'a
+    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
+    = note: the struct `Type<'a>` is invariant over the parameter `'a`
15    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
17 error: lifetime may not live long enough


26    |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
27    |
28    = help: consider adding the following bound: `'b: 'a`
-    = note: requirement occurs because of the type Type<'_>, which makes the generic argument '_ invariant
-    = note: the struct Type<'a> is invariant over the parameter 'a
+    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
+    = note: the struct `Type<'a>` is invariant over the parameter `'a`
31    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
32 
33 help: `'a` and `'b` must be the same: replace one with the other

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross.nll/project-fn-ret-invariant.krisskross.nll.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |              --  -- lifetime `'b` defined here
   |              |
   |              lifetime `'a` defined here
...
LL |     (a, b) //[krisskross]~ ERROR E0623
   |     ^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:56:5
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |              --  -- lifetime `'b` defined here
   |              |
   |              lifetime `'a` defined here
...
LL |     (a, b) //[krisskross]~ ERROR E0623
   |     ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui (nll)] ui/associated-types/cache/project-fn-ret-invariant.rs#transmute stdout ----
diff of stderr:

7 LL |     bar(foo, x)
8    |     ^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
9    |
-    = note: requirement occurs because of the type Type<'_>, which makes the generic argument '_ invariant
-    = note: the struct Type<'a> is invariant over the parameter 'a
+    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
+    = note: the struct `Type<'a>` is invariant over the parameter `'a`
12    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
14 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.transmute.nll/project-fn-ret-invariant.transmute.nll.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`


error in revision `transmute`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "transmute" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.transmute.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.transmute.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn baz<'a, 'b>(x: Type<'a>) -> Type<'static> {
   |        -- lifetime `'a` defined here
...
LL |     bar(foo, x) //[transmute]~ ERROR E0759
   |     ^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
   |
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui (nll)] ui/associated-types/cache/project-fn-ret-invariant.rs#oneuse stdout ----
diff of stderr:

10    |             ^^^^^^^^^ argument requires that `'a` must outlive `'b`
11    |
12    = help: consider adding the following bound: `'a: 'b`
-    = note: requirement occurs because of the type Type<'_>, which makes the generic argument '_ invariant
-    = note: the struct Type<'a> is invariant over the parameter 'a
+    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
+    = note: the struct `Type<'a>` is invariant over the parameter `'a`
15    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
17 error: lifetime may not live long enough


26    |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`
27    |
28    = help: consider adding the following bound: `'b: 'a`
-    = note: requirement occurs because of the type Type<'_>, which makes the generic argument '_ invariant
-    = note: the struct Type<'a> is invariant over the parameter 'a
+    = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
+    = note: the struct `Type<'a>` is invariant over the parameter `'a`
31    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
32 
33 help: `'a` and `'b` must be the same: replace one with the other

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse.nll/project-fn-ret-invariant.oneuse.nll.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`


error in revision `oneuse`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "oneuse" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse.nll/auxiliary"
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
   |        lifetime `'a` defined here
...
LL |     let b = bar(f, y); //[oneuse]~ ERROR lifetime mismatch [E0623]
   |             ^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of the type `Type<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Type<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 2 previous errors
------------------------------------------



---- [ui (nll)] ui/hr-subtype/hr-subtype.rs#free_inv_x_vs_free_inv_y stdout ----

13    | |______________- in this macro invocation
14    |
14    |
15    = help: consider adding the following bound: `'x: 'y`
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: requirement occurs because of the type Inv<'_>, which makes the generic argument '_ invariant
-    = note: the struct Inv<'a> is invariant over the parameter 'a
+    = note: requirement occurs because of the type `Inv<'_>`, which makes the generic argument `'_` invariant
+    = note: the struct `Inv<'a>` is invariant over the parameter `'a`
18    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
19    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)

33    | |______________- in this macro invocation
34    |
34    |
35    = help: consider adding the following bound: `'x: 'y`
-    = note: requirement occurs because of the type Inv<'_>, which makes the generic argument '_ invariant
-    = note: the struct Inv<'a> is invariant over the parameter 'a
+    = note: requirement occurs because of the type `Inv<'_>`, which makes the generic argument `'_` invariant
+    = note: the struct `Inv<'a>` is invariant over the parameter `'a`
38    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
39    = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_inv_x_vs_free_inv_y.nll/hr-subtype.free_inv_x_vs_free_inv_y.nll.stderr
To only update this specific test, also pass `--test-args hr-subtype/hr-subtype.rs`


error in revision `free_inv_x_vs_free_inv_y`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hr-subtype/hr-subtype.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "free_inv_x_vs_free_inv_y" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_inv_x_vs_free_inv_y.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hr-subtype/hr-subtype.free_inv_x_vs_free_inv_y.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |           fn subtype<'x, 'y: 'x, 'z: 'y>() {
   |                      --  -- lifetime `'y` defined here
   |                      |
   |                      lifetime `'x` defined here
LL |               gimme::<$t2>(None::<$t1>);
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'x` must outlive `'y`
...
LL | / check! { free_inv_x_vs_free_inv_y: (fn(Inv<'x>),
LL | | fn(Inv<'y>)) }
   |
   |
   = help: consider adding the following bound: `'x: 'y`
   = note: requirement occurs because of the type `Inv<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Inv<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)
error: lifetime may not live long enough
  --> /checkout/src/test/ui/hr-subtype/hr-subtype.rs:45:13
   |
   |
LL |           fn supertype<'x, 'y: 'x, 'z: 'y>() {
   |                        --  -- lifetime `'y` defined here
   |                        |
   |                        lifetime `'x` defined here
LL |               gimme::<$t1>(None::<$t2>);
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'x` must outlive `'y`
...
LL | / check! { free_inv_x_vs_free_inv_y: (fn(Inv<'x>),
LL | | fn(Inv<'y>)) }
   |
   |
   = help: consider adding the following bound: `'x: 'y`
   = note: requirement occurs because of the type `Inv<'_>`, which makes the generic argument `'_` invariant
   = note: the struct `Inv<'a>` is invariant over the parameter `'a`
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
   = note: this error originates in the macro `check` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors
------------------------------------------



---- [ui (nll)] ui/match/match-ref-mut-invariance.rs stdout ----
diff of stderr:

9    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
10    |
11    = help: consider adding the following bound: `'a: 'b`
-    = note: requirement occurs because of a mutable reference to &i32
+    = note: requirement occurs because of a mutable reference to `&i32`
13    = note: mutable references are invariant over their type parameter
14    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance.nll/match-ref-mut-invariance.nll.stderr
To only update this specific test, also pass `--test-args match/match-ref-mut-invariance.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-ref-mut-invariance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | impl<'b> S<'b> {
   |      -- lifetime `'b` defined here
LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
   |            -- lifetime `'a` defined here
LL |         match self.0 { ref mut x => x } //~ ERROR mismatched types
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to `&i32`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui (nll)] ui/match/match-ref-mut-let-invariance.rs stdout ----
diff of stderr:

10    |         ^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
11    |
12    = help: consider adding the following bound: `'a: 'b`
-    = note: requirement occurs because of a mutable reference to &i32
+    = note: requirement occurs because of a mutable reference to `&i32`
14    = note: mutable references are invariant over their type parameter
15    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance.nll/match-ref-mut-let-invariance.nll.stderr
To only update this specific test, also pass `--test-args match/match-ref-mut-let-invariance.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-ref-mut-let-invariance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance.nll/auxiliary"
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
LL |         x //~ ERROR mismatched types
   |         ^ associated function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to `&i32`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui (nll)] ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs stdout ----
diff of stderr:

23    |     ^^^^^^^^^^ argument requires that `'b` must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`
-    = note: requirement occurs because of a mutable reference to &isize
+    = note: requirement occurs because of a mutable reference to `&isize`
27    = note: mutable references are invariant over their type parameter
28    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll/region-multiple-lifetime-bounds-on-fns-where-clause.nll.stderr
To only update this specific test, also pass `--test-args regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
LL |     // Illegal now because there is no `'b:'a` declaration.
LL |     *x = *y; //~ ERROR E0623
   |     ^^^^^^^ assignment requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:16:5
   |
   |
LL | fn c<'a,'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
   |      -- -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
...
LL |     a(x, y, z); //~ ERROR lifetime mismatch [E0623]
   |     ^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to `&isize`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'r, 's, 't0, 't1, 't2, 't3> fn(&'r mut &'s isize, &'t0 mut &'t1 isize, &'t2 mut &'t3 isize)`
              found fn pointer `for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'r, 's, 't0, 't1, 't2, 't3> fn(&'r mut &'s isize, &'t0 mut &'t1 isize, &'t2 mut &'t3 isize)`
              found fn pointer `for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected fn pointer `for<'r, 's, 't0, 't1, 't2, 't3> fn(&'r mut &'s isize, &'t0 mut &'t1 isize, &'t2 mut &'t3 isize)`
              found fn pointer `for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize)`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui (nll)] ui/regions/region-lifetime-bounds-on-fns-where-clause.rs stdout ----
diff of stderr:

23    |     ^^^^^^^ argument requires that `'b` must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`
-    = note: requirement occurs because of a mutable reference to &isize
+    = note: requirement occurs because of a mutable reference to `&isize`
27    = note: mutable references are invariant over their type parameter
28    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll/region-lifetime-bounds-on-fns-where-clause.nll.stderr
To only update this specific test, also pass `--test-args regions/region-lifetime-bounds-on-fns-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn b<'a, 'b>(x: &mut &'a isize, y: &mut &'b isize) {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
LL |     // Illegal now because there is no `'b:'a` declaration.
LL |     *x = *y; //~ ERROR E0623
   |     ^^^^^^^ assignment requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:14:5
   |
   |
LL | fn c<'a,'b>(x: &mut &'a isize, y: &mut &'b isize) {
   |      -- -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
...
LL |     a(x, y); //~ ERROR lifetime mismatch [E0623]
   |     ^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to `&isize`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error[E0308]: mismatched types
  --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:20:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR mismatched types
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
