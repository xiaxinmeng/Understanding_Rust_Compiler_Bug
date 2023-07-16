plain
........................................................................................ 9152/13157
......i................................................................................. 9240/13157
........................................................................................ 9328/13157
........................................................................................ 9416/13157
........F.....F............F............................................................ 9504/13157
........................................................................................ 9680/13157
........................................................................................ 9768/13157
.......ii...............i............................................................ii. 9856/13157
........................................................................................ 9944/13157
---

---- [ui] src/test/ui/empty/empty-struct-tuple-pat.rs stdout ----
diff of stderr:

1 error[E0530]: match bindings cannot shadow tuple structs
3    |
- LL | struct Empty2();
- LL | struct Empty2();
-    | ---------------- the tuple struct `Empty2` is defined here
- ...
7 LL |         Empty2 => ()
-    |         ^^^^^^ cannot be named the same as a tuple struct
+    |         |
+    |         cannot be named the same as a tuple struct
+    |         cannot be named the same as a tuple struct
+    |         help: try specify the pattern arguments: `Empty2(..)`
9 
10 error[E0530]: match bindings cannot shadow tuple structs

12    |
- LL | use empty_struct::*;
- LL | use empty_struct::*;
-    |     --------------- the tuple struct `XEmpty6` is imported here
- ...
16 LL |         XEmpty6 => ()
-    |         ^^^^^^^ cannot be named the same as a tuple struct
+    |         |
+    |         cannot be named the same as a tuple struct
+    |         cannot be named the same as a tuple struct
+    |         help: try specify the pattern arguments: `XEmpty6(..)`
18 
19 error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E::Empty4`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/empty-struct-tuple-pat.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/empty-struct-tuple-pat.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-tuple-pat.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-tuple-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0530]: match bindings cannot shadow tuple structs
   |
   |
LL |         Empty2 => () //~ ERROR match bindings cannot shadow tuple structs
   |         |
   |         cannot be named the same as a tuple struct
   |         cannot be named the same as a tuple struct
   |         help: try specify the pattern arguments: `Empty2(..)`

error[E0530]: match bindings cannot shadow tuple structs
   |
   |
LL |         XEmpty6 => () //~ ERROR match bindings cannot shadow tuple structs
   |         |
   |         cannot be named the same as a tuple struct
   |         cannot be named the same as a tuple struct
   |         help: try specify the pattern arguments: `XEmpty6(..)`

error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E::Empty4`
   |
LL |     Empty4()
LL |     Empty4()
   |     -------- `E::Empty4` defined here
...
LL |         E::Empty4 => ()
   |         ^^^^^^^^^ help: use the tuple variant pattern syntax instead: `E::Empty4()`

error[E0532]: expected unit struct, unit variant or constant, found tuple variant `XE::XEmpty5`
   |
   |
LL |         XE::XEmpty5 => (),
   |
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:7:5
   |
LL |     XEmpty4,
LL |     XEmpty4,
   |     ------- similarly named unit variant `XEmpty4` defined here
LL |     XEmpty5(),
   |     ------- `XE::XEmpty5` defined here
help: use the tuple variant pattern syntax instead
   |
   |
LL |         XE::XEmpty5(/* fields */) => (),
help: a unit variant with a similar name exists
   |
   |
LL |         XE::XEmpty4 => (),

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0530, E0532.
Some errors have detailed explanations: E0530, E0532.
For more information about an error, try `rustc --explain E0530`.
------------------------------------------


---- [ui] src/test/ui/pattern/pattern-binding-disambiguation.rs stdout ----
diff of stderr:

1 error[E0530]: match bindings cannot shadow tuple structs
3    |
- LL | struct TupleStruct();
- LL | struct TupleStruct();
-    | --------------------- the tuple struct `TupleStruct` is defined here
7 LL |         TupleStruct => {}
-    |         ^^^^^^^^^^^ cannot be named the same as a tuple struct
+    |         ^^^^^^^^^^^
+    |         |
+    |         |
+    |         cannot be named the same as a tuple struct
+    |         help: try specify the pattern arguments: `TupleStruct(..)`
9 
10 error[E0530]: match bindings cannot shadow tuple variants

