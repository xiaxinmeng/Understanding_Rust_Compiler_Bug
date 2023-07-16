plain
................i.i.................ii...i............F................................. 264/13330
........................................................................................ 352/13330
........................................................................................ 440/13330
........................................................................................ 528/13330
........................................F......F........................................ 616/13330
............................................................i........................... 792/13330
........................................................i............................... 880/13330
........................................................................................ 968/13330
..........FF............................................................................ 1056/13330
..........FF............................................................................ 1056/13330
........................................................................................ 1144/13330
........................................................................................ 1232/13330
..............................................i......................................... 1320/13330
......................................F...........................F..................... 1408/13330
........................................................................................ 1584/13330
........................................................................F............... 1672/13330
.....F.............................................i......ii............................ 1760/13330
........................................................................................ 1848/13330
---
........................................................................................ 3432/13330
........................................................................................ 3520/13330
........................................................................................ 3608/13330
........................................................................................ 3696/13330
.............................F.......................................................... 3784/13330
..................................F...........i..........i..........i.........F......... 3872/13330
............................................i........................................... 4048/13330
....................................................F................................... 4136/13330
.i...................................................................................... 4224/13330
.....................................................................F.................. 4312/13330
.....................................................................F.................. 4312/13330
........................................................................................ 4400/13330
........................................................................................ 4488/13330
........................................................................................ 4576/13330
...................................................................................F.... 4664/13330
.........................F............F................................................. 4752/13330
........................................................................................ 4928/13330
........................................................................................ 5016/13330
........................................................................................ 5104/13330
.......................................i..........................F..................... 5192/13330
---
........................................................................................ 6160/13330
........................................................................................ 6248/13330
........................................................................................ 6336/13330
.......i................................................................................ 6424/13330
.....F.................................................................................. 6512/13330
...i..........F..........F..........................F................................... 6600/13330
F...................................................................i.................F. 6688/13330
..............................i......................................................... 6864/13330
.......................................................................................i 6952/13330
.......................................................................................i 6952/13330
....iF........................................i..................i.............i........ 7040/13330
..................................................................i..................... 7216/13330
........................................................F............................... 7304/13330
........................................................................................ 7392/13330
......................................................F.....ii.......................... 7480/13330
---
...i.................................................................................... 9328/13330
........................................................................................ 9416/13330
........................................................................................ 9504/13330
........................................................................................ 9592/13330
......................................................F.F............................... 9680/13330
....................F......F...........F................................................ 9768/13330
......ii...............i.............F..............................................ii.. 9944/13330
.........................F.............................................................. 10032/13330
........................................................................................ 10120/13330
........................................................................................ 10208/13330
........................................................................................ 10208/13330
......................................................F................................. 10296/13330
..................................................................F..................... 10384/13330
........................................................................................ 10472/13330
........................................................................................ 10560/13330
...............................i..i.i................................................... 10648/13330
................F...F......................................F..............i............. 10736/13330
.iii..iiiiii.i.......................................................................... 10912/13330
........................................................................................ 11000/13330
........................................................................................ 11088/13330
........................................................................................ 11176/13330
........................................................................................ 11176/13330
........................................................................................ 11264/13330
.........................F.............................................................. 11352/13330
........................................................................................ 11440/13330
..........................................................F............................. 11528/13330
...F..............F..................................................................... 11616/13330
...........................................i........i........i.....i.................... 11792/13330
.i...................................................................................... 11880/13330
........................................................................................ 11968/13330
........................................................................................ 12056/13330
........................................................................................ 12056/13330
........................................................................................ 12144/13330
........................................................................................ 12232/13330
....................................................................................F... 12320/13330
........................................................................................ 12408/13330
...................................................F...............i.................... 12496/13330
.............FF......................................................................... 12584/13330
..................................................F...................................FF 12672/13330
...F.F.................F................................................................ 12760/13330
....................................................F.FF................................ 12848/13330
........................................................................................ 13024/13330
........................................................................................ 13112/13330
........................................................................................ 13200/13330
.......................iii.............................................................. 13288/13330
.......................iii.............................................................. 13288/13330
........................F.................
failures:

---- [ui] src/test/ui/asm/x86_64/type-check-2.rs stdout ----
diff of stderr:

63    |
64    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
65 
- error: cannot use value of type `fn() {main}` for inline assembly
+ error: cannot use value of type `fn() {{main}}` for inline assembly
68    |
68    |
69 LL |         asm!("{}", inout(reg) f);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2/type-check-2.stderr
To only update this specific test, also pass `--test-args asm/x86_64/type-check-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/type-check-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid `sym` operand
   |
   |
LL |         asm!("{}", sym x);
   |                        ^ is a local variable
   |
   = help: `sym` operands must refer to either a function or a static

error: invalid `sym` operand
   |
   |
LL | global_asm!("{}", sym C);
   |                   ^^^^^ is an `i32`
   |
   = help: `sym` operands must refer to either a function or a static

error: invalid `sym` operand
   |
   |
LL |         asm!("{}", sym C);
   |                    ^^^^^ is an `i32`
   |
   = help: `sym` operands must refer to either a function or a static
error: arguments for inline assembly must be copyable
  --> /checkout/src/test/ui/asm/x86_64/type-check-2.rs:40:32
   |
   |
LL |         asm!("{}", in(xmm_reg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
   |
   |
   = note: `SimdNonCopy` does not implement the Copy trait

error: cannot use value of type `[closure@/checkout/src/test/ui/asm/x86_64/type-check-2.rs:52:28: 52:36]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) |x: i32| x);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `Vec<i32>` for inline assembly
  --> /checkout/src/test/ui/asm/x86_64/type-check-2.rs:54:28
   |
   |
LL |         asm!("{}", in(reg) vec![0]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot use value of type `(i32, i32, i32)` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) (1, 2, 3));
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `[i32; 3]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) [1, 2, 3]);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `fn() {{main}}` for inline assembly
   |
   |
LL |         asm!("{}", inout(reg) f);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `&mut i32` for inline assembly
  --> /checkout/src/test/ui/asm/x86_64/type-check-2.rs:69:31
   |
   |
LL |         asm!("{}", inout(reg) r);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: aborting due to 10 previous errors
------------------------------------------



---- [ui] src/test/ui/associated-types/substs-ppaux.rs#verbose stdout ----
diff of stderr:

2   --> $DIR/substs-ppaux.rs:16:17
3    |
4 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>} defined here
+    |     --------------------------- fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}} defined here
6 ...
7 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
8    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
10    |            expected due to this
11    |
12    = note: expected unit type `()`
12    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}`
+                 found fn item `fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}}`
14 help: use parentheses to call this function
15    |
16 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();
20   --> $DIR/substs-ppaux.rs:25:17
21    |
21    |
22 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
+    |     --------------------------- fn() {{<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}} defined here
24 ...
25 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
26    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
28    |            expected due to this
29    |
30    = note: expected unit type `()`
30    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}`
+                 found fn item `fn() {{<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}}`
32 help: use parentheses to call this function
33    |
34 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();
38   --> $DIR/substs-ppaux.rs:33:17
39    |
39    |
40 LL |     fn baz() {}
-    |     -------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz} defined here
+    |     -------- fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::baz}} defined here
42 ...
43 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
44    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
46    |            expected due to this
47    |
48    = note: expected unit type `()`
48    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz}`
+                 found fn item `fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::baz}}`
50 help: use parentheses to call this function
51    |
52 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();
56   --> $DIR/substs-ppaux.rs:41:17
57    |
57    |
58 LL | fn foo<'z>() where &'z (): Sized {
-    | -------------------------------- fn() {foo::<ReStatic>} defined here
+    | -------------------------------- fn() {{foo::<ReStatic>}} defined here
60 ...
61 LL |     let x: () = foo::<'static>;
62    |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
64    |            expected due to this
65    |
66    = note: expected unit type `()`
66    = note: expected unit type `()`
-                 found fn item `fn() {foo::<ReStatic>}`
+                 found fn item `fn() {{foo::<ReStatic>}}`
68 help: use parentheses to call this function
69    |
70 LL |     let x: () = foo::<'static>();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/substs-ppaux.verbose.stderr
To only update this specific test, also pass `--test-args associated-types/substs-ppaux.rs`


error in revision `verbose`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "verbose" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:16:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:25:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- fn() {{<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:33:17
   |
   |
LL |     fn baz() {}
   |     -------- fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::baz}} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{<i8 as Foo<ReStatic, ReStatic, u8>>::baz}}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:41:17
   |
   |
LL | fn foo<'z>() where &'z (): Sized {
   | -------------------------------- fn() {{foo::<ReStatic>}} defined here
...
LL |     let x: () = foo::<'static>;
   |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{foo::<ReStatic>}}`
help: use parentheses to call this function
   |
LL |     let x: () = foo::<'static>();

error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:49:5
   |
   |
LL |     <str as Foo<u8>>::bar;
   |
   = help: the trait `Sized` is not implemented for `str`
   = help: the trait `Sized` is not implemented for `str`
note: required because of the requirements on the impl of `Foo<'_#0r, '_#1r, u8>` for `str`
   |
   |
LL | impl<'a,'b,T,S> Foo<'a, 'b, S> for T {}

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0308.
---
diff of stderr:

