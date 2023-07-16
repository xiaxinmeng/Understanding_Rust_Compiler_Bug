plain
........................................................................................ 11880/13261
........................................................................................ 11968/13261
........................................................................................ 12056/13261
........................................................................................ 12144/13261
..............................................................FF.................F...... 12232/13261
F..........F............................................................................ 12320/13261
........................................................................................ 12496/13261
........................................................................................ 12584/13261
........................................................................................ 12672/13261
........................................................................................ 12760/13261
---
..........................................iii........................................... 13200/13261
.............................................................
failures:

---- [ui] src/test/ui/transmutability/primitives/bool.rs stdout ----


1 error[E0277]: the trait bound `bool: BikeshedIntrinsicFrom<u8, assert::Context, false, false, false, true>` is not satisfied
-   --> $DIR/bool.rs:23:35
3    |
3    |
4 LL |     assert::is_transmutable::<u8, bool>();
5    |                                   ^^^^ the trait `BikeshedIntrinsicFrom<u8, assert::Context, false, false, false, true>` is not implemented for `bool`
6    |
7 note: required by a bound in `is_transmutable`
-   --> $DIR/bool.rs:13:14
+   --> $DIR/bool.rs:12:14
+   --> $DIR/bool.rs:12:14
9    |
10 LL |     pub fn is_transmutable<Src, Dst>()
11    |            --------------- required by a bound in this

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/primitives/bool/bool.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmutability/primitives/bool.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/primitives/bool.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/primitives/bool" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/primitives/bool/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `bool: BikeshedIntrinsicFrom<u8, assert::Context, false, false, false, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/primitives/bool.rs:22:35
   |
LL |     assert::is_transmutable::<u8, bool>(); //~ ERROR not satisfied
   |                                   ^^^^ the trait `BikeshedIntrinsicFrom<u8, assert::Context, false, false, false, true>` is not implemented for `bool`
note: required by a bound in `is_transmutable`
  --> /checkout/src/test/ui/transmutability/primitives/bool.rs:12:14
   |
   |
LL |     pub fn is_transmutable<Src, Dst>()
   |            --------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, false, false, false, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_transmutable`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/transmutability/primitives/unit.rs stdout ----


1 error[E0277]: the trait bound `u8: BikeshedIntrinsicFrom<(), should_have_correct_size::Context, true, true, true, true>` is not satisfied
-   --> $DIR/unit.rs:22:35
3    |
3    |
4 LL |     assert::is_transmutable::<(), u8, Context>();
5    |                                   ^^ the trait `BikeshedIntrinsicFrom<(), should_have_correct_size::Context, true, true, true, true>` is not implemented for `u8`
6    |
7 note: required by a bound in `is_transmutable`
-   --> $DIR/unit.rs:11:14
+   --> $DIR/unit.rs:12:14
+   --> $DIR/unit.rs:12:14
9    |
10 LL |     pub fn is_transmutable<Src, Dst, Context>()
11    |            --------------- required by a bound in this

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/primitives/unit/unit.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmutability/primitives/unit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/primitives/unit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/primitives/unit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/primitives/unit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `u8: BikeshedIntrinsicFrom<(), should_have_correct_size::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/primitives/unit.rs:23:35
   |
LL |     assert::is_transmutable::<(), u8, Context>(); //~ ERROR not satisfied
   |                                   ^^ the trait `BikeshedIntrinsicFrom<(), should_have_correct_size::Context, true, true, true, true>` is not implemented for `u8`
note: required by a bound in `is_transmutable`
  --> /checkout/src/test/ui/transmutability/primitives/unit.rs:12:14
   |
   |
