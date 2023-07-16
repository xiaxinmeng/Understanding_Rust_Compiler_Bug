plain
...........................ii................i..i..ii............................................... 7400/11868
.................................................................................................... 7500/11868
.................................................................................................... 7600/11868
.................................................................................................... 7700/11868
...F..F.......................................................................i..ii................. 7800/11868
.................................................................................................... 8000/11868
..............................................i..................................................... 8100/11868
........................................i........................................................... 8200/11868
.............................................................................i...................... 8300/11868
---

---- [ui] ui/c-variadic/variadic-ffi-4.rs stdout ----
diff of stderr:

64    |                                               has type `&mut VaListImpl<'1>`
65 LL |     ap0 = &mut ap1;
66    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
+    |
+    = note: requirement occurs because of a mutable reference/pointer to core::ffi::VaListImpl<'_>
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
68 error: lifetime may not live long enough
69   --> $DIR/variadic-ffi-4.rs:28:5

74    |                                               has type `&mut VaListImpl<'1>`
74    |                                               has type `&mut VaListImpl<'1>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
75 LL |     ap0 = &mut ap1;
76    |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
+    |
+    = note: requirement occurs because of a mutable reference/pointer to core::ffi::VaListImpl<'_>
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
77 
78 error[E0597]: `ap1` does not live long enough


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/variadic-ffi-4.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'f`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ returning this value requires that `'1` must outlive `'f`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:14:5
   |
   |
LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
   |                                               -- has type `VaListImpl<'1>`
LL |     ap //~ ERROR: lifetime may not live long enough
   |     ^^ returning this value requires that `'1` must outlive `'static`
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
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'1` must outlive `'2`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'2` must outlive `'1`
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
   = note: requirement occurs because of a mutable reference/pointer to core::ffi::VaListImpl<'_>
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
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
   = note: requirement occurs because of a mutable reference/pointer to core::ffi::VaListImpl<'_>
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error[E0597]: `ap1` does not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                                        - let's call the lifetime of this reference `'3`
LL |     ap0 = &mut ap1;
   |     |     |
   |     |     |
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
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/nll/type-check-pointer-coercions.rs stdout ----
diff of stderr:

34    |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
35    |
36    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
38 error: lifetime may not live long enough
39   --> $DIR/type-check-pointer-coercions.rs:13:5


47    |     ^ returning this value requires that `'a` must outlive `'b`
48    |
49    = help: consider adding the following bound: `'a: 'b`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
50 
51 help: `'b` and `'a` must be the same: replace one with the other


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions/type-check-pointer-coercions.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions/type-check-pointer-coercions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/type-check-pointer-coercions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/type-check-pointer-coercions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-coercions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:4:5
   |
LL | fn shared_to_const<'a, 'b>(x: &&'a i32) -> *const &'b i32 {
   |                    --  -- lifetime `'b` defined here
   |                    |
   |                    lifetime `'a` defined here
LL |     x   //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:8:5
   |
   |
LL | fn unique_to_const<'a, 'b>(x: &mut &'a i32) -> *const &'b i32 {
   |                    --  -- lifetime `'b` defined here
   |                    |
   |                    lifetime `'a` defined here
LL |     x   //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:13:5
   |
   |
LL | fn unique_to_mut<'a, 'b>(x: &mut &'a i32) -> *mut &'b i32 {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     // Two errors because *mut is invariant
LL |     x   //~ ERROR
   |     ^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:13:5
   |
   |
LL | fn unique_to_mut<'a, 'b>(x: &mut &'a i32) -> *mut &'b i32 {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     // Two errors because *mut is invariant
LL |     x   //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'b` and `'a` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:18:5
   |
   |
LL | fn mut_to_const<'a, 'b>(x: *mut &'a i32) -> *const &'b i32 {
   |                 --  -- lifetime `'b` defined here
   |                 |
   |                 lifetime `'a` defined here
LL |     x   //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:24:5
   |
   |
LL | fn array_elem<'a, 'b>(x: &'a i32) -> *const &'b i32 {
   |               --  -- lifetime `'b` defined here
   |               |
   |               lifetime `'a` defined here
...
LL |     y   //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:30:5
   |
   |
LL | fn array_coerce<'a, 'b>(x: &'a i32) -> *const [&'b i32; 3] {
   |                 --  -- lifetime `'b` defined here
   |                 |
   |                 lifetime `'a` defined here
...
LL |     y   //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-coercions.rs:36:5
   |
   |
LL | fn nested_array<'a, 'b>(x: &'a i32) -> *const [&'b i32; 2] {
   |                 --  -- lifetime `'b` defined here
   |                 |
   |                 lifetime `'a` defined here
...
LL |     y   //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to 8 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/nll/type-check-pointer-comparisons.rs stdout ----
diff of stderr:

9    |     ^ requires that `'a` must outlive `'b`
10    |
11    = help: consider adding the following bound: `'a: 'b`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
13 error: lifetime may not live long enough
14   --> $DIR/type-check-pointer-comparisons.rs:6:10


21    |          ^ requires that `'b` must outlive `'a`
22    |
23    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
24 
25 help: `'a` and `'b` must be the same: replace one with the other


35    |     ^ requires that `'a` must outlive `'b`
36    |
37    = help: consider adding the following bound: `'a: 'b`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
39 error: lifetime may not live long enough
40   --> $DIR/type-check-pointer-comparisons.rs:12:10


47    |          ^ requires that `'b` must outlive `'a`
48    |
49    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
50 
51 help: `'a` and `'b` must be the same: replace one with the other


61    |     ^ requires that `'a` must outlive `'b`
62    |
63    = help: consider adding the following bound: `'a: 'b`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
65 error: lifetime may not live long enough
66   --> $DIR/type-check-pointer-comparisons.rs:18:10


73    |          ^ requires that `'b` must outlive `'a`
74    |
75    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference/pointer to &i32
+    = note: mutable references/pointers are invariant over their type parameter
+    = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
76 
77 help: `'a` and `'b` must be the same: replace one with the other


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/type-check-pointer-comparisons.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/type-check-pointer-comparisons.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/type-check-pointer-comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/type-check-pointer-comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:6:5
   |
LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     x == y;
   |     ^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:6:10
   |
   |
LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     x == y;
   |          ^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:12:5
   |
   |
LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
   |                --  -- lifetime `'b` defined here
   |                |
   |                lifetime `'a` defined here
LL |     x == y;
   |     ^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:12:10
   |
   |
LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
   |                --  -- lifetime `'b` defined here
   |                |
   |                lifetime `'a` defined here
LL |     x == y;
   |          ^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:18:5
   |
   |
LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
   |                   --  -- lifetime `'b` defined here
   |                   |
   |                   lifetime `'a` defined here
LL |     f == g;
   |     ^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:18:10
   |
   |
LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
   |                   --  -- lifetime `'b` defined here
   |                   |
   |                   lifetime `'a` defined here
LL |     f == g;
   |          ^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference/pointer to &i32
   = note: mutable references/pointers are invariant over their type parameter
   = help: See <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 6 previous errors


------------------------------------------
---
test result: FAILED. 11769 passed; 3 failed; 96 ignored; 0 measured; 0 filtered out; finished in 122.15s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:36
