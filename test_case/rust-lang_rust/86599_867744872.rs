plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12013 tests
.................................................................................................... 100/12013
..........................................ii.F........ii...........F................................ 200/12013
.................................................................................................... 400/12013
.................................................................................................... 500/12013
.................................................................................................... 600/12013
.........................................i.......................................................... 700/12013
---

---- [ui] ui/asm/bad-options.rs stdout ----
diff of stderr:

28 LL |         asm!("{}", out(reg) foo, options(noreturn));
30 
30 
- error: expected one of `)` or `att_syntax`, found `nomem`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
33    |
33    |
34 LL | global_asm!("", options(nomem));

-    |                         ^^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
36 
- error: expected one of `)` or `att_syntax`, found `readonly`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `readonly`
39    |
39    |
40 LL | global_asm!("", options(readonly));

-    |                         ^^^^^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
42 
- error: expected one of `)` or `att_syntax`, found `noreturn`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `noreturn`
45    |
45    |
46 LL | global_asm!("", options(noreturn));

-    |                         ^^^^^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
48 
- error: expected one of `)` or `att_syntax`, found `pure`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `pure`
51    |
51    |
52 LL | global_asm!("", options(pure));

-    |                         ^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^ expected one of `)`, `att_syntax`, or `raw`
54 
- error: expected one of `)` or `att_syntax`, found `nostack`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nostack`
57    |
57    |
58 LL | global_asm!("", options(nostack));

-    |                         ^^^^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
60 
- error: expected one of `)` or `att_syntax`, found `preserves_flags`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `preserves_flags`
63    |
63    |
64 LL | global_asm!("", options(preserves_flags));

-    |                         ^^^^^^^^^^^^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^^^^^^^^^^^^ expected one of `)`, `att_syntax`, or `raw`
67 error: aborting due to 11 previous errors
68 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-options/bad-options.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/bad-options.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-options/auxiliary"
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


error: asm with `pure` option must have at least one output
   |
   |
LL |         asm!("", options(pure, nomem, noreturn));


error: asm with `pure` option must have at least one output
   |
   |
LL |         asm!("{}", in(reg) foo, options(pure, nomem));


error: asm outputs are not allowed with the `noreturn` option
   |
   |
LL |         asm!("{}", out(reg) foo, options(noreturn));


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
error: aborting due to 11 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/asm/parse-error.rs stdout ----
diff of stderr:

64 LL |         asm!("{}", sym foo + bar);
66 
66 
- error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, or `readonly`, found `foo`
+ error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
69    |
69    |
70 LL |         asm!("", options(foo));
-    |                          ^^^ expected one of 8 possible tokens
+    |                          ^^^ expected one of 9 possible tokens
72 
72 
73 error: expected one of `)` or `,`, found `foo`


76 LL |         asm!("", options(nomem foo));
77    |                                ^^^ expected one of `)` or `,`
78 
- error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, or `readonly`, found `foo`
+ error: expected one of `)`, `att_syntax`, `nomem`, `noreturn`, `nostack`, `preserves_flags`, `pure`, `raw`, or `readonly`, found `foo`
81    |
81    |
82 LL |         asm!("", options(nomem, foo));
-    |                                 ^^^ expected one of 8 possible tokens
+    |                                 ^^^ expected one of 9 possible tokens
84 
85 error: arguments are not allowed after options
85 error: arguments are not allowed after options
86   --> $DIR/parse-error.rs:37:31

200 LL | global_asm!("{}", const(reg) FOO);
201    |                              ^^^ expected one of `,`, `.`, `?`, or an operator
202 
- error: expected one of `)` or `att_syntax`, found `FOO`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `FOO`
205    |
205    |
206 LL | global_asm!("", options(FOO));

-    |                         ^^^ expected one of `)` or `att_syntax`
+    |                         ^^^ expected one of `)`, `att_syntax`, or `raw`
208 
- error: expected one of `)` or `att_syntax`, found `nomem`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
211    |
211    |
212 LL | global_asm!("", options(nomem FOO));

-    |                         ^^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
214 
- error: expected one of `)` or `att_syntax`, found `nomem`
+ error: expected one of `)`, `att_syntax`, or `raw`, found `nomem`
217    |
217    |
218 LL | global_asm!("", options(nomem, FOO));

-    |                         ^^^^^ expected one of `)` or `att_syntax`
+    |                         ^^^^^ expected one of `)`, `att_syntax`, or `raw`
221 error: arguments are not allowed after options
222   --> $DIR/parse-error.rs:87:30



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error/parse-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/parse-error.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/parse-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/parse-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/parse-error.rs:11:14
   |
LL |         asm!(foo);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/parse-error.rs:13:19
   |
   |
LL |         asm!("{}" foo);
   |                   ^^^ expected `,`
error: expected operand, options, or additional template string
  --> /checkout/src/test/ui/asm/parse-error.rs:15:20
   |
   |
LL |         asm!("{}", foo);
   |                    ^^^ expected operand, options, or additional template string

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
  --> /checkout/src/test/ui/asm/parse-error.rs:21:27
   |
   |
LL |         asm!("{}", in(reg));
   |                           ^ expected expression
error: expected register class or explicit register
  --> /checkout/src/test/ui/asm/parse-error.rs:23:26
   |
   |
LL |         asm!("{}", inout(=) foo => bar);

error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:25:37
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
  --> /checkout/src/test/ui/asm/parse-error.rs:37:31
   |
   |
LL |         asm!("{}", options(), const foo);
   |                    ---------  ^^^^^^^^^ argument
   |                    previous options

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/parse-error.rs:40:36
  --> /checkout/src/test/ui/asm/parse-error.rs:40:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ duplicate argument
   |                     previously here

error: argument never used
  --> /checkout/src/test/ui/asm/parse-error.rs:40:36
  --> /checkout/src/test/ui/asm/parse-error.rs:40:36
   |
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                    ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`
error: explicit register arguments cannot have names
  --> /checkout/src/test/ui/asm/parse-error.rs:45:18
   |
   |