LL |     pub fn is_transmutable<Src, Dst, Context>()
   |            --------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_transmutable`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/transmutability/visibility/assume/should_accept_if_dst_has_unreachable_field.rs stdout ----


2   --> $DIR/should_accept_if_dst_has_unreachable_field.rs:32:9
3    |
4 LL |     #[repr(C)] pub(self) struct Zst; // <- unreachable type
-    |                --------------------- `dst::Zst` declared as private
+    |                -------------------- `dst::Zst` declared as private
6 ...
7 LL |         pub(in super) field: Zst,
8    |         ^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/visibility/assume/should_accept_if_dst_has_unreachable_field/should_accept_if_dst_has_unreachable_field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmutability/visibility/assume/should_accept_if_dst_has_unreachable_field.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/visibility/assume/should_accept_if_dst_has_unreachable_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/visibility/assume/should_accept_if_dst_has_unreachable_field" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/visibility/assume/should_accept_if_dst_has_unreachable_field/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0446]: private type `dst::Zst` in public interface
  --> /checkout/src/test/ui/transmutability/visibility/assume/should_accept_if_dst_has_unreachable_field.rs:32:9
   |
LL |     #[repr(C)] pub(self) struct Zst; // <- unreachable type
   |                -------------------- `dst::Zst` declared as private
...
LL |         pub(in super) field: Zst, //~ ERROR private type
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0446`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/transmutability/visibility/should_accept_if_src_has_unreachable_field.rs stdout ----


2   --> $DIR/should_accept_if_src_has_unreachable_field.rs:23:9
3    |
4 LL |     #[repr(C)] pub(self) struct Zst; // <- unreachable type
-    |                --------------------- `src::Zst` declared as private
+    |                -------------------- `src::Zst` declared as private
6 ...
7 LL |         pub(in super) field: Zst,
8    |         ^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/visibility/should_accept_if_src_has_unreachable_field/should_accept_if_src_has_unreachable_field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmutability/visibility/should_accept_if_src_has_unreachable_field.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/visibility/should_accept_if_src_has_unreachable_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/visibility/should_accept_if_src_has_unreachable_field" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/visibility/should_accept_if_src_has_unreachable_field/auxiliary"
stdout: none
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
--- stderr -------------------------------
error[E0446]: private type `src::Zst` in public interface
  --> /checkout/src/test/ui/transmutability/visibility/should_accept_if_src_has_unreachable_field.rs:23:9
   |
LL |     #[repr(C)] pub(self) struct Zst; // <- unreachable type
   |                -------------------- `src::Zst` declared as private
...
LL |         pub(in super) field: Zst, //~ ERROR private type
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0446`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs stdout ----


1 error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<should_reject_repr_rust::unit::repr_rust, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:20:52
+   --> $DIR/should_require_well_defined_layout.rs:21:52
3    |
4 LL |         assert::is_maybe_transmutable::<repr_rust, ()>();
5    |                                                    ^^ the trait `BikeshedIntrinsicFrom<should_reject_repr_rust::unit::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
6    |
6    |
7 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
9    |
10 LL |     pub fn is_maybe_transmutable<Src, Dst>()
11    |            --------------------- required by a bound in this

14    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
15 
16 error[E0277]: the trait bound `should_reject_repr_rust::unit::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:21:47
+   --> $DIR/should_require_well_defined_layout.rs:22:47
18    |
19 LL |         assert::is_maybe_transmutable::<u128, repr_rust>();
20    |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `should_reject_repr_rust::unit::repr_rust`
21    |
21    |
22 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
24    |
25 LL |     pub fn is_maybe_transmutable<Src, Dst>()
26    |            --------------------- required by a bound in this

29    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
30 
31 error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<should_reject_repr_rust::tuple::repr_rust, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:26:52
+   --> $DIR/should_require_well_defined_layout.rs:27:52
33    |
34 LL |         assert::is_maybe_transmutable::<repr_rust, ()>();
35    |                                                    ^^ the trait `BikeshedIntrinsicFrom<should_reject_repr_rust::tuple::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
36    |
36    |
37 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
39    |
40 LL |     pub fn is_maybe_transmutable<Src, Dst>()
41    |            --------------------- required by a bound in this

