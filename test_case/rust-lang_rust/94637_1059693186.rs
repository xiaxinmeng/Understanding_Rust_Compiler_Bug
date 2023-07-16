plain
............................................iiiiiiiiiiii...............ii.................i....i.... 200/12698
.....F.............................................................................................. 300/12698
.................................................................................................... 400/12698
.................................................................................................... 500/12698
..............................F....F..........i..................................................... 600/12698
...................i..i............................................................................. 800/12698
.....i.............................................................................................. 900/12698
.....i.............................................................................................. 900/12698
.................................F.F................................................................ 1000/12698
.................................................................................................... 1200/12698
..................................i................................................................. 1300/12698
..................................i................................................................. 1300/12698
...........F......................F.................................i............................... 1400/12698
.............................................................................................F...... 1600/12698
...........F.......................................i................................................ 1700/12698
.................................................................................................... 1800/12698
....................................................................i............................... 1900/12698
---
...........................iiiii.................................................................... 3200/12698
.................................................................................................... 3300/12698
..F................................................................................................. 3400/12698
.................................................................................................... 3500/12698
.............................................................................F...................... 3600/12698
..............................................................F......i..........i.........i........F 3700/12698
...........................................i........................................................ 3900/12698
.....................................F...................................i.......................... 4000/12698
.......................................................................i.................i.......... 4100/12698
.............i...................................................................................... 4200/12698
.............i...................................................................................... 4200/12698
.................................................................................................... 4300/12698
.................................................................................................... 4400/12698
............F.......................................F...F.............................F.........F... 4500/12698
........................F........................................................................... 4700/12698
.................................................................................................... 4800/12698
...................................................................i.........................F...... 4900/12698
...............................i.................................................................... 5000/12698
---
................................F................................................................... 5900/12698
.................................................................................................... 6000/12698
.................................................................................................... 6100/12698
......................................i............................................................. 6200/12698
............................F....................................................................... 6300/12698
...................i.....F.................F.................................F...................... 6400/12698
....F...............................................................................i..............F 6500/12698
.................................................................................................... 6700/12698
.................................................................................................... 6700/12698
.............................i.F...i.......................................i................i....... 6800/12698
.............................................................i...................................... 7000/12698
.............................F...................................................................... 7100/12698
................................................................................................ii.. 7200/12698
..............................ii.........................................................i.......... 7300/12698
---
............................................................................i....................... 8900/12698
.................................................................................................... 9000/12698
.................................................................................................... 9100/12698
.................................................................................................... 9200/12698
.......................................................................F...F........................ 9300/12698
..................F........F............F........................................................... 9400/12698
......i.........................................................ii........................F......... 9600/12698
.................................................................................................... 9700/12698
.................................................................................................... 9800/12698
....................................................................................F............... 9900/12698
---
.................................................................................................... 10700/12698
.................................................................................................... 10800/12698
...................................................................................F................ 10900/12698
.................................................................................................... 11000/12698
................................................................................F................... 11100/12698
.........F...................F...................................................................... 11200/12698
........i.....i............................i........................................................ 11400/12698
.................................................................................................... 11500/12698
.................................................................................................... 11600/12698
.................................................................................................... 11700/12698
---
---- [ui] ui/asm/x86_64/type-check-2.rs stdout ----
diff of stderr:

39    |
40    = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
41 
- error: cannot use value of type `fn() {main}` for inline assembly
+ error: cannot use value of type `{individual function type for main}` for inline assembly
44    |
44    |
45 LL |         asm!("{}", inout(reg) f);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2/type-check-2.stderr
To only update this specific test, also pass `--test-args asm/x86_64/type-check-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/type-check-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/type-check-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: arguments for inline assembly must be copyable
   |
   |
LL |         asm!("{}", in(xmm_reg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
   |
   |
   = note: `SimdNonCopy` does not implement the Copy trait

error: cannot use value of type `[closure@/checkout/src/test/ui/asm/x86_64/type-check-2.rs:56:28: 56:38]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) |x: i32| x);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `Vec<i32>` for inline assembly
  --> /checkout/src/test/ui/asm/x86_64/type-check-2.rs:58:28
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

error: cannot use value of type `{individual function type for main}` for inline assembly
   |
   |
LL |         asm!("{}", inout(reg) f);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `&mut i32` for inline assembly
  --> /checkout/src/test/ui/asm/x86_64/type-check-2.rs:73:31
   |
   |
LL |         asm!("{}", inout(reg) r);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: asm `sym` operand must point to a fn or static
   |
   |
LL |         asm!("{}", sym C);


error: asm `sym` operand must point to a fn or static
   |
   |
LL |         asm!("{}", sym x);


error[E0381]: use of possibly-uninitialized variable: `x`
   |
   |
LL |         asm!("{}", in(reg) x);
   |                            ^ use of possibly-uninitialized `x`

error[E0381]: use of possibly-uninitialized variable: `y`
   |
   |
LL |         asm!("{}", inout(reg) y);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
   |
   |
LL |         let v: Vec<u64> = vec![0, 1, 2];
   |             - help: consider changing this to be mutable: `mut v`
LL |         asm!("{}", in(reg) v[0]);
LL |         asm!("{}", out(reg) v[0]);
   |                             ^ cannot borrow as mutable

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
   |
   |
LL |         let v: Vec<u64> = vec![0, 1, 2];
   |             - help: consider changing this to be mutable: `mut v`
...
LL |         asm!("{}", inout(reg) v[0]);
   |                               ^ cannot borrow as mutable
error: aborting due to 13 previous errors

Some errors have detailed explanations: E0381, E0596.
For more information about an error, try `rustc --explain E0381`.
---
diff of stderr:

2   --> $DIR/substs-ppaux.rs:16:17
3    |
4 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>} defined here
+    |     --------------------------- {individual function type for <i8 as Foo<'static, 'static, u8>>::bar::<'static, char>} defined here
6 ...
7 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
8    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
10    |            expected due to this
11    |
12    = note: expected unit type `()`
12    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}`
+                 found fn item `{individual function type for <i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}`
14 help: use parentheses to call this function
15    |
16 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();
20   --> $DIR/substs-ppaux.rs:25:17
21    |
21    |
22 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<'static, 'static>>::bar::<'static, char>} defined here
+    |     --------------------------- {individual function type for <i8 as Foo<'static, 'static>>::bar::<'static, char>} defined here
24 ...
25 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
26    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
28    |            expected due to this
29    |
30    = note: expected unit type `()`
30    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<'static, 'static>>::bar::<'static, char>}`
+                 found fn item `{individual function type for <i8 as Foo<'static, 'static>>::bar::<'static, char>}`
32 help: use parentheses to call this function
33    |
34 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();
38   --> $DIR/substs-ppaux.rs:33:17
39    |
39    |
40 LL |     fn baz() {}
-    |     -------- fn() {<i8 as Foo<'static, 'static, u8>>::baz} defined here
+    |     -------- {individual function type for <i8 as Foo<'static, 'static, u8>>::baz} defined here
42 ...
43 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
44    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
46    |            expected due to this
47    |
48    = note: expected unit type `()`
48    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<'static, 'static, u8>>::baz}`
+                 found fn item `{individual function type for <i8 as Foo<'static, 'static, u8>>::baz}`
50 help: use parentheses to call this function
51    |
52 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();
56   --> $DIR/substs-ppaux.rs:41:17
57    |
57    |
58 LL | fn foo<'z>() where &'z (): Sized {
-    | -------------------------------- fn() {foo::<'static>} defined here
+    | -------------------------------- {individual function type for foo::<'static>} defined here
60 ...
61 LL |     let x: () = foo::<'static>;
62    |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
64    |            expected due to this
65    |
66    = note: expected unit type `()`
66    = note: expected unit type `()`
-                 found fn item `fn() {foo::<'static>}`
+                 found fn item `{individual function type for foo::<'static>}`
68 help: use parentheses to call this function
69    |
70 LL |     let x: () = foo::<'static>();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal/substs-ppaux.normal.stderr
To only update this specific test, also pass `--test-args associated-types/substs-ppaux.rs`


error in revision `normal`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "normal" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:16:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- {individual function type for <i8 as Foo<'static, 'static, u8>>::bar::<'static, char>} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for <i8 as Foo<'static, 'static, u8>>::bar::<'static, char>}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:25:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- {individual function type for <i8 as Foo<'static, 'static>>::bar::<'static, char>} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for <i8 as Foo<'static, 'static>>::bar::<'static, char>}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:33:17
   |
   |
LL |     fn baz() {}
   |     -------- {individual function type for <i8 as Foo<'static, 'static, u8>>::baz} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for <i8 as Foo<'static, 'static, u8>>::baz}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:41:17
   |
   |
