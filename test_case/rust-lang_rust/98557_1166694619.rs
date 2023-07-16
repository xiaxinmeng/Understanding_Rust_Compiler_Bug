plain
diff of 32bit.stderr:

2   --> $DIR/alloc.rs:9:1
3    |
4 LL | const LAYOUT_INVALID_ZERO: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000000, but expected a valid enum tag
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x00000000, but expected a valid enum tag
6    |
7    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
8    = note: the raw bytes of the constant (size: 8, align: 4) {
13   --> $DIR/alloc.rs:13:1
14    |
14    |
15 LL | const LAYOUT_INVALID_THREE: Layout = unsafe { Layout::from_size_align_unchecked(9, 3) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align.0.<enum-tag>: encountered 0x00000003, but expected a valid enum tag
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x00000003, but expected a valid enum tag
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/std/alloc.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const LAYOUT_INVALID_ZERO: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x00000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/std/alloc.rs:13:1
  --> /checkout/src/test/ui/consts/std/alloc.rs:13:1
   |
LL | const LAYOUT_INVALID_THREE: Layout = unsafe { Layout::from_size_align_unchecked(9, 3) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .align.0.<enum-tag>: encountered 0x00000003, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
           }

error: aborting due to 2 previous errors