12    |
12    |
- LL | use E::*;
-    |     ---- the tuple variant `TupleVariant` is imported here
16 LL |         TupleVariant => {}
-    |         ^^^^^^^^^^^^ cannot be named the same as a tuple variant
+    |         ^^^^^^^^^^^^
+    |         |
+    |         |
+    |         cannot be named the same as a tuple variant
+    |         help: try specify the pattern arguments: `TupleVariant(..)`
19 error[E0530]: match bindings cannot shadow struct variants
20   --> $DIR/pattern-binding-disambiguation.rs:36:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-binding-disambiguation/pattern-binding-disambiguation.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pattern-binding-disambiguation.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pattern-binding-disambiguation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-binding-disambiguation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pattern-binding-disambiguation/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0530]: match bindings cannot shadow tuple structs
   |
   |
LL |         TupleStruct => {} //~ ERROR match bindings cannot shadow tuple structs
   |         |
   |         cannot be named the same as a tuple struct
   |         cannot be named the same as a tuple struct
   |         help: try specify the pattern arguments: `TupleStruct(..)`

error[E0530]: match bindings cannot shadow tuple variants
   |
   |
LL |         TupleVariant => {} //~ ERROR match bindings cannot shadow tuple variants
   |         |
   |         cannot be named the same as a tuple variant
   |         cannot be named the same as a tuple variant
   |         help: try specify the pattern arguments: `TupleVariant(..)`
error[E0530]: match bindings cannot shadow struct variants
  --> /checkout/src/test/ui/pattern/pattern-binding-disambiguation.rs:36:9
   |
LL | use E::*;
LL | use E::*;
   |     ---- the struct variant `BracedVariant` is imported here
...
LL |         BracedVariant => {} //~ ERROR match bindings cannot shadow struct variants
   |         ^^^^^^^^^^^^^ cannot be named the same as a struct variant
error[E0530]: match bindings cannot shadow statics
  --> /checkout/src/test/ui/pattern/pattern-binding-disambiguation.rs:42:9
   |
   |
LL | static STATIC: () = ();
   | ----------------------- the static `STATIC` is defined here
...
LL |         STATIC => {} //~ ERROR match bindings cannot shadow statics
   |         ^^^^^^ cannot be named the same as a static

error[E0530]: let bindings cannot shadow tuple structs
   |
LL | struct TupleStruct();
LL | struct TupleStruct();
   | --------------------- the tuple struct `TupleStruct` is defined here
...
LL |     let TupleStruct = doesnt_matter; //~ ERROR let bindings cannot shadow tuple structs
   |         ^^^^^^^^^^^ cannot be named the same as a tuple struct

error[E0530]: let bindings cannot shadow tuple variants
   |
LL | use E::*;
LL | use E::*;
   |     ---- the tuple variant `TupleVariant` is imported here
...
LL |     let TupleVariant = doesnt_matter; //~ ERROR let bindings cannot shadow tuple variants
   |         ^^^^^^^^^^^^ cannot be named the same as a tuple variant
error[E0530]: let bindings cannot shadow struct variants
  --> /checkout/src/test/ui/pattern/pattern-binding-disambiguation.rs:53:9
   |
LL | use E::*;
LL | use E::*;
   |     ---- the struct variant `BracedVariant` is imported here
...
LL |     let BracedVariant = doesnt_matter; //~ ERROR let bindings cannot shadow struct variants
   |         ^^^^^^^^^^^^^ cannot be named the same as a struct variant
error[E0530]: let bindings cannot shadow statics
  --> /checkout/src/test/ui/pattern/pattern-binding-disambiguation.rs:55:9
   |
   |
LL | static STATIC: () = ();
   | ----------------------- the static `STATIC` is defined here
