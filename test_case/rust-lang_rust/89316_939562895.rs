plain
test [ui] ui/asm/x86_64/const.rs#thirunsafeck ... ignored
test [ui] ui/asm/x86_64/duplicate-options.rs ... ignored
test [ui] ui/asm/x86_64/interpolated-idents.rs ... ignored
test [ui] ui/asm/x86_64/issue-82869.rs ... ignored
test [ui] ui/asm/x86_64/multiple-clobber-abi.rs ... ignored
test [ui] ui/asm/x86_64/srcloc.rs ... ignored
test [ui] ui/asm/x86_64/sym.rs ... ignored
test [ui] ui/asm/x86_64/target-feature-attr.rs ... ignored
test [ui] ui/asm/x86_64/type-check-2.rs ... ignored
---

---- [ui] ui/asm/aarch64/bad-options.rs stdout ----
diff of stderr:

73    |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
75 error: invalid ABI for `clobber_abi`
-   --> $DIR/bad-options.rs:20:18
77    |
77    |
78 LL |         asm!("", clobber_abi("foo"));
+    |                              ^^^^^
80    |
80    |
81    = note: the following ABIs are supported on this target: `C`, `system`, `efiapi`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-options/bad-options.stderr
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-options/bad-options.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/aarch64/bad-options.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/bad-options.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-options/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `nomem` and `readonly` options are mutually exclusive
   |
   |
LL |         asm!("", options(nomem, readonly));


error: the `pure` and `noreturn` options are mutually exclusive
   |
   |
LL |         asm!("", options(pure, nomem, noreturn));


error: asm with the `pure` option must have at least one output
   |
   |
LL |         asm!("", options(pure, nomem, noreturn));


error: asm with the `pure` option must have at least one output
   |
   |
LL |         asm!("{}", in(reg) foo, options(pure, nomem));


error: asm outputs are not allowed with the `noreturn` option
   |
   |
LL |         asm!("{}", out(reg) foo, options(noreturn));


error: asm with `clobber_abi` must specify explicit registers for outputs
   |
   |
LL |         asm!("{}", out(reg) foo, clobber_abi("C"));
   |                    ^^^^^^^^^^^^  ---------------- clobber_abi
   |                    generic outputs


error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem));
   |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `readonly`
   |
   |
LL | global_asm!("", options(readonly));
   |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `noreturn`
   |
   |
LL | global_asm!("", options(noreturn));
   |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `pure`
   |
   |
LL | global_asm!("", options(pure));
   |                         ^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `nostack`
   |
   |
LL | global_asm!("", options(nostack));
   |                         ^^^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `preserves_flags`
   |
   |
LL | global_asm!("", options(preserves_flags));
   |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: invalid ABI for `clobber_abi`
   |
   |
LL |         asm!("", clobber_abi("foo"));
   |
   |
   = note: the following ABIs are supported on this target: `C`, `system`, `efiapi`
error: aborting due to 13 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/asm/aarch64/parse-error.rs stdout ----
diff of stderr:

96 LL |         asm!("", clobber_abi(foo));
98 
98 
- error: expected `)`, found `foo`
+ error: expected one of `)` or `,`, found `foo`
101    |
101    |
102 LL |         asm!("", clobber_abi("C" foo));
-    |                                  ^^^ expected `)`
-    |                                  ^^^ expected `)`
+    |                                  ^^^ expected one of `)` or `,`
104 
- error: expected `)`, found `,`
-   --> $DIR/parse-error.rs:44:33
+ error: expected string literal
107    |
107    |
108 LL |         asm!("", clobber_abi("C", foo));
-    |                                 ^ expected `)`
110 
111 error: arguments are not allowed after clobber_abi
112   --> $DIR/parse-error.rs:46:38


132    |                    |
133    |                    options
134 
+ error: `clobber_abi` specified multiple times
+    |
+    |
+ LL |         asm!("", clobber_abi("C"), clobber_abi("C"));
+    |                  ----------------  ^^^^^^^^^^^^^^^^
+    |                  |
+    |                  `clobber_abi` previously specified here
+    |
+    = help: combine the `clobber_abi` arguments into a single comma-separated list
135 error: duplicate argument named `a`
136   --> $DIR/parse-error.rs:54:36
137    |


292 LL | global_asm!("", clobber_abi(FOO));
294 
294 
- error: expected `)`, found `FOO`
+ error: expected one of `)` or `,`, found `FOO`
297    |
297    |
298 LL | global_asm!("", clobber_abi("C" FOO));
-    |                                 ^^^ expected `)`
-    |                                 ^^^ expected `)`
+    |                                 ^^^ expected one of `)` or `,`
300 
- error: expected `)`, found `,`
-   --> $DIR/parse-error.rs:113:32
+ error: expected string literal
303    |
303    |
304 LL | global_asm!("", clobber_abi("C", FOO));
-    |                                ^ expected `)`
306 
307 error: arguments are not allowed after clobber_abi
308   --> $DIR/parse-error.rs:115:37


