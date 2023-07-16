plain
...............................................i........................................ 6864/14093
........................................................................................ 6952/14093
.............i.........................................................ii.ii........i... 7040/14093
.i......................................................................i............... 7128/14093
........................................F................F.............................. 7216/14093
................i..................i.............i...................................... 7392/14093
..........................i............................................................. 7480/14093
................................................i....................................... 7568/14093
........................................................................................ 7656/14093
---
........................................i........................................i...... 9240/14093
..............................................................i......................... 9328/14093
................................................................................F....... 9416/14093
......................................................................................i. 9504/14093
..............F..F...................................................................... 9592/14093
.........i.............................................................................. 9768/14093
........................................................................................ 9856/14093
........................................................................................ 9944/14093
........................................................................................ 10032/14093
---
To only update this specific test, also pass `--test-args lexer/error-stage.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/error-stage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/error-stage" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/error-stage/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error: binary float literal is not supported
   |
   |
LL |     0b10.0f32; //~ ERROR binary float literal is not supported

error: binary float literal is not supported
  --> /checkout/src/test/ui/lexer/error-stage.rs:68:5
   |
   |
LL |     0b10.0f32; //~ ERROR binary float literal is not supported

error: binary float literal is not supported
  --> /checkout/src/test/ui/lexer/error-stage.rs:78:5
   |
   |
LL |     0b10.0f32; //~ ERROR binary float literal is not supported

error: suffixes on string literals are invalid
  --> /checkout/src/test/ui/lexer/error-stage.rs:74:5
   |
   |
LL |     "string"any_suffix; //~ ERROR suffixes on string literals are invalid
   |     ^^^^^^^^^^^^^^^^^^ invalid suffix `any_suffix`

error: invalid width `123` for integer literal
   |
   |
LL |     10u123; //~ ERROR invalid width `123` for integer literal
   |
   |
   = help: valid widths are 8, 16, 32, 64 and 128

error: invalid width `123` for float literal
   |
   |
LL |     10.0f123; //~ ERROR invalid width `123` for float literal
   |
   |
   = help: valid widths are 32 and 64
error: binary float literal is not supported
  --> /checkout/src/test/ui/lexer/error-stage.rs:77:5
   |
   |
LL |     0b10f32; //~ ERROR binary float literal is not supported

error: integer literal is too large
  --> /checkout/src/test/ui/lexer/error-stage.rs:79:5
   |
   |
LL |     999340282366920938463463374607431768211455999; //~ ERROR integer literal is too large
   |
   = note: value exceeds limit of 340282366920938463463374607431768211455

error: aborting due to 8 previous errors
---
125    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    = note: value exceeds limit of 340282366920938463463374607431768211455
126 
127 error: octal float literal is not supported


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-numeric-literals/lex-bad-numeric-literals.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-numeric-literals/lex-bad-numeric-literals.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lexer/lex-bad-numeric-literals.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-numeric-literals" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-numeric-literals/auxiliary"
stdout: none
--- stderr -------------------------------
error: octal float literal is not supported
   |
   |
LL |     0o1.0; //~ ERROR: octal float literal is not supported


error: octal float literal is not supported
   |
   |
LL |     0o3.0f32; //~ ERROR: octal float literal is not supported


error: octal float literal is not supported
   |
   |
LL |     0o4e4; //~ ERROR: octal float literal is not supported


error: octal float literal is not supported
   |
   |
LL |     0o5.0e5; //~ ERROR: octal float literal is not supported


error: octal float literal is not supported
   |
   |
LL |     0o6e6f32; //~ ERROR: octal float literal is not supported


error: octal float literal is not supported
   |
   |
LL |     0o7.0e7f64; //~ ERROR: octal float literal is not supported

error: hexadecimal float literal is not supported
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:9:5
   |
   |
LL |     0x8.0e+9; //~ ERROR: hexadecimal float literal is not supported

error: hexadecimal float literal is not supported
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:10:5
   |
   |
LL |     0x9.0e-9; //~ ERROR: hexadecimal float literal is not supported

error[E0768]: no valid digits found for number
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:11:5
   |
   |
LL |     0o; //~ ERROR: no valid digits

error: expected at least one digit in exponent
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:12:5
   |
   |
LL |     1e+; //~ ERROR: expected at least one digit in exponent

error: hexadecimal float literal is not supported
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:13:5
   |
   |
LL |     0x539.0; //~ ERROR: hexadecimal float literal is not supported

error[E0768]: no valid digits found for number
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:18:5
   |
   |
LL |     0x; //~ ERROR: no valid digits

error[E0768]: no valid digits found for number
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:19:5
   |
   |
LL |     0xu32; //~ ERROR: no valid digits

error[E0768]: no valid digits found for number
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:20:5
   |
   |
LL |     0ou32; //~ ERROR: no valid digits

error[E0768]: no valid digits found for number
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:21:5
   |
   |
LL |     0bu32; //~ ERROR: no valid digits

error[E0768]: no valid digits found for number
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:22:5
   |
   |
LL |     0b; //~ ERROR: no valid digits


error: octal float literal is not supported
   |
   |
LL |     0o123.456; //~ ERROR: octal float literal is not supported

error: binary float literal is not supported
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:26:5
   |
   |
LL |     0b111.101; //~ ERROR: binary float literal is not supported


error: octal float literal is not supported
   |
   |
LL |     0o2f32; //~ ERROR: octal float literal is not supported

error: integer literal is too large
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:14:5
   |
---
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: value exceeds limit of 340282366920938463463374607431768211455

error: octal float literal is not supported
   |
   |
LL |     0o123f64; //~ ERROR: octal float literal is not supported

error: binary float literal is not supported
  --> /checkout/src/test/ui/lexer/lex-bad-numeric-literals.rs:25:5
   |
   |
LL |     0b101f64; //~ ERROR: binary float literal is not supported

error: aborting due to 23 previous errors

For more information about this error, try `rustc --explain E0768`.
---
To only update this specific test, also pass `--test-args macros/issue-104769-concat_bytes-invalid-literal.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-104769-concat_bytes-invalid-literal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-104769-concat_bytes-invalid-literal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-104769-concat_bytes-invalid-literal/auxiliary"
stdout: none
--- stderr -------------------------------
error: invalid suffix `Y` for number literal
   |
   |