...
LL |     let STATIC = doesnt_matter; //~ ERROR let bindings cannot shadow statics
   |         ^^^^^^ cannot be named the same as a static
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0530`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/pattern/pat-tuple-overfield.rs stdout ----
diff of stderr:

1 error[E0530]: match bindings cannot shadow tuple structs
3    |
3    |
- LL | struct Z1();
-    | ------------ the tuple struct `Z1` is defined here
7 LL |         Z1 => {}
-    |         ^^ cannot be named the same as a tuple struct
+    |         ^^
+    |         |
+    |         |
+    |         cannot be named the same as a tuple struct
+    |         help: try specify the pattern arguments: `Z1(..)`
10 error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
11   --> $DIR/pat-tuple-overfield.rs:52:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/pat-tuple-overfield.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-tuple-overfield.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-tuple-overfield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0530]: match bindings cannot shadow tuple structs
   |
   |
LL |         Z1 => {} //~ ERROR match bindings cannot shadow tuple structs
   |         |
   |         cannot be named the same as a tuple struct
   |         cannot be named the same as a tuple struct
   |         help: try specify the pattern arguments: `Z1(..)`
error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:52:9
   |
LL | struct Z0;
LL | struct Z0;
   | ---------- `Z0` defined here
LL | struct Z1();
   | ------------ similarly named tuple struct `Z1` defined here
...
LL |         Z0() => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
   |
help: use this syntax instead
   |
   |
LL |         Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
help: a tuple struct with a similar name exists
   |
   |
LL |         Z1() => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`

error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:53:9
   |
   |
LL | struct Z0;
   | ---------- `Z0` defined here
LL | struct Z1();
   | ------------ similarly named tuple struct `Z1` defined here
...
LL |         Z0(_) => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
   |
help: use this syntax instead
   |
   |
LL |         Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
help: a tuple struct with a similar name exists
   |
   |
LL |         Z1(_) => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`

error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:54:9
   |
   |
LL | struct Z0;
   | ---------- `Z0` defined here
LL | struct Z1();
   | ------------ similarly named tuple struct `Z1` defined here
...
LL |         Z0(_, _) => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
   |
help: use this syntax instead
   |
   |
LL |         Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
help: a tuple struct with a similar name exists
   |
   |
LL |         Z1(_, _) => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`

error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:64:9
   |
   |
LL |     Z0,
   |     -- `E1::Z0` defined here
LL |     Z1(),
   |     ---- similarly named tuple variant `Z1` defined here
...
LL |         E1::Z0() => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
   |
help: use this syntax instead
   |
   |
LL |         E1::Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
help: a tuple variant with a similar name exists
   |
   |
LL |         E1::Z1() => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`

error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:65:9
   |
   |
LL |     Z0,
   |     -- `E1::Z0` defined here
LL |     Z1(),
   |     ---- similarly named tuple variant `Z1` defined here
...
LL |         E1::Z0(_) => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
   |
help: use this syntax instead
   |
   |
LL |         E1::Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
help: a tuple variant with a similar name exists
   |
   |
LL |         E1::Z1(_) => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`

error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:66:9
   |
   |
LL |     Z0,
   |     -- `E1::Z0` defined here
LL |     Z1(),
   |     ---- similarly named tuple variant `Z1` defined here
...
LL |         E1::Z0(_, _) => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
   |
help: use this syntax instead
   |
   |
LL |         E1::Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
help: a tuple variant with a similar name exists
   |
   |
