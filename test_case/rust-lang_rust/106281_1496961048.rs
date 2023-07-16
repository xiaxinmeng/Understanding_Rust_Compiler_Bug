plain

4 LL |     std::mem::transmute(v)
5    |     ^^^^^^^^^^^^^^^^^^^
6    |
-    = note: source type: `[[u32; H+1]; W]` (generic size Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Expr(Binop(Add, Const { ty: usize, kind: Param(H/#1) }, Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) })) }, Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }, Const { ty: usize, kind: Param(W/#0) })) })) })
-    = note: target type: `[[u32; W+1]; H]` (generic size Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Expr(Binop(Add, Const { ty: usize, kind: Param(W/#0) }, Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) })) }, Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }, Const { ty: usize, kind: Param(H/#1) })) })) })
+    = note: source type: `[[u32; H+1]; W]` (generic size)
+    = note: target type: `[[u32; W+1]; H]` (generic size)
9 
10 error[E0512]: cannot transmute between types of different sizes, or dependently-sized types

34 LL |     std::mem::transmute(v)
35    |     ^^^^^^^^^^^^^^^^^^^
36    |
36    |
-    = note: source type: `[[u32; H]; W]` (generic size Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }, Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Param(H/#1) }, Const { ty: usize, kind: Param(W/#0) })) })) })
-    = note: target type: `[u32; W * H * H]` (generic size Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }, Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Param(H/#1) }, Const { ty: usize, kind: Expr(Binop(Mul, Const { ty: usize, kind: Param(H/#1) }, Const { ty: usize, kind: Param(W/#0) })) })) })) })
+    = note: source type: `[[u32; H]; W]` (generic size)
+    = note: target type: `[u32; W * H * H]` (generic size)
39 
40 error[E0512]: cannot transmute between types of different sizes, or dependently-sized types


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute-fail/transmute-fail.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute-fail/transmute-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/transmute-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/transmute-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/transmute-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> fake-test-src-base/const-generics/transmute-fail.rs:7:5
LL |     std::mem::transmute(v)
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: source type: `[[u32; H+1]; W]` (generic size)
   = note: target type: `[[u32; W+1]; H]` (generic size)

error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> fake-test-src-base/const-generics/transmute-fail.rs:16:5
LL |     std::mem::transmute(v)
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: source type: `[[u32; H]; W]` (this type does not have a fixed size)
   = note: target type: `[[u32; W]; H]` (size can vary because of [u32; W])
error[E0308]: mismatched types
  --> fake-test-src-base/const-generics/transmute-fail.rs:12:53
   |
   |
LL | fn bar<const W: bool, const H: usize>(v: [[u32; H]; W]) -> [[u32; W]; H] {
   |                                                     ^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> fake-test-src-base/const-generics/transmute-fail.rs:12:67
   |
   |
LL | fn bar<const W: bool, const H: usize>(v: [[u32; H]; W]) -> [[u32; W]; H] {
   |                                                                   ^ expected `usize`, found `bool`

error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> fake-test-src-base/const-generics/transmute-fail.rs:23:5
LL |     std::mem::transmute(v)
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: source type: `[[u32; H]; W]` (generic size)
   = note: target type: `[u32; W * H * H]` (generic size)

error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> fake-test-src-base/const-generics/transmute-fail.rs:30:5
LL |     std::mem::transmute(v)
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: source type: `[[[u32; 8888888]; 9999999]; 777777777]` (values of the type `[[[u32; 8888888]; 9999999]; 777777777]` are too big for the current architecture)
   = note: target type: `[[[u32; 9999999]; 777777777]; 8888888]` (values of the type `[[[u32; 9999999]; 777777777]; 8888888]` are too big for the current architecture)
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0512.
For more information about an error, try `rustc --explain E0308`.