44    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
45 
46 error[E0277]: the trait bound `should_reject_repr_rust::tuple::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:27:47
+   --> $DIR/should_require_well_defined_layout.rs:28:47
48    |
49 LL |         assert::is_maybe_transmutable::<u128, repr_rust>();
50    |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `should_reject_repr_rust::tuple::repr_rust`
51    |
51    |
52 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
54    |
55 LL |     pub fn is_maybe_transmutable<Src, Dst>()
56    |            --------------------- required by a bound in this

59    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
60 
61 error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<should_reject_repr_rust::braces::repr_rust, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:32:52
+   --> $DIR/should_require_well_defined_layout.rs:33:52
63    |
64 LL |         assert::is_maybe_transmutable::<repr_rust, ()>();
65    |                                                    ^^ the trait `BikeshedIntrinsicFrom<should_reject_repr_rust::braces::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
66    |
66    |
67 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
69    |
70 LL |     pub fn is_maybe_transmutable<Src, Dst>()
71    |            --------------------- required by a bound in this

74    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
75 
76 error[E0277]: the trait bound `should_reject_repr_rust::braces::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:33:47
+   --> $DIR/should_require_well_defined_layout.rs:34:47
78    |
79 LL |         assert::is_maybe_transmutable::<u128, repr_rust>();
80    |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `should_reject_repr_rust::braces::repr_rust`
81    |
81    |
82 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
84    |
85 LL |     pub fn is_maybe_transmutable<Src, Dst>()
86    |            --------------------- required by a bound in this

89    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
90 
91 error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<aligned::repr_rust, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:38:52
+   --> $DIR/should_require_well_defined_layout.rs:39:52
93    |
94 LL |         assert::is_maybe_transmutable::<repr_rust, ()>();
95    |                                                    ^^ the trait `BikeshedIntrinsicFrom<aligned::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
96    |
96    |
97 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
99    |
100 LL |     pub fn is_maybe_transmutable<Src, Dst>()
101    |            --------------------- required by a bound in this

104    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
105 
106 error[E0277]: the trait bound `aligned::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:39:47
+   --> $DIR/should_require_well_defined_layout.rs:40:47
108    |
109 LL |         assert::is_maybe_transmutable::<u128, repr_rust>();
110    |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `aligned::repr_rust`
111    |
111    |
112 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
114    |
115 LL |     pub fn is_maybe_transmutable<Src, Dst>()
116    |            --------------------- required by a bound in this

119    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
120 
121 error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<packed::repr_rust, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:44:52
+   --> $DIR/should_require_well_defined_layout.rs:45:52
123    |
124 LL |         assert::is_maybe_transmutable::<repr_rust, ()>();
125    |                                                    ^^ the trait `BikeshedIntrinsicFrom<packed::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
126    |
126    |
127 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
129    |
130 LL |     pub fn is_maybe_transmutable<Src, Dst>()
131    |            --------------------- required by a bound in this

134    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
135 
136 error[E0277]: the trait bound `packed::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:45:47
+   --> $DIR/should_require_well_defined_layout.rs:46:47
138    |
139 LL |         assert::is_maybe_transmutable::<u128, repr_rust>();
140    |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `packed::repr_rust`
141    |
141    |
142 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
144    |
145 LL |     pub fn is_maybe_transmutable<Src, Dst>()
146    |            --------------------- required by a bound in this

149    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
150 
151 error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<nested::repr_c, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:51:49
+   --> $DIR/should_require_well_defined_layout.rs:52:49
153    |
154 LL |         assert::is_maybe_transmutable::<repr_c, ()>();
155    |                                                 ^^ the trait `BikeshedIntrinsicFrom<nested::repr_c, assert::Context, true, true, true, true>` is not implemented for `()`
156    |
156    |
157 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
159    |
160 LL |     pub fn is_maybe_transmutable<Src, Dst>()
161    |            --------------------- required by a bound in this

164    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`
165 
166 error[E0277]: the trait bound `nested::repr_c: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
-   --> $DIR/should_require_well_defined_layout.rs:52:47
+   --> $DIR/should_require_well_defined_layout.rs:53:47
168    |
169 LL |         assert::is_maybe_transmutable::<u128, repr_c>();
170    |                                               ^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `nested::repr_c`
171    |
171    |
172 note: required by a bound in `is_maybe_transmutable`
-   --> $DIR/should_require_well_defined_layout.rs:12:14
+   --> $DIR/should_require_well_defined_layout.rs:13:14
174    |
175 LL |     pub fn is_maybe_transmutable<Src, Dst>()
176    |            --------------------- required by a bound in this

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/structs/repr/should_require_well_defined_layout/should_require_well_defined_layout.stderr
To only update this specific test, also pass `--test-args transmutability/structs/repr/should_require_well_defined_layout.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/structs/repr/should_require_well_defined_layout" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/structs/repr/should_require_well_defined_layout/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<should_reject_repr_rust::unit::repr_rust, assert::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:21:52
   |