LL |         E1::Z1(_, _) => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`

error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:69:9
   |
   |
LL |     Z0,
   |     -- similarly named unit variant `Z0` defined here
LL |     Z1(),
   |     ---- `E1::Z1` defined here
...
LL |         E1::Z1 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
   |
help: use the tuple variant pattern syntax instead
   |
   |
LL |         E1::Z1() => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
help: a unit variant with a similar name exists
   |
   |
LL |         E1::Z0 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:19:9
   |
   |
LL |     match (1, 2, 3) {
   |           --------- this expression has type `({integer}, {integer}, {integer})`
LL |         (1, 2, 3, 4) => {} //~ ERROR mismatched types
   |         ^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 4 elements
   |
   = note: expected tuple `({integer}, {integer}, {integer})`
              found tuple `(_, _, _, _)`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:20:9
   |
LL |     match (1, 2, 3) {
LL |     match (1, 2, 3) {
   |           --------- this expression has type `({integer}, {integer}, {integer})`
LL |         (1, 2, 3, 4) => {} //~ ERROR mismatched types
LL |         (1, 2, .., 3, 4) => {} //~ ERROR mismatched types
   |         ^^^^^^^^^^^^^^^^ expected a tuple with 3 elements, found one with 4 elements
   |
   = note: expected tuple `({integer}, {integer}, {integer})`
              found tuple `(_, _, _, _)`
error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:24:11
   |
   |
LL | struct S(u8, u8, u8);
   |          --  --  -- tuple struct has 3 fields
...
LL |         S(1, 2, 3, 4) => {}

error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:26:11
   |
   |
LL | struct S(u8, u8, u8);
   |          --  --  -- tuple struct has 3 fields
...
LL |         S(1, 2, .., 3, 4) => {}
   |           ^  ^      ^  ^ expected 3 fields, found 4
error[E0023]: this pattern has 6 fields, but the corresponding tuple struct has 5 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:31:11
   |
LL | struct M(
---
   |     --
LL |     u8,
   |     -- tuple struct has 5 fields
...
LL |         M(1, 2, 3, 4, 5, 6) => {}

error[E0023]: this pattern has 6 fields, but the corresponding tuple struct has 5 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:33:11
   |
---

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 0 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:59:12
   |
LL | struct Z1();
   | --------- tuple struct has 0 fields
...
LL |         Z1(_) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 0 fields

error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 0 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:60:12
   |
   |
LL | struct Z1();
   | --------- tuple struct has 0 fields
...
LL |         Z1(_, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 0 fields

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 0 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:71:16
   |
   |
LL |     Z1(),
   |     -- tuple variant has 0 fields
...
LL |         E1::Z1(_) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 0 fields

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 0 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:72:16
   |
   |
LL |     Z1(),
   |     -- tuple variant has 0 fields
...
LL |         E1::Z1(_, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 0 fields

error: aborting due to 19 previous errors

Some errors have detailed explanations: E0023, E0308, E0530, E0532.
Some errors have detailed explanations: E0023, E0308, E0530, E0532.
For more information about an error, try `rustc --explain E0023`.
------------------------------------------


---- [ui] src/test/ui/pattern/pat-tuple-field-count-cross.rs stdout ----
diff of stderr:

1 error[E0530]: match bindings cannot shadow tuple structs
3    |
3    |
- LL | use declarations_for_tuple_field_count_errors::*;
-    |     -------------------------------------------- the tuple struct `Z1` is imported here
7 LL |         Z1 => {}
-    |         ^^ cannot be named the same as a tuple struct
+    |         ^^
+    |         |
+    |         |
+    |         cannot be named the same as a tuple struct
+    |         help: try specify the pattern arguments: `Z1(..)`
10 error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
11   --> $DIR/pat-tuple-field-count-cross.rs:9:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-field-count-cross/pat-tuple-field-count-cross.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-tuple-field-count-cross.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-field-count-cross" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-field-count-cross/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0530]: match bindings cannot shadow tuple structs
   |
   |
LL |         Z1 => {} //~ ERROR match bindings cannot shadow tuple structs
   |         |
   |         cannot be named the same as a tuple struct
   |         cannot be named the same as a tuple struct
   |         help: try specify the pattern arguments: `Z1(..)`
error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:9:9
   |
   |
LL |         Z0() => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:1:1
   |
LL | pub struct Z0;
LL | pub struct Z0;
   | ------------- `Z0` defined here
LL | pub struct Z1();
   | ------------- similarly named tuple struct `Z1` defined here
help: use this syntax instead
   |
   |
LL |         Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
help: a tuple struct with a similar name exists
   |
   |
LL |         Z1() => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`

