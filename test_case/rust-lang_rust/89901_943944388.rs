plain

---- [ui] ui/asm/aarch64/interpolated-idents.rs stdout ----
diff of stderr:

7 LL | /     m!(in out lateout inout inlateout const sym
8 LL | |        pure nomem readonly preserves_flags
9 LL | |        noreturn nostack options);
+    | |________________________________- in this macro invocation
11    |
11    |
12    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


20 LL | /     m!(in out lateout inout inlateout const sym
21 LL | |        pure nomem readonly preserves_flags
22 LL | |        noreturn nostack options);
+    | |________________________________- in this macro invocation
24    |
24    |
25    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

38    | |
38    | |
39 LL | |        pure nomem readonly preserves_flags
40 LL | |        noreturn nostack options);
-    | |_________________________________|
-    | |_________________________________in this macro invocation
-    | |_________________________________in this macro invocation
-    | |_________________________________in this macro invocation
---
+    | |________________________________in this macro invocation
+    | |________________________________in this macro invocation
+    |                                  in this macro invocation
47    |
48    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents/interpolated-idents.stderr
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents/interpolated-idents.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/aarch64/interpolated-idents.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/interpolated-idents.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/interpolated-idents/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `nomem` and `readonly` options are mutually exclusive
   |
   |
LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack));
...
...
LL | /     m!(in out lateout inout inlateout const sym
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack options);
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error: the `pure` and `noreturn` options are mutually exclusive
   |
   |
LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack));
...
...
LL | /     m!(in out lateout inout inlateout const sym
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack options);
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm outputs are not allowed with the `noreturn` option
   |
   |
LL |               asm!("", $in(x) x, $out(x) x, $lateout(x) x, $inout(x) x, $inlateout(x) x,
...
...
LL |       m!(in out lateout inout inlateout const sym
   | |_____|
   | |_____|
   | |_____|
   | |
   | |
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack options);
   | |________________________________|
   | |________________________________in this macro invocation
   | |________________________________in this macro invocation
   | |________________________________in this macro invocation
   | |________________________________in this macro invocation
   |                                  in this macro invocation
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors


------------------------------------------
---
8   --> $DIR/parse-error.rs:11:14

236   --> $DIR/parse-error.rs:90:1
237    |
238 LL | global_asm!();
+    | ^^^^^^^^^^^^^
240 
241 error: asm template must be a string literal
242   --> $DIR/parse-error.rs:92:13
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


error: expected `)`, found `foo`
   |
   |
LL |         asm!("", clobber_abi("C" foo));
   |                                  ^^^ expected `)`

error: expected `)`, found `,`
   |
   |
LL |         asm!("", clobber_abi("C", foo));
   |                                 ^ expected `)`
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

error: clobber_abi specified multiple times
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:53:36
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:53:36
   |
LL |         asm!("", clobber_abi("C"), clobber_abi("C"));
   |                  ----------------  ^^^^^^^^^^^^^^^^
   |                  |
   |                  clobber_abi previously specified here
error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:55:36
   |
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ duplicate argument
   |                     previously here

error: argument never used
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:55:36
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:55:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                    ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`
error: explicit register arguments cannot have names
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:60:18
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
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:68:35
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:68:35
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
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:75:14
   |
   |
LL |         asm!(format!("{{{}}}", 0), in(reg) foo);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:77:21
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
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:90:1
   |
   |
LL | global_asm!();

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:92:13
   |
   |
LL | global_asm!(FOO);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:94:18
   |
   |
LL | global_asm!("{}" FOO);
   |                  ^^^ expected `,`
error: expected operand, options, or additional template string
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:96:19
   |
   |
LL | global_asm!("{}", FOO);
   |                   ^^^ expected operand, options, or additional template string
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:98:24
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
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:108:30
   |
   |
LL | global_asm!("{}", options(), const FOO);
   |                   ---------  ^^^^^^^^^ argument
   |                   previous options

error: expected string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:110:29
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:110:29
   |
LL | global_asm!("", clobber_abi(FOO));


error: expected `)`, found `FOO`
   |
   |
LL | global_asm!("", clobber_abi("C" FOO));
   |                                 ^^^ expected `)`

error: expected `)`, found `,`
   |
   |
LL | global_asm!("", clobber_abi("C", FOO));
   |                                ^ expected `)`
error: arguments are not allowed after clobber_abi
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:116:37
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
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:119:28
   |
   |
LL | global_asm!("", options(), clobber_abi("C"));
   |                 ---------  ^^^^^^^^^^^^^^^^
   |                 options

error: clobber_abi is not allowed after options
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:121:30
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:121:30
   |
LL | global_asm!("{}", options(), clobber_abi("C"), const FOO);
   |                   ---------  ^^^^^^^^^^^^^^^^
   |                   options

error: clobber_abi specified multiple times
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:123:35
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:123:35
   |
LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));
   |                 ----------------  ^^^^^^^^^^^^^^^^
   |                 |
   |                 clobber_abi previously specified here
error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:125:35
   |
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                    -------------  ^^^^^^^^^^^^^ duplicate argument
   |                    previously here

error: argument never used
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:125:35
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:125:35
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
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:132:13
   |
   |
LL | global_asm!(format!("{{{}}}", 0), const FOO);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:134:20
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
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:55:31
   |
LL |     let mut foo = 0;
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                               ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:55:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:62:45
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("x0") foo, a = const bar);
   |                                             ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:65:45
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("x0") foo, a = const bar);
   |                                             ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:68:41
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
diff of stderr:

77   --> $DIR/type-check-2.rs:20:9
78    |
79 LL |         asm!("{}", inout(reg) y);
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly-uninitialized `y`
81 
82 error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2/type-check-2.stderr
To only update this specific test, also pass `--test-args asm/aarch64/type-check-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/type-check-2.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/type-check-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: arguments for inline assembly must be copyable
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:46:31
   |
LL |         asm!("{:v}", in(vreg) SimdNonCopy(0.0, 0.0, 0.0, 0.0));
   |
   |
   = note: `SimdNonCopy` does not implement the Copy trait

error: cannot use value of type `[closure@/checkout/src/test/ui/asm/aarch64/type-check-2.rs:58:28: 58:38]` for inline assembly
   |
   |
LL |         asm!("{}", in(reg) |x: i32| x);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly

error: cannot use value of type `Vec<i32>` for inline assembly
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

error: cannot use value of type `fn() {main}` for inline assembly
   |
   |
LL |         asm!("{}", inout(reg) f);
   |
   |
   = note: only integers, floats, SIMD vectors, pointers and function pointers can be used as arguments for inline assembly
error: cannot use value of type `&mut i32` for inline assembly
  --> /checkout/src/test/ui/asm/aarch64/type-check-2.rs:75:31
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
test result: FAILED. 12145 passed; 3 failed; 144 ignored; 0 measured; 0 filtered out; finished in 143.02s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:52
