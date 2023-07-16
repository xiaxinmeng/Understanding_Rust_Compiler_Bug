plain

---- [ui] ui/pattern/pat-tuple-overfield.rs stdout ----
diff of stderr:

1 error[E0530]: match bindings cannot shadow tuple structs
-   --> $DIR/pat-tuple-overfield.rs:43:9
3    |
3    |
4 LL | struct Z1();
5    | ------------ the tuple struct `Z1` is defined here
8    |         ^^ cannot be named the same as a tuple struct
9 
10 error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
-   --> $DIR/pat-tuple-overfield.rs:38:9
-   --> $DIR/pat-tuple-overfield.rs:38:9
+   --> $DIR/pat-tuple-overfield.rs:36:9
12    |
13 LL | struct Z0;
14    | ---------- `Z0` defined here
28    |         ~~
29 
30 error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
-   --> $DIR/pat-tuple-overfield.rs:39:9
-   --> $DIR/pat-tuple-overfield.rs:39:9
+   --> $DIR/pat-tuple-overfield.rs:37:9
32    |
33 LL | struct Z0;
34    | ---------- `Z0` defined here
48    |         ~~
49 
50 error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
-   --> $DIR/pat-tuple-overfield.rs:40:9
-   --> $DIR/pat-tuple-overfield.rs:40:9
+   --> $DIR/pat-tuple-overfield.rs:38:9
52    |
53 LL | struct Z0;
54    | ---------- `Z0` defined here
68    |         ~~
69 
70 error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
-   --> $DIR/pat-tuple-overfield.rs:50:9
-   --> $DIR/pat-tuple-overfield.rs:50:9
+   --> $DIR/pat-tuple-overfield.rs:48:9
72    |
73 LL |     Z0,
74    |     -- `E1::Z0` defined here
88    |             ~~
89 
90 error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
-   --> $DIR/pat-tuple-overfield.rs:51:9
-   --> $DIR/pat-tuple-overfield.rs:51:9
+   --> $DIR/pat-tuple-overfield.rs:49:9
92    |
93 LL |     Z0,
94    |     -- `E1::Z0` defined here
108    |             ~~
109 
110 error[E0532]: expected tuple struct or tuple variant, found unit variant `E1::Z0`
-   --> $DIR/pat-tuple-overfield.rs:52:9
-   --> $DIR/pat-tuple-overfield.rs:52:9
+   --> $DIR/pat-tuple-overfield.rs:50:9
112    |
113 LL |     Z0,
114    |     -- `E1::Z0` defined here
128    |             ~~
129 
130 error[E0532]: expected unit struct, unit variant or constant, found tuple variant `E1::Z1`
-   --> $DIR/pat-tuple-overfield.rs:55:9
-   --> $DIR/pat-tuple-overfield.rs:55:9
+   --> $DIR/pat-tuple-overfield.rs:53:9
132    |
133 LL |     Z0,
134    |     -- similarly named unit variant `Z0` defined here
148    |             ~~
149 
150 error[E0308]: mismatched types
-   --> $DIR/pat-tuple-overfield.rs:21:9
-   --> $DIR/pat-tuple-overfield.rs:21:9
+   --> $DIR/pat-tuple-overfield.rs:19:9
152    |
153 LL |     match (1, 2, 3) {
154    |           --------- this expression has type `({integer}, {integer}, {integer})`

159               found tuple `(_, _, _, _)`
161 error[E0308]: mismatched types
-   --> $DIR/pat-tuple-overfield.rs:22:9
+   --> $DIR/pat-tuple-overfield.rs:20:9
163    |
163    |
164 LL |     match (1, 2, 3) {
165    |           --------- this expression has type `({integer}, {integer}, {integer})`

171               found tuple `(_, _, _, _)`
172 
173 error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
-   --> $DIR/pat-tuple-overfield.rs:26:11
175    |
175    |
176 LL | struct S(u8, u8, u8);
177    |          --  --  -- tuple struct has 3 fields

180    |         - ^  ^  ^  ^ expected 3 fields, found 4
181 
182 error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
-   --> $DIR/pat-tuple-overfield.rs:28:11
184    |
184    |
185 LL | struct S(u8, u8, u8);
186    |          --  --  -- tuple struct has 3 fields

189    |         - ^  ^      ^  ^ expected 3 fields, found 4
190 
191 error[E0023]: this pattern has 6 fields, but the corresponding tuple struct has 5 fields
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/pat-tuple-overfield.rs:33:11
193    |
194 LL | struct M(
195    |        - tuple struct defined here


208    |         - ^  ^  ^  ^  ^  ^ expected 5 fields, found 6
209 
210 error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 0 fields
-   --> $DIR/pat-tuple-overfield.rs:45:12
212    |
212    |
213 LL | struct Z1();
214    |          --- tuple struct has 0 fields

217    |         -- ^ expected 0 fields, found 1
218 
219 error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 0 fields
-   --> $DIR/pat-tuple-overfield.rs:46:12
221    |
221    |
222 LL | struct Z1();
223    |          --- tuple struct has 0 fields

226    |         -- ^  ^ expected 0 fields, found 2
227 
228 error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 0 fields
-   --> $DIR/pat-tuple-overfield.rs:57:16
230    |
231 LL |     Z1(),
231 LL |     Z1(),
232    |       -- tuple variant has 0 fields

235    |         ------ ^ expected 0 fields, found 1
236 
237 error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 0 fields
-   --> $DIR/pat-tuple-overfield.rs:58:16
239    |
240 LL |     Z1(),
240 LL |     Z1(),
241    |       -- tuple variant has 0 fields

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/pat-tuple-overfield.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/pat-tuple-overfield.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-tuple-overfield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-tuple-overfield/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0530]: match bindings cannot shadow tuple structs
   |
   |
LL | struct Z1();
   | ------------ the tuple struct `Z1` is defined here
...
LL |         Z1 => {} //~ ERROR match bindings cannot shadow tuple structs
   |         ^^ cannot be named the same as a tuple struct
error[E0532]: expected tuple struct or tuple variant, found unit struct `Z0`
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:36:9
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
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:37:9
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
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:38:9
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
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:48:9
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
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:49:9
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
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:50:9
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
  --> /checkout/src/test/ui/pattern/pat-tuple-overfield.rs:53:9
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
   |
   |
LL | struct S(u8, u8, u8);
   |          --  --  -- tuple struct has 3 fields
...
LL |         S(1, 2, 3, 4) => {}
   |         - ^  ^  ^  ^ expected 3 fields, found 4

error[E0023]: this pattern has 4 fields, but the corresponding tuple struct has 3 fields
   |
   |
LL | struct S(u8, u8, u8);
   |          --  --  -- tuple struct has 3 fields
...
LL |         S(1, 2, .., 3, 4) => {}
   |         - ^  ^      ^  ^ expected 3 fields, found 4

error[E0023]: this pattern has 6 fields, but the corresponding tuple struct has 5 fields
   |
LL | struct M(
   |        - tuple struct defined here
LL |     u8,
LL |     u8,
   |     --
LL |     u8,
   |     --
LL |     u8,
   |     --
LL |     u8,
   |     --
LL |     u8,
   |     -- tuple struct has 5 fields
...
LL |         M(1, 2, 3, 4, 5, 6) => {}
   |         - ^  ^  ^  ^  ^  ^ expected 5 fields, found 6

error[E0023]: this pattern has 1 field, but the corresponding tuple struct has 0 fields
   |
   |
LL | struct Z1();
   |          --- tuple struct has 0 fields
...
LL |         Z1(_) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple struct has 0 fields
   |         -- ^ expected 0 fields, found 1

error[E0023]: this pattern has 2 fields, but the corresponding tuple struct has 0 fields
   |
   |
LL | struct Z1();
   |          --- tuple struct has 0 fields
...
LL |         Z1(_, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple struct has 0 fields
   |         -- ^  ^ expected 0 fields, found 2

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 0 fields
   |
LL |     Z1(),
LL |     Z1(),
   |       -- tuple variant has 0 fields
...
LL |         E1::Z1(_) => {} //~ ERROR this pattern has 1 field, but the corresponding tuple variant has 0 fields
   |         ------ ^ expected 0 fields, found 1

error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 0 fields
   |
LL |     Z1(),
LL |     Z1(),
   |       -- tuple variant has 0 fields
...
LL |         E1::Z1(_, _) => {} //~ ERROR this pattern has 2 fields, but the corresponding tuple variant has 0 fields
   |         ------ ^  ^ expected 0 fields, found 2
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0023, E0308, E0530, E0532.
For more information about an error, try `rustc --explain E0023`.
---
To only update this specific test, also pass `--test-args type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent.rs:21:5
   |
LL |     <E>::V(); //~ ERROR this function takes 1 argument but 0 arguments were supplied
   |     ^^^^^^-- supplied 0 arguments
   |     expected 1 argument
   |
note: tuple variant defined here
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent.rs:5:5
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent.rs:5:5
   |
LL |     V(u8)
   |     ^

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type-alias-enum-variants/enum-variant-priority-higher-than-other-inherent.rs:22:17
   |
LL |     let _: u8 = <E2>::V; //~ ERROR mismatched types
   |            --   ^^^^^^^ expected `u8`, found enum `E2`
   |            expected due to this

error: aborting due to 2 previous errors

---
+    |
+ note: tuple struct defined here
+   --> $DIR/struct-enum-wrong-args.rs:2:8
+    |
+ LL | struct Wrapper(i32);
32 
33 error[E0061]: this struct takes 1 argument but 2 arguments were supplied
34   --> $DIR/struct-enum-wrong-args.rs:10:13


37    |             ^^^^^^^ -  - supplied 2 arguments
38    |             |
39    |             expected 1 argument
+    |
+ note: tuple struct defined here
+   --> $DIR/struct-enum-wrong-args.rs:2:8
+    |
+ LL | struct Wrapper(i32);
40 
41 error[E0061]: this struct takes 2 arguments but 0 arguments were supplied
42   --> $DIR/struct-enum-wrong-args.rs:11:13


45    |             ^^^^^^^^^^^^^-- supplied 0 arguments
46    |             |
47    |             expected 2 arguments
+    |
+ note: tuple struct defined here
+   --> $DIR/struct-enum-wrong-args.rs:3:8
+    |
+ LL | struct DoubleWrapper(i32, i32);
48 
49 error[E0061]: this struct takes 2 arguments but 1 argument was supplied
50   --> $DIR/struct-enum-wrong-args.rs:12:13


53    |             ^^^^^^^^^^^^^ - supplied 1 argument
54    |             |
55    |             expected 2 arguments
+    |
+ note: tuple struct defined here
+   --> $DIR/struct-enum-wrong-args.rs:3:8
+    |
+ LL | struct DoubleWrapper(i32, i32);
56 
57 error[E0061]: this struct takes 2 arguments but 3 arguments were supplied
58   --> $DIR/struct-enum-wrong-args.rs:13:13


61    |             ^^^^^^^^^^^^^ -  -  - supplied 3 arguments
62    |             |
63    |             expected 2 arguments
+    |
+ note: tuple struct defined here
+   --> $DIR/struct-enum-wrong-args.rs:3:8
+    |
+ LL | struct DoubleWrapper(i32, i32);
64 
65 error: aborting due to 8 previous errors
66 

---
To only update this specific test, also pass `--test-args typeck/struct-enum-wrong-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/struct-enum-wrong-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/struct-enum-wrong-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/struct-enum-wrong-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0061]: this enum variant takes 1 argument but 2 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:6:13
   |
LL |     let _ = Some(3, 2); //~ ERROR this enum variant takes
   |             ^^^^ -  - supplied 2 arguments
   |             expected 1 argument

error[E0061]: this enum variant takes 1 argument but 3 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:7:13
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:7:13
   |
LL |     let _ = Ok(3, 6, 2); //~ ERROR this enum variant takes
   |             ^^ -  -  - supplied 3 arguments
   |             expected 1 argument

error[E0061]: this enum variant takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:8:13
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:8:13
   |
LL |     let _ = Ok(); //~ ERROR this enum variant takes
   |             ^^-- supplied 0 arguments
   |             expected 1 argument

error[E0061]: this struct takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:9:13
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:9:13
   |
LL |     let _ = Wrapper(); //~ ERROR this struct takes
   |             ^^^^^^^-- supplied 0 arguments
   |             expected 1 argument
   |
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:2:8
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:2:8
   |
LL | struct Wrapper(i32);

error[E0061]: this struct takes 1 argument but 2 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:10:13
   |
   |
LL |     let _ = Wrapper(5, 2); //~ ERROR this struct takes
   |             ^^^^^^^ -  - supplied 2 arguments
   |             expected 1 argument
   |
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:2:8
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:2:8
   |
LL | struct Wrapper(i32);

error[E0061]: this struct takes 2 arguments but 0 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:11:13
   |
   |
LL |     let _ = DoubleWrapper(); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^-- supplied 0 arguments
   |             expected 2 arguments
   |
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
   |
LL | struct DoubleWrapper(i32, i32);

error[E0061]: this struct takes 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:12:13
   |
   |
LL |     let _ = DoubleWrapper(5); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^ - supplied 1 argument
   |             expected 2 arguments
   |
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
   |
LL | struct DoubleWrapper(i32, i32);

error[E0061]: this struct takes 2 arguments but 3 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:13:13
   |
   |
LL |     let _ = DoubleWrapper(5, 2, 7); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^ -  -  - supplied 3 arguments
   |             expected 2 arguments
   |
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
   |
LL | struct DoubleWrapper(i32, i32);

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0061`.
---
test result: FAILED. 12056 passed; 3 failed; 102 ignored; 0 measured; 0 filtered out; finished in 123.17s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:20