LL |     concat_bytes!(7Y);
   |                   ^^ invalid suffix `Y`
   |
   = help: the suffix must be one of the numeric types (`u32`, `isize`, `f32`, etc.)
error: integer literal is too large
  --> /checkout/src/test/ui/macros/issue-104769-concat_bytes-invalid-literal.rs:6:19
   |
LL |     concat_bytes!(888888888888888888888888888888888888888);
---
To only update this specific test, also pass `--test-args parser/int-literal-too-large-span.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/int-literal-too-large-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/int-literal-too-large-span" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/int-literal-too-large-span/auxiliary"
stdout: none
--- stderr -------------------------------
error: integer literal is too large
   |
LL |     9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
---
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-5544-a/issue-5544-a.stderr
To only update this specific test, also pass `--test-args parser/issues/issue-5544-a.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-5544-a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-5544-a" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-5544-a/auxiliary"
stdout: none
--- stderr -------------------------------
error: integer literal is too large
  --> /checkout/src/test/ui/parser/issues/issue-5544-a.rs:2:19
LL |     let __isize = 340282366920938463463374607431768211456; // 2^128
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: value exceeds limit of 340282366920938463463374607431768211455
---
---- [ui] src/test/ui/parser/issues/issue-5544-b.rs stdout ----
diff of stderr:

3    |
4 LL |     let __isize = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ff;
+    |
+    = note: value exceeds limit of 340282366920938463463374607431768211455
6 
7 error: aborting due to previous error
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-5544-b/issue-5544-b.stderr
To only update this specific test, also pass `--test-args parser/issues/issue-5544-b.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-5544-b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-5544-b" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-5544-b/auxiliary"
stdout: none
--- stderr -------------------------------
error: integer literal is too large
   |
   |
LL |     let __isize = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff_ff;
   |
   = note: value exceeds limit of 340282366920938463463374607431768211455

error: aborting due to previous error