LL | fn foo<'z>() where &'z (): Sized {
   | -------------------------------- {individual function type for foo::<'static>} defined here
...
LL |     let x: () = foo::<'static>;
   |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for foo::<'static>}`
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
---
diff of stderr:

2   --> $DIR/substs-ppaux.rs:16:17
3    |
4 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>} defined here
+    |     --------------------------- {individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>} defined here
6 ...
7 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
8    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
10    |            expected due to this
11    |
12    = note: expected unit type `()`
12    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}`
+                 found fn item `{individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}`
14 help: use parentheses to call this function
15    |
16 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();
20   --> $DIR/substs-ppaux.rs:25:17
21    |
21    |
22 LL |     fn bar<'a, T>() where T: 'a {}
-    |     --------------------------- fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
+    |     --------------------------- {individual function type for <i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
24 ...
25 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
26    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
28    |            expected due to this
29    |
30    = note: expected unit type `()`
30    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}`
+                 found fn item `{individual function type for <i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}`
32 help: use parentheses to call this function
33    |
34 LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();
38   --> $DIR/substs-ppaux.rs:33:17
39    |
39    |
40 LL |     fn baz() {}
-    |     -------- fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz} defined here
+    |     -------- {individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::baz} defined here
42 ...
43 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
44    |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
46    |            expected due to this
47    |
48    = note: expected unit type `()`
48    = note: expected unit type `()`
-                 found fn item `fn() {<i8 as Foo<ReStatic, ReStatic, u8>>::baz}`
+                 found fn item `{individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::baz}`
50 help: use parentheses to call this function
51    |
52 LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();
56   --> $DIR/substs-ppaux.rs:41:17
57    |
57    |
58 LL | fn foo<'z>() where &'z (): Sized {
-    | -------------------------------- fn() {foo::<ReStatic>} defined here
+    | -------------------------------- {individual function type for foo::<ReStatic>} defined here
60 ...
61 LL |     let x: () = foo::<'static>;
62    |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
64    |            expected due to this
65    |
66    = note: expected unit type `()`
66    = note: expected unit type `()`
-                 found fn item `fn() {foo::<ReStatic>}`
+                 found fn item `{individual function type for foo::<ReStatic>}`
68 help: use parentheses to call this function
69    |
70 LL |     let x: () = foo::<'static>();

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/substs-ppaux.verbose.stderr
To only update this specific test, also pass `--test-args associated-types/substs-ppaux.rs`


error in revision `verbose`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "verbose" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:16:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- {individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::bar::<ReStatic, char>}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:25:17
   |
   |
LL |     fn bar<'a, T>() where T: 'a {}
   |     --------------------------- {individual function type for <i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for <i8 as Foo<ReStatic, ReStatic>>::bar::<ReStatic, char>}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u32>>::bar::<'static, char>();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:33:17
   |
   |
LL |     fn baz() {}
   |     -------- {individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::baz} defined here
...
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz;
   |            --   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for <i8 as Foo<ReStatic, ReStatic, u8>>::baz}`
help: use parentheses to call this function
   |
LL |     let x: () = <i8 as Foo<'static, 'static,  u8>>::baz();

error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/substs-ppaux.rs:41:17
   |
   |
LL | fn foo<'z>() where &'z (): Sized {
   | -------------------------------- {individual function type for foo::<ReStatic>} defined here
...
LL |     let x: () = foo::<'static>;
   |            --   ^^^^^^^^^^^^^^ expected `()`, found fn item
   |            expected due to this
   |
   = note: expected unit type `()`
   = note: expected unit type `()`
                found fn item `{individual function type for foo::<ReStatic>}`
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
Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] ui/binop/issue-77910-2.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {foo}`
+ error[E0369]: binary operation `==` cannot be applied to type `{individual function type for foo}`
3    |
3    |
4 LL |     if foo == y {}

5    |        --- ^^ - _
6    |        |
-    |        for<'r> fn(&'r i32) -> &'r i32 {foo}
+    |        {individual function type for foo}
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-2/issue-77910-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/issue-77910-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `{individual function type for foo}`
   |
   |
LL |     if foo == y {}
   |        --- ^^ - _
   |        |
   |        {individual function type for foo}
error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
------------------------------------------
------------------------------------------


---- [ui] ui/binop/issue-77910-1.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {foo}`
+ error[E0369]: binary operation `==` cannot be applied to type `{individual function type for foo}`
3    |
3    |
4 LL |     assert_eq!(foo, y);
5    |     ^^^^^^^^^^^^^^^^^^
6    |     |
6    |     |
-    |     for<'r> fn(&'r i32) -> &'r i32 {foo}
+    |     {individual function type for foo}
9    |
9    |
10    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
11 
- error[E0277]: `for<'r> fn(&'r i32) -> &'r i32 {foo}` doesn't implement `Debug`
+ error[E0277]: `{individual function type for foo}` doesn't implement `Debug`
14    |
14    |
15 LL |     assert_eq!(foo, y);

-    |     ^^^^^^^^^^^^^^^^^^ `for<'r> fn(&'r i32) -> &'r i32 {foo}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^ `{individual function type for foo}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
17    |
-    = help: the trait `Debug` is not implemented for `for<'r> fn(&'r i32) -> &'r i32 {foo}`
+    = help: the trait `Debug` is not implemented for `{individual function type for foo}`
19    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
21 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/issue-77910-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/issue-77910-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `{individual function type for foo}`
   |
   |
LL |     assert_eq!(foo, y);
   |     |
   |     |
   |     {individual function type for foo}
   |
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `{individual function type for foo}` doesn't implement `Debug`
   |
   |
LL |     assert_eq!(foo, y);
   |     ^^^^^^^^^^^^^^^^^^ `{individual function type for foo}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `{individual function type for foo}`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] ui/c-variadic/issue-32201.rs stdout ----
diff of stderr:

- error[E0617]: can't pass `fn(*const u8) {bar}` to variadic function
+ error[E0617]: can't pass `{individual function type for bar}` to variadic function
3    |
3    |
4 LL |         foo(0, bar);

-    |                ^^^ help: cast the value to `fn(*const u8)`: `bar as fn(*const u8)`
+    |                ^^^ help: cast the value to a function pointer: `bar as fn(*const u8)`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/issue-32201/issue-32201.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args c-variadic/issue-32201.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/issue-32201.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/issue-32201" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/issue-32201/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0617]: can't pass `{individual function type for bar}` to variadic function
   |
   |
LL |         foo(0, bar);
   |                ^^^ help: cast the value to a function pointer: `bar as fn(*const u8)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0617`.
------------------------------------------
------------------------------------------


---- [ui] ui/cast/cast-to-bare-fn.rs stdout ----
diff of stderr:

- error[E0605]: non-primitive cast: `fn(isize) {foo}` as `extern "C" fn() -> isize`
+ error[E0605]: non-primitive cast: `{individual function type for foo}` as `extern "C" fn() -> isize`
3    |
4 LL |     let x = foo as extern "C" fn() -> isize;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-to-bare-fn/cast-to-bare-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cast/cast-to-bare-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cast/cast-to-bare-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-to-bare-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/cast-to-bare-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0605]: non-primitive cast: `{individual function type for foo}` as `extern "C" fn() -> isize`
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


---- [ui] ui/closures/coerce-unsafe-to-closure.rs stdout ----
diff of stderr:

- error[E0277]: expected a `FnOnce<(&str,)>` closure, found `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
+ error[E0277]: expected a `FnOnce<(&str,)>` closure, found `{individual function type for transmute::<_, _>}`
2   --> $DIR/coerce-unsafe-to-closure.rs:2:44
3    |
4 LL |     let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);

