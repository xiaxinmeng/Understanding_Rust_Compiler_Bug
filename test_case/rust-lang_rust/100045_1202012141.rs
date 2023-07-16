plain

running 13265 tests
........................................................................................ 88/13265
......................................................................iiiiiiiiiiiiii.... 176/13265
...............i.i.................ii...i.F..F.......................................... 264/13265
........................................................................................ 440/13265
........................................................................................ 528/13265
........................................................................................ 616/13265
........................................................................................ 704/13265
---

45    |                    |             clobber_abi
46    |                    generic outputs
47 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nomem`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
50    |
50    |
51 LL | global_asm!("", options(nomem));

-    |                         ^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
53 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `readonly`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `readonly`
56    |
56    |
57 LL | global_asm!("", options(readonly));

-    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `noreturn`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `noreturn`
62    |
62    |
63 LL | global_asm!("", options(noreturn));

-    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
65 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `pure`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `pure`
68    |
68    |
69 LL | global_asm!("", options(pure));

-    |                         ^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^ expected one of `)`, `att_syntax`, or `raw`
71 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nostack`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nostack`
74    |
74    |
75 LL | global_asm!("", options(nostack));

-    |                         ^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
77 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `preserves_flags`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `preserves_flags`
80    |
80    |
81 LL | global_asm!("", options(preserves_flags));

-    |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
84 error: invalid ABI for `clobber_abi`
85   --> $DIR/bad-options.rs:20:18



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-options/bad-options.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/x86_64/bad-options.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/bad-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-options" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/bad-options/auxiliary"
stdout: none
--- stderr -------------------------------
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


error: asm with `clobber_abi` must specify explicit registers for outputs
   |
   |
LL |         asm!("{}", out(reg) foo, clobber_abi("C"), clobber_abi("C"));
   |                    ^^^^^^^^^^^^  ----------------  ---------------- clobber_abi
   |                    |             clobber_abi
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
  --> /checkout/src/test/ui/asm/x86_64/bad-options.rs:20:18
   |
   |
LL |         asm!("", clobber_abi("foo"));
   |
   |
   = note: the following ABIs are supported on this target: `C`, `system`, `efiapi`, `win64`, `sysv64`

error: `C` ABI specified multiple times
   |
   |
LL |         asm!("{}", out(reg) foo, clobber_abi("C"), clobber_abi("C"));
   |                                  ----------------  ^^^^^^^^^^^^^^^^
   |                                  previously specified here

error: aborting due to 15 previous errors
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/asm/x86_64/parse-error.rs stdout ----
diff of stderr:

266 LL | global_asm!("{}", const(reg) FOO);
267    |                              ^^^ expected one of `,`, `.`, `?`, or an operator
268 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `FOO`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `FOO`
271    |
271    |
272 LL | global_asm!("", options(FOO));

-    |                         ^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^ expected one of `)`, `att_syntax`, or `raw`
274 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nomem`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
277    |
277    |
278 LL | global_asm!("", options(nomem FOO));

-    |                         ^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
280 
- error: expected one of `)`, `att_syntax`, `may_unwind`, or `raw`, found `nomem`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
283    |
283    |
284 LL | global_asm!("", options(nomem, FOO));

-    |                         ^^^^^ expected one of `)`, `att_syntax`, `may_unwind`, or `raw`
+    |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
287 error: arguments are not allowed after options
288   --> $DIR/parse-error.rs:110:30



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/parse-error/parse-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/x86_64/parse-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/parse-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/parse-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/parse-error/auxiliary"
stdout: none
--- stderr -------------------------------
error: requires at least a template string argument
   |
LL |         asm!();
   |         ^^^^^^

---
   |
LL |         asm!("{}" foo);
   |                   ^^^ expected `,`

error: expected operand, clobber_abi, options, or additional template string
   |
LL |         asm!("{}", foo);
LL |         asm!("{}", foo);
   |                    ^^^ expected operand, clobber_abi, options, or additional template string
error: expected `(`, found `foo`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:19:23
   |
LL |         asm!("{}", in foo);
LL |         asm!("{}", in foo);
   |                       ^^^ expected `(`

error: expected `)`, found `foo`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:21:27
   |
LL |         asm!("{}", in(reg foo));
   |                           ^^^ expected `)`
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:23:27
   |
   |
LL |         asm!("{}", in(reg));

error: expected register class or explicit register
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:25:26
   |
   |
LL |         asm!("{}", inout(=) foo => bar);

error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:27:37
   |
   |
LL |         asm!("{}", inout(reg) foo =>);


error: expected one of `!`, `,`, `.`, `::`, `?`, `{`, or an operator, found `=>`
   |
   |
LL |         asm!("{}", in(reg) foo => bar);
   |                                ^^ expected one of 7 possible tokens
error: expected a path for argument to `sym`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:31:24
   |
   |
LL |         asm!("{}", sym foo + bar);


error: expected one of `)`, `att_syntax`, `may_unwind`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(foo));
   |                          ^^^ expected one of 10 possible tokens

error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", options(nomem foo));
   |                                ^^^ expected one of `)` or `,`