2   --> $DIR/substs-ppaux.rs:16:17
3    |
4 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>} defined here
+    |     --------------------------- fn() {{<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}} defined here
6 ...
7 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
8    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
10    |            expected due to this
11    |
12    = note: expected unit type `()`
12    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}`
+                 found fn item `fn() {{<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}}`
14 help: use parentheses to call this function
15    |
16 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();
20   --> $DIR/substs-ppaux.rs:25:17
21    |
21    |
22 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<'static, 'static>>::bar::<'static, char>} defined here
+    |     --------------------------- fn() {{<i8 as Foo<'static, 'static>>::bar::<'static, char>}} defined here
24 ...
25 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
26    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
28    |            expected due to this
29    |
30    = note: expected unit type `()`
30    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<'static, 'static>>::bar::<'static, char>}`
+                 found fn item `fn() {{<i8 as Foo<'static, 'static>>::bar::<'static, char>}}`
32 help: use parentheses to call this function
33    |
34 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();
38   --> $DIR/substs-ppaux.rs:33:17
39    |
39    |
40 LL |     fn baz() {}
-    |     -------- fn() {<i8 as Foo<'static, 'static, u8>>::baz} defined here
+    |     -------- fn() {{<i8 as Foo<'static, 'static, u8>>::baz}} defined here
42 ...
43 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
44    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
46    |            expected due to this
47    |
48    = note: expected unit type `()`
48    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<'static, 'static, u8>>::baz}`
+                 found fn item `fn() {{<i8 as Foo<'static, 'static, u8>>::baz}}`
50 help: use parentheses to call this function
51    |
52 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();
56   --> $DIR/substs-ppaux.rs:41:17
57    |
57    |
58 LL | fn foo<'z>() where &'z (): Sized {
-    | -------------------------------- fn() {foo::<'static>} defined here
+    | -------------------------------- fn() {{foo::<'static>}} defined here
60 ...
61 LL |     let x: () = foo::<'static>;
62    |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
64    |            expected due to this
65    |
66    = note: expected unit type `()`
66    = note: expected unit type `()`
-                 found fn item `fn() {foo::<'static>}`
+                 found fn item `fn() {{foo::<'static>}}`
68 help: use parentheses to call this function
69    |
70 LL |     let x: () = foo::<'static>();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal/substs-ppaux.normal.stderr
To only update this specific test, also pass `--test-args associated-types/substs-ppaux.rs`


error in revision `normal`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "normal" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:16:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- fn() {{<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:25:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- fn() {{<i8 as Foo<'static, 'static>>::bar::<'static, char>}} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{<i8 as Foo<'static, 'static>>::bar::<'static, char>}}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:33:17
   |
   |
LL |     fn baz() {}
   |     -------- fn() {{<i8 as Foo<'static, 'static, u8>>::baz}} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{<i8 as Foo<'static, 'static, u8>>::baz}}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:41:17
   |
   |
LL | fn foo<'z>() where &'z (): Sized {
   | -------------------------------- fn() {{foo::<'static>}} defined here
...
LL |     let x: () = foo::<'static>;
   |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `fn() {{foo::<'static>}}`
help: use parentheses to call this function
   |
LL |     let x: () = foo::<'static>();

error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:49:5
   |
   |
LL |     <str as Foo<u8>>::bar;
   |
   = help: the trait `Sized` is not implemented for `str`
   = help: the trait `Sized` is not implemented for `str`
note: required because of the requirements on the impl of `Foo<'_, '_, u8>` for `str`
   |
   |
LL | impl<'a,'b,T,S> Foo<'a, 'b, S> for T {}

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0277, E0308.
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/binop/issue-77910-2.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {foo}`
+ error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {{foo}}`
3    |
3    |
4 LL |     if foo == y {}
5    |        --- ^^ - _
6    |        |
6    |        |
-    |        for<'r> fn(&'r i32) -> &'r i32 {foo}
+    |        for<'r> fn(&'r i32) -> &'r i32 {{foo}}
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-2/issue-77910-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/issue-77910-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {{foo}}`
   |
   |
LL |     if foo == y {}
   |        --- ^^ - _
   |        |
   |        for<'r> fn(&'r i32) -> &'r i32 {{foo}}
error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/binop/issue-77910-1.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {foo}`
+ error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {{foo}}`
3    |
3    |
4 LL |     assert_eq!(foo, y);
5    |     ^^^^^^^^^^^^^^^^^^
6    |     |
6    |     |
-    |     for<'r> fn(&'r i32) -> &'r i32 {foo}
+    |     for<'r> fn(&'r i32) -> &'r i32 {{foo}}
9    |
9    |
10    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
11 
- error[E0277]: `for<'r> fn(&'r i32) -> &'r i32 {foo}` doesn't implement `Debug`
+ error[E0277]: `for<'r> fn(&'r i32) -> &'r i32 {{foo}}` doesn't implement `Debug`
14    |
14    |
15 LL | fn foo(s: &i32) -> &i32 {
16    |    --- consider calling this function
17 ...
17 ...
18 LL |     assert_eq!(foo, y);
-    |     ^^^^^^^^^^^^^^^^^^ `for<'r> fn(&'r i32) -> &'r i32 {foo}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^ `for<'r> fn(&'r i32) -> &'r i32 {{foo}}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
20    |
-    = help: the trait `Debug` is not implemented for `for<'r> fn(&'r i32) -> &'r i32 {foo}`
+    = help: the trait `Debug` is not implemented for `for<'r> fn(&'r i32) -> &'r i32 {{foo}}`
22    = help: use parentheses to call the function: `foo(s)`
23    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/issue-77910-1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/issue-77910-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/issue-77910-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {{foo}}`
   |
   |
LL |     assert_eq!(foo, y);
   |     |
   |     |
   |     for<'r> fn(&'r i32) -> &'r i32 {{foo}}
   |
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `for<'r> fn(&'r i32) -> &'r i32 {{foo}}` doesn't implement `Debug`
   |
   |
LL | fn foo(s: &i32) -> &i32 {
...
...
LL |     assert_eq!(foo, y);
   |     ^^^^^^^^^^^^^^^^^^ `for<'r> fn(&'r i32) -> &'r i32 {{foo}}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `for<'r> fn(&'r i32) -> &'r i32 {{foo}}`
   = help: use parentheses to call the function: `foo(s)`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/c-variadic/issue-32201.rs stdout ----
diff of stderr:

- error[E0617]: can't pass `fn(*const u8) {bar}` to variadic function
+ error[E0617]: can't pass `fn(*const u8) {{bar}}` to variadic function
3    |
3    |
4 LL |         foo(0, bar);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/issue-32201/issue-32201.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args c-variadic/issue-32201.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/issue-32201.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/issue-32201" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/issue-32201/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0617]: can't pass `fn(*const u8) {{bar}}` to variadic function
   |
   |
LL |         foo(0, bar);
   |                ^^^ help: cast the value to `fn(*const u8)`: `bar as fn(*const u8)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0617`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/cast/cast-to-bare-fn.rs stdout ----
diff of stderr:

- error[E0605]: non-primitive cast: `fn(isize) {foo}` as `extern "C" fn() -> isize`
+ error[E0605]: non-primitive cast: `fn(isize) {{foo}}` as `extern "C" fn() -> isize`
3    |
4 LL |     let x = foo as extern "C" fn() -> isize;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-to-bare-fn/cast-to-bare-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cast/cast-to-bare-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cast/cast-to-bare-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-to-bare-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-to-bare-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0605]: non-primitive cast: `fn(isize) {{foo}}` as `extern "C" fn() -> isize`
   |
LL |     let x = foo as extern "C" fn() -> isize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast


error[E0605]: non-primitive cast: `u64` as `fn(isize) -> (isize, isize)`
   |
   |
LL |     let y = v as extern "Rust" fn(isize) -> (isize, isize);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0605`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/closures/coerce-unsafe-to-closure.rs stdout ----
diff of stderr:

- error[E0277]: expected a `FnOnce<(&str,)>` closure, found `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
+ error[E0277]: expected a `FnOnce<(&str,)>` closure, found `unsafe extern "rust-intrinsic" fn(_) -> _ {{transmute::<_, _>}}`
2   --> $DIR/coerce-unsafe-to-closure.rs:2:44
3    |
4 LL |     let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);
6    |                                        |
7    |                                        required by a bound introduced by this call
8    |
8    |
-    = help: the trait `FnOnce<(&str,)>` is not implemented for `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
+    = help: the trait `FnOnce<(&str,)>` is not implemented for `unsafe extern "rust-intrinsic" fn(_) -> _ {{transmute::<_, _>}}`
10    = note: unsafe function cannot be called generically without an unsafe block
11 note: required by a bound in `Option::<T>::map`
12   --> $SRC_DIR/core/src/option.rs:LL:COL

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure/coerce-unsafe-to-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/coerce-unsafe-to-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/coerce-unsafe-to-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `FnOnce<(&str,)>` closure, found `unsafe extern "rust-intrinsic" fn(_) -> _ {{transmute::<_, _>}}`
   |
   |
LL |     let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);
   |                                        --- ^^^^^^^^^^^^^^^^^^^ call the function in a closure: `|| unsafe { /* code */ }`
   |                                        required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<(&str,)>` is not implemented for `unsafe extern "rust-intrinsic" fn(_) -> _ {{transmute::<_, _>}}`
   = note: unsafe function cannot be called generically without an unsafe block
note: required by a bound in `Option::<T>::map`
   |
   |
LL |         F: ~const FnOnce(T) -> U,
   |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/closures/closure_cap_coerce_many_fail.rs stdout ----
diff of stderr:

4 LL |       let _ = match "+" {
5    |  _____________-
6 LL | |         "+" => add,
-    | |                --- this is found to be of type `fn(i32, i32) -> i32 {add}`
+    | |                --- this is found to be of type `fn(i32, i32) -> i32 {{add}}`
8 LL | |         "-" => |a, b| (a - b + cap) as i32,
9    | |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn item, found closure
10 LL | |         _ => unimplemented!(),
11 LL | |     };
12    | |_____- `match` arms have incompatible types
13    |
13    |
-    = note: expected fn item `fn(i32, i32) -> i32 {add}`
+    = note: expected fn item `fn(i32, i32) -> i32 {{add}}`
15               found closure `[closure@$DIR/closure_cap_coerce_many_fail.rs:9:16: 9:22]`
17 error[E0308]: `match` arms have incompatible types


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure_cap_coerce_many_fail/closure_cap_coerce_many_fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/closure_cap_coerce_many_fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure_cap_coerce_many_fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure_cap_coerce_many_fail/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:9:16
   |
   |
LL |       let _ = match "+" {
   |  _____________-
LL | |         "+" => add,
   | |                --- this is found to be of type `fn(i32, i32) -> i32 {{add}}`
LL | |         "-" => |a, b| (a - b + cap) as i32,
   | |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn item, found closure
LL | |         _ => unimplemented!(),
LL | |     };
   |
   |
   = note: expected fn item `fn(i32, i32) -> i32 {{add}}`
              found closure `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:9:16: 9:22]`
error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:18:16
   |
   |
LL |       let _ = match "+" {
   |  _____________-
LL | |         "+" => |a, b| (a + b) as i32,
   | |                |
   | |                the expected closure
   | |                the expected closure
   | |                this is found to be of type `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:17:16: 17:22]`
LL | |         "-" => |a, b| (a - b + cap) as i32,
   | |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected closure, found a different closure
LL | |         _ => unimplemented!(),
LL | |     };
   |
   |
   = note: expected closure `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:17:16: 17:22]`
              found closure `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:18:16: 18:22]`
   = note: no two closures, even if identical, have the same type
   = help: consider boxing your closure and/or using it as a trait object
error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:27:16
   |
   |
