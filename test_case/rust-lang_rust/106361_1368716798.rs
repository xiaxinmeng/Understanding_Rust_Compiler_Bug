plain
To only update this specific test, also pass `--test-args lexer/error-stage.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/error-stage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/error-stage" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/error-stage/auxiliary"
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
   = note: value exceeds limit of `340282366920938463463374607431768211455`

error: aborting due to 8 previous errors
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
