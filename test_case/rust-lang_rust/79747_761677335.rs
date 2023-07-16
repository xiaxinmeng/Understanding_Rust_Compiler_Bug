plain
diff of stderr:

21    | |_____^
22    |
23    = note: this pattern will always match, so the loop will never exit
-    = help: consider using a `loop { ... }` and adding a `let` inside it
+    = help: consider instead using a `loop { ... }` with a `let` inside it
25 
26 error: irrefutable if-let guard pattern
27   --> $DIR/deny-irrefutable-let-patterns.rs:14:18

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns/deny-irrefutable-let-patterns.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns/deny-irrefutable-let-patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/usefulness/deny-irrefutable-let-patterns.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/deny-irrefutable-let-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/deny-irrefutable-let-patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: irrefutable if-let pattern
   |
   |
LL |     if let _ = 5 {} //~ ERROR irrefutable if-let pattern
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/pattern/usefulness/deny-irrefutable-let-patterns.rs:4:9
   |
   |
LL | #![deny(irrefutable_let_patterns)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this pattern will always match, so the if-let is useless
   = help: consider removing the if-let and using a `let`
error: irrefutable while-let pattern
  --> /checkout/src/test/ui/pattern/usefulness/deny-irrefutable-let-patterns.rs:9:5
   |
   |
LL | /     while let _ = 5 { //~ ERROR irrefutable while-let pattern
LL | |         break;
LL | |     }
   |
   |
   = note: this pattern will always match, so the loop will never exit
   = help: consider instead using a `loop { ... }` with a `let` inside it

error: irrefutable if-let guard pattern
   |
   |
LL |         _ if let _ = 2 => {} //~ ERROR irrefutable if-let guard pattern
   |
   |
   = note: this pattern will always match, so the guard is useless
   = help: consider removing the guard and adding a `let` inside the match arm
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/while-let.rs stdout ----
diff of stderr:

11    |
12    = note: `#[warn(irrefutable_let_patterns)]` on by default
13    = note: this pattern will always match, so the loop will never exit
-    = help: consider using a `loop { ... }` and adding a `let` inside it
+    = help: consider instead using a `loop { ... }` with a `let` inside it
16 
17 warning: irrefutable while-let pattern

26    | |_______- in this macro invocation
26    | |_______- in this macro invocation
27    |
28    = note: this pattern will always match, so the loop will never exit
-    = help: consider using a `loop { ... }` and adding a `let` inside it
+    = help: consider instead using a `loop { ... }` with a `let` inside it
31 
32 warning: irrefutable while-let pattern

39    | |_____^
39    | |_____^
40    |
41    = note: this pattern will always match, so the loop will never exit
-    = help: consider using a `loop { ... }` and adding a `let` inside it
+    = help: consider instead using a `loop { ... }` with a `let` inside it
44 warning: 3 warnings emitted
45 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/while-let.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args while-let.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/while-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: irrefutable while-let pattern
  --> /checkout/src/test/ui/while-let.rs:7:13
   |
LL |               while let $p = $e $b
...
...
LL | /     foo!(_a, 1, {
LL | |         println!("irrefutable pattern");
LL | |     });
   | |_______- in this macro invocation
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = note: this pattern will always match, so the loop will never exit
   = help: consider instead using a `loop { ... }` with a `let` inside it

warning: irrefutable while-let pattern
  --> /checkout/src/test/ui/while-let.rs:7:13
   |
   |
LL |               while let $p = $e $b
...
...
LL | /     bar!(_a, 1, {
LL | |         println!("irrefutable pattern");
LL | |     });
   | |_______- in this macro invocation
   |
   = note: this pattern will always match, so the loop will never exit
   = help: consider instead using a `loop { ... }` with a `let` inside it

warning: irrefutable while-let pattern
  --> /checkout/src/test/ui/while-let.rs:27:5
   |
   |
LL | /     while let _a = 1 { //~ WARN irrefutable while-let
LL | |         println!("irrefutable pattern");
LL | |         break;
LL | |     }
   |
   |
   = note: this pattern will always match, so the loop will never exit
   = help: consider instead using a `loop { ... }` with a `let` inside it
warning: 3 warnings emitted


------------------------------------------
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:46