LL |         asm!("", a = in("eax") foo);


error: named arguments cannot follow explicit register arguments
   |
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument


error: named arguments cannot follow explicit register arguments
   |
   |
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                     -------------  ^^^^^^^^^^^^^ named argument
   |                     explicit register argument

error: positional arguments cannot follow named arguments or explicit register arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:53:36
  --> /checkout/src/test/ui/asm/parse-error.rs:53:36
   |
LL |         asm!("{1}", in("eax") foo, const bar);
   |                     -------------  ^^^^^^^^^ positional argument
   |                     explicit register argument


error: expected one of `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `""`
   |
   |
LL |         asm!("", options(), "");
   |                             ^^ expected one of 8 possible tokens

error: expected one of `const`, `in`, `inlateout`, `inout`, `lateout`, `options`, `out`, or `sym`, found `"{}"`
   |
   |
LL |         asm!("{}", in(reg) foo, "{}", out(reg) foo);
   |                                 ^^^^ expected one of 8 possible tokens
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/parse-error.rs:60:14
   |
   |
LL |         asm!(format!("{{{}}}", 0), in(reg) foo);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/parse-error.rs:62:21
   |
LL |         asm!("{1}", format!("{{{}}}", 0), in(reg) foo, out(reg) bar);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: requires at least a template string argument
error: requires at least a template string argument
  --> /checkout/src/test/ui/asm/parse-error.rs:69:1
   |
LL | global_asm!();

error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/parse-error.rs:71:13
   |
   |
LL | global_asm!(FOO);

error: expected token: `,`
  --> /checkout/src/test/ui/asm/parse-error.rs:73:18
   |
   |
LL | global_asm!("{}" FOO);
   |                  ^^^ expected `,`
error: expected operand, options, or additional template string
  --> /checkout/src/test/ui/asm/parse-error.rs:75:19
   |
   |
LL | global_asm!("{}", FOO);
   |                   ^^^ expected operand, options, or additional template string
error: expected expression, found end of macro arguments
  --> /checkout/src/test/ui/asm/parse-error.rs:77:24
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
  --> /checkout/src/test/ui/asm/parse-error.rs:87:30
   |
   |
LL | global_asm!("{}", options(), const FOO);
   |                   ---------  ^^^^^^^^^ argument
   |                   previous options

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/parse-error.rs:89:35
  --> /checkout/src/test/ui/asm/parse-error.rs:89:35
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                    -------------  ^^^^^^^^^^^^^ duplicate argument
   |                    previously here

error: argument never used
  --> /checkout/src/test/ui/asm/parse-error.rs:89:35
  --> /checkout/src/test/ui/asm/parse-error.rs:89:35
   |
LL | global_asm!("{a}", a = const FOO, a = const BAR);
   |                                   ^^^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {1} */"`

error: expected one of `const` or `options`, found `""`
   |
   |
LL | global_asm!("", options(), "");
   |                            ^^ expected one of `const` or `options`

error: expected one of `const` or `options`, found `"{}"`
   |
   |
LL | global_asm!("{}", const FOO, "{}", const FOO);
   |                              ^^^^ expected one of `const` or `options`
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/parse-error.rs:96:13
   |
   |
LL | global_asm!(format!("{{{}}}", 0), const FOO);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm template must be a string literal
error: asm template must be a string literal
  --> /checkout/src/test/ui/asm/parse-error.rs:98:20
   |
LL | global_asm!("{1}", format!("{{{}}}", 0), const FOO, const BAR);
   |
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0435]: attempt to use a non-constant value in a constant
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/parse-error.rs:37:37
   |
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{}", options(), const foo);
   |                                     ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/parse-error.rs:40:31
   |
LL |     let mut foo = 0;
LL |     let mut foo = 0;
   |      ---------- help: consider using `const` instead of `let`: `const foo`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                               ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/parse-error.rs:40:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", a = const foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/parse-error.rs:47:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/parse-error.rs:50:46
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{a}", in("eax") foo, a = const bar);
   |                                              ^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/asm/parse-error.rs:53:42
   |
LL |     let mut bar = 0;
LL |     let mut bar = 0;
   |      ---------- help: consider using `const` instead of `let`: `const bar`
...
LL |         asm!("{1}", in("eax") foo, const bar);
   |                                          ^^^ non-constant value
error: aborting due to 47 previous errors

For more information about this error, try `rustc --explain E0435`.

---
test result: FAILED. 11909 passed; 2 failed; 102 ignored; 0 measured; 0 filtered out; finished in 119.87s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:03