LL |         assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
   |                                                    ^^ the trait `BikeshedIntrinsicFrom<should_reject_repr_rust::unit::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
   |
note: required by a bound in `is_maybe_transmutable`
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:13:14
   |
LL |     pub fn is_maybe_transmutable<Src, Dst>()
   |            --------------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`

error[E0277]: the trait bound `should_reject_repr_rust::unit::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:22:47
   |
LL |         assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
   |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `should_reject_repr_rust::unit::repr_rust`
   |
note: required by a bound in `is_maybe_transmutable`
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:13:14
   |
LL |     pub fn is_maybe_transmutable<Src, Dst>()
   |            --------------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`

error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<should_reject_repr_rust::tuple::repr_rust, assert::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:27:52
   |
LL |         assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
   |                                                    ^^ the trait `BikeshedIntrinsicFrom<should_reject_repr_rust::tuple::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
   |
note: required by a bound in `is_maybe_transmutable`
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:13:14
   |
LL |     pub fn is_maybe_transmutable<Src, Dst>()
   |            --------------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`

error[E0277]: the trait bound `should_reject_repr_rust::tuple::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:28:47
   |
LL |         assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
   |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `should_reject_repr_rust::tuple::repr_rust`
   |
note: required by a bound in `is_maybe_transmutable`
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:13:14
   |
LL |     pub fn is_maybe_transmutable<Src, Dst>()
   |            --------------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`

error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<should_reject_repr_rust::braces::repr_rust, assert::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:33:52
   |
LL |         assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
   |                                                    ^^ the trait `BikeshedIntrinsicFrom<should_reject_repr_rust::braces::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
   |
note: required by a bound in `is_maybe_transmutable`
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:13:14
   |
LL |     pub fn is_maybe_transmutable<Src, Dst>()
   |            --------------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`

error[E0277]: the trait bound `should_reject_repr_rust::braces::repr_rust: BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:34:47
   |
LL |         assert::is_maybe_transmutable::<u128, repr_rust>(); //~ ERROR not satisfied
   |                                               ^^^^^^^^^ the trait `BikeshedIntrinsicFrom<u128, assert::Context, true, true, true, true>` is not implemented for `should_reject_repr_rust::braces::repr_rust`
   |
note: required by a bound in `is_maybe_transmutable`
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:13:14
   |
LL |     pub fn is_maybe_transmutable<Src, Dst>()
   |            --------------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `is_maybe_transmutable`

error[E0277]: the trait bound `(): BikeshedIntrinsicFrom<aligned::repr_rust, assert::Context, true, true, true, true>` is not satisfied
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:39:52
   |
LL |         assert::is_maybe_transmutable::<repr_rust, ()>(); //~ ERROR not satisfied
   |                                                    ^^ the trait `BikeshedIntrinsicFrom<aligned::repr_rust, assert::Context, true, true, true, true>` is not implemented for `()`
   |
note: required by a bound in `is_maybe_transmutable`
  --> /checkout/src/test/ui/transmutability/structs/repr/should_require_well_defined_layout.rs:13:14
   |
LL |     pub fn is_maybe_transmutable<Src, Dst>()
   |            --------------------- required by a bound in this
LL |     where
LL |         Dst: BikeshedIntrinsicFrom<Src, Context, true, true, true, true>