-    |                                        --- ^^^^^^^^^^^^^^^^^^^ expected an `FnOnce<(&str,)>` closure, found `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
+    |                                        --- ^^^^^^^^^^^^^^^^^^^ expected an `FnOnce<(&str,)>` closure, found `{individual function type for transmute::<_, _>}`
7    |                                        required by a bound introduced by this call
8    |


-    = help: the trait `FnOnce<(&str,)>` is not implemented for `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
+    = help: the trait `FnOnce<(&str,)>` is not implemented for `{individual function type for transmute::<_, _>}`
10 note: required by a bound in `Option::<T>::map`
11   --> $SRC_DIR/core/src/option.rs:LL:COL


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure/coerce-unsafe-to-closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure/coerce-unsafe-to-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/coerce-unsafe-to-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/coerce-unsafe-to-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `FnOnce<(&str,)>` closure, found `{individual function type for transmute::<_, _>}`
   |
   |
LL |     let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);
   |                                        --- ^^^^^^^^^^^^^^^^^^^ expected an `FnOnce<(&str,)>` closure, found `{individual function type for transmute::<_, _>}`
   |                                        required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<(&str,)>` is not implemented for `{individual function type for transmute::<_, _>}`
note: required by a bound in `Option::<T>::map`
   |
   |
LL |         F: ~const FnOnce(T) -> U,
   |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] ui/closures/closure_cap_coerce_many_fail.rs stdout ----
diff of stderr:

4 LL |       let _ = match "+" {
5    |  _____________-
6 LL | |         "+" => add,
-    | |                --- this is found to be of type `fn(i32, i32) -> i32 {add}`
+    | |                --- this is found to be of type `{individual function type for add}`
8 LL | |         "-" => |a, b| (a - b + cap) as i32,
9    | |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn item, found closure
10 LL | |         _ => unimplemented!(),
11 LL | |     };
12    | |_____- `match` arms have incompatible types
13    |
13    |
-    = note: expected type `fn(i32, i32) -> i32 {add}`
+    = note: expected type `{individual function type for add}`
15            found closure `[closure@$DIR/closure_cap_coerce_many_fail.rs:9:16: 9:43]`
17 error[E0308]: `match` arms have incompatible types


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure_cap_coerce_many_fail/closure_cap_coerce_many_fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/closure_cap_coerce_many_fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure_cap_coerce_many_fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure_cap_coerce_many_fail/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:9:16
   |
   |
LL |       let _ = match "+" {
   |  _____________-
LL | |         "+" => add,
   | |                --- this is found to be of type `{individual function type for add}`
LL | |         "-" => |a, b| (a - b + cap) as i32,
   | |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn item, found closure
LL | |         _ => unimplemented!(),
LL | |     };
   |
   |
   = note: expected type `{individual function type for add}`
           found closure `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:9:16: 9:43]`
error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:18:16
   |
   |
LL |       let _ = match "+" {
   |  _____________-
LL | |         "+" => |a, b| (a + b) as i32,
   | |                --------------------- this is found to be of type `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:17:16: 17:37]`
LL | |         "-" => |a, b| (a - b + cap) as i32,
   | |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected closure, found a different closure
LL | |         _ => unimplemented!(),
LL | |     };
   |
   |
   = note: expected type `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:17:16: 17:37]`
           found closure `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:18:16: 18:43]`
   = note: no two closures, even if identical, have the same type
   = help: consider boxing your closure and/or using it as a trait object
error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:27:16
   |
   |
LL |       let _ = match "+" {
   |  _____________-
LL | |         "+" => |a, b| (a + b + cap) as i32,
   | |                --------------------------- this is found to be of type `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:26:16: 26:43]`
LL | |         "-" => |a, b| (a - b) as i32,
   | |                ^^^^^^^^^^^^^^^^^^^^^ expected closure, found a different closure
LL | |         _ => unimplemented!(),
LL | |     };
   |
   |
   = note: expected type `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:26:16: 26:43]`
           found closure `[closure@/checkout/src/test/ui/closures/closure_cap_coerce_many_fail.rs:27:16: 27:37]`
---
diff of stderr:

7    |            expected due to this
8    |
9    = note: expected reference `&for<'r> fn(&'r B) -> u32`
-               found reference `&for<'r> fn(&'r B) -> u32 {B::func}`
+               found reference `&{individual function type for B::func}`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322/issue-24322.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-24322.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24322.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24322/auxiliary"
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
              found reference `&{individual function type for B::func}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-29124.rs stdout ----
diff of stderr:

- error[E0599]: no method named `x` found for fn item `fn() -> Ret {Obj::func}` in the current scope
+ error[E0599]: no method named `x` found for fn item `{individual function type for Obj::func}` in the current scope
3    |
3    |
4 LL |     Obj::func.x();

-    |               ^ method not found in `fn() -> Ret {Obj::func}`
+    |               ^ method not found in `{individual function type for Obj::func}`
6    |
7    = note: `Obj::func` is a function, perhaps you wish to call it


- error[E0599]: no method named `x` found for fn item `fn() -> Ret {func}` in the current scope
+ error[E0599]: no method named `x` found for fn item `{individual function type for func}` in the current scope
11    |
12 LL |     func.x();


-    |          ^ method not found in `fn() -> Ret {func}`
+    |          ^ method not found in `{individual function type for func}`
14    |
15    = note: `func` is a function, perhaps you wish to call it


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29124/issue-29124.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29124/issue-29124.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-29124.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-29124.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29124" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29124/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `x` found for fn item `{individual function type for Obj::func}` in the current scope
   |
   |
LL |     Obj::func.x();
   |               ^ method not found in `{individual function type for Obj::func}`
   |
   = note: `Obj::func` is a function, perhaps you wish to call it

error[E0599]: no method named `x` found for fn item `{individual function type for func}` in the current scope
   |
LL |     func.x();
LL |     func.x();
   |          ^ method not found in `{individual function type for func}`
   |
   = note: `func` is a function, perhaps you wish to call it
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-35241.rs stdout ----
diff of stderr:

2   --> $DIR/issue-35241.rs:3:20
3    |
4 LL | struct Foo(u32);
-    | ---------------- fn(u32) -> Foo {Foo} defined here
+    | ---------------- {individual function type for Foo} defined here
7 LL | fn test() -> Foo { Foo }
8    |              ---   ^^^ expected struct `Foo`, found fn item

10    |              expected `Foo` because of return type
10    |              expected `Foo` because of return type
11    |
12    = note: expected struct `Foo`
-              found fn item `fn(u32) -> Foo {Foo}`
+              found fn item `{individual function type for Foo}`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-35241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-35241/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-35241.rs:3:20
   |
LL | struct Foo(u32);
LL | struct Foo(u32);
   | ---------------- {individual function type for Foo} defined here
LL | 
LL | fn test() -> Foo { Foo } //~ ERROR mismatched types
   |              ---   ^^^ expected struct `Foo`, found fn item
   |              expected `Foo` because of return type
   |
   = note: expected struct `Foo`
   = note: expected struct `Foo`
             found fn item `{individual function type for Foo}`
help: use parentheses to instantiate this tuple struct
   |
LL | fn test() -> Foo { Foo(_) } //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] ui/issues/issue-5216.rs stdout ----
diff of stderr:

5    |                     ^ expected struct `Box`, found fn item
6    |
7    = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
-              found fn item `fn() {f}`
+              found fn item `{individual function type for f}`
10 error[E0308]: mismatched types
11   --> $DIR/issue-5216.rs:8:19


