plain

- error: expected `)`, found `Foo`
-   --> $DIR/feature-gate-builtin_syntax.rs:6:25
-    |
- LL |     builtin # offset_of(Foo, v);
-    |                         ^^^ expected `)`
- 
7 error[E0658]: `builtin #` syntax is unstable
9    |

13    = note: see issue #110680 <https://github.com/rust-lang/rust/issues/110680> for more information
14    = help: add `#![feature(builtin_syntax)]` to the crate attributes to enable
---
19 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-builtin_syntax/feature-gate-builtin_syntax.stderr
To only update this specific test, also pass `--test-args feature-gates/feature-gate-builtin_syntax.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/feature-gates/feature-gate-builtin_syntax.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-builtin_syntax" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-builtin_syntax/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: `builtin #` syntax is unstable
  --> fake-test-src-base/feature-gates/feature-gate-builtin_syntax.rs:6:15
   |
LL |     builtin # offset_of(Foo, v); //~ ERROR `builtin #` syntax is unstable
   |
   = note: see issue #110680 <https://github.com/rust-lang/rust/issues/110680> for more information
   = help: add `#![feature(builtin_syntax)]` to the crate attributes to enable

---
diff of stderr:

5    |                                  ^ missing tokens in macro arguments
6    |
7 note: while trying to match `,`
-   --> $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
+   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
10 error: unexpected end of macro invocation
11   --> $DIR/offset-of-arg-count.rs:7:45

14    |                                             ^ missing tokens in macro arguments
14    |                                             ^ missing tokens in macro arguments
15    |
16 note: while trying to match meta-variable `$fields:tt`
-   --> $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
+   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
18 
19 error: no rules expected the token `too`

31    |                     ^ missing tokens in macro arguments
32    |
32    |
33 note: while trying to match meta-variable `$fields:tt`
-   --> $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
+   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
36 error: expected identifier, found `,`
37   --> $DIR/offset-of-arg-count.rs:12:21



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-arg-count/offset-of-arg-count.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args offset-of/offset-of-arg-count.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/offset-of/offset-of-arg-count.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-arg-count" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-arg-count/auxiliary"
stdout: none
error: unexpected end of macro invocation
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:6:34
   |
   |
LL |     offset_of!(NotEnoughArguments); //~ ERROR unexpected end of macro invocation
   |                                  ^ missing tokens in macro arguments
note: while trying to match `,`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:1321:34

error: unexpected end of macro invocation
error: unexpected end of macro invocation
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:7:45
   |
LL |     offset_of!(NotEnoughArgumentsWithAComma, ); //~ ERROR unexpected end of macro invocation
   |                                             ^ missing tokens in macro arguments
   |
note: while trying to match meta-variable `$fields:tt`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:1321:38

error: no rules expected the token `too`
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:8:34
   |
LL |     offset_of!(Container, field, too many arguments); //~ ERROR no rules expected the token `too`
   |                                  ^^^ no rules expected this token in macro call
   = note: while trying to match sequence end

error: unexpected end of macro invocation
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:11:21
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:11:21
   |
LL |     offset_of!(S, f.); //~ ERROR unexpected end of macro invocation
   |                     ^ missing tokens in macro arguments
   |
note: while trying to match meta-variable `$fields:tt`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:1321:38
error: expected identifier, found `,`
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:12:21
   |
   |
LL |     offset_of!(S, f.,); //~ ERROR expected identifier

error: aborting due to 5 previous errors
------------------------------------------



---- [ui] tests/ui/parser/builtin-syntax.rs stdout ----
diff of stderr:

+ error: unknown `builtin #` construct `foobar`
+    |
+    |
+ LL |     builtin # foobar();
+ 
+ 
1 error: expected identifier after `builtin #`
3    |


4 LL |     builtin # {}();
6 
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
8 
---
To only update this specific test, also pass `--test-args parser/builtin-syntax.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/builtin-syntax.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/builtin-syntax" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/builtin-syntax/auxiliary"
stdout: none
--- stderr -------------------------------
error: unknown `builtin #` construct `foobar`
  --> fake-test-src-base/parser/builtin-syntax.rs:4:5
   |
LL |     builtin # foobar(); //~ ERROR unknown `builtin #` construct


error: expected identifier after `builtin #`
  --> fake-test-src-base/parser/builtin-syntax.rs:8:15
   |
LL |     builtin # {}(); //~ ERROR expected identifier after

error: aborting due to 2 previous errors
------------------------------------------