334    |                   |
335    |                   options
336 
- error: `clobber_abi` cannot be used with `global_asm!`
+ error: `clobber_abi` specified multiple times
339    |
339    |
340 LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));
-    |                 ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^
+    |                 ----------------  ^^^^^^^^^^^^^^^^
+    |                 |
+    |                 |
+    |                 `clobber_abi` previously specified here
+    |
+    = help: combine the `clobber_abi` arguments into a single comma-separated list
343 error: duplicate argument named `a`
344   --> $DIR/parse-error.rs:124:35


447 LL |         asm!("{1}", in("x0") foo, const bar);
448    |                                         ^^^ non-constant value
- error: aborting due to 65 previous errors
+ error: aborting due to 66 previous errors
451 
452 For more information about this error, try `rustc --explain E0435`.
---
To only update this specific test, also pass `--test-args asm/aarch64/parse-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/parse-error.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/parse-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/parse-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:11:14
   |
LL |         asm!(foo);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:13:19
   |
   |
LL |         asm!("{}" foo);
   |                   ^^^ expected `,`

error: expected operand, clobber_abi, options, or additional template string
   |
   |
LL |         asm!("{}", foo);
   |                    ^^^ expected operand, clobber_abi, options, or additional template string

error: expected `(`, found `foo`
   |
   |
LL |         asm!("{}", in foo);
   |                       ^^^ expected `(`

error: expected `)`, found `foo`
   |
   |
LL |         asm!("{}", in(reg foo));
   |                           ^^^ expected `)`
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:21:27
   |
   |
LL |         asm!("{}", in(reg));
   |                           ^ expected expression
error: expected register class or explicit register
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:23:26
   |
   |
LL |         asm!("{}", inout(=) foo => bar);

error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:25:37
   |
   |
LL |         asm!("{}", inout(reg) foo =>);
   |                                     ^ expected expression

error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, or an operator, found `=>`
   |
   |
LL |         asm!("{}", in(reg) foo => bar);
   |                                ^^ expected one of 7 possible tokens

error: argument to `sym` must be a path expression
   |
   |
LL |         asm!("{}", sym foo + bar);


error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(foo));
   |                          ^^^ expected one of 9 possible tokens

error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", options(nomem foo));
   |                                ^^^ expected one of `)` or `,`

error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(nomem, foo));
   |                                 ^^^ expected one of 9 possible tokens
error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:37:31
   |
   |
LL |         asm!("{}", options(), const foo);
   |                    ---------  ^^^^^^^^^ argument
   |                    previous options

error: expected string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:40:30
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:40:30
   |
LL |         asm!("", clobber_abi(foo));


error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", clobber_abi("C" foo));
   |                                  ^^^ expected one of `)` or `,`
error: expected string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:44:35
   |
   |
LL |         asm!("", clobber_abi("C", foo));

error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:46:38
   |
   |
LL |         asm!("{}", clobber_abi("C"), const foo);
   |                    ----------------  ^^^^^^^^^ argument
   |                    clobber_abi

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:49:29
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:49:29
   |
LL |         asm!("", options(), clobber_abi("C"));
   |                  ---------  ^^^^^^^^^^^^^^^^
   |                  options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:51:31
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:51:31
   |
LL |         asm!("{}", options(), clobber_abi("C"), const foo);
   |                    ---------  ^^^^^^^^^^^^^^^^
   |                    options


error: `clobber_abi` specified multiple times
   |
   |
LL |         asm!("", clobber_abi("C"), clobber_abi("C"));
   |                  ----------------  ^^^^^^^^^^^^^^^^
   |                  |
   |                  `clobber_abi` previously specified here
   |
   = help: combine the `clobber_abi` arguments into a single comma-separated list
error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:54:36
   |
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ duplicate argument
   |                     previously here

error: argument never used
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:54:36
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:54:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                    ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`
error: explicit register arguments cannot have names
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:59:18
   |
   |
LL |         asm!("", a = in("x0") foo);


error: named arguments cannot follow explicit register arguments
   |
   |
LL |         asm!("{a}", in("x0") foo, a = const bar);
   |                     ------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument


error: named arguments cannot follow explicit register arguments
   |
   |
LL |         asm!("{a}", in("x0") foo, a = const bar);
   |                     ------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: positional arguments cannot follow named arguments or explicit register arguments
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:67:35
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:67:35
   |
LL |         asm!("{1}", in("x0") foo, const bar);
   |                     ------------  ^^^^^^^^^ positional argument
   |                     explicit register argument