14    |                   ^ expected struct `Box`, found fn item
15    |
16    = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
-              found fn item `fn() {g}`
+              found fn item `{individual function type for g}`
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216/issue-5216.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-5216.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5216.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5216/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-5216.rs:3:21
   |
   |
LL | pub static C: S = S(f); //~ ERROR mismatched types
   |                     ^ expected struct `Box`, found fn item
   |
   = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
             found fn item `{individual function type for f}`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-5216.rs:8:19
   |
   |
LL | pub static D: T = g; //~ ERROR mismatched types
   |                   ^ expected struct `Box`, found fn item
   |
   = note: expected struct `Box<(dyn FnMut() + Sync + 'static)>`
             found fn item `{individual function type for g}`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-59488.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
+ error[E0369]: binary operation `>` cannot be applied to type `{individual function type for foo}`
3    |
4 LL |     foo > 12;


5    |     --- ^ -- {integer}
-    |     fn() -> i32 {foo}
-    |     fn() -> i32 {foo}
+    |     {individual function type for foo}
8    |
9 help: you might have forgotten to call this function

17 LL |     foo > 12;
18    |           ^^ expected fn item, found integer
19    |
19    |
-    = note: expected fn item `fn() -> i32 {foo}`
+    = note: expected fn item `{individual function type for foo}`
22 
22 
- error[E0369]: binary operation `>` cannot be applied to type `fn(i64) -> i64 {bar}`
+ error[E0369]: binary operation `>` cannot be applied to type `{individual function type for bar}`
25    |
26 LL |     bar > 13;


27    |     --- ^ -- {integer}
28    |     |
-    |     fn(i64) -> i64 {bar}
+    |     {individual function type for bar}
30    |
31 help: you might have forgotten to call this function

39 LL |     bar > 13;
40    |           ^^ expected fn item, found integer
41    |
41    |
-    = note: expected fn item `fn(i64) -> i64 {bar}`
+    = note: expected fn item `{individual function type for bar}`
44 
44 
- error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
+ error[E0369]: binary operation `>` cannot be applied to type `{individual function type for foo}`
47    |
48 LL |     foo > foo;


-    |     --- ^ --- fn() -> i32 {foo}
+    |     --- ^ --- {individual function type for foo}
-    |     fn() -> i32 {foo}
-    |     fn() -> i32 {foo}
+    |     {individual function type for foo}
52    |
53 help: you might have forgotten to call this function


59 LL |     foo > foo();
61 
61 
- error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
+ error[E0369]: binary operation `>` cannot be applied to type `{individual function type for foo}`
64    |
64    |
65 LL |     foo > bar;

-    |     --- ^ --- fn(i64) -> i64 {bar}
+    |     --- ^ --- {individual function type for bar}
-    |     fn() -> i32 {foo}
-    |     fn() -> i32 {foo}
+    |     {individual function type for foo}
70 error[E0308]: mismatched types
71   --> $DIR/issue-59488.rs:25:11


76    = note: expected fn item `fn() -> i32 {foo}`
77               found fn item `fn(i64) -> i64 {bar}`
78 
- error[E0369]: binary operation `==` cannot be applied to type `fn(usize) -> Foo {Foo::Bar}`
+ error[E0369]: binary operation `==` cannot be applied to type `{individual function type for Foo::Bar}`
81    |
81    |
82 LL |     assert_eq!(Foo::Bar, i);
83    |     ^^^^^^^^^^^^^^^^^^^^^^^
84    |     |
84    |     |
-    |     fn(usize) -> Foo {Foo::Bar}
-    |     fn(usize) -> Foo {Foo::Bar}
+    |     {individual function type for Foo::Bar}
+    |     {individual function type for Foo::Bar}
87    |
88    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


- error[E0277]: `fn(usize) -> Foo {Foo::Bar}` doesn't implement `Debug`
+ error[E0277]: `{individual function type for Foo::Bar}` doesn't implement `Debug`
92    |
92    |
93 LL |     assert_eq!(Foo::Bar, i);