error: expected one of `)`, `att_syntax`, `may_unwind`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
   |
   |
LL |         asm!("", options(nomem, foo));
   |                                 ^^^ expected one of 10 possible tokens
error: arguments are not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:39:31
   |
LL |         asm!("{}", options(), const foo);
LL |         asm!("{}", options(), const foo);
   |                    ---------  ^^^^^^^^^ argument
   |                    |
   |                    previous options

error: at least one abi must be provided as an argument to `clobber_abi`
   |
   |
LL |         asm!("", clobber_abi());

error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:44:30
   |
   |
LL |         asm!("", clobber_abi(foo));


error: expected one of `)` or `,`, found `foo`
   |
   |
LL |         asm!("", clobber_abi("C" foo));
   |                                  ^^^ expected one of `)` or `,`
error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:48:35
   |
   |
LL |         asm!("", clobber_abi("C", foo));

error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:50:38
   |
   |
LL |         asm!("{}", clobber_abi("C"), const foo);
   |                    ----------------  ^^^^^^^^^ argument
   |                    clobber_abi

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:53:29
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:53:29
   |
LL |         asm!("", options(), clobber_abi("C"));
   |                  ---------  ^^^^^^^^^^^^^^^^
   |                  options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:31
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:55:31
   |
LL |         asm!("{}", options(), clobber_abi("C"), const foo);
   |                    ---------  ^^^^^^^^^^^^^^^^
   |                    options

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ duplicate argument
   |                     previously here

error: argument never used
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                    ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`
error: explicit register arguments cannot have names
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:62:18
   |
   |
LL |         asm!("", a = in("eax") foo);

error: named arguments cannot follow explicit register arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:64:36
   |
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: named arguments cannot follow explicit register arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:67:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:67:36
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: positional arguments cannot follow named arguments or explicit register arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:70:36
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:70:36
   |
LL |         asm!("{1}", in("eax") foo, const bar);
   |                     -------------  ^^^^^^^^^ positional argument
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
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:77:14
   |
   |
LL |         asm!(format!("{{{}}}", 0), in(reg) foo);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:79:21
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
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:92:1
   |
   |
LL | global_asm!();

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:94:13
   |
   |
LL | global_asm!(FOO);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:96:18
   |
   |
LL | global_asm!("{}" FOO);
   |                  ^^^ expected `,`
error: expected operand, options, or additional template string
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:98:19
   |
   |
LL | global_asm!("{}", FOO);
   |                   ^^^ expected operand, options, or additional template string
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:100:24
   |
   |
LL | global_asm!("{}", const);


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
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:110:30
   |
   |
LL | global_asm!("{}", options(), const FOO);
   |                   ---------  ^^^^^^^^^ argument
   |                   previous options

error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:112:29
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:112:29
   |
LL | global_asm!("", clobber_abi(FOO));


error: expected one of `)` or `,`, found `FOO`
   |
   |
LL | global_asm!("", clobber_abi("C" FOO));
   |                                 ^^^ expected one of `)` or `,`
error: expected string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:116:34
   |
   |
LL | global_asm!("", clobber_abi("C", FOO));

error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:118:37
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
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:121:28
   |
   |
LL | global_asm!("", options(), clobber_abi("C"));
   |                 ---------  ^^^^^^^^^^^^^^^^
   |                 options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:123:30
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:123:30
   |
LL | global_asm!("{}", options(), clobber_abi("C"), const FOO);
   |                   ---------  ^^^^^^^^^^^^^^^^
   |                   options


error: `clobber_abi` cannot be used with `global_asm!`
   |
   |
LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:127:35
   |
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                    -------------  ^^^^^^^^^^^^^ duplicate argument
   |                    previously here

error: argument never used
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:127:35
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:127:35
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                                   ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`

error: expected one of `clobber_abi`, `const`, `options`, or `sym`, found `""`
   |
   |
LL | global_asm!("", options(), "");
   |                            ^^ expected one of `clobber_abi`, `const`, `options`, or `sym`

error: expected one of `clobber_abi`, `const`, `options`, or `sym`, found `"{}"`
   |
   |
LL | global_asm!("{}", const FOO, "{}", const FOO);
   |                              ^^^^ expected one of `clobber_abi`, `const`, `options`, or `sym`
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:134:13
   |
   |
LL | global_asm!(format!("{{{}}}", 0), const FOO);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:136:20
   |
LL | global_asm!("{1}", format!("{{{}}}", 0), const FOO, const BAR);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0435]: attempt to use a non-constant value in a constant
---
   |
LL |     let mut foo = 0;
   |     ----------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{}", clobber_abi("C"), const foo);
   |                                            ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:31
   |
LL |     let mut foo = 0;
LL |     let mut foo = 0;
   |     ----------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                               ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:57:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |     ----------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:64:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |     ----------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:67:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |     ----------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/x86_64/parse-error.rs:70:42
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |     ----------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{1}", in("eax") foo, const bar);
   |                                          ^^^ non-constant value
error: aborting due to 66 previous errors

For more information about this error, try `rustc --explain E0435`.
------------------------------------------
