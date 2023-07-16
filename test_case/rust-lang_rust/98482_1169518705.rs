plain
diff of 32bit.stderr:

2   --> $DIR/alloc.rs:9:1
3    |
4 LL | const LAYOUT_INVALID_ZERO: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000000, but expected a valid enum tag
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000000, but expected a valid enum tag
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
8    = note: the raw bytes of the constant (size: 8, align: 4) {
13   --> $DIR/alloc.rs:13:1
14    |
14    |
15 LL | const LAYOUT_INVALID_THREE: Layout = unsafe { Layout::from_size_align_unchecked(9, 3) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000003, but expected a valid enum tag
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000003, but expected a valid enum tag
17    |
18    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
19    = note: the raw bytes of the constant (size: 8, align: 4) {

The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc/alloc.32bit.stderr
To only update this specific test, also pass `--test-args consts/std/alloc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/std/alloc.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const LAYOUT_INVALID_ZERO: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000000, but expected a valid enum tag
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/std/alloc.rs:13:1
  --> /checkout/src/test/ui/consts/std/alloc.rs:13:1
   |
LL | const LAYOUT_INVALID_THREE: Layout = unsafe { Layout::from_size_align_unchecked(9, 3) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000003, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error: aborting due to 2 previous errors

---
diff of stderr:

2   --> $DIR/wasm-custom-section-relocations.rs:4:1
3    |
4 LL | pub static A: &[u8] = &[1];
+    | ^^^^^^^^^^^^^^^^^^^
6 
6 
7 error: statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references

9    |
9    |
10 LL | pub static D: &usize = &C;
+    | ^^^^^^^^^^^^^^^^^^^^
12 
13 error: aborting due to 2 previous errors
14 
---
To only update this specific test, also pass `--test-args wasm-custom-section-relocations.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wasm-custom-section-relocations.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm-custom-section-relocations" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wasm-custom-section-relocations/auxiliary"
stdout: none
--- stderr -------------------------------
error: statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references
   |
   |
LL | pub static A: &[u8] = &[1]; //~ ERROR: no extra levels of indirection


error: statics with a custom `#[link_section]` must be a simple list of bytes on the wasm target with no extra levels of indirection such as references
   |
   |
LL | pub static D: &usize = &C; //~ ERROR: no extra levels of indirection

error: aborting due to 2 previous errors
------------------------------------------