-    |     ^^^^^^^^^^^^^^^^^^^^^^^ `fn(usize) -> Foo {Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ `{individual function type for Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
95    |
-    = help: the trait `Debug` is not implemented for `fn(usize) -> Foo {Foo::Bar}`
+    = help: the trait `Debug` is not implemented for `{individual function type for Foo::Bar}`
97    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
99 error: aborting due to 9 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/issue-59488.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-59488.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59488.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59488/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `>` cannot be applied to type `{individual function type for foo}`
   |
LL |     foo > 12;
LL |     foo > 12;
   |     --- ^ -- {integer}
   |     |
   |     {individual function type for foo}
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
   = note: expected fn item `{individual function type for foo}`


error[E0369]: binary operation `>` cannot be applied to type `{individual function type for bar}`
   |
LL |     bar > 13;
LL |     bar > 13;
   |     --- ^ -- {integer}
   |     |
   |     {individual function type for bar}
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
   = note: expected fn item `{individual function type for bar}`


error[E0369]: binary operation `>` cannot be applied to type `{individual function type for foo}`
   |
LL |     foo > foo;
LL |     foo > foo;
   |     --- ^ --- {individual function type for foo}
   |     |
   |     {individual function type for foo}
   |
help: you might have forgotten to call this function
LL |     foo() > foo;
   |        ++
   |        ++
help: you might have forgotten to call this function
   |
LL |     foo > foo();


error[E0369]: binary operation `>` cannot be applied to type `{individual function type for foo}`
   |
   |
LL |     foo > bar;
   |     --- ^ --- {individual function type for bar}
   |     |
   |     {individual function type for foo}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59488.rs:25:11
   |
   |
LL |     foo > bar;
   |           ^^^ expected fn item, found a different fn item
   |
   = note: expected fn item `fn() -> i32 {foo}`
              found fn item `fn(i64) -> i64 {bar}`

error[E0369]: binary operation `==` cannot be applied to type `{individual function type for Foo::Bar}`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     |
   |     |
   |     {individual function type for Foo::Bar}
   |     {individual function type for Foo::Bar}
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `{individual function type for Foo::Bar}` doesn't implement `Debug`
   |
   |
LL |     assert_eq!(Foo::Bar, i);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ `{individual function type for Foo::Bar}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `{individual function type for Foo::Bar}`
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
+    |     - ^^ -------- {individual function type for A::Value}
7    |     A
8    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/issue-62375.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-62375.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-62375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `A`
   |
   |
LL |     a == A::Value;
   |     - ^^ -------- {individual function type for A::Value}
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


---- [ui] ui/issues/issue-66667-function-cmp-cycle.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `fn() {second}`
+ error[E0369]: binary operation `==` cannot be applied to type `{individual function type for second}`
3    |
4 LL |     second == 1


5    |     ------ ^^ - {integer}
6    |     |
-    |     fn() {second}
+    |     {individual function type for second}
9 error[E0308]: mismatched types
10   --> $DIR/issue-66667-function-cmp-cycle.rs:2:15

12 LL |     second == 1
12 LL |     second == 1
13    |               ^ expected fn item, found integer
14    |
-    = note: expected fn item `fn() {second}`
+    = note: expected fn item `{individual function type for second}`
16                  found type `{integer}`
17 
- error[E0369]: binary operation `==` cannot be applied to type `fn() {first}`
+ error[E0369]: binary operation `==` cannot be applied to type `{individual function type for first}`
20    |
21 LL |     first == 1


22    |     ----- ^^ - {integer}
-    |     fn() {first}
-    |     fn() {first}
+    |     {individual function type for first}
26 error[E0308]: mismatched types
27   --> $DIR/issue-66667-function-cmp-cycle.rs:7:14

29 LL |     first == 1
29 LL |     first == 1
30    |              ^ expected fn item, found integer
31    |
-    = note: expected fn item `fn() {first}`
+    = note: expected fn item `{individual function type for first}`
33                  found type `{integer}`
34 
- error[E0369]: binary operation `==` cannot be applied to type `fn() {bar}`
+ error[E0369]: binary operation `==` cannot be applied to type `{individual function type for bar}`
37    |
38 LL |     bar == 1


39    |     --- ^^ - {integer}
-    |     fn() {bar}
-    |     fn() {bar}
+    |     {individual function type for bar}
43 error[E0308]: mismatched types
44   --> $DIR/issue-66667-function-cmp-cycle.rs:12:12

46 LL |     bar == 1
46 LL |     bar == 1
47    |            ^ expected fn item, found integer
48    |
-    = note: expected fn item `fn() {bar}`
+    = note: expected fn item `{individual function type for bar}`
50                  found type `{integer}`
52 error: aborting due to 6 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66667-function-cmp-cycle/issue-66667-function-cmp-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-66667-function-cmp-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66667-function-cmp-cycle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66667-function-cmp-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `{individual function type for second}`
   |
   |
LL |     second == 1 //~ ERROR binary operation
   |     ------ ^^ - {integer}
   |     |
   |     {individual function type for second}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs:2:15
   |
   |
LL |     second == 1 //~ ERROR binary operation
   |               ^ expected fn item, found integer
   |
   = note: expected fn item `{individual function type for second}`
                 found type `{integer}`

error[E0369]: binary operation `==` cannot be applied to type `{individual function type for first}`
   |
   |
LL |     first == 1 //~ ERROR binary operation
   |     ----- ^^ - {integer}
   |     |
   |     {individual function type for first}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs:7:14
   |
   |
LL |     first == 1 //~ ERROR binary operation
   |              ^ expected fn item, found integer
   |
   = note: expected fn item `{individual function type for first}`
                 found type `{integer}`

error[E0369]: binary operation `==` cannot be applied to type `{individual function type for bar}`
   |
   |
LL |     bar == 1 //~ ERROR binary operation
   |     --- ^^ - {integer}
   |     |
   |     {individual function type for bar}
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-66667-function-cmp-cycle.rs:12:12
   |
   |
LL |     bar == 1 //~ ERROR binary operation
   |            ^ expected fn item, found integer
   |
   = note: expected fn item `{individual function type for bar}`
                 found type `{integer}`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs stdout ----
diff of stderr:

- error[E0369]: binary operation `==` cannot be applied to type `fn() -> i32 {a}`
+ error[E0369]: binary operation `==` cannot be applied to type `{individual function type for a}`
3    |
4 LL |     assert_eq!(a, 0);

5    |     ^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^
6    |     |
-    |     fn() -> i32 {a}
+    |     {individual function type for a}
9    |
9    |
10    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
19 LL |     assert_eq!(a, 0);
20    |     ^^^^^^^^^^^^^^^^ expected fn item, found integer
21    |
21    |
-    = note: expected fn item `fn() -> i32 {a}`
+    = note: expected fn item `{individual function type for a}`
23                  found type `i32`
24    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


- error[E0277]: `fn() -> i32 {a}` doesn't implement `Debug`
+ error[E0277]: `{individual function type for a}` doesn't implement `Debug`
28    |
28    |
29 LL | fn a() -> i32 {
30    |    - consider calling this function
31 ...
32 LL |     assert_eq!(a, 0);
32 LL |     assert_eq!(a, 0);
-    |     ^^^^^^^^^^^^^^^^ `fn() -> i32 {a}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^ `{individual function type for a}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
-    = help: the trait `Debug` is not implemented for `fn() -> i32 {a}`
-    = help: the trait `Debug` is not implemented for `fn() -> i32 {a}`
+    = help: the trait `Debug` is not implemented for `{individual function type for a}`
36    = help: use parentheses to call the function: `a()`
37    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/issue-70724-add_type_neq_err_label-unwrap.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/issue-70724-add_type_neq_err_label-unwrap.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-70724-add_type_neq_err_label-unwrap.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `{individual function type for a}`
   |
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     {individual function type for a}
   |
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
help: you might have forgotten to call this function
   |
LL |                 if !(*left_val() == *right_val) {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-70724-add_type_neq_err_label-unwrap.rs:6:5
   |
   |
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^ expected fn item, found integer
   |
   = note: expected fn item `{individual function type for a}`
                 found type `i32`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `{individual function type for a}` doesn't implement `Debug`
   |
   |
LL | fn a() -> i32 {
...
LL |     assert_eq!(a, 0);
LL |     assert_eq!(a, 0);
   |     ^^^^^^^^^^^^^^^^ `{individual function type for a}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `{individual function type for a}`
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
+            found fn item `{individual function type for String::new}`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87490/issue-87490.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-87490.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-87490.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87490" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-87490/auxiliary"
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
           found fn item `{individual function type for String::new}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] ui/lifetimes/lifetime-errors/issue_74400.rs stdout ----
diff of stderr:

4 LL |     f(data, identity)
5    |     ^ implementation of `FnOnce` is not general enough
6    |
-    = note: `fn(&'2 T) -> &'2 T {identity::<&'2 T>}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
+    = note: `{individual function type for identity::<&'2 T>}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
8    = note: ...but it actually implements `FnOnce<(&'2 T,)>`, for some specific lifetime `'2`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400/issue_74400.stderr
To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/issue_74400.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/issue_74400.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/issue_74400/auxiliary"
stdout: none
--- stderr -------------------------------
error: implementation of `FnOnce` is not general enough
   |
   |
LL |     f(data, identity) //~ ERROR implementation of `FnOnce` is not general
   |     ^ implementation of `FnOnce` is not general enough
   |
   = note: `{individual function type for identity::<&'2 T>}` must implement `FnOnce<(&'1 T,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 T,)>`, for some specific lifetime `'2`
error: aborting due to previous error
------------------------------------------



---- [ui] ui/lint/trivial_casts.rs stdout ----
diff of stderr:

120    |
121    = help: cast can be replaced by coercion; this might require a temporary variable
122 
- error: trivial cast: `&fn(i32) {baz}` as `&dyn Fn(i32)`
+ error: trivial cast: `&{individual function type for baz}` as `&dyn Fn(i32)`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/trivial_casts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/trivial_casts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/trivial_casts/auxiliary"
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

error: trivial cast: `&{individual function type for baz}` as `&dyn Fn(i32)`
   |
   |
LL |     let _ = &baz as &dyn Fn(i32); //~ERROR `&fn(i32) {baz}` as `&dyn Fn(i32)`
   |
   |
   = help: cast can be replaced by coercion; this might require a temporary variable

error: trivial cast: `&[closure@/checkout/src/test/ui/lint/trivial_casts.rs:72:13: 72:25]` as `&dyn Fn(i32)`
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



---- [ui] ui/match/issue-82392.rs stdout ----
diff of stdout:

11                 ({ } as
12                     ()) else if (let Some(a) =
13                        ((Some as
-                                fn(i32) -> Option<i32> {Option::<i32>::Some})((3 as i32)) as
-                            Option<i32>) as bool) ({ } as ()) as ())
+                                {individual function type for Option::<i32>::Some})((3 as
+                                i32)) as Option<i32>) as bool) ({ } as ()) as ())
16                } as ())


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/issue-82392.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/issue-82392.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args match/issue-82392.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/issue-82392.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=hir,typed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/auxiliary"
#[prelude_import]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;
// https://github.com/rust-lang/rust/issues/82329
// https://github.com/rust-lang/rust/issues/82329
// compile-flags: -Zunpretty=hir,typed
// check-pass
pub fn main() ({
pub fn main() ({
        (if (true as bool)
                ({ } as
                    ()) else if (let Some(a) =
                       ((Some as
                               {individual function type for Option::<i32>::Some})((3 as
                               i32)) as Option<i32>) as bool) ({ } as ()) as ())
               } as ())
stderr: none


---- [ui] ui/mismatched_types/cast-rfc0401.rs stdout ----
---- [ui] ui/mismatched_types/cast-rfc0401.rs stdout ----
diff of stderr:

14    |
15    = note: vtable kinds may not match
16 
- error[E0609]: no field `f` on type `fn() {main}`
+ error[E0609]: no field `f` on type `{individual function type for main}`
19    |
19    |
20 LL |     let _ = main.f as *const u32;

62 LL |     let _ = v as f32;
64 
64 
- error[E0606]: casting `fn() {main}` as `f64` is invalid
+ error[E0606]: casting `{individual function type for main}` as `f64` is invalid
67    |
68 LL |     let _ = main as f64;

172 LL |     let _ = foo as *mut str;
172 LL |     let _ = foo as *mut str;
173    |             ^^^^^^^^^^^^^^^
174 
- error[E0606]: casting `fn() {main}` as `*mut str` is invalid
+ error[E0606]: casting `{individual function type for main}` as `*mut str` is invalid
177    |
178 LL |     let _ = main as *mut str;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/cast-rfc0401.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/cast-rfc0401.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary"
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

error[E0609]: no field `f` on type `{individual function type for main}`
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


error[E0606]: casting `{individual function type for main}` as `f64` is invalid
   |
   |
LL |     let _ = main as f64; //~ ERROR is invalid


error[E0606]: casting `&*const u8` as `usize` is invalid
   |
   |
LL |     let _ = &v as usize; //~ ERROR is invalid
   |
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
   |             ^^^^^^^^^^^^^^^ invalid cast
   |
help: try `char::from_u32` instead
   |
   |
LL |     let _ = 0x61u32 as char; //~ ERROR can be cast as


error[E0606]: casting `bool` as `f32` is invalid
   |
   |
LL |     let _ = false as f32; //~ ERROR is invalid
   |
   = help: cast through an integer first


error[E0606]: casting `E` as `f32` is invalid
   |
   |
LL |     let _ = E::A as f32; //~ ERROR is invalid
   |
   = help: cast through an integer first


error[E0606]: casting `char` as `f32` is invalid
   |
   |
LL |     let _ = 'a' as f32; //~ ERROR is invalid
   |
   = help: cast through an integer first


error[E0606]: casting `bool` as `*const u8` is invalid
   |
   |
LL |     let _ = false as *const u8; //~ ERROR is invalid


error[E0606]: casting `E` as `*const u8` is invalid
   |
   |
LL |     let _ = E::A as *const u8; //~ ERROR is invalid


error[E0606]: casting `char` as `*const u8` is invalid
   |
   |
LL |     let _ = 'a' as *const u8; //~ ERROR is invalid


error[E0606]: casting `usize` as `*const [u8]` is invalid
   |
---

---- [ui] ui/reify-intrinsic.rs stdout ----
diff of stderr:

13 LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute(...);
15 
15 
- error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
+ error[E0606]: casting `{individual function type for transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
18    |
18    |
19 LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args reify-intrinsic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary"
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
help: use parentheses to call this function
   |
LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute(...);


error[E0606]: casting `{individual function type for transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
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
   = note: expected type `extern "rust-intrinsic" fn(_) -> _ {likely}`
           found fn item `extern "rust-intrinsic" fn(_) -> _ {unlikely}`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0606.
For more information about an error, try `rustc --explain E0308`.
---
diff of stderr:

313   --> $DIR/privacy-enum-ctor.rs:27:20
314    |
315 LL |             Fn(u8),
-    |             ------ fn(u8) -> Z {Z::Fn} defined here
+    |             ------ {individual function type for Z::Fn} defined here
317 ...
318 LL |         let _: Z = Z::Fn;
319    |                -   ^^^^^ expected enum `Z`, found fn item
321    |                expected due to this
322    |
323    = note: expected enum `Z`
323    = note: expected enum `Z`
-            found fn item `fn(u8) -> Z {Z::Fn}`
+            found fn item `{individual function type for Z::Fn}`
325 help: use parentheses to instantiate this tuple variant
326    |
327 LL |         let _: Z = Z::Fn(_);
348   --> $DIR/privacy-enum-ctor.rs:43:16
349    |
350 LL |         Fn(u8),
350 LL |         Fn(u8),
-    |         ------ fn(u8) -> E {E::Fn} defined here
+    |         ------ {individual function type for E::Fn} defined here
352 ...
353 LL |     let _: E = m::E::Fn;
354    |            -   ^^^^^^^^ expected enum `E`, found fn item
356    |            expected due to this
357    |
358    = note: expected enum `E`
358    = note: expected enum `E`
-            found fn item `fn(u8) -> E {E::Fn}`
+            found fn item `{individual function type for E::Fn}`
360 help: use parentheses to instantiate this tuple variant
361    |
362 LL |     let _: E = m::E::Fn(_);
383   --> $DIR/privacy-enum-ctor.rs:51:16
384    |
385 LL |         Fn(u8),
385 LL |         Fn(u8),
-    |         ------ fn(u8) -> E {E::Fn} defined here
+    |         ------ {individual function type for E::Fn} defined here
387 ...
388 LL |     let _: E = E::Fn;
389    |            -   ^^^^^ expected enum `E`, found fn item
391    |            expected due to this
392    |
393    = note: expected enum `E`
393    = note: expected enum `E`
-            found fn item `fn(u8) -> E {E::Fn}`
+            found fn item `{individual function type for E::Fn}`
395 help: use parentheses to instantiate this tuple variant
396    |
397 LL |     let _: E = E::Fn(_);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/privacy-enum-ctor.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/privacy-enum-ctor.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/privacy-enum-ctor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/privacy-enum-ctor/auxiliary"
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
   | |_________^
help: you might have meant to use the following enum variant
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
   | |_________^
help: you might have meant to use the following enum variant
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
LL | /             Struct {
LL | |                 s: u8,
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
   |
LL | /     pub enum E {
LL | |         Fn(u8),
LL | |         Struct {
LL | |             s: u8,
LL | |         },
LL | |         Unit,
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

error[E0423]: expected value, found struct variant `m::E::Struct`
   |
LL | /         Struct {
LL | /         Struct {
LL | |             s: u8,
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
   |
LL | /     pub enum E {
LL | |         Fn(u8),
LL | |         Struct {
LL | |             s: u8,
LL | |         },
LL | |         Unit,
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
   | |_________^
help: you might have meant to use the following enum variant
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
LL | /             Struct {
LL | |                 s: u8,
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
   |             ------ {individual function type for Z::Fn} defined here
...
LL |         let _: Z = Z::Fn;
   |                -   ^^^^^ expected enum `Z`, found fn item
   |                expected due to this
   |
   = note: expected enum `Z`
   = note: expected enum `Z`
           found fn item `{individual function type for Z::Fn}`
help: use parentheses to instantiate this tuple variant
   |
LL |         let _: Z = Z::Fn(_);


error[E0618]: expected function, found enum variant `Z::Unit`
   |
LL |             Unit,
LL |             Unit,
   |             ---- `Z::Unit` defined here
...
LL |         let _ = Z::Unit();
   |                 |
   |                 call expression requires function
   |
   |
help: `Z::Unit` is a unit variant, you need to write it without the parentheses
   |
LL -         let _ = Z::Unit();
LL +         let _ = Z::Unit;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:43:16
   |
   |
LL |         Fn(u8),
   |         ------ {individual function type for E::Fn} defined here
...
LL |     let _: E = m::E::Fn;
   |            -   ^^^^^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `{individual function type for E::Fn}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = m::E::Fn(_);


error[E0618]: expected function, found enum variant `m::E::Unit`
   |
LL |         Unit,
LL |         Unit,
   |         ---- `m::E::Unit` defined here
...
LL |     let _: E = m::E::Unit();
   |                |
   |                call expression requires function
   |
   |
help: `m::E::Unit` is a unit variant, you need to write it without the parentheses
   |
LL -     let _: E = m::E::Unit();
LL +     let _: E = m::E::Unit;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:51:16
   |
   |
LL |         Fn(u8),
   |         ------ {individual function type for E::Fn} defined here
...
LL |     let _: E = E::Fn;
   |            -   ^^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `{individual function type for E::Fn}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = E::Fn(_);


error[E0618]: expected function, found enum variant `E::Unit`
   |
LL |         Unit,
LL |         Unit,
   |         ---- `E::Unit` defined here
...
LL |     let _: E = E::Unit();
   |                |
   |                call expression requires function
   |
   |
help: `E::Unit` is a unit variant, you need to write it without the parentheses
   |
LL -     let _: E = E::Unit();
LL +     let _: E = E::Unit;

error: aborting due to 23 previous errors

Some errors have detailed explanations: E0308, E0412, E0423, E0603, E0618.
Some errors have detailed explanations: E0308, E0412, E0423, E0603, E0618.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] ui/rfc1623.rs stdout ----
diff of stderr:

4 LL |     f: &id,
5    |        ^^^ implementation of `FnOnce` is not general enough
6    |
-    = note: `fn(&'2 Foo<'_>) -> &'2 Foo<'_> {id::<&'2 Foo<'_>>}` must implement `FnOnce<(&'1 Foo<'b>,)>`, for any lifetime `'1`...
+    = note: `{individual function type for id::<&'2 Foo<'_>>}` must implement `FnOnce<(&'1 Foo<'b>,)>`, for any lifetime `'1`...
8    = note: ...but it actually implements `FnOnce<(&'2 Foo<'_>,)>`, for some specific lifetime `'2`
10 error: aborting due to previous error
---
diff of stderr:

22   --> $DIR/fn-or-tuple-struct-without-args.rs:29:20
23    |
24 LL | fn foo(a: usize, b: usize) -> usize { a }
-    | ----------------------------------- fn(usize, usize) -> usize {foo} defined here
+    | ----------------------------------- {individual function type for foo} defined here
27 LL |     let _: usize = foo;
28    |            -----   ^^^ expected `usize`, found fn item

30    |            expected due to this
30    |            expected due to this
31    |
32    = note: expected type `usize`
-            found fn item `fn(usize, usize) -> usize {foo}`
+            found fn item `{individual function type for foo}`
34 help: use parentheses to call this function
35    |
36 LL |     let _: usize = foo(a, b);
40   --> $DIR/fn-or-tuple-struct-without-args.rs:30:16
41    |
41    |
42 LL | struct S(usize, usize);
-    | ----------------------- fn(usize, usize) -> S {S} defined here
+    | ----------------------- {individual function type for S} defined here
44 ...
45 LL |     let _: S = S;
46    |            -   ^ expected struct `S`, found fn item
48    |            expected due to this
49    |
50    = note: expected struct `S`
50    = note: expected struct `S`
-              found fn item `fn(usize, usize) -> S {S}`
+              found fn item `{individual function type for S}`
52 help: use parentheses to instantiate this tuple struct
53    |
54 LL |     let _: S = S(_, _);
58   --> $DIR/fn-or-tuple-struct-without-args.rs:31:20
59    |
60 LL | fn bar() -> usize { 42 }
60 LL | fn bar() -> usize { 42 }
-    | ----------------- fn() -> usize {bar} defined here
+    | ----------------- {individual function type for bar} defined here
62 ...
63 LL |     let _: usize = bar;
64    |            -----   ^^^ expected `usize`, found fn item
66    |            expected due to this
67    |
68    = note: expected type `usize`
68    = note: expected type `usize`
-            found fn item `fn() -> usize {bar}`
+            found fn item `{individual function type for bar}`
70 help: use parentheses to call this function
71    |
72 LL |     let _: usize = bar();
76   --> $DIR/fn-or-tuple-struct-without-args.rs:32:16
77    |
78 LL | struct V();
78 LL | struct V();
-    | ----------- fn() -> V {V} defined here
+    | ----------- {individual function type for V} defined here
80 ...
81 LL |     let _: V = V;
82    |            -   ^ expected struct `V`, found fn item
84    |            expected due to this
85    |
86    = note: expected struct `V`
86    = note: expected struct `V`
-              found fn item `fn() -> V {V}`
+              found fn item `{individual function type for V}`
88 help: use parentheses to instantiate this tuple struct
89    |
90 LL |     let _: V = V();
94   --> $DIR/fn-or-tuple-struct-without-args.rs:33:20
95    |
95    |
96 LL |     fn baz(x: usize, y: usize) -> usize { x }
-    |     ----------------------------------- fn(usize, usize) -> usize {<_ as T>::baz} defined here
+    |     ----------------------------------- {individual function type for <_ as T>::baz} defined here
98 ...
99 LL |     let _: usize = T::baz;
100    |            -----   ^^^^^^ expected `usize`, found fn item
102    |            expected due to this
103    |
104    = note: expected type `usize`
104    = note: expected type `usize`
-            found fn item `fn(usize, usize) -> usize {<_ as T>::baz}`
+            found fn item `{individual function type for <_ as T>::baz}`
106 help: use parentheses to call this function
107    |
108 LL |     let _: usize = T::baz(x, y);
112   --> $DIR/fn-or-tuple-struct-without-args.rs:34:20
113    |
113    |
114 LL |     fn bat(x: usize) -> usize { 42 }
-    |     ------------------------- fn(usize) -> usize {<_ as T>::bat} defined here
+    |     ------------------------- {individual function type for <_ as T>::bat} defined here
116 ...
117 LL |     let _: usize = T::bat;
118    |            -----   ^^^^^^ expected `usize`, found fn item
120    |            expected due to this
121    |
122    = note: expected type `usize`
122    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<_ as T>::bat}`
+            found fn item `{individual function type for <_ as T>::bat}`
124 help: use parentheses to call this function
125    |
126 LL |     let _: usize = T::bat(x);
130   --> $DIR/fn-or-tuple-struct-without-args.rs:35:16
131    |
132 LL |     A(usize),
132 LL |     A(usize),
-    |     -------- fn(usize) -> E {E::A} defined here
+    |     -------- {individual function type for E::A} defined here
134 ...
135 LL |     let _: E = E::A;
136    |            -   ^^^^ expected enum `E`, found fn item
138    |            expected due to this
139    |
140    = note: expected enum `E`
140    = note: expected enum `E`
-            found fn item `fn(usize) -> E {E::A}`
+            found fn item `{individual function type for E::A}`
142 help: use parentheses to instantiate this tuple variant
143    |
144 LL |     let _: E = E::A(_);
148   --> $DIR/fn-or-tuple-struct-without-args.rs:37:20
149    |
149    |
150 LL |     fn baz(x: usize, y: usize) -> usize { x }
-    |     ----------------------------------- fn(usize, usize) -> usize {<X as T>::baz} defined here
+    |     ----------------------------------- {individual function type for <X as T>::baz} defined here
152 ...
153 LL |     let _: usize = X::baz;
154    |            -----   ^^^^^^ expected `usize`, found fn item
156    |            expected due to this
157    |
158    = note: expected type `usize`
158    = note: expected type `usize`
-            found fn item `fn(usize, usize) -> usize {<X as T>::baz}`
+            found fn item `{individual function type for <X as T>::baz}`
160 help: use parentheses to call this function
161    |
162 LL |     let _: usize = X::baz(x, y);
166   --> $DIR/fn-or-tuple-struct-without-args.rs:38:20
167    |
167    |
168 LL |     fn bat(x: usize) -> usize { 42 }
-    |     ------------------------- fn(usize) -> usize {<X as T>::bat} defined here
+    |     ------------------------- {individual function type for <X as T>::bat} defined here
170 ...
171 LL |     let _: usize = X::bat;
172    |            -----   ^^^^^^ expected `usize`, found fn item
174    |            expected due to this
175    |
176    = note: expected type `usize`
176    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<X as T>::bat}`
+            found fn item `{individual function type for <X as T>::bat}`
178 help: use parentheses to call this function
179    |
180 LL |     let _: usize = X::bat(x);
184   --> $DIR/fn-or-tuple-struct-without-args.rs:39:20
185    |
185    |
186 LL |     fn bax(x: usize) -> usize { 42 }
-    |     ------------------------- fn(usize) -> usize {<X as T>::bax} defined here
+    |     ------------------------- {individual function type for <X as T>::bax} defined here
188 ...
189 LL |     let _: usize = X::bax;
190    |            -----   ^^^^^^ expected `usize`, found fn item
192    |            expected due to this
193    |
194    = note: expected type `usize`
194    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<X as T>::bax}`
+            found fn item `{individual function type for <X as T>::bax}`
196 help: use parentheses to call this function
197    |
198 LL |     let _: usize = X::bax(x);
202   --> $DIR/fn-or-tuple-struct-without-args.rs:40:20
203    |
203    |
204 LL |     fn bach(x: usize) -> usize;
-    |     --------------------------- fn(usize) -> usize {<X as T>::bach} defined here
+    |     --------------------------- {individual function type for <X as T>::bach} defined here
206 ...
207 LL |     let _: usize = X::bach;
208    |            -----   ^^^^^^^ expected `usize`, found fn item
210    |            expected due to this
211    |
212    = note: expected type `usize`
212    = note: expected type `usize`
-            found fn item `fn(usize) -> usize {<X as T>::bach}`
+            found fn item `{individual function type for <X as T>::bach}`
214 help: use parentheses to call this function
215    |
216 LL |     let _: usize = X::bach(x);
220   --> $DIR/fn-or-tuple-struct-without-args.rs:41:20
221    |
221    |
222 LL |     fn ban(&self) -> usize { 42 }
-    |     ---------------------- for<'r> fn(&'r X) -> usize {<X as T>::ban} defined here
+    |     ---------------------- {individual function type for <X as T>::ban} defined here
224 ...
225 LL |     let _: usize = X::ban;
226    |            -----   ^^^^^^ expected `usize`, found fn item
228    |            expected due to this
229    |
230    = note: expected type `usize`
230    = note: expected type `usize`
-            found fn item `for<'r> fn(&'r X) -> usize {<X as T>::ban}`
+            found fn item `{individual function type for <X as T>::ban}`
232 help: use parentheses to call this function
233    |
234 LL |     let _: usize = X::ban(_);
238   --> $DIR/fn-or-tuple-struct-without-args.rs:42:20
239    |
239    |
240 LL |     fn bal(&self) -> usize;
-    |     ----------------------- for<'r> fn(&'r X) -> usize {<X as T>::bal} defined here
+    |     ----------------------- {individual function type for <X as T>::bal} defined here
242 ...
243 LL |     let _: usize = X::bal;
244    |            -----   ^^^^^^ expected `usize`, found fn item
246    |            expected due to this
247    |
248    = note: expected type `usize`
248    = note: expected type `usize`
-            found fn item `for<'r> fn(&'r X) -> usize {<X as T>::bal}`
+            found fn item `{individual function type for <X as T>::bal}`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-without-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/fn-or-tuple-struct-without-args/auxiliary"
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
   | ----------------------------------- {individual function type for foo} defined here
...
LL |     let _: usize = foo; //~ ERROR mismatched types
   |            -----   ^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for foo}`
help: use parentheses to call this function
   |
LL |     let _: usize = foo(a, b); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:30:16
   |
   |
LL | struct S(usize, usize);
   | ----------------------- {individual function type for S} defined here
...
LL |     let _: S = S; //~ ERROR mismatched types
   |            -   ^ expected struct `S`, found fn item
   |            expected due to this
   |
   = note: expected struct `S`
   = note: expected struct `S`
             found fn item `{individual function type for S}`
help: use parentheses to instantiate this tuple struct
   |
LL |     let _: S = S(_, _); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:31:20
   |
   |
LL | fn bar() -> usize { 42 }
   | ----------------- {individual function type for bar} defined here
...
LL |     let _: usize = bar; //~ ERROR mismatched types
   |            -----   ^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for bar}`
help: use parentheses to call this function
   |
LL |     let _: usize = bar(); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:32:16
   |
   |
LL | struct V();
   | ----------- {individual function type for V} defined here
...
LL |     let _: V = V; //~ ERROR mismatched types
   |            -   ^ expected struct `V`, found fn item
   |            expected due to this
   |
   = note: expected struct `V`
   = note: expected struct `V`
             found fn item `{individual function type for V}`
help: use parentheses to instantiate this tuple struct
   |
LL |     let _: V = V(); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:33:20
   |
   |
LL |     fn baz(x: usize, y: usize) -> usize { x }
   |     ----------------------------------- {individual function type for <_ as T>::baz} defined here
...
LL |     let _: usize = T::baz; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <_ as T>::baz}`
help: use parentheses to call this function
   |
LL |     let _: usize = T::baz(x, y); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:34:20
   |
   |
LL |     fn bat(x: usize) -> usize { 42 }
   |     ------------------------- {individual function type for <_ as T>::bat} defined here
...
LL |     let _: usize = T::bat; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <_ as T>::bat}`
help: use parentheses to call this function
   |
LL |     let _: usize = T::bat(x); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:35:16
   |
   |
LL |     A(usize),
   |     -------- {individual function type for E::A} defined here
...
LL |     let _: E = E::A; //~ ERROR mismatched types
   |            -   ^^^^ expected enum `E`, found fn item
   |            expected due to this
   |
   = note: expected enum `E`
   = note: expected enum `E`
           found fn item `{individual function type for E::A}`
help: use parentheses to instantiate this tuple variant
   |
LL |     let _: E = E::A(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:37:20
   |
   |
LL |     fn baz(x: usize, y: usize) -> usize { x }
   |     ----------------------------------- {individual function type for <X as T>::baz} defined here
...
LL |     let _: usize = X::baz; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <X as T>::baz}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::baz(x, y); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:38:20
   |
   |
LL |     fn bat(x: usize) -> usize { 42 }
   |     ------------------------- {individual function type for <X as T>::bat} defined here
...
LL |     let _: usize = X::bat; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <X as T>::bat}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bat(x); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:39:20
   |
   |
LL |     fn bax(x: usize) -> usize { 42 }
   |     ------------------------- {individual function type for <X as T>::bax} defined here
...
LL |     let _: usize = X::bax; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <X as T>::bax}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bax(x); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:40:20
   |
   |
LL |     fn bach(x: usize) -> usize;
   |     --------------------------- {individual function type for <X as T>::bach} defined here
...
LL |     let _: usize = X::bach; //~ ERROR mismatched types
   |            -----   ^^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <X as T>::bach}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bach(x); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:41:20
   |
   |
LL |     fn ban(&self) -> usize { 42 }
   |     ---------------------- {individual function type for <X as T>::ban} defined here
...
LL |     let _: usize = X::ban; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <X as T>::ban}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::ban(_); //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/fn-or-tuple-struct-without-args.rs:42:20
   |
   |
LL |     fn bal(&self) -> usize;
   |     ----------------------- {individual function type for <X as T>::bal} defined here
...
LL |     let _: usize = X::bal; //~ ERROR mismatched types
   |            -----   ^^^^^^ expected `usize`, found fn item
   |            expected due to this
   |
   = note: expected type `usize`
   = note: expected type `usize`
           found fn item `{individual function type for <X as T>::bal}`
help: use parentheses to call this function
   |
LL |     let _: usize = X::bal(_); //~ ERROR mismatched types

---

5    |                   ^^^^ expected `bool`, found fn item
6    |
7    = note: expected type `bool`
-            found fn item `fn() {main}`
+            found fn item `{individual function type for main}`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-type-error/while-type-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args while-type-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/while-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-type-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-type-error/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/while-type-error.rs:3:19
   |
   |
LL | fn main() { while main { } }
   |                   ^^^^ expected `bool`, found fn item
   = note: expected type `bool`
   = note: expected type `bool`
           found fn item `{individual function type for main}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
