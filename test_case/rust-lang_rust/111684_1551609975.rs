plain

---- [ui] tests/ui/offset-of/offset-of-arg-count.rs stdout ----
diff of stderr:

39 LL |     offset_of!(S, f.,);
41 
41 
+ error: expected one of `.`, `;`, `?`, `}`, or an operator, found `)`
+    |
+    |
+ LL |     offset_of!(S, f.,);
+    |     ^^^^^^^^^^^^^^^^^^ expected one of `.`, `;`, `?`, `}`, or an operator
+    = note: this error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)
+ 
+ 
42 error: no rules expected the token `..`
44    |

55    |
56    = note: while trying to match sequence start
---
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
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:1322:34

error: unexpected end of macro invocation
error: unexpected end of macro invocation
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:7:45
   |
LL |     offset_of!(NotEnoughArgumentsWithAComma, ); //~ ERROR unexpected end of macro invocation
   |                                             ^ missing tokens in macro arguments
   |
note: while trying to match meta-variable `$fields:tt`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:1322:38

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
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:1322:38
error: expected identifier, found `,`
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:12:21
   |
   |
LL |     offset_of!(S, f.,); //~ ERROR expected identifier


error: expected one of `.`, `;`, `?`, `}`, or an operator, found `)`
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:12:5
   |
LL |     offset_of!(S, f.,); //~ ERROR expected identifier
   |     ^^^^^^^^^^^^^^^^^^ expected one of `.`, `;`, `?`, `}`, or an operator
   = note: this error originates in the macro `offset_of` (in Nightly builds, run with -Z macro-backtrace for more info)


error: no rules expected the token `..`
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:13:20
   |
LL |     offset_of!(S, f..); //~ ERROR no rules expected the token
   |                    ^^ no rules expected this token in macro call
   = note: while trying to match sequence start


error: no rules expected the token `..`
  --> fake-test-src-base/offset-of/offset-of-arg-count.rs:14:20
   |
LL |     offset_of!(S, f..,); //~ ERROR no rules expected the token
   |                    ^^ no rules expected this token in macro call
   = note: while trying to match sequence start

error: aborting due to 8 previous errors
------------------------------------------
