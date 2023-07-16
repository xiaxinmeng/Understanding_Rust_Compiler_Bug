plain
test [ui] ui/borrowck/regions-escape-bound-fn-2.rs ... ok
test [ui] ui/borrowck/mut-borrow-in-loop-2.rs ... ok
test [ui] ui/borrowck/return-local-binding-from-desugaring.rs ... ok
test [ui] ui/borrowck/kindck-implicit-close-over-mut-var.rs ... ok
test [ui] ui/borrowck/suggest-local-var-imm-and-mut.rs ... ok
test [ui] ui/borrowck/suggest-local-var-double-mut.rs ... ok
test [ui] ui/borrowck/two-phase-across-loop.rs ... ok
test [ui] ui/borrowck/lazy-init.rs ... ok
test [ui] ui/borrowck/regions-escape-unboxed-closure.rs ... ok
---
test [ui (nll)] ui/borrowck/two-phase-reservation-sharing-interference-2.rs#nll2018 ... ignored
test [ui (nll)] ui/borrowck/regions-escape-bound-fn.rs ... ok
test [ui (nll)] ui/borrowck/two-phase-across-loop.rs ... ok
test [ui (nll)] ui/borrowck/two-phase-activation-sharing-interference.rs#nll_target ... ok
test [ui (nll)] ui/borrowck/suggest-local-var-imm-and-mut.rs ... FAILED
test [ui (nll)] ui/borrowck/two-phase-multi-mut.rs ... ok
test [ui (nll)] ui/borrowck/two-phase-allow-access-during-reservation.rs#nll_target ... ok
test [ui (nll)] ui/borrowck/two-phase-cannot-nest-mut-self-calls.rs ... ok
test [ui (nll)] ui/borrowck/kindck-implicit-close-over-mut-var.rs ... ok
---
test [ui (nll)] ui/threads-sendsync/mpsc_stress.rs ... ok

failures:

---- [ui (nll)] ui/borrowck/suggest-local-var-imm-and-mut.rs stdout ----


7    |             |    |   mutable borrow occurs here
8    |             |    immutable borrow later used by call
9    |             immutable borrow occurs here
+ help: try adding a local storing this argument...
+ help: try adding a local storing this argument...
+   --> $DIR/suggest-local-var-imm-and-mut.rs:12:22
+    |
+ LL |             self.foo(self.bar());
+    |                      ^^^^^^^^^^
+ help: ...and then using that local as the argument to this call
+   --> $DIR/suggest-local-var-imm-and-mut.rs:12:13
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+ LL |             self.foo(self.bar());
10 
10 
11 error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
-   --> $DIR/suggest-local-var-imm-and-mut.rs:24:29
+   --> $DIR/suggest-local-var-imm-and-mut.rs:24:39
13    |
14 LL |             Self::foo(self, Self::bar(self));
-    |             --------- ----  ^^^^^^^^^^^^^^^ mutable borrow occurs here
+    |             --------- ----            ^^^^ mutable borrow occurs here
16    |             |         |
17    |             |         immutable borrow occurs here
18    |             immutable borrow later used by call

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/suggest-local-var-imm-and-mut.nll/suggest-local-var-imm-and-mut.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/suggest-local-var-imm-and-mut.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/suggest-local-var-imm-and-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/suggest-local-var-imm-and-mut.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/suggest-local-var-imm-and-mut.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> /checkout/src/test/ui/borrowck/suggest-local-var-imm-and-mut.rs:12:22
   |
LL |             self.foo(self.bar()); //~ ERROR
   |             |    |   |
   |             |    |   |
   |             |    |   mutable borrow occurs here
   |             |    immutable borrow later used by call
   |             immutable borrow occurs here
help: try adding a local storing this argument...
help: try adding a local storing this argument...
  --> /checkout/src/test/ui/borrowck/suggest-local-var-imm-and-mut.rs:12:22
   |
LL |             self.foo(self.bar()); //~ ERROR
   |                      ^^^^^^^^^^
help: ...and then using that local as the argument to this call
  --> /checkout/src/test/ui/borrowck/suggest-local-var-imm-and-mut.rs:12:13
   |
LL |             self.foo(self.bar()); //~ ERROR


error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
  --> /checkout/src/test/ui/borrowck/suggest-local-var-imm-and-mut.rs:24:39
   |
LL |             Self::foo(self, Self::bar(self)); //~ ERROR
   |             --------- ----            ^^^^ mutable borrow occurs here
   |             |         |
   |             |         immutable borrow occurs here
   |             immutable borrow later used by call
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0502`.


------------------------------------------



failures:
    [ui (nll)] ui/borrowck/suggest-local-var-imm-and-mut.rs
test result: FAILED. 12324 passed; 1 failed; 156 ignored; 0 measured; 0 filtered out; finished in 114.27s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:19:30
