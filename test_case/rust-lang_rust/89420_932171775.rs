plain
test [ui] ui/const-generics/params-in-ct-in-ty-param-lazy-norm.rs#min ... ok
test [ui] ui/const-generics/parser-error-recovery/issue-89013-type.rs ... ok
test [ui] ui/const-generics/parser-error-recovery/issue-89013-no-assoc.rs ... ok
test [ui] ui/const-generics/occurs-check/unused-substs-4.rs ... ok
test [ui] ui/const-generics/parser-error-recovery/issue-89013-no-kw.rs ... ok
test [ui] ui/const-generics/parser-error-recovery/issue-89013.rs ... ok
test [ui] ui/const-generics/raw-ptr-const-param-deref.rs#min ... ok
test [ui] ui/const-generics/raw-ptr-const-param.rs#full ... ok
test [ui] ui/const-generics/raw-ptr-const-param.rs#min ... ok
test [ui] ui/const-generics/slice-const-param-mismatch.rs#min ... ok
---
134 
- error: clobber_abi specified multiple times
-   --> $DIR/parse-error.rs:53:36
-    |
- LL |         asm!("", clobber_abi("C"), clobber_abi("C"));
-    |                  ----------------  ^^^^^^^^^^^^^^^^
-    |                  |
-    |                  clobber_abi previously specified here
143 error: duplicate argument named `a`
144   --> $DIR/parse-error.rs:55:36
145    |


342    |                   |
343    |                   options
344 
- error: clobber_abi specified multiple times
-   --> $DIR/parse-error.rs:123:35
+ error: `clobber_abi` cannot be used with `global_asm!`
347    |
347    |
348 LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));
-    |                 ----------------  ^^^^^^^^^^^^^^^^
-    |                 |
-    |                 clobber_abi previously specified here
352 
353 error: duplicate argument named `a`
354   --> $DIR/parse-error.rs:125:35


457 LL |         asm!("{1}", in("x0") foo, const bar);
458    |                                         ^^^ non-constant value
- error: aborting due to 66 previous errors
+ error: aborting due to 65 previous errors
461 
462 For more information about this error, try `rustc --explain E0435`.
462 For more information about this error, try `rustc --explain E0435`.
463 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/parse-error/parse-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/aarch64/parse-error.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
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
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
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

error: duplicate argument named `a`
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:55:36
  --> /checkout/src/test/ui/asm/aarch64/parse-error.rs:55:36
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


error: `clobber_abi` cannot be used with `global_asm!`
   |
   |
LL | global_asm!("", clobber_abi("C"), clobber_abi("C"));

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
error: aborting due to 65 previous errors

For more information about this error, try `rustc --explain E0435`.

---
test result: FAILED. 12096 passed; 1 failed; 141 ignored; 0 measured; 0 filtered out; finished in 143.50s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:20:23
