plain
.................................................................................................... 7400/11440
.................................................................................................... 7500/11440
........................................................................i..ii....................... 7600/11440
........................................ii.......................................................... 7700/11440
.................................................F..i........i....F..F.............................. 7800/11440
......................................i............................................................. 8000/11440
.................................................................................................... 8100/11440
.................................................................................................... 8200/11440
..................i................................................................................. 8300/11440
---

---- [ui] ui/or-patterns/fn-param-wrap-parens.rs stdout ----
diff of stderr:

- error: an or-pattern parameter must be wrapped in parenthesis
+ error: an or-pattern parameter must be wrapped in parentheses
2   --> $DIR/fn-param-wrap-parens.rs:14:9
3    |
4 LL | fn fun1(A | B: E) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/fn-param-wrap-parens/fn-param-wrap-parens.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/fn-param-wrap-parens/fn-param-wrap-parens.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/fn-param-wrap-parens.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/fn-param-wrap-parens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/fn-param-wrap-parens" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/fn-param-wrap-parens/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: an or-pattern parameter must be wrapped in parentheses
  --> /checkout/src/test/ui/or-patterns/fn-param-wrap-parens.rs:14:9
   |
LL | fn fun1(A | B: E) {} //~ ERROR an or-pattern parameter must be wrapped in parenthesis
   |         ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/or-patterns/remove-leading-vert.rs stdout ----
diff of stderr:

- error: an or-pattern parameter must be wrapped in parenthesis
+ error: an or-pattern parameter must be wrapped in parentheses
3    |
3    |
4 LL |     fn fun1( | A: E) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/remove-leading-vert/remove-leading-vert.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/remove-leading-vert/remove-leading-vert.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/remove-leading-vert.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/remove-leading-vert.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/remove-leading-vert" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/remove-leading-vert/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: an or-pattern parameter must be wrapped in parentheses
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:12:14
   |
LL |     fn fun1( | A: E) {} //~ ERROR an or-pattern parameter must be wrapped in parenthesis
   |              ^^^ help: remove the leading `|`: `A`

error: unexpected `||` before function parameter
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:13:14
   |
LL |     fn fun2( || A: E) {} //~ ERROR unexpected `||` before function parameter
   |              ^^ help: remove the `||`
   |
   = note: alternatives in or-patterns are separated with `|`, not `||`

error: unexpected token `||` in pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:15:11
   |
LL |     let ( || A): (E); //~ ERROR unexpected token `||` in pattern
   |           ^^ help: use a single `|` to separate multiple alternative patterns: `|`

error: unexpected token `||` in pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:18:11
   |
LL |     let [ || A ]: [E; 1]; //~ ERROR unexpected token `||` in pattern
   |           ^^ help: use a single `|` to separate multiple alternative patterns: `|`

error: unexpected token `||` in pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:20:13
   |
LL |     let TS( || A ): TS; //~ ERROR unexpected token `||` in pattern
   |             ^^ help: use a single `|` to separate multiple alternative patterns: `|`

error: unexpected token `||` in pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:22:17
   |
LL |     let NS { f: || A }: NS; //~ ERROR unexpected token `||` in pattern
   |                 ^^ help: use a single `|` to separate multiple alternative patterns: `|`

error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:27:13
   |
LL |     let ( A | ): E; //~ ERROR a trailing `|` is not allowed in an or-pattern
   |           - ^ help: remove the `|`
   |           while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:28:12
   |
LL |     let (a |,): (E,); //~ ERROR a trailing `|` is not allowed in an or-pattern
   |          - ^ help: remove the `|`
   |          while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:29:17
   |
LL |     let ( A | B | ): E; //~ ERROR a trailing `|` is not allowed in an or-pattern
   |           -     ^ help: remove the `|`
   |           while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:30:17
   |
LL |     let [ A | B | ]: [E; 1]; //~ ERROR a trailing `|` is not allowed in an or-pattern
   |           -     ^ help: remove the `|`
   |           while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:31:18
   |