LL |       let _ = match "+" {
   |  _____________-
LL | |         "+" => |a, b| (a + b + cap) as i32,
   | |                |
   | |                the expected closure
   | |                the expected closure
   | |                this is found to be of type `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:26:16: 26:22]`
LL | |         "-" => |a, b| (a - b) as i32,
   | |                ^^^^^^^^^^^^^^^^^^^^^ expected closure, found a different closure
LL | |         _ => unimplemented!(),
---

5    |        ^ expected `bool`, found fn item
6    |
7    = note: expected type `bool`
-            found fn item `fn() {f}`
+            found fn item `fn() {{f}}`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/if-typeck/if-typeck.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args expr/if/if-typeck.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/expr/if/if-typeck.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/if-typeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/if/if-typeck/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/expr/if/if-typeck.rs:9:8
   |
   |
LL |     if f { }
   |        ^ expected `bool`, found fn item
   = note: expected type `bool`
   = note: expected type `bool`
           found fn item `fn() {{f}}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/extern/extern-wrong-value-type.rs stdout ----
diff of stderr:

- error[E0277]: expected a `Fn<()>` closure, found `extern "C" fn() {f}`
+ error[E0277]: expected a `Fn<()>` closure, found `extern "C" fn() {{f}}`
3    |
3    |
4 LL |     is_fn(f);

-    |     ----- ^ expected an `Fn<()>` closure, found `extern "C" fn() {f}`
+    |     ----- ^ expected an `Fn<()>` closure, found `extern "C" fn() {{f}}`
7    |     required by a bound introduced by this call
8    |


-    = help: the trait `Fn<()>` is not implemented for `extern "C" fn() {f}`
-    = note: wrap the `extern "C" fn() {f}` in a closure with no arguments: `|| { /* code */ }`
+    = help: the trait `Fn<()>` is not implemented for `extern "C" fn() {{f}}`
+    = note: wrap the `extern "C" fn() {{f}}` in a closure with no arguments: `|| { /* code */ }`
11 note: required by a bound in `is_fn`
13    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-wrong-value-type/extern-wrong-value-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extern/extern-wrong-value-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-wrong-value-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-wrong-value-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-wrong-value-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `Fn<()>` closure, found `extern "C" fn() {{f}}`
   |
   |
LL |     is_fn(f);
   |     ----- ^ expected an `Fn<()>` closure, found `extern "C" fn() {{f}}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Fn<()>` is not implemented for `extern "C" fn() {{f}}`
   = note: wrap the `extern "C" fn() {{f}}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `is_fn`
   |
   |
LL | fn is_fn<F>(_: F) where F: Fn() {}
   |                            ^^^^ required by this bound in `is_fn`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/fn/fn-compare-mismatch.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `fn() {f}`
+ error[E0369]: binary operation `==` cannot be applied to type `fn() {{f}}`
2   --> $DIR/fn-compare-mismatch.rs:4:15
3    |
4 LL |     let x = f == g;

-    |             - ^^ - fn() {g}
+    |             - ^^ - fn() {{g}}
-    |             fn() {f}
+    |             fn() {{f}}
8    |
8    |
9 help: you might have forgotten to call this function


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-compare-mismatch/fn-compare-mismatch.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-compare-mismatch/fn-compare-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/fn-compare-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/fn-compare-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-compare-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-compare-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `fn() {{f}}`
   |
   |
LL |     let x = f == g;
   |             - ^^ - fn() {{g}}
   |             fn() {{f}}
   |
   |
help: you might have forgotten to call this function
   |
LL |     let x = f() == g;
   |              ++
help: you might have forgotten to call this function
   |
LL |     let x = f == g();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/fn/fn-compare-mismatch.rs:4:18
   |
   |
LL |     let x = f == g;
   |                  ^ expected fn item, found a different fn item
   |
   = note: expected fn item `fn() {f}`
              found fn item `fn() {g}`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/generator/print/generator-print-verbose-2.rs stdout ----
diff of stderr:

23 LL |     assert_sync(|| {
24    |     ^^^^^^^^^^^ generator is not `Sync`
25    |
-    = help: within `[main::{closure#0} upvar_tys=() {Cell<i32>, ()}]`, the trait `Sync` is not implemented for `Cell<i32>`
+    = help: within `[main::{closure#0} upvar_tys=() {{Cell<i32>, ()}}]`, the trait `Sync` is not implemented for `Cell<i32>`
27 note: generator is not `Sync` as this value is used across a yield
29    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-2/generator-print-verbose-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/print/generator-print-verbose-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/print/generator-print-verbose-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `Cell<i32>` cannot be shared between threads safely
   |
LL |     assert_send(|| {
LL |     assert_send(|| {
   |     ^^^^^^^^^^^ `Cell<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `Cell<i32>`
   = note: required because of the requirements on the impl of `Send` for `&'_#4r Cell<i32>`
note: required because it's used within this generator
   |
LL |     assert_send(|| {
   |                 ^^
note: required by a bound in `assert_send`
note: required by a bound in `assert_send`
  --> /checkout/src/test/ui/generator/print/generator-print-verbose-2.rs:10:23
   |
LL |     fn assert_send<T: Send>(_: T) {}
   |                       ^^^^ required by this bound in `assert_send`
error: generator cannot be shared between threads safely
  --> /checkout/src/test/ui/generator/print/generator-print-verbose-2.rs:12:5
   |
LL |     assert_sync(|| {
LL |     assert_sync(|| {
   |     ^^^^^^^^^^^ generator is not `Sync`
   |
   = help: within `[main::{closure#0} upvar_tys=() {{Cell<i32>, ()}}]`, the trait `Sync` is not implemented for `Cell<i32>`
note: generator is not `Sync` as this value is used across a yield
   |
LL |         let a = Cell::new(2);
LL |         let a = Cell::new(2);
   |             - has type `Cell<i32>` which is not `Sync`
LL |         yield;
   |         ^^^^^ yield occurs here, with `a` maybe used later
LL |     });
   |     - `a` is later dropped here
note: required by a bound in `assert_sync`
   |
   |
LL |     fn assert_sync<T: Sync>(_: T) {}
   |                       ^^^^ required by this bound in `assert_sync`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/hrtb/issue-30786.rs stdout ----
diff of stderr:

23 LL |     let filter = map.stream.filterx(|x: &_| true);
25 
25 
- error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>`, but its trait bounds were not satisfied
+ error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>`, but its trait bounds were not satisfied
28    |
28    |
29 LL | pub struct Filter<S, F> {
33    | doesn't satisfy `_: StreamExt`
34 ...
34 ...
35 LL |     let count = filter.countx();
-    |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>` due to unsatisfied trait bounds
+    |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>` due to unsatisfied trait bounds
38 note: the following trait bounds were not satisfied:
38 note: the following trait bounds were not satisfied:
-       `&'a mut &Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>: Stream`
-       `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>: Stream`
-       `&'a mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>: Stream`
+       `&'a mut &Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>: Stream`
+       `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>: Stream`
+       `&'a mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@$DIR/issue-30786.rs:129:30: 129:37]>: Stream`
43    |
43    |
44 LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786/issue-30786.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hrtb/issue-30786.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-30786.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:34]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Map<S, F> {
   | |
   | |
   | method `filterx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let filter = map.filterx(|x: &_| true);
   |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:34]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:34]>: Stream`
      `&'a mut &mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:34]>: Stream`
      `&'a mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:34]>: Stream`
   |
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
help: one of the expressions' fields has a method of the same name
   |
LL |     let filter = map.stream.filterx(|x: &_| true);


error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:37]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Filter<S, F> {
   | |
   | |
   | method `countx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let count = filter.countx();
   |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:37]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:37]>: Stream`
      `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:37]>: Stream`
      `&'a mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {{identity::<u64>}}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:37]>: Stream`
   |
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
help: one of the expressions' fields has a method of the same name
   |
LL |     let count = filter.stream.countx();

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] src/test/ui/hygiene/impl_items.rs stdout ----
diff of stderr:

- error: type `for<'r> fn(&'r foo::S) {foo::S::f}` is private
+ error: type `for<'r> fn(&'r foo::S) {{foo::S::f}}` is private
3    |
3    |
4 LL |         let _: () = S.f();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/impl_items.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/impl_items.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/impl_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/auxiliary"
stdout: none
--- stderr -------------------------------
error: type `for<'r> fn(&'r foo::S) {{foo::S::f}}` is private
   |
   |
LL |         let _: () = S.f(); //~ ERROR type `for<'r> fn(&'r foo::S) {foo::S::f}` is private
   |                       ^ private type
...
LL |     foo::m!();
   |
   |
   = note: this error originates in the macro `foo::m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/hygiene/intercrate.rs stdout ----
diff of stderr:

- error: type `fn() -> u32 {foo::bar::f}` is private
+ error: type `fn() -> u32 {{foo::bar::f}}` is private
3    |
3    |
4 LL |     assert_eq!(intercrate::foo::m!(), 1);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/intercrate.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/intercrate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/intercrate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/auxiliary"
stdout: none
--- stderr -------------------------------
error: type `fn() -> u32 {{foo::bar::f}}` is private
   |
   |
LL |     assert_eq!(intercrate::foo::m!(), 1);
   |
   |
   = note: this error originates in the macro `intercrate::foo::m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/intrinsics/const-eval-select-bad.rs stdout ----
diff of stderr:

51 LL |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
52    |        ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
53 
- error[E0271]: type mismatch resolving `<fn(i32) -> bool {bar} as FnOnce<(i32,)>>::Output == i32`
+ error[E0271]: type mismatch resolving `<fn(i32) -> bool {{bar}} as FnOnce<(i32,)>>::Output == i32`
56    |
56    |
57 LL |     const_eval_select((1,), foo, bar);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad/const-eval-select-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intrinsics/const-eval-select-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/const-eval-select-bad/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:7:27: 7:29]: FnOnce<()>` is not satisfied
   |
   |
LL |     const_eval_select((), || {}, || {});
   |     -----------------     ^^^^^ expected an `FnOnce<()>` closure, found `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:7:27: 7:29]`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `~const FnOnce<()>` is not implemented for `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:7:27: 7:29]`
note: the trait `FnOnce<()>` is implemented for `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:7:27: 7:29]`, but that implementation is not `const`
   |
   |
LL |     const_eval_select((), || {}, || {});
   |                           ^^^^^
   = note: wrap the `[closure@/checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:7:27: 7:29]` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0277]: the trait bound `{integer}: FnOnce<()>` is not satisfied
   |
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------     ^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `~const FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
   |
   |
LL |     const_eval_select((), 42, 0xDEADBEEF);
   |     -----------------         ^^^^^^^^^^ expected an `FnOnce<()>` closure, found `{integer}`
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `const_eval_select`
   |
   |
LL |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`

error[E0271]: type mismatch resolving `<fn(i32) -> bool {{bar}} as FnOnce<(i32,)>>::Output == i32`
   |
   |
LL |     const_eval_select((1,), foo, bar);
   |     ^^^^^^^^^^^^^^^^^ expected `i32`, found `bool`
note: required by a bound in `const_eval_select`
  --> /checkout/library/core/src/intrinsics.rs:2694:20
   |
   |
LL |     G: FnOnce<ARG, Output = RET> + ~const Destruct,
   |                    ^^^^^^^^^^^^ required by this bound in `const_eval_select`
error[E0631]: type mismatch in function arguments
  --> /checkout/src/test/ui/intrinsics/const-eval-select-bad.rs:34:32
   |
   |
LL | const fn foo(n: i32) -> i32 {
...
...
LL |     const_eval_select((true,), foo, baz);
   |     |
   |     required by a bound introduced by this call
   |
   |
   = note: expected function signature `fn(bool) -> _`
              found function signature `fn(i32) -> _`
note: required by a bound in `const_eval_select`
   |
   |
LL |     F: ~const FnOnce<ARG, Output = RET>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `const_eval_select`
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0271, E0277, E0631.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.
------------------------------------------

---
diff of stderr:

7    |            expected due to this
8    |
9    = note: expected reference `&for<'r> fn(&'r B) -> u32`
-               found reference `&for<'r> fn(&'r B) -> u32 {B::func}`
+               found reference `&for<'r> fn(&'r B) -> u32 {{B::func}}`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322/issue-24322.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-24322.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24322.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-24322.rs:8:29
   |
   |
LL |     let x: &fn(&B) -> u32 = &B::func; //~ ERROR mismatched types
   |            --------------   ^^^^^^^^ expected fn pointer, found fn item
   |            expected due to this
   |
   |
   = note: expected reference `&for<'r> fn(&'r B) -> u32`
              found reference `&for<'r> fn(&'r B) -> u32 {{B::func}}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-35241.rs stdout ----
diff of stderr:

2   --> $DIR/issue-35241.rs:3:20
3    |
4 LL | struct Foo(u32);
-    | ---------- fn(u32) -> Foo {Foo} defined here
+    | ---------- fn(u32) -> Foo {{Foo}} defined here
7 LL | fn test() -> Foo { Foo }
8    |              ---   ^^^ expected struct `Foo`, found fn item

10    |              expected `Foo` because of return type
10    |              expected `Foo` because of return type
11    |
12    = note: expected struct `Foo`
-              found fn item `fn(u32) -> Foo {Foo}`
+              found fn item `fn(u32) -> Foo {{Foo}}`
14 help: use parentheses to instantiate this tuple struct
15    |
16 LL | fn test() -> Foo { Foo(_) }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241/issue-35241.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-35241.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-35241.rs:3:20
   |
LL | struct Foo(u32);
LL | struct Foo(u32);
   | ---------- fn(u32) -> Foo {{Foo}} defined here
LL |
LL | fn test() -> Foo { Foo } //~ ERROR mismatched types
   |              ---   ^^^ expected struct `Foo`, found fn item
   |              expected `Foo` because of return type
   |
   = note: expected struct `Foo`
   = note: expected struct `Foo`
             found fn item `fn(u32) -> Foo {{Foo}}`
help: use parentheses to instantiate this tuple struct
   |
LL | fn test() -> Foo { Foo(_) } //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
---
diff of stderr:

7    |                   arguments to this struct are incorrect
8    |
9    = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
-              found fn item `fn() {f}`
+              found fn item `fn() {{f}}`
12   --> $DIR/issue-5216.rs:2:8
13    |


21    |                   ^ expected struct `Box`, found fn item
22    |
23    = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
-              found fn item `fn() {g}`
+              found fn item `fn() {{g}}`
26 error: aborting due to 2 previous errors
27 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216/issue-5216.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-5216.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5216.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-5216.rs:3:21
   |
   |
LL | pub static C: S = S(f); //~ ERROR mismatched types
   |                   - ^ expected struct `Box`, found fn item
   |                   arguments to this struct are incorrect
   |
   |
   = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
             found fn item `fn() {{f}}`
  --> /checkout/src/test/ui/issues/issue-5216.rs:2:8
   |
   |
LL | struct S(Box<dyn FnMut() + Sync>);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-5216.rs:8:19
   |
   |
LL | pub static D: T = g; //~ ERROR mismatched types
   |                   ^ expected struct `Box`, found fn item
   |
   = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
             found fn item `fn() {{g}}`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-59488.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
+ error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {{foo}}`
3    |
4 LL |     foo > 12;


5    |     --- ^ -- {integer}
-    |     fn() -> i32 {foo}
-    |     fn() -> i32 {foo}
+    |     fn() -> i32 {{foo}}
8    |
9 help: you might have forgotten to call this function

17 LL |     foo > 12;
18    |           ^^ expected fn item, found integer
19    |
19    |
-    = note: expected fn item `fn() -> i32 {foo}`
+    = note: expected fn item `fn() -> i32 {{foo}}`
22 
22 
- error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {bar}`
+ error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {{bar}}`
25    |
26 LL |     bar > 13;


27    |     --- ^ -- {integer}
28    |     |
-    |     fn(i64) -> i64 {bar}
+    |     fn(i64) -> i64 {{bar}}
30    |
31 help: you might have forgotten to call this function

39 LL |     bar > 13;
40    |           ^^ expected fn item, found integer
41    |
41    |
-    = note: expected fn item `fn(i64) -> i64 {bar}`
+    = note: expected fn item `fn(i64) -> i64 {{bar}}`
44 
44 
- error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
+ error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {{foo}}`
47    |
48 LL |     foo > foo;


-    |     --- ^ --- fn() -> i32 {foo}
+    |     --- ^ --- fn() -> i32 {{foo}}
-    |     fn() -> i32 {foo}
-    |     fn() -> i32 {foo}
+    |     fn() -> i32 {{foo}}
52    |
53 help: you might have forgotten to call this function


59 LL |     foo > foo();
61 
61 
- error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
+ error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {{foo}}`
64    |
64    |
65 LL |     foo > bar;

-    |     --- ^ --- fn(i64) -> i64 {bar}
+    |     --- ^ --- fn(i64) -> i64 {{bar}}
-    |     fn() -> i32 {foo}
-    |     fn() -> i32 {foo}
+    |     fn() -> i32 {{foo}}
70 error[E0308]: mismatched types
71   --> $DIR/issue-59488.rs:25:11


76    = note: expected fn item `fn() -> i32 {foo}`
77               found fn item `fn(i64) -> i64 {bar}`
78 
- error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {Foo::Bar}`
+ error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {{Foo::Bar}}`
81    |
81    |
82 LL |     assert_eq!(Foo::Bar, i);
83    |     ^^^^^^^^^^^^^^^^^^^^^^^
84    |     |
84    |     |
-    |     fn(usize) -> Foo {Foo::Bar}
-    |     fn(usize) -> Foo {Foo::Bar}
+    |     fn(usize) -> Foo {{Foo::Bar}}
+    |     fn(usize) -> Foo {{Foo::Bar}}
87    |
88    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


- error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug`
+ error[E0277]: `fn(usize) -> Foo {{Foo::Bar}}` doesn't implement `Debug`
92    |
92    |
93 LL |     assert_eq!(Foo::Bar, i);

-    |     ^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {{Foo::Bar}}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
95    |
-    = help: the trait `Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
+    = help: the trait `Debug` is not implemented for `fn(usize) -> Foo {{Foo::Bar}}`
97    = help: the following other types implement trait `Debug`:
98              extern "C" fn() -> Ret
99              extern "C" fn(A, B) -> Ret

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-59488.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {{foo}}`
   |
LL |     foo > 12;
LL |     foo > 12;
   |     --- ^ -- {integer}
   |     |
   |     fn() -> i32 {{foo}}
   |
help: you might have forgotten to call this function
LL |     foo() > 12;
   |        ++

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:14:11
   |
LL |     foo > 12;
   |           ^^ expected fn item, found integer
   |
   = note: expected fn item `fn() -> i32 {{foo}}`


error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {{bar}}`
   |
LL |     bar > 13;
LL |     bar > 13;
   |     --- ^ -- {integer}
   |     |
   |     fn(i64) -> i64 {{bar}}
   |
help: you might have forgotten to call this function
   |
LL |     bar( /* arguments */ ) > 13;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:18:11
   |
   |
LL |     bar > 13;
   |           ^^ expected fn item, found integer
   |
   = note: expected fn item `fn(i64) -> i64 {{bar}}`


error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {{foo}}`
   |
LL |     foo > foo;
LL |     foo > foo;
   |     --- ^ --- fn() -> i32 {{foo}}
   |     |
   |     fn() -> i32 {{foo}}
   |
help: you might have forgotten to call this function
LL |     foo() > foo;
   |        ++
   |        ++
help: you might have forgotten to call this function
   |
LL |     foo > foo();


error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {{foo}}`
   |
   |
LL |     foo > bar;
   |     --- ^ --- fn(i64) -> i64 {{bar}}
   |     |
   |     fn() -> i32 {{foo}}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
   |
   |
LL |     foo > bar;
   |           ^^^ expected fn item, found a different fn item
   |
   = note: expected fn item `fn() -> i32 {foo}`
              found fn item `fn(i64) -> i64 {bar}`

error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {{Foo::Bar}}`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     |
   |     |
   |     fn(usize) -> Foo {{Foo::Bar}}
   |     fn(usize) -> Foo {{Foo::Bar}}
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `fn(usize) -> Foo {{Foo::Bar}}` doesn't implement `Debug`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {{Foo::Bar}}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `fn(usize) -> Foo {{Foo::Bar}}`
   = help: the following other types implement trait `Debug`:
             extern "C" fn() -> Ret
             extern "C" fn(A, B) -> Ret
             extern "C" fn(A, B, ...) -> Ret
             extern "C" fn(A, B, C) -> Ret
             extern "C" fn(A, B, C, ...) -> Ret
             extern "C" fn(A, B, C, D) -> Ret
             extern "C" fn(A, B, C, D, ...) -> Ret
             extern "C" fn(A, B, C, D, E) -> Ret
           and 68 others
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0277, E0308, E0369.
For more information about an error, try `rustc --explain E0277`.
---
diff of stderr:

2   --> $DIR/issue-62375.rs:7:7
3    |
4 LL |     a == A::Value;
-    |     - ^^ -------- fn(()) -> A {A::Value}
+    |     - ^^ -------- fn(()) -> A {{A::Value}}
7    |     A
8    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/issue-62375.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-62375.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-62375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `A`
   |
   |
LL |     a == A::Value;
   |     - ^^ -------- fn(()) -> A {{A::Value}}
   |     A
   |
   |
note: an implementation of `PartialEq<_>` might be missing for `A`
   |
   |
LL | enum A {
   | ^^^^^^ must implement `PartialEq<_>`
help: consider annotating `A` with `#[derive(PartialEq)]`
   |
LL | #[derive(PartialEq)]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
For more information about this error, try `rustc --explain E0369`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-66667-function-cmp-cycle.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `fn() {second}`
+ error[E0369]: binary operation `==` cannot be applied to type `fn() {{second}}`
3    |
4 LL |     second == 1


5    |     ------ ^^ - {integer}
6    |     |
-    |     fn() {second}
+    |     fn() {{second}}
9 error[E0308]: mismatched types
10   --> $DIR/issue-66667-function-cmp-cycle.rs:2:15

12 LL |     second == 1
12 LL |     second == 1
13    |               ^ expected fn item, found integer
14    |
-    = note: expected fn item `fn() {second}`
+    = note: expected fn item `fn() {{second}}`
16                  found type `{integer}`
17 
- error[E0369]: binary operation `==` cannot be applied to type `fn() {first}`
+ error[E0369]: binary operation `==` cannot be applied to type `fn() {{first}}`
20    |
21 LL |     first == 1


22    |     ----- ^^ - {integer}
-    |     fn() {first}
-    |     fn() {first}
+    |     fn() {{first}}
26 error[E0308]: mismatched types
27   --> $DIR/issue-66667-function-cmp-cycle.rs:7:14

29 LL |     first == 1
29 LL |     first == 1
30    |              ^ expected fn item, found integer
31    |
-    = note: expected fn item `fn() {first}`
+    = note: expected fn item `fn() {{first}}`
33                  found type `{integer}`
34 
- error[E0369]: binary operation `==` cannot be applied to type `fn() {bar}`
+ error[E0369]: binary operation `==` cannot be applied to type `fn() {{bar}}`
37    |
38 LL |     bar == 1


39    |     --- ^^ - {integer}
-    |     fn() {bar}
-    |     fn() {bar}
+    |     fn() {{bar}}
43 error[E0308]: mismatched types
44   --> $DIR/issue-66667-function-cmp-cycle.rs:12:12

46 LL |     bar == 1
46 LL |     bar == 1
47    |            ^ expected fn item, found integer
48    |
-    = note: expected fn item `fn() {bar}`
+    = note: expected fn item `fn() {{bar}}`
50                  found type `{integer}`
52 error: aborting due to 6 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66667-function-cmp-cycle/issue-66667-function-cmp-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-66667-function-cmp-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66667-function-cmp-cycle" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66667-function-cmp-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `fn() {{second}}`
   |
   |
LL |     second == 1 //~ ERROR binary operation
   |     ------ ^^ - {integer}
   |     |
   |     fn() {{second}}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs:2:15
   |
   |
LL |     second == 1 //~ ERROR binary operation
   |               ^ expected fn item, found integer
   |
   = note: expected fn item `fn() {{second}}`
                 found type `{integer}`

error[E0369]: binary operation `==` cannot be applied to type `fn() {{first}}`
   |
   |
LL |     first == 1 //~ ERROR binary operation
   |     ----- ^^ - {integer}
   |     |
   |     fn() {{first}}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs:7:14
   |
   |
LL |     first == 1 //~ ERROR binary operation
   |              ^ expected fn item, found integer
   |
   = note: expected fn item `fn() {{first}}`
                 found type `{integer}`

error[E0369]: binary operation `==` cannot be applied to type `fn() {{bar}}`
   |
   |
LL |     bar == 1 //~ ERROR binary operation
   |     --- ^^ - {integer}
   |     |
   |     fn() {{bar}}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs:12:12
   |
   |
LL |     bar == 1 //~ ERROR binary operation
   |            ^ expected fn item, found integer
   |
   = note: expected fn item `fn() {{bar}}`
                 found type `{integer}`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `fn() -> i32 {a}`
+ error[E0369]: binary operation `==` cannot be applied to type `fn() -> i32 {{a}}`
3    |
4 LL |     assert_eq!(a, 0);

5    |     ^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^
6    |     |
-    |     fn() -> i32 {a}
+    |     fn() -> i32 {{a}}
8    |     {integer}
9    |
10    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
20 LL |     assert_eq!(a, 0);
21    |     ^^^^^^^^^^^^^^^^ expected fn item, found integer
22    |
22    |
-    = note: expected fn item `fn() -> i32 {a}`
+    = note: expected fn item `fn() -> i32 {{a}}`
24                  found type `i32`
25    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


- error[E0277]: `fn() -> i32 {a}` doesn't implement `Debug`
+ error[E0277]: `fn() -> i32 {{a}}` doesn't implement `Debug`
29    |
29    |
30 LL | fn a() -> i32 {
31    |    - consider calling this function
32 ...
33 LL |     assert_eq!(a, 0);
33 LL |     assert_eq!(a, 0);
-    |     ^^^^^^^^^^^^^^^^ `fn() -> i32 {a}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^ `fn() -> i32 {{a}}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    = help: the trait `Debug` is not implemented for `fn() -> i32 {a}`
-    = help: the trait `Debug` is not implemented for `fn() -> i32 {a}`
+    = help: the trait `Debug` is not implemented for `fn() -> i32 {{a}}`
37    = help: use parentheses to call the function: `a()`
38    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/issue-70724-add_type_neq_err_label-unwrap.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/issue-70724-add_type_neq_err_label-unwrap.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-70724-add_type_neq_err_label-unwrap.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `fn() -> i32 {{a}}`
   |
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     fn() -> i32 {{a}}
   |     {integer}
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: you might have forgotten to call this function
   |
   |
LL |                 if !(*left_val() == *right_val) {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs:6:5
   |
   |
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^ expected fn item, found integer
   |
   = note: expected fn item `fn() -> i32 {{a}}`
                 found type `i32`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `fn() -> i32 {{a}}` doesn't implement `Debug`
   |
   |
LL | fn a() -> i32 {
...
LL |     assert_eq!(a, 0);
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^ `fn() -> i32 {{a}}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `fn() -> i32 {{a}}`
   = help: use parentheses to call the function: `a()`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308, E0369.
For more information about an error, try `rustc --explain E0277`.
---
diff of stderr:

7    |     ^^^^^^^^^^^ expected `usize`, found fn item
8    |
9    = note: expected type `usize`
-            found fn item `fn() -> String {String::new}`
+            found fn item `fn() -> String {{String::new}}`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87490/issue-87490.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-87490.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-87490.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87490" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87490/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-87490.rs:9:5
   |
   |
LL | fn follow(_: &str) -> <&str as StreamOnce>::Position {
   |                       ------------------------------ expected `usize` because of return type
LL |     String::new  //~ ERROR mismatched types
   |     ^^^^^^^^^^^ expected `usize`, found fn item
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn() -> String {{String::new}}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lifetimes/lifetime-errors/issue_74400.rs stdout ----
diff of stderr:

29 LL |     f(data, identity)
30    |     ^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
31    |
-    = note: `fn(&'2 T) -> &'2 T {identity::<&'2 T>}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
+    = note: `fn(&'2 T) -> &'2 T {{identity::<&'2 T>}}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
33    = note: ...but it actually implements `FnOnce<(&'2 T,)>`, for some specific lifetime `'2`
35 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400/issue_74400.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/issue_74400.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/issue_74400.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
LL |     f(data, identity)
LL |     f(data, identity)
   |     ^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn g<T: 'static>(data: &[T]) {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lifetimes/lifetime-errors/issue_74400.rs:12:5
   |
   |
LL |     f(data, identity)
   |     ^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'r> Fn<(&'r T,)>`
              found trait `Fn<(&T,)>`
note: the lifetime requirement is introduced here
   |
   |
LL | fn f<T, S>(data: &[T], key: impl Fn(&T) -> S) {


error: implementation of `FnOnce` is not general enough
   |
LL |     f(data, identity)
LL |     f(data, identity)
   |     ^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&'2 T) -> &'2 T {{identity::<&'2 T>}}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 T,)>`, for some specific lifetime `'2`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0310.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/lint/trivial_casts.rs stdout ----
diff of stderr:

120    |
121    = help: cast can be replaced by coercion; this might require a temporary variable
122 
- error: trivial cast: `&fn(i32) {baz}` as `&dyn Fn(i32)`
+ error: trivial cast: `&fn(i32) {{baz}}` as `&dyn Fn(i32)`
125    |
125    |
126 LL |     let _ = &baz as &dyn Fn(i32);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/trivial_casts/trivial_casts.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/trivial_casts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/trivial_casts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/trivial_casts" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/trivial_casts/auxiliary"
stdout: none
--- stderr -------------------------------
error: trivial numeric cast: `i32` as `i32`
   |
   |
LL |     let _ = 42_i32 as i32; //~ ERROR trivial numeric cast: `i32` as `i32`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/trivial_casts.rs:4:24
   |
   |
LL | #![deny(trivial_casts, trivial_numeric_casts)]
   |                        ^^^^^^^^^^^^^^^^^^^^^
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial numeric cast: `u8` as `u8`
   |
   |
LL |     let _ = 42_u8 as u8; //~ ERROR trivial numeric cast: `u8` as `u8`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&u32` as `*const u32`
   |
   |
LL |     let _ = x as *const u32; //~ERROR trivial cast: `&u32` as `*const u32`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/trivial_casts.rs:4:9
   |
   |
LL | #![deny(trivial_casts, trivial_numeric_casts)]
   |         ^^^^^^^^^^^^^
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&mut u32` as `*mut u32`
   |
   |
LL |     let _ = x as *mut u32; //~ERROR trivial cast: `&mut u32` as `*mut u32`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&[u32; 3]` as `&[u32]`
   |
   |
LL |     let _ = x as &[u32]; //~ERROR trivial cast: `&[u32; 3]` as `&[u32]`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&[u32; 3]` as `*const [u32]`
   |
   |
LL |     let _ = x as *const [u32]; //~ERROR trivial cast: `&[u32; 3]` as `*const [u32]`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&mut [u32; 3]` as `&mut [u32]`
   |
   |
LL |     let _ = x as &mut [u32]; //~ERROR trivial cast: `&mut [u32; 3]` as `&mut [u32]`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&mut [u32; 3]` as `*mut [u32]`
   |
   |
LL |     let _ = x as *mut [u32]; //~ERROR trivial cast: `&mut [u32; 3]` as `*mut [u32]`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `Box<[u32; 3]>` as `Box<[u32]>`
   |
LL |     let _ = x as Box<[u32]>;
   |             ^^^^^^^^^^^^^^^
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&Bar` as `&dyn Foo`
   |
   |
LL |     let _ = x as &dyn Foo; //~ERROR trivial cast: `&Bar` as `&dyn Foo`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&Bar` as `*const dyn Foo`
   |
   |
LL |     let _ = x as *const dyn Foo; //~ERROR trivial cast: `&Bar` as `*const dyn Foo`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&mut Bar` as `&mut dyn Foo`
   |
   |
LL |     let _ = x as &mut dyn Foo; //~ERROR trivial cast: `&mut Bar` as `&mut dyn Foo`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&mut Bar` as `*mut dyn Foo`
   |
   |
LL |     let _ = x as *mut dyn Foo; //~ERROR trivial cast: `&mut Bar` as `*mut dyn Foo`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `Box<Bar>` as `Box<dyn Foo>`
   |
   |
LL |     let _ = x as Box<dyn Foo>; //~ERROR `Box<Bar>` as `Box<dyn Foo>`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&fn(i32) {{baz}}` as `&dyn Fn(i32)`
   |
   |
LL |     let _ = &baz as &dyn Fn(i32); //~ERROR `&fn(i32) {baz}` as `&dyn Fn(i32)`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&[closure@/checkout/src/test/ui/lint/trivial_casts.rs:72:13: 72:22]` as `&dyn Fn(i32)`
   |
   |
LL |     let _ = &x as &dyn Fn(i32); //~ERROR trivial cast
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'a Bar` as `&'a Bar`
   |
   |
LL |     let _ = a as &'a Bar; //~ERROR trivial cast
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'b Bar` as `&'a Bar`
   |
   |
LL |     let _ = b as &'a Bar; //~ERROR trivial cast
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&'b Bar` as `&'b Bar`
   |
   |
LL |     let _ = b as &'b Bar; //~ERROR trivial cast
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable
error: aborting due to 19 previous errors
------------------------------------------



---- [ui] src/test/ui/macros/format-args-temporaries-in-write.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/format-args-temporaries-in-write.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-args-temporaries-in-write" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/format-args-temporaries-in-write/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/match/issue-82392.rs stdout ----
diff of stdout:


11                 ({ } as
12                     ()) else if (let Some(a) =
13                        ((Some as
-                                fn(i32) -> Option<i32> {Option::<i32>::Some})((3 as i32)) as
-                            Option<i32>) as bool) ({ } as ()) as ())
+                                fn(i32) -> Option<i32> {{Option::<i32>::Some}})((3 as i32))
+                            as Option<i32>) as bool) ({ } as ()) as ())
16                } as ())


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/issue-82392.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/issue-82392.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args match/issue-82392.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/issue-82392.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=hir,typed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/auxiliary"
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// https://github.com/rust-lang/rust/issues/82329
// https://github.com/rust-lang/rust/issues/82329
// compile-flags: -Zunpretty=hir,typed
// check-pass
fn main() ({
fn main() ({
        (if (true as bool)
                ({ } as
                    ()) else if (let Some(a) =
                       ((Some as
                               fn(i32) -> Option<i32> {{Option::<i32>::Some}})((3 as i32))
                           as Option<i32>) as bool) ({ } as ()) as ())
               } as ())
stderr: none


---- [ui] src/test/ui/mismatched_types/cast-rfc0401.rs stdout ----
---- [ui] src/test/ui/mismatched_types/cast-rfc0401.rs stdout ----
diff of stderr:

14    |
15    = note: vtable kinds may not match
16 
- error[E0609]: no field `f` on type `fn() {main}`
+ error[E0609]: no field `f` on type `fn() {{main}}`
19    |
19    |
20 LL |     let _ = main.f as *const u32;

62 LL |     let _ = v as f32;
64 
64 
- error[E0606]: casting `fn() {main}` as `f64` is invalid
+ error[E0606]: casting `fn() {{main}}` as `f64` is invalid
67    |
68 LL |     let _ = main as f64;

171 LL |     let _ = foo as *mut str;
171 LL |     let _ = foo as *mut str;
172    |             ^^^^^^^^^^^^^^^
173 
- error[E0606]: casting `fn() {main}` as `*mut str` is invalid
+ error[E0606]: casting `fn() {{main}}` as `*mut str` is invalid
176    |
177 LL |     let _ = main as *mut str;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/cast-rfc0401.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/cast-rfc0401.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0606]: casting `*const U` as `*const V` is invalid
   |
   |
LL |     u as *const V //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match

error[E0606]: casting `*const U` as `*const str` is invalid
   |
   |
LL |     u as *const str //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match

error[E0609]: no field `f` on type `fn() {{main}}`
   |
   |
LL |     let _ = main.f as *const u32; //~ ERROR no field


error[E0605]: non-primitive cast: `*const u8` as `&u8`
   |
   |
LL |     let _ = v as &u8; //~ ERROR non-primitive cast
   |             ^^^^^^^^ invalid cast
help: consider borrowing the value
   |
   |
LL -     let _ = v as &u8; //~ ERROR non-primitive cast
LL +     let _ = &*v; //~ ERROR non-primitive cast


error[E0605]: non-primitive cast: `*const u8` as `E`
   |
   |
LL |     let _ = v as E; //~ ERROR non-primitive cast
   |             ^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0605]: non-primitive cast: `*const u8` as `fn()`
   |
   |
LL |     let _ = v as fn(); //~ ERROR non-primitive cast
   |             ^^^^^^^^^ invalid cast

error[E0605]: non-primitive cast: `*const u8` as `(u32,)`
   |
   |
LL |     let _ = v as (u32,); //~ ERROR non-primitive cast
   |             ^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0605]: non-primitive cast: `Option<&*const u8>` as `*const u8`
   |
   |
LL |     let _ = Some(&v) as *const u8; //~ ERROR non-primitive cast
   |             ^^^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0606]: casting `*const u8` as `f32` is invalid
   |
   |
LL |     let _ = v as f32; //~ ERROR is invalid


error[E0606]: casting `fn() {{main}}` as `f64` is invalid
   |
   |
LL |     let _ = main as f64; //~ ERROR is invalid


error[E0606]: casting `&*const u8` as `usize` is invalid
   |
   |
LL |     let _ = &v as usize; //~ ERROR is invalid
   |
   = help: cast through a raw pointer first


error[E0606]: casting `f32` as `*const u8` is invalid
   |
   |
LL |     let _ = f as *const u8; //~ ERROR is invalid

error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:39:13
   |
   |
LL |     let _ = 3_i32 as bool; //~ ERROR cannot cast
   |             ^^^^^^^^^^^^^ help: compare with zero instead: `3_i32 != 0`
error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:40:13
   |
   |
LL |     let _ = E::A as bool; //~ ERROR cannot cast
   |             ^^^^^^^^^^^^ unsupported cast

error[E0604]: only `u8` can be cast as `char`, not `u32`
   |
   |
LL |     let _ = 0x61u32 as char; //~ ERROR can be cast as
   |             |
   |             invalid cast
   |             invalid cast
   |             help: try `char::from_u32` instead: `char::from_u32(0x61u32)`

error[E0606]: casting `bool` as `f32` is invalid
   |
   |
LL |     let _ = false as f32; //~ ERROR is invalid
   |
   = help: cast through an integer first


error[E0606]: casting `E` as `f32` is invalid
   |
---

---- [ui] src/test/ui/reify-intrinsic.rs stdout ----
diff of stderr:

9    = note: expected fn pointer `unsafe extern "rust-intrinsic" fn(isize) -> usize`
10                  found fn item `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
11 
- error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
+ error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {{transmute::<_, _>}}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
14    |
14    |
15 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args reify-intrinsic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: cannot coerce intrinsics to function pointers
   |
   |
LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
   |            -------------------------------------------------   ^^^^^^^^^^^^^^^^^^^ cannot coerce intrinsics to function pointers
   |            expected due to this
   |
   |
   = note: expected fn pointer `unsafe extern "rust-intrinsic" fn(isize) -> usize`
                 found fn item `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`

error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {{transmute::<_, _>}}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
   |
   |
LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;

error[E0308]: cannot coerce intrinsics to function pointers
  --> /checkout/src/test/ui/reify-intrinsic.rs:18:9
   |
   |
LL |         std::intrinsics::unlikely,
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ cannot coerce intrinsics to function pointers
   |
   = note: expected fn item `extern "rust-intrinsic" fn(_) -> _ {likely}`
              found fn item `extern "rust-intrinsic" fn(_) -> _ {unlikely}`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0606.
For more information about an error, try `rustc --explain E0308`.
---
diff of stderr:

318   --> $DIR/privacy-enum-ctor.rs:27:20
319    |
320 LL |             Fn(u8),
-    |             -- fn(u8) -> Z {Z::Fn} defined here
+    |             -- fn(u8) -> Z {{Z::Fn}} defined here
322 ...
323 LL |         let _: Z = Z::Fn;
324    |                -   ^^^^^ expected enum `Z`, found fn item
326    |                expected due to this
327    |
328    = note: expected enum `Z`
328    = note: expected enum `Z`
-            found fn item `fn(u8) -> Z {Z::Fn}`
+            found fn item `fn(u8) -> Z {{Z::Fn}}`
330 help: use parentheses to instantiate this tuple variant
331    |
332 LL |         let _: Z = Z::Fn(_);
353   --> $DIR/privacy-enum-ctor.rs:43:16
354    |
355 LL |         Fn(u8),
355 LL |         Fn(u8),
-    |         -- fn(u8) -> E {E::Fn} defined here
+    |         -- fn(u8) -> E {{E::Fn}} defined here
357 ...
358 LL |     let _: E = m::E::Fn;
359    |            -   ^^^^^^^^ expected enum `E`, found fn item
361    |            expected due to this
362    |
363    = note: expected enum `E`
363    = note: expected enum `E`
-            found fn item `fn(u8) -> E {E::Fn}`
+            found fn item `fn(u8) -> E {{E::Fn}}`
365 help: use parentheses to instantiate this tuple variant
366    |
367 LL |     let _: E = m::E::Fn(_);
388   --> $DIR/privacy-enum-ctor.rs:51:16
389    |
390 LL |         Fn(u8),
390 LL |         Fn(u8),
-    |         -- fn(u8) -> E {E::Fn} defined here
+    |         -- fn(u8) -> E {{E::Fn}} defined here
392 ...
393 LL |     let _: E = E::Fn;
394    |            -   ^^^^^ expected enum `E`, found fn item
396    |            expected due to this
397    |
398    = note: expected enum `E`
398    = note: expected enum `E`
-            found fn item `fn(u8) -> E {E::Fn}`
+            found fn item `fn(u8) -> E {{E::Fn}}`
400 help: use parentheses to instantiate this tuple variant
401    |
402 LL |     let _: E = E::Fn(_);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/privacy-enum-ctor.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/privacy-enum-ctor.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/privacy-enum-ctor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0423]: expected value, found enum `n::Z`
   |
LL |         n::Z;
   |         ^^^^
   |
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:11:9
   |
LL | /         pub(in m) enum Z {
LL | |             Fn(u8),
LL | |             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             Unit,
LL | |         }
help: you might have meant to use the following enum variant
   |
   |
LL |         m::Z::Unit;
help: the following enum variants are available
   |
   |
LL |         (m::Z::Fn(/* fields */));
   |         ~~~~~~~~~~~~~~~~~~~~~~~~
LL |         (m::Z::Struct { /* fields */ });

error[E0423]: expected value, found enum `Z`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:25:9
   |
   |
LL |         Z;
   |         ^
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:11:9
   |
LL | /         pub(in m) enum Z {
LL | |             Fn(u8),
LL | |             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             Unit,
LL | |         }
help: you might have meant to use the following enum variant
   |
   |
LL |         m::Z::Unit;
help: the following enum variants are available
   |
   |
LL |         (m::Z::Fn(/* fields */));
   |         ~~~~~~~~~~~~~~~~~~~~~~~~
LL |         (m::Z::Struct { /* fields */ });


error[E0423]: expected value, found struct variant `Z::Struct`
   |
LL | /             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             },
   | |_____________- `Z::Struct` defined here
...
LL |           let _: Z = Z::Struct;
   |                      ^^^^^^^^^ help: use struct literal syntax instead: `Z::Struct { s: val }`

error[E0423]: expected value, found enum `m::E`
   |
LL |     fn f() {
LL |     fn f() {
   |     ------ similarly named function `f` defined here
...
LL |     let _: E = m::E;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:2:5
   |
---
LL | |     }
   | |_____^
help: you might have meant to use the following enum variant
   |
LL |     let _: E = E::Unit;
help: the following enum variants are available
   |
   |
LL |     let _: E = (E::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~
LL |     let _: E = (E::Struct { /* fields */ });
help: a function with a similar name exists
   |
   |
LL |     let _: E = m::f;
help: consider importing one of these items instead
   |
LL | use std::f32::consts::E;
   |
   |
LL | use std::f64::consts::E;
   |
help: if you import `E`, refer to it directly
   |
LL -     let _: E = m::E;
LL +     let _: E = E;


error[E0423]: expected value, found struct variant `m::E::Struct`
   |
LL | /         Struct {
LL | |             s: u8,
LL | |         },
LL | |         },
   | |_________- `m::E::Struct` defined here
...
LL |       let _: E = m::E::Struct;
   |                  ^^^^^^^^^^^^ help: use struct literal syntax instead: `m::E::Struct { s: val }`
error[E0423]: expected value, found enum `E`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:49:16
   |
   |
LL |     let _: E = E;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:2:5
   |
---
LL | |     }
   | |_____^
help: you might have meant to use the following enum variant
   |
LL |     let _: E = E::Unit;
help: the following enum variants are available
   |
   |
LL |     let _: E = (E::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~
LL |     let _: E = (E::Struct { /* fields */ });
help: consider importing one of these items instead
   |
LL | use std::f32::consts::E;
   |
   |
LL | use std::f64::consts::E;
   |

error[E0423]: expected value, found struct variant `E::Struct`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:53:16
   |
LL | /         Struct {
LL | |             s: u8,
LL | |         },
   | |_________- `E::Struct` defined here
...
LL |       let _: E = E::Struct;
   |                  ^^^^^^^^^ help: use struct literal syntax instead: `E::Struct { s: val }`
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:57:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible

error[E0423]: expected value, found enum `m::n::Z`
   |
   |
LL |     let _: Z = m::n::Z;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:11:9
   |
   |
LL | /         pub(in m) enum Z {
LL | |             Fn(u8),
LL | |             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             Unit,
LL | |         }
help: you might have meant to use the following enum variant
   |
   |
LL |     let _: Z = m::Z::Unit;
help: the following enum variants are available
   |
   |
LL |     let _: Z = (m::Z::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~~~~
LL |     let _: Z = (m::Z::Struct { /* fields */ });

error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:61:12
   |
   |
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Fn;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:64:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Struct;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible

error[E0423]: expected value, found struct variant `m::n::Z::Struct`
   |
LL | /             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             },
   | |_____________- `m::n::Z::Struct` defined here
...
LL |       let _: Z = m::n::Z::Struct;
   |                  ^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `m::n::Z::Struct { s: val }`
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:68:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Unit {};
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {
   |         ^^^^^^^^^^^^^^^^ not accessible

error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Fn;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Struct;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Unit {};
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:27:20
   |
   |
LL |             Fn(u8),
   |             -- fn(u8) -> Z {{Z::Fn}} defined here
...
LL |         let _: Z = Z::Fn;
   |                -   ^^^^^ expected enum `Z`, found fn item
   |                expected due to this
   |
   = note: expected enum `Z`
   = note: expected enum `Z`
           found fn item `fn(u8) -> Z {{Z::Fn}}`
help: use parentheses to instantiate this tuple variant
   |
LL |         let _: Z = Z::Fn(_);


error[E0618]: expected function, found enum variant `Z::Unit`
   |
LL |             Unit,
LL |             Unit,
   |             ---- enum variant `Z::Unit` defined here
...
LL |         let _ = Z::Unit();
   |                 |
   |                 call expression requires function
   |
   |
help: `Z::Unit` is a unit enum variant, and does not take parentheses to be constructed
   |
LL -         let _ = Z::Unit();
LL +         let _ = Z::Unit;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:43:16
   |
   |
LL |         Fn(u8),
   |         -- fn(u8) -> E {{E::Fn}} defined here
...
LL |     let _: E = m::E::Fn;
   |            -   ^^^^^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `fn(u8) -> E {{E::Fn}}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = m::E::Fn(_);


error[E0618]: expected function, found enum variant `m::E::Unit`
   |
LL |         Unit,
LL |         Unit,
   |         ---- enum variant `m::E::Unit` defined here
...
LL |     let _: E = m::E::Unit();
   |                |
   |                call expression requires function
   |
   |
help: `m::E::Unit` is a unit enum variant, and does not take parentheses to be constructed
   |
LL -     let _: E = m::E::Unit();
LL +     let _: E = m::E::Unit;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:51:16
   |
   |
LL |         Fn(u8),
   |         -- fn(u8) -> E {{E::Fn}} defined here
...
LL |     let _: E = E::Fn;
   |            -   ^^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `fn(u8) -> E {{E::Fn}}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = E::Fn(_);


error[E0618]: expected function, found enum variant `E::Unit`
   |
LL |         Unit,
LL |         Unit,
   |         ---- enum variant `E::Unit` defined here
...
LL |     let _: E = E::Unit();
   |                |
   |                call expression requires function
   |
   |
help: `E::Unit` is a unit enum variant, and does not take parentheses to be constructed
   |
LL -     let _: E = E::Unit();
LL +     let _: E = E::Unit;

error: aborting due to 23 previous errors

Some errors have detailed explanations: E0308, E0412, E0423, E0603, E0618.
Some errors have detailed explanations: E0308, E0412, E0423, E0603, E0618.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/rfc1623-2.rs stdout ----
diff of stderr:

23 LL |     &(non_elidable as for<'a> fn(&'a u8, &'a u8) -> &'a u8);
24    |                       +++++++     ++      ++         ++
25 
- error[E0605]: non-primitive cast: `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 {non_elidable}` as `for<'r, 's> fn(&'r u8, &'s u8) -> &u8`
+ error[E0605]: non-primitive cast: `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 {{non_elidable}}` as `for<'r, 's> fn(&'r u8, &'s u8) -> &u8`
27   --> $DIR/rfc1623-2.rs:10:6
28    |
29 LL |     &(non_elidable as fn(&u8, &u8) -> &u8);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623-2/rfc1623-2.stderr
To only update this specific test, also pass `--test-args rfc1623-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1623-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL | static NON_ELIDABLE_FN: &fn(&u8, &u8) -> &u8 =
   |                             ---  ---     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or argument 2
   = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider making the type lifetime-generic with a new `'a` lifetime
   |
LL | static NON_ELIDABLE_FN: &for<'a> fn(&'a u8, &'a u8) -> &'a u8 =
   |                          +++++++     ++      ++         ++
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/rfc1623-2.rs:10:39
   |
   |
LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
   |                          ---  ---     ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from argument 1 or argument 2
help: consider making the type lifetime-generic with a new `'a` lifetime
   |
LL |     &(non_elidable as for<'a> fn(&'a u8, &'a u8) -> &'a u8);
   |                       +++++++     ++      ++         ++

error[E0605]: non-primitive cast: `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8 {{non_elidable}}` as `for<'r, 's> fn(&'r u8, &'s u8) -> &u8`
   |
   |
LL |     &(non_elidable as fn(&u8, &u8) -> &u8);
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0106, E0605.
For more information about an error, try `rustc --explain E0106`.
For more information about an error, try `rustc --explain E0106`.
------------------------------------------


---- [ui] src/test/ui/rfc1623.rs stdout ----
diff of stderr:

22 LL |     f: &id,
23    |        ^^^ implementation of `FnOnce` is not general enough
24    |
-    = note: `fn(&'2 Foo<'_>) -> &'2 Foo<'_> {id::<&'2 Foo<'_>>}` must implement `FnOnce<(&'1 Foo<'b>,)>`, for any lifetime `'1`...
+    = note: `fn(&'2 Foo<'_>) -> &'2 Foo<'_> {{id::<&'2 Foo<'_>>}}` must implement `FnOnce<(&'1 Foo<'b>,)>`, for any lifetime `'1`...
26    = note: ...but it actually implements `FnOnce<(&'2 Foo<'_>,)>`, for some specific lifetime `'2`
27 
28 error: implementation of `FnOnce` is not general enough
31 LL |     f: &id,
31 LL |     f: &id,
32    |        ^^^ implementation of `FnOnce` is not general enough
33    |
-    = note: `fn(&Foo<'2>) -> &Foo<'2> {id::<&Foo<'2>>}` must implement `FnOnce<(&'a Foo<'1>,)>`, for any lifetime `'1`...
+    = note: `fn(&Foo<'2>) -> &Foo<'2> {{id::<&Foo<'2>>}}` must implement `FnOnce<(&'a Foo<'1>,)>`, for any lifetime `'1`...
35    = note: ...but it actually implements `FnOnce<(&Foo<'2>,)>`, for some specific lifetime `'2`
37 error: aborting due to 4 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623/rfc1623.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc1623.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc1623.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc1623/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/rfc1623.rs:28:8
   |
LL |     f: &id,
   |        ^^^ one type is more general than the other
   |        ^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> Fn<(&'a Foo<'b>,)>`
              found trait `Fn<(&Foo<'_>,)>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/rfc1623.rs:28:8
   |
LL |     f: &id,
LL |     f: &id,
   |        ^^^ one type is more general than the other
   |
   = note: expected trait `for<'a, 'b> Fn<(&'a Foo<'b>,)>`
              found trait `Fn<(&Foo<'_>,)>`

error: implementation of `FnOnce` is not general enough
   |
LL |     f: &id,
LL |     f: &id,
   |        ^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&'2 Foo<'_>) -> &'2 Foo<'_> {{id::<&'2 Foo<'_>>}}` must implement `FnOnce<(&'1 Foo<'b>,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 Foo<'_>,)>`, for some specific lifetime `'2`

error: implementation of `FnOnce` is not general enough
   |
LL |     f: &id,
LL |     f: &id,
   |        ^^^ implementation of `FnOnce` is not general enough
   |
   = note: `fn(&Foo<'2>) -> &Foo<'2> {{id::<&Foo<'2>>}}` must implement `FnOnce<(&'a Foo<'1>,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&Foo<'2>,)>`, for some specific lifetime `'2`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/rfcs/rfc-2396-target_feature-11/fn-traits.rs stdout ----
diff of stderr:

- error[E0277]: expected a `Fn<()>` closure, found `fn() {foo}`
+ error[E0277]: expected a `Fn<()>` closure, found `fn() {{foo}}`
2   --> $DIR/fn-traits.rs:24:10
4 LL |     call(foo);


-    |     ---- ^^^ expected an `Fn<()>` closure, found `fn() {foo}`
+    |     ---- ^^^ expected an `Fn<()>` closure, found `fn() {{foo}}`
7    |     required by a bound introduced by this call
8    |


-    = help: the trait `Fn<()>` is not implemented for `fn() {foo}`
-    = note: wrap the `fn() {foo}` in a closure with no arguments: `|| { /* code */ }`
+    = help: the trait `Fn<()>` is not implemented for `fn() {{foo}}`
+    = note: wrap the `fn() {{foo}}` in a closure with no arguments: `|| { /* code */ }`
11    = note: `#[target_feature]` functions do not implement the `Fn` traits
12 note: required by a bound in `call`
13   --> $DIR/fn-traits.rs:11:17

15 LL | fn call(f: impl Fn()) {
16    |                 ^^^^ required by this bound in `call`
17 
- error[E0277]: expected a `FnMut<()>` closure, found `fn() {foo}`
+ error[E0277]: expected a `FnMut<()>` closure, found `fn() {{foo}}`
19   --> $DIR/fn-traits.rs:25:14
21 LL |     call_mut(foo);


-    |     -------- ^^^ expected an `FnMut<()>` closure, found `fn() {foo}`
+    |     -------- ^^^ expected an `FnMut<()>` closure, found `fn() {{foo}}`
24    |     required by a bound introduced by this call
25    |


-    = help: the trait `FnMut<()>` is not implemented for `fn() {foo}`
-    = note: wrap the `fn() {foo}` in a closure with no arguments: `|| { /* code */ }`
+    = help: the trait `FnMut<()>` is not implemented for `fn() {{foo}}`
+    = note: wrap the `fn() {{foo}}` in a closure with no arguments: `|| { /* code */ }`
28    = note: `#[target_feature]` functions do not implement the `Fn` traits
29 note: required by a bound in `call_mut`
30   --> $DIR/fn-traits.rs:15:21

32 LL | fn call_mut(f: impl FnMut()) {
33    |                     ^^^^^^^ required by this bound in `call_mut`
34 
- error[E0277]: expected a `FnOnce<()>` closure, found `fn() {foo}`
+ error[E0277]: expected a `FnOnce<()>` closure, found `fn() {{foo}}`
36   --> $DIR/fn-traits.rs:26:15
38 LL |     call_once(foo);


-    |     --------- ^^^ expected an `FnOnce<()>` closure, found `fn() {foo}`
+    |     --------- ^^^ expected an `FnOnce<()>` closure, found `fn() {{foo}}`
41    |     required by a bound introduced by this call
42    |


-    = help: the trait `FnOnce<()>` is not implemented for `fn() {foo}`
-    = note: wrap the `fn() {foo}` in a closure with no arguments: `|| { /* code */ }`
+    = help: the trait `FnOnce<()>` is not implemented for `fn() {{foo}}`
+    = note: wrap the `fn() {{foo}}` in a closure with no arguments: `|| { /* code */ }`
45    = note: `#[target_feature]` functions do not implement the `Fn` traits
46 note: required by a bound in `call_once`
47   --> $DIR/fn-traits.rs:19:22

49 LL | fn call_once(f: impl FnOnce()) {
50    |                      ^^^^^^^^ required by this bound in `call_once`
51 
- error[E0277]: expected a `Fn<()>` closure, found `unsafe fn() {foo_unsafe}`
+ error[E0277]: expected a `Fn<()>` closure, found `unsafe fn() {{foo_unsafe}}`
53   --> $DIR/fn-traits.rs:28:10
54    |
---
diff of stderr:

22   --> $DIR/fn-or-tuple-struct-without-args.rs:29:20
23    |
24 LL | fn foo(a: usize, b: usize) -> usize { a }
-    | ----------------------------------- fn(usize, usize) -> usize {foo} defined here
+    | ----------------------------------- fn(usize, usize) -> usize {{foo}} defined here
27 LL |     let _: usize = foo;
28    |            -----   ^^^ expected `usize`, found fn item

30    |            expected due to this
30    |            expected due to this
31    |
32    = note: expected type `usize`
-            found fn item `fn(usize, usize) -> usize {foo}`
+            found fn item `fn(usize, usize) -> usize {{foo}}`
34 help: use parentheses to call this function
35    |
36 LL |     let _: usize = foo(_, _);
40   --> $DIR/fn-or-tuple-struct-without-args.rs:30:16
41    |
41    |
42 LL | struct S(usize, usize);
-    | -------- fn(usize, usize) -> S {S} defined here
+    | -------- fn(usize, usize) -> S {{S}} defined here
44 ...
45 LL |     let _: S = S;
46    |            -   ^ expected struct `S`, found fn item
48    |            expected due to this
49    |
50    = note: expected struct `S`
50    = note: expected struct `S`
-              found fn item `fn(usize, usize) -> S {S}`
+              found fn item `fn(usize, usize) -> S {{S}}`
52 help: use parentheses to instantiate this tuple struct
53    |
54 LL |     let _: S = S(_, _);
58   --> $DIR/fn-or-tuple-struct-without-args.rs:31:20
59    |
60 LL | fn bar() -> usize { 42 }
60 LL | fn bar() -> usize { 42 }
-    | ----------------- fn() -> usize {bar} defined here
+    | ----------------- fn() -> usize {{bar}} defined here
62 ...
63 LL |     let _: usize = bar;
64    |            -----   ^^^ expected `usize`, found fn item
66    |            expected due to this
67    |
68    = note: expected type `usize`
68    = note: expected type `usize`
-            found fn item `fn() -> usize {bar}`
+            found fn item `fn() -> usize {{bar}}`
70 help: use parentheses to call this function
71    |
72 LL |     let _: usize = bar();
76   --> $DIR/fn-or-tuple-struct-without-args.rs:32:16
77    |
78 LL | struct V();
78 LL | struct V();
-    | -------- fn() -> V {V} defined here
+    | -------- fn() -> V {{V}} defined here
80 ...
81 LL |     let _: V = V;
82    |            -   ^ expected struct `V`, found fn item
84    |            expected due to this
85    |
86    = note: expected struct `V`
86    = note: expected struct `V`
-              found fn item `fn() -> V {V}`
+              found fn item `fn() -> V {{V}}`
88 help: use parentheses to instantiate this tuple struct
89    |
90 LL |     let _: V = V();
94   --> $DIR/fn-or-tuple-struct-without-args.rs:33:20
95    |
95    |
96 LL |     fn baz(x: usize, y: usize) -> usize { x }
-    |     ----------------------------------- fn(usize, usize) -> usize {<_ as T>::baz} defined here
+    |     ----------------------------------- fn(usize, usize) -> usize {{<_ as T>::baz}} defined here
98 ...
99 LL |     let _: usize = T::baz;
100    |            -----   ^^^^^^ expected `usize`, found fn item
102    |            expected due to this
103    |
104    = note: expected type `usize`
104    = note: expected type `usize`
-            found fn item `fn(usize, usize) -> usize {<_ as T>::baz}`
+            found fn item `fn(usize, usize) -> usize {{<_ as T>::baz}}`
106 help: use parentheses to call this function
107    |
108 LL |     let _: usize = T::baz(_, _);
112   --> $DIR/fn-or-tuple-struct-without-args.rs:34:20
113    |
113    |
114 LL |     fn bat(x: usize) -> usize { 42 }
-    |     ------------------------- fn(usize) -> usize {<_ as T>::bat} defined here
+    |     ------------------------- fn(usize) -> usize {{<_ as T>::bat}} defined here
116 ...
117 LL |     let _: usize = T::bat;
118    |            -----   ^^^^^^ expected `usize`, found fn item
120    |            expected due to this
121    |
122    = note: expected type `usize`
122    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<_ as T>::bat}`
+            found fn item `fn(usize) -> usize {{<_ as T>::bat}}`
124 help: use parentheses to call this function
125    |
126 LL |     let _: usize = T::bat(_);
130   --> $DIR/fn-or-tuple-struct-without-args.rs:35:16
131    |
132 LL |     A(usize),
132 LL |     A(usize),
-    |     - fn(usize) -> E {E::A} defined here
+    |     - fn(usize) -> E {{E::A}} defined here
134 ...
135 LL |     let _: E = E::A;
136    |            -   ^^^^ expected enum `E`, found fn item
138    |            expected due to this
139    |
140    = note: expected enum `E`
140    = note: expected enum `E`
-            found fn item `fn(usize) -> E {E::A}`
+            found fn item `fn(usize) -> E {{E::A}}`
142 help: use parentheses to instantiate this tuple variant
143    |
144 LL |     let _: E = E::A(_);
148   --> $DIR/fn-or-tuple-struct-without-args.rs:37:20
149    |
149    |
150 LL |     fn baz(x: usize, y: usize) -> usize { x }
-    |     ----------------------------------- fn(usize, usize) -> usize {<X as T>::baz} defined here
+    |     ----------------------------------- fn(usize, usize) -> usize {{<X as T>::baz}} defined here
152 ...
153 LL |     let _: usize = X::baz;
154    |            -----   ^^^^^^ expected `usize`, found fn item
156    |            expected due to this
157    |
158    = note: expected type `usize`
158    = note: expected type `usize`
-            found fn item `fn(usize, usize) -> usize {<X as T>::baz}`
+            found fn item `fn(usize, usize) -> usize {{<X as T>::baz}}`
160 help: use parentheses to call this function
161    |
162 LL |     let _: usize = X::baz(_, _);
166   --> $DIR/fn-or-tuple-struct-without-args.rs:38:20
167    |
167    |
168 LL |     fn bat(x: usize) -> usize { 42 }
-    |     ------------------------- fn(usize) -> usize {<X as T>::bat} defined here
+    |     ------------------------- fn(usize) -> usize {{<X as T>::bat}} defined here
170 ...
171 LL |     let _: usize = X::bat;
172    |            -----   ^^^^^^ expected `usize`, found fn item
174    |            expected due to this
175    |
176    = note: expected type `usize`
176    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<X as T>::bat}`
+            found fn item `fn(usize) -> usize {{<X as T>::bat}}`
178 help: use parentheses to call this function
179    |
180 LL |     let _: usize = X::bat(_);
184   --> $DIR/fn-or-tuple-struct-without-args.rs:39:20
185    |
185    |
186 LL |     fn bax(x: usize) -> usize { 42 }
-    |     ------------------------- fn(usize) -> usize {<X as T>::bax} defined here
+    |     ------------------------- fn(usize) -> usize {{<X as T>::bax}} defined here
188 ...
189 LL |     let _: usize = X::bax;
190    |            -----   ^^^^^^ expected `usize`, found fn item
192    |            expected due to this
193    |
194    = note: expected type `usize`
194    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<X as T>::bax}`
+            found fn item `fn(usize) -> usize {{<X as T>::bax}}`
196 help: use parentheses to call this function
197    |
198 LL |     let _: usize = X::bax(_);
202   --> $DIR/fn-or-tuple-struct-without-args.rs:40:20
203    |
203    |
204 LL |     fn bach(x: usize) -> usize;
-    |     --------------------------- fn(usize) -> usize {<X as T>::bach} defined here
+    |     --------------------------- fn(usize) -> usize {{<X as T>::bach}} defined here
206 ...
207 LL |     let _: usize = X::bach;
208    |            -----   ^^^^^^^ expected `usize`, found fn item
210    |            expected due to this
211    |
212    = note: expected type `usize`
212    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<X as T>::bach}`
+            found fn item `fn(usize) -> usize {{<X as T>::bach}}`
214 help: use parentheses to call this function
215    |
216 LL |     let _: usize = X::bach(_);
220   --> $DIR/fn-or-tuple-struct-without-args.rs:41:20
221    |
221    |
222 LL |     fn ban(&self) -> usize { 42 }
-    |     ---------------------- for<'r> fn(&'r X) -> usize {<X as T>::ban} defined here
+    |     ---------------------- for<'r> fn(&'r X) -> usize {{<X as T>::ban}} defined here
224 ...
225 LL |     let _: usize = X::ban;
226    |            -----   ^^^^^^ expected `usize`, found fn item
228    |            expected due to this
229    |
230    = note: expected type `usize`
230    = note: expected type `usize`
-            found fn item `for<'r> fn(&'r X) -> usize {<X as T>::ban}`
+            found fn item `for<'r> fn(&'r X) -> usize {{<X as T>::ban}}`
232 help: use parentheses to call this function
233    |
234 LL |     let _: usize = X::ban(_);
238   --> $DIR/fn-or-tuple-struct-without-args.rs:42:20
239    |
239    |
240 LL |     fn bal(&self) -> usize;
-    |     ----------------------- for<'r> fn(&'r X) -> usize {<X as T>::bal} defined here
+    |     ----------------------- for<'r> fn(&'r X) -> usize {{<X as T>::bal}} defined here
242 ...
243 LL |     let _: usize = X::bal;
244    |            -----   ^^^^^^ expected `usize`, found fn item
246    |            expected due to this
247    |
248    = note: expected type `usize`
248    = note: expected type `usize`
-            found fn item `for<'r> fn(&'r X) -> usize {<X as T>::bal}`
+            found fn item `for<'r> fn(&'r X) -> usize {{<X as T>::bal}}`
250 help: use parentheses to call this function
251    |
252 LL |     let _: usize = X::bal(_);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-without-args/fn-or-tuple-struct-without-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/fn-or-tuple-struct-without-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-without-args" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-without-args/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0423]: expected value, found struct variant `E::B`
   |
LL |     A(usize),
   |     -------- similarly named tuple variant `A` defined here
   |     -------- similarly named tuple variant `A` defined here
LL |     B { a: usize },
   |     -------------- `E::B` defined here
...
LL |     let _: E = E::B; //~ ERROR expected value, found struct variant `E::B`
   |
help: use struct literal syntax instead
   |
   |
LL |     let _: E = E::B { a: val }; //~ ERROR expected value, found struct variant `E::B`
help: a tuple variant with a similar name exists
   |
   |
LL |     let _: E = E::A; //~ ERROR expected value, found struct variant `E::B`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:29:20
   |
   |
LL | fn foo(a: usize, b: usize) -> usize { a }
   | ----------------------------------- fn(usize, usize) -> usize {{foo}} defined here
...
LL |     let _: usize = foo; //~ ERROR mismatched types
   |            -----   ^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn(usize, usize) -> usize {{foo}}`
help: use parentheses to call this function
   |
LL |     let _: usize = foo(_, _); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:30:16
   |
   |
LL | struct S(usize, usize);
   | -------- fn(usize, usize) -> S {{S}} defined here
...
LL |     let _: S = S; //~ ERROR mismatched types
   |            -   ^ expected struct `S`, found fn item
   |            expected due to this
   |
   = note: expected struct `S`
   = note: expected struct `S`
             found fn item `fn(usize, usize) -> S {{S}}`
help: use parentheses to instantiate this tuple struct
   |
LL |     let _: S = S(_, _); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:31:20
   |
   |
LL | fn bar() -> usize { 42 }
   | ----------------- fn() -> usize {{bar}} defined here
...
LL |     let _: usize = bar; //~ ERROR mismatched types
   |            -----   ^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn() -> usize {{bar}}`
help: use parentheses to call this function
   |
LL |     let _: usize = bar(); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:32:16
   |
   |
LL | struct V();
   | -------- fn() -> V {{V}} defined here
...
LL |     let _: V = V; //~ ERROR mismatched types
   |            -   ^ expected struct `V`, found fn item
   |            expected due to this
   |
   = note: expected struct `V`
   = note: expected struct `V`
             found fn item `fn() -> V {{V}}`
help: use parentheses to instantiate this tuple struct
   |
LL |     let _: V = V(); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:33:20
   |
   |
LL |     fn baz(x: usize, y: usize) -> usize { x }
   |     ----------------------------------- fn(usize, usize) -> usize {{<_ as T>::baz}} defined here
...
LL |     let _: usize = T::baz; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn(usize, usize) -> usize {{<_ as T>::baz}}`
help: use parentheses to call this function
   |
LL |     let _: usize = T::baz(_, _); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:34:20
   |
   |
LL |     fn bat(x: usize) -> usize { 42 }
   |     ------------------------- fn(usize) -> usize {{<_ as T>::bat}} defined here
...
LL |     let _: usize = T::bat; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn(usize) -> usize {{<_ as T>::bat}}`
help: use parentheses to call this function
   |
LL |     let _: usize = T::bat(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:35:16
   |
   |
LL |     A(usize),
   |     - fn(usize) -> E {{E::A}} defined here
...
LL |     let _: E = E::A; //~ ERROR mismatched types
   |            -   ^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `fn(usize) -> E {{E::A}}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = E::A(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:37:20
   |
   |
LL |     fn baz(x: usize, y: usize) -> usize { x }
   |     ----------------------------------- fn(usize, usize) -> usize {{<X as T>::baz}} defined here
...
LL |     let _: usize = X::baz; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn(usize, usize) -> usize {{<X as T>::baz}}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::baz(_, _); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:38:20
   |
   |
LL |     fn bat(x: usize) -> usize { 42 }
   |     ------------------------- fn(usize) -> usize {{<X as T>::bat}} defined here
...
LL |     let _: usize = X::bat; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn(usize) -> usize {{<X as T>::bat}}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bat(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:39:20
   |
   |
LL |     fn bax(x: usize) -> usize { 42 }
   |     ------------------------- fn(usize) -> usize {{<X as T>::bax}} defined here
...
LL |     let _: usize = X::bax; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn(usize) -> usize {{<X as T>::bax}}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bax(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:40:20
   |
   |
LL |     fn bach(x: usize) -> usize;
   |     --------------------------- fn(usize) -> usize {{<X as T>::bach}} defined here
...
LL |     let _: usize = X::bach; //~ ERROR mismatched types
   |            -----   ^^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `fn(usize) -> usize {{<X as T>::bach}}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bach(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:41:20
   |
   |
LL |     fn ban(&self) -> usize { 42 }
   |     ---------------------- for<'r> fn(&'r X) -> usize {{<X as T>::ban}} defined here
...
LL |     let _: usize = X::ban; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `for<'r> fn(&'r X) -> usize {{<X as T>::ban}}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::ban(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:42:20
   |
   |
LL |     fn bal(&self) -> usize;
   |     ----------------------- for<'r> fn(&'r X) -> usize {{<X as T>::bal}} defined here
...
LL |     let _: usize = X::bal; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `for<'r> fn(&'r X) -> usize {{<X as T>::bal}}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bal(_); //~ ERROR mismatched types

---

5    |                   ^^^^ expected `bool`, found fn item
6    |
7    = note: expected type `bool`
-            found fn item `fn() {main}`
+            found fn item `fn() {{main}}`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-type-error/while-type-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args while-type-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/while-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-type-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-type-error/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/while-type-error.rs:3:19
   |
   |
LL | fn main() { while main { } }
   |                   ^^^^ expected `bool`, found fn item
   = note: expected type `bool`
   = note: expected type `bool`
           found fn item `fn() {{main}}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