error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:10:9
   |
   |
LL |         Z0(x) => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:1:1
   |
LL | pub struct Z0;
LL | pub struct Z0;
   | ------------- `Z0` defined here
LL | pub struct Z1();
   | ------------- similarly named tuple struct `Z1` defined here
help: use this syntax instead
   |
   |
LL |         Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`
help: a tuple struct with a similar name exists
   |
   |
LL |         Z1(x) => {} //~ ERROR expected tuple struct or tuple variant, found unit struct `Z0`

error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:31:9
   |
   |
LL |         E1::Z0() => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:15
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }
   |               --  -- similarly named tuple variant `Z1` defined here
   |               |
   |               `E1::Z0` defined here
help: use this syntax instead
   |
   |
LL |         E1::Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
help: a tuple variant with a similar name exists
   |
   |
LL |         E1::Z1() => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`

error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:32:9
   |
   |
LL |         E1::Z0(x) => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:15
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }
   |               --  -- similarly named tuple variant `Z1` defined here
   |               |
   |               `E1::Z0` defined here
help: use this syntax instead
   |
   |
LL |         E1::Z0 => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`
help: a tuple variant with a similar name exists
   |
   |
LL |         E1::Z1(x) => {} //~ ERROR expected tuple struct or tuple variant, found unit variant `E1::Z0`

error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:35:9
   |
   |
LL |         E1::Z1 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:19
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }
   |               --  -- `E1::Z1` defined here
   |               |
   |               similarly named unit variant `Z0` defined here
help: use the tuple variant pattern syntax instead
   |
   |
LL |         E1::Z1(/* fields */) => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
help: a unit variant with a similar name exists
   |
   |
LL |         E1::Z0 => {} //~ ERROR expected unit struct, unit variant or constant, found tuple variant `E1::Z1`

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 0 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:14:12
   |
   |
LL |         Z1(x) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 0 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:2:1
   |
   |
LL | pub struct Z1();
   | ------------- tuple struct has 0 fields
error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:18:9
   |
   |
LL |         S() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:4:14
   |
   |
LL | pub struct S(pub u8, pub u8, pub u8);
   |              ------  ------  ------ tuple struct has 3 fields
help: use `_` to explicitly ignore each field
   |
   |
LL |         S(_, _, _) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields
help: use `..` to ignore all fields
   |
   |
LL |         S(..) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:19:11
   |
   |
LL |         S(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:4:14
   |
   |
LL | pub struct S(pub u8, pub u8, pub u8);
   |              ------  ------  ------ tuple struct has 3 fields
help: use `_` to explicitly ignore each field
   |
   |
LL |         S(1, _, _) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields
help: use `..` to ignore the rest of the fields
   |
   |
LL |         S(1, ..) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields

error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:20:11
   |
   |
LL |         S(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:4:14
   |
   |
LL | pub struct S(pub u8, pub u8, pub u8);
   |              ------  ------  ------ tuple struct has 3 fields
help: use `_` to explicitly ignore each field
   |
   |
LL |         S(xyz, abc, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 3 fields

error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:21:11
   |
   |
LL |         S(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:4:14
   |
   |
LL | pub struct S(pub u8, pub u8, pub u8);
   |              ------  ------  ------ tuple struct has 3 fields
error[E0023]: this pattern has 0 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:24:9
   |
   |
LL |         M() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:5:12
   |
LL | pub struct M(
---
   |     ------ tuple struct has 3 fields
   |
help: use `_` to explicitly ignore each field
   |
LL |         M(_, _, _) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields
help: use `..` to ignore all fields
   |
   |
LL |         M(..) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple struct has 3 fields

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:25:11
   |
   |
LL |         M(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:5:12
   |
LL | pub struct M(
---
   |     ------ tuple struct has 3 fields
   |
help: use `_` to explicitly ignore each field
   |
LL |         M(1, _, _) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields
help: use `..` to ignore the rest of the fields
   |
   |
LL |         M(1, ..) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 3 fields

error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:26:11
   |
   |
LL |         M(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:5:12
   |
LL | pub struct M(
---
   |     ------ tuple struct has 3 fields
   |
help: use `_` to explicitly ignore each field
   |
LL |         M(xyz, abc, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 3 fields

error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:27:11
   |
   |
LL |         M(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple struct has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:5:12
   |
LL | pub struct M(
---

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 0 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:36:16
   |
LL |         E1::Z1(x) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 0 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:19
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }

error[E0023]: this pattern has 0 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:39:9
   |
   |
LL |         E1::S() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:27
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }
   |
help: use `_` to explicitly ignore each field
   |
   |
LL |         E1::S(_, _, _) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
help: use `..` to ignore all fields
   |
   |
LL |         E1::S(..) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:40:15
   |
   |
LL |         E1::S(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:27
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }
   |
help: use `_` to explicitly ignore each field
   |
   |
LL |         E1::S(1, _, _) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
help: use `..` to ignore the rest of the fields
   |
   |
LL |         E1::S(1, ..) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:41:15
   |
   |
LL |         E1::S(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:27
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }
   |
help: use `_` to explicitly ignore each field
   |
   |
LL |         E1::S(xyz, abc, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 4 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:42:15
   |
   |
LL |         E1::S(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:11:27
   |
   |
LL | pub enum E1 { Z0, Z1(), S(u8, u8, u8) }

error[E0023]: this pattern has 0 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:46:9
   |
   |
LL |         E2::S() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:14:7
   |
   |
LL |     S(u8, u8, u8),
   |
help: use `_` to explicitly ignore each field
   |
   |
LL |         E2::S(_, _, _) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
help: use `..` to ignore all fields
   |
   |
LL |         E2::S(..) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:47:15
   |
   |
LL |         E2::S(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:14:7
   |
   |
LL |     S(u8, u8, u8),
   |
help: use `_` to explicitly ignore each field
   |
   |
LL |         E2::S(1, _, _) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
help: use `..` to ignore the rest of the fields
   |
   |
LL |         E2::S(1, ..) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:48:15
   |
   |
LL |         E2::S(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:14:7
   |
   |
LL |     S(u8, u8, u8),
   |
help: use `_` to explicitly ignore each field
   |
   |
LL |         E2::S(xyz, abc, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 4 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:49:15
   |
   |
LL |         E2::S(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:14:7
   |
   |
LL |     S(u8, u8, u8),

error[E0023]: this pattern has 0 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:52:9
   |
   |
LL |         E2::M() => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:15:5
   |
LL |     M(
---
   |         -- tuple variant has 3 fields
   |
help: use `_` to explicitly ignore each field
   |
LL |         E2::M(_, _, _) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields
help: use `..` to ignore all fields
   |
   |
LL |         E2::M(..) => {} //~ ERROR this pattern has 0 fields, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:53:15
   |
   |
LL |         E2::M(1) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:15:5
   |
LL |     M(
---
   |         -- tuple variant has 3 fields
   |
help: use `_` to explicitly ignore each field
   |
LL |         E2::M(1, _, _) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields
help: use `..` to ignore the rest of the fields
   |
   |
LL |         E2::M(1, ..) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:54:15
   |
   |
LL |         E2::M(xyz, abc) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:15:5
   |
LL |     M(
---
   |         -- tuple variant has 3 fields
   |
help: use `_` to explicitly ignore each field
   |
LL |         E2::M(xyz, abc, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 3 fields

error[E0023]: this pattern has 4 fields, but the corresponding tuple variant has 3 fields
  --> /checkout/src/test/ui/pattern/pat-tuple-field-count-cross.rs:55:15
   |
   |
LL |         E2::M(1, 2, 3, 4) => {} //~ ERROR this pattern has 4 fields, but the corresponding tuple variant has 3 fields
   |
  ::: /checkout/src/test/ui/pattern/auxiliary/declarations-for-tuple-field-count-errors.rs:15:5
   |
LL |     M(