LL |     let S { f: B | }; //~ ERROR a trailing `|` is not allowed in an or-pattern
   |                - ^ help: remove the `|`
   |                while parsing this or-pattern starting here


error: unexpected token `||` in pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:32:13
   |
LL |     let ( A || B | ): E; //~ ERROR unexpected token `||` in pattern
   |           - ^^ help: use a single `|` to separate multiple alternative patterns: `|`
   |           while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:32:18
   |
LL |     let ( A || B | ): E; //~ ERROR unexpected token `||` in pattern
   |           -      ^ help: remove the `|`
   |           while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:35:11
   |
LL |         A | => {} //~ ERROR a trailing `|` is not allowed in an or-pattern
   |         - ^ help: remove the `|`
   |         while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:36:11
   |
LL |         A || => {} //~ ERROR a trailing `|` is not allowed in an or-pattern
   |         - ^^ help: remove the `||`
   |         while parsing this or-pattern starting here
   |
   |
   = note: alternatives in or-patterns are separated with `|`, not `||`

error: unexpected token `||` in pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:37:11
   |
LL |         A || B | => {} //~ ERROR unexpected token `||` in pattern
   |         - ^^ help: use a single `|` to separate multiple alternative patterns: `|`
   |         while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:37:16
   |
LL |         A || B | => {} //~ ERROR unexpected token `||` in pattern
   |         -      ^ help: remove the `|`
   |         while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:39:17
   |
LL |         | A | B | => {}
   |         -       ^ help: remove the `|`
   |         while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:43:11
   |
LL |     let a | : u8 = 0; //~ ERROR a trailing `|` is not allowed in an or-pattern
   |         - ^ help: remove the `|`
   |         while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:44:11
   |
LL |     let a | = 0; //~ ERROR a trailing `|` is not allowed in an or-pattern
   |         - ^ help: remove the `|`
   |         while parsing this or-pattern starting here


error: a trailing `|` is not allowed in an or-pattern
  --> /checkout/src/test/ui/or-patterns/remove-leading-vert.rs:45:11
   |
LL |     let a | ; //~ ERROR a trailing `|` is not allowed in an or-pattern
   |         - ^ help: remove the `|`
   |         while parsing this or-pattern starting here

error: aborting due to 21 previous errors



------------------------------------------


---- [ui] ui/or-patterns/or-patterns-syntactic-fail.rs stdout ----
diff of stderr:

- error: an or-pattern parameter must be wrapped in parenthesis
+ error: an or-pattern parameter must be wrapped in parentheses
2   --> $DIR/or-patterns-syntactic-fail.rs:17:13
3    |
4 LL |     fn fun1(A | B: E) {}

5    |             ^^^^^ help: wrap the pattern in parentheses: `(A | B)`
6 
- error: an or-pattern parameter must be wrapped in parenthesis
+ error: an or-pattern parameter must be wrapped in parentheses
8   --> $DIR/or-patterns-syntactic-fail.rs:19:13
9    |
10 LL |     fn fun2(| A | B: E) {}

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/or-patterns-syntactic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args or-patterns/or-patterns-syntactic-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/or-patterns/or-patterns-syntactic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: an or-pattern parameter must be wrapped in parentheses
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:17:13
   |
LL |     fn fun1(A | B: E) {} //~ ERROR an or-pattern parameter must be wrapped in parenthesis
   |             ^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error: an or-pattern parameter must be wrapped in parentheses
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:19:13
   |
LL |     fn fun2(| A | B: E) {}
   |             ^^^^^^^ help: wrap the pattern in parentheses: `(A | B)`

error[E0369]: no implementation for `E | ()`
  --> /checkout/src/test/ui/or-patterns/or-patterns-syntactic-fail.rs:13:22
   |
LL |     let _ = |A | B: E| (); //~ ERROR no implementation for `E | ()`
   |                  ----^ -- ()
   |                  E
   |
   |
   = note: an implementation of `std::ops::BitOr` might be missing for `E`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0369`.

---
test result: FAILED. 11345 passed; 3 failed; 92 ignored; 0 measured; 0 filtered out; finished in 138.49s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:24
