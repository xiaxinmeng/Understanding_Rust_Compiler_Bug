plain
diff of stderr:

2   --> $DIR/expand-expr.rs:25:1
3    |
4 LL | expand_expr_is!("", file!());
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = help: message: assert failed
8            expected: `"\"\""`
8            expected: `"\"\""`

18   --> $DIR/expand-expr.rs:99:1
19    |
20 LL | expand_expr_is!("string", "string"; hello);
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
22    |
22    |
23    = help: message: expand_expr failed: ExpandError

32   --> $DIR/expand-expr.rs:103:1
33    |
33    |
34 LL | expand_expr_is!("fail", $);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
36    |
36    |
37    = help: message: expand_expr failed: ExpandError

46   --> $DIR/expand-expr.rs:105:1
47    |
47    |
48 LL | expand_expr_is!("fail", echo_tts!($));
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
50    |
50    |
51    = help: message: expand_expr failed: ExpandError

60   --> $DIR/expand-expr.rs:106:1
61    |
61    |
62 LL | expand_expr_is!("fail", echo_pm!($));
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
64    |
64    |
65    = help: message: expand_expr failed: ExpandError

88   --> $DIR/expand-expr.rs:115:1
89    |
89    |
90 LL | expand_expr_is!("fail", arbitrary_expression() + "etc");
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
92    |
92    |
93    = help: message: expand_expr failed: ExpandError

96   --> $DIR/expand-expr.rs:116:1
97    |
97    |
98 LL | expand_expr_is!("fail", echo_tts!(arbitrary_expression() + "etc"));
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
100    |
100    |
101    = help: message: expand_expr failed: ExpandError

104   --> $DIR/expand-expr.rs:117:1
105    |
105    |
106 LL | expand_expr_is!("fail", echo_expr!(arbitrary_expression() + "etc"));
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
108    |
108    |
109    = help: message: expand_expr failed: ExpandError

112   --> $DIR/expand-expr.rs:118:1
113    |
113    |
114 LL | expand_expr_is!("fail", echo_pm!(arbitrary_expression() + "etc"));
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
116    |
116    |
117    = help: message: expand_expr failed: ExpandError


122 LL | const _: u32 = recursive_expand!();
124    |
124    |
-    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`expand_expr`)
+    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`expand_expr`)
126    = note: this error originates in the macro `recursive_expand` (in Nightly builds, run with -Z macro-backtrace for more info)
128 error: aborting due to 16 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-expr/expand-expr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/expand-expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/expand-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-expr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-expr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:25:1
   |
LL | expand_expr_is!("", file!()); //~ ERROR: proc macro panicked
   |
   = help: message: assert failed
           expected: `"\"\""`
           expected: `"\"\""`
           expanded: `"\"/checkout/src/test/ui/proc-macro/expand-expr.rs\""`

error: expected one of `.`, `?`, or an operator, found `;`
   |
   |
LL | expand_expr_is!("string", "string"; hello); //~ ERROR: expected one of `.`, `?`, or an operator, found `;`
   |                                   ^ expected one of `.`, `?`, or an operator
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:99:1
   |
   |
LL | expand_expr_is!("string", "string"; hello); //~ ERROR: expected one of `.`, `?`, or an operator, found `;`
   |
   |
   = help: message: expand_expr failed: ExpandError
error: expected expression, found `$`
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:103:25
   |
   |
LL | expand_expr_is!("fail", $); //~ ERROR: expected expression, found `$`
   |                         ^ expected expression
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:103:1
   |
   |
LL | expand_expr_is!("fail", $); //~ ERROR: expected expression, found `$`
   |
   |
   = help: message: expand_expr failed: ExpandError
error: expected expression, found `$`
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:31:23
   |
   |
LL |     ($($t:tt)*) => { $($t)* };  //~ ERROR: expected expression, found `$`

error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:105:1
   |
   |
LL | expand_expr_is!("fail", echo_tts!($)); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_expr failed: ExpandError
error: expected expression, found `$`
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:106:34
   |
   |
LL | expand_expr_is!("fail", echo_pm!($)); //~ ERROR: expected expression, found `$`
   |                                  ^ expected expression
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:106:1
   |
   |
LL | expand_expr_is!("fail", echo_pm!($)); //~ ERROR: expected expression, found `$`
   |
   |
   = help: message: expand_expr failed: ExpandError

error: macro expansion ignores token `hello` and any following
   |
   |
LL | expand_expr_is!("string", echo_tts!("string"; hello)); //~ ERROR: macro expansion ignores token `hello` and any following
   |                           --------------------^^^^^-- help: you might be missing a semicolon here: `;`
   |                           caused by the macro expansion here
   |
   |
   = note: the usage of `echo_tts!` is likely invalid in expression context

error: macro expansion ignores token `;` and any following
   |
   |
LL | expand_expr_is!("string", echo_pm!("string"; hello)); //~ ERROR: macro expansion ignores token `;` and any following
   |                           -----------------^-------- help: you might be missing a semicolon here: `;`
   |                           caused by the macro expansion here
   |
   |
   = note: the usage of `echo_pm!` is likely invalid in expression context
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:115:1
   |
   |
LL | expand_expr_is!("fail", arbitrary_expression() + "etc"); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_expr failed: ExpandError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:116:1
   |
   |
LL | expand_expr_is!("fail", echo_tts!(arbitrary_expression() + "etc")); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_expr failed: ExpandError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:117:1
   |
   |
LL | expand_expr_is!("fail", echo_expr!(arbitrary_expression() + "etc")); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_expr failed: ExpandError
error: proc macro panicked
  --> /checkout/src/test/ui/proc-macro/expand-expr.rs:118:1
   |
   |
LL | expand_expr_is!("fail", echo_pm!(arbitrary_expression() + "etc")); //~ ERROR: proc macro panicked
   |
   |
   = help: message: expand_expr failed: ExpandError

error: recursion limit reached while expanding `recursive_expand!`
   |
   |
LL | const _: u32 = recursive_expand!(); //~ ERROR: recursion limit reached while expanding `recursive_expand!`
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`expand_expr`)
   = note: this error originates in the macro `recursive_expand` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 16 previous errors


------------------------------------------
---
test result: FAILED. 12271 passed; 1 failed; 110 ignored; 0 measured; 0 filtered out; finished in 130.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:34
