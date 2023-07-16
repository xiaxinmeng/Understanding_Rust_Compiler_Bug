plain

---- [ui] ui/proc-macro/expand-literal.rs stdout ----
diff of stderr:

49    = help: message: expand_literal failed: ExpandLiteralError
50 
51 error: macro expansion ignores token `hello` and any following
-   --> $DIR/expand-literal.rs:116:50
53    |
53    |
54 LL | expand_literal_is!("string", echo_tts!("string"; hello));
55    |                              --------------------^^^^^-- help: you might be missing a semicolon here: `;`

59    = note: the usage of `echo_tts!` is likely invalid in expression context
60 
61 error: macro expansion ignores token `;` and any following
-   --> $DIR/expand-literal.rs:117:47
63    |
63    |
64 LL | expand_literal_is!("string", echo_pm!("string"; hello));
65    |                              -----------------^-------- help: you might be missing a semicolon here: `;`

69    = note: the usage of `echo_pm!` is likely invalid in expression context
71 error: proc macro panicked
-   --> $DIR/expand-literal.rs:119:1
+   --> $DIR/expand-literal.rs:120:1
73    |
73    |
74 LL | expand_literal_is!("fail", arbitrary_expression() + "etc");
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


77    = help: message: expand_literal failed: ExpandLiteralError
79 error: proc macro panicked
-   --> $DIR/expand-literal.rs:120:1
+   --> $DIR/expand-literal.rs:121:1
81    |
81    |
82 LL | expand_literal_is!("fail", echo_tts!(arbitrary_expression() + "etc"));


85    = help: message: expand_literal failed: ExpandLiteralError
87 error: proc macro panicked
-   --> $DIR/expand-literal.rs:121:1
+   --> $DIR/expand-literal.rs:122:1
89    |
89    |
90 LL | expand_literal_is!("fail", echo_expr!(arbitrary_expression() + "etc"));


93    = help: message: expand_literal failed: ExpandLiteralError
95 error: proc macro panicked
-   --> $DIR/expand-literal.rs:122:1
+   --> $DIR/expand-literal.rs:123:1
97    |
97    |
98 LL | expand_literal_is!("fail", echo_pm!(arbitrary_expression() + "etc"));


101    = help: message: expand_literal failed: ExpandLiteralError
102 
103 error: recursion limit reached while expanding `recursive_expand!`
-   --> $DIR/expand-literal.rs:124:16
105    |
105    |
106 LL | const _: u32 = recursive_expand!();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-literal/expand-literal.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-literal/expand-literal.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/expand-literal.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/expand-literal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-literal" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-literal/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:25:1
   |
LL | expand_literal_is!("", file!()); //~ ERROR: proc macro panicked
   |
   = help: message: assert failed
           expected: `"\"\""`
           expected: `"\"\""`
           expanded: `"\"/checkout/src/test/ui/proc-macro/expand-literal.rs\""`
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:107:1
   |
   |
LL | expand_literal_is!("fail", true); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:108:1
   |
   |
LL | expand_literal_is!("fail", false); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:109:1
   |
   |
LL | expand_literal_is!("fail", echo_tts!(true)); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:110:1
   |
   |
LL | expand_literal_is!("fail", echo_tts!(false)); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:113:1
   |
   |
LL | expand_literal_is!("string", "string"; hello); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError

error: macro expansion ignores token `hello` and any following
   |
   |
LL | expand_literal_is!("string", echo_tts!("string"; hello)); //~ ERROR: macro expansion ignores token `hello` and any following
   |                              --------------------^^^^^-- help: you might be missing a semicolon here: `;`
   |                              caused by the macro expansion here
   |
   |
   = note: the usage of `echo_tts!` is likely invalid in expression context

error: macro expansion ignores token `;` and any following
   |
   |
LL | expand_literal_is!("string", echo_pm!("string"; hello)); //~ ERROR: macro expansion ignores token `;` and any following
   |                              -----------------^-------- help: you might be missing a semicolon here: `;`
   |                              caused by the macro expansion here
   |
   |
   = note: the usage of `echo_pm!` is likely invalid in expression context
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:120:1
   |
   |
LL | expand_literal_is!("fail", arbitrary_expression() + "etc"); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:121:1
   |
   |
LL | expand_literal_is!("fail", echo_tts!(arbitrary_expression() + "etc")); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:122:1
   |
   |
LL | expand_literal_is!("fail", echo_expr!(arbitrary_expression() + "etc")); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-literal.rs:123:1
   |
   |
LL | expand_literal_is!("fail", echo_pm!(arbitrary_expression() + "etc")); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_literal failed: ExpandLiteralError

error: recursion limit reached while expanding `recursive_expand!`
   |
   |
LL | const _: u32 = recursive_expand!(); //~ ERROR: recursion limit reached while expanding `recursive_expand!`
   |
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`expand_literal`)
   = note: this error originates in the macro `recursive_expand` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 13 previous errors


------------------------------------------
---
test result: FAILED. 12056 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 121.25s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:15