error: expected one of `clobber_abi`, `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `""`
   |
   |
LL |         asm!("", options(), "");
   |                             ^^ expected one of 9 possible tokens

error: expected one of `clobber_abi`, `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `"{}"`
   |
   |
LL |         asm!("{}", in(reg) foo, "{}", out(reg) foo);
   |                                 ^^^^ expected one of 9 possible tokens
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:74:14
   |
   |
LL |         asm!(format!("{{{}}}", 0), in(reg) foo);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:76:21
   |
LL |         asm!("{1}", format!("{{{}}}", 0), in(reg) foo, out(reg) bar);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", in(reg) _);


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", inout(reg) _);


error: _ cannot be used for input operands
   |
   |
LL |         asm!("{}", inlateout(reg) _);

error: requires at least a template string argument
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:89:1
   |
   |
LL | global_asm!();

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:91:13
   |
   |
LL | global_asm!(FOO);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:93:18
   |
   |
LL | global_asm!("{}" FOO);
   |                  ^^^ expected `,`
error: expected operand, options, or additional template string
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:95:19
   |
   |
LL | global_asm!("{}", FOO);
   |                   ^^^ expected operand, options, or additional template string
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:97:24
   |
   |
LL | global_asm!("{}", const);
   |                        ^ expected expression

error: expected one of `,`, `.`, `?`, or an operator, found `FOO`
   |
   |
LL | global_asm!("{}", const(reg) FOO);
   |                              ^^^ expected one of `,`, `.`, `?`, or an operator

error: expected one of `)`, `att_syntax`, or `raw`, found `FOO`
   |
   |
LL | global_asm!("", options(FOO));
   |                         ^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem FOO));
   |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`

error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
   |
   |
LL | global_asm!("", options(nomem, FOO));
   |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:107:30
   |
   |
LL | global_asm!("{}", options(), const FOO);
   |                   ---------  ^^^^^^^^^ argument
   |                   previous options

error: expected string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:109:29
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:109:29
   |
LL | global_asm!("", clobber_abi(FOO));


error: expected one of `)` or `,`, found `FOO`
   |
   |
LL | global_asm!("", clobber_abi("C" FOO));
   |                                 ^^^ expected one of `)` or `,`
error: expected string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:113:34
   |
   |
LL | global_asm!("", clobber_abi("C", FOO));

error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:115:37
   |
   |
LL | global_asm!("{}", clobber_abi("C"), const FOO);
   |                   ----------------  ^^^^^^^^^ argument
   |                   clobber_abi


error: `clobber_abi` cannot be used with `global_asm!`
   |
   |
LL | global_asm!("{}", clobber_abi("C"), const FOO);

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:118:28
   |
   |
LL | global_asm!("", options(), clobber_abi("C"));
   |                 ---------  ^^^^^^^^^^^^^^^^
   |                 options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:120:30
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:120:30
   |
LL | global_asm!("{}", options(), clobber_abi("C"), const FOO);
   |                   ---------  ^^^^^^^^^^^^^^^^
   |                   options


error: `clobber_abi` specified multiple times
   |
   |
LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));
   |                 ----------------  ^^^^^^^^^^^^^^^^
   |                 |
   |                 `clobber_abi` previously specified here
   |
   = help: combine the `clobber_abi` arguments into a single comma-separated list
error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:124:35
   |
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                    -------------  ^^^^^^^^^^^^^ duplicate argument
   |                    previously here

error: argument never used
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:124:35
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:124:35
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                                   ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`

error: expected one of `clobber_abi`, `const`, or `options`, found `""`
   |
   |
LL | global_asm!("", options(), "");
   |                            ^^ expected one of `clobber_abi`, `const`, or `options`

error: expected one of `clobber_abi`, `const`, or `options`, found `"{}"`
   |
   |
LL | global_asm!("{}", const FOO, "{}", const FOO);
   |                              ^^^^ expected one of `clobber_abi`, `const`, or `options`
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:131:13
   |
   |
LL | global_asm!(format!("{{{}}}", 0), const FOO);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:133:20
   |
LL | global_asm!("{1}", format!("{{{}}}", 0), const FOO, const BAR);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0435]: attempt to use a non-constant value in a constant
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:37:37
   |
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{}", options(), const foo);
   |                                     ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:46:44
   |
LL |     let mut foo = 0;
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{}", clobber_abi("C"), const foo);
   |                                            ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:54:31
   |
LL |     let mut foo = 0;
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                               ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:54:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:61:45
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("x0") foo, a = const bar);
   |                                             ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:64:45
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("x0") foo, a = const bar);
   |                                             ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:67:41
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{1}", in("x0") foo, const bar);
   |                                         ^^^ non-constant value
error: aborting due to 66 previous errors

For more information about this error, try `rustc --explain E0435`.

---
test result: FAILED. 12125 passed; 2 failed; 144 ignored; 0 measured; 0 filtered out; finished in 140.54s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:33
