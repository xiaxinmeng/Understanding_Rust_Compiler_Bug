plain

---- [ui] ui/asm/aarch64/bad-reg.rs stdout ----
diff of stderr:

74 LL |         asm!("", in("xzr") foo);
76 
76 
- error: invalid register `x18`: x18 is used as a reserved register on some targets and cannot be used as an operand for inline asm
-   --> $DIR/bad-reg.rs:32:18
-    |
- LL |         asm!("", in("x18") foo);
- 
- 
83 error: invalid register `x19`: x19 is used internally by LLVM and cannot be used as an operand for inline asm
85    |


148 LL |         asm!("", in("v0") foo, out("q0") bar);
150 
- error: aborting due to 19 previous errors
+ error: aborting due to 18 previous errors
152 
152 
153 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg/bad-reg.stderr
To only update this specific test, also pass `--test-args asm/aarch64/bad-reg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/bad-reg.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+fp" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/bad-reg/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
stderr:
------------------------------------------
error: invalid register class `foo`: unknown register class
   |
   |
LL |         asm!("{}", in(foo) foo);


error: invalid register `foo`: unknown register
   |
   |
LL |         asm!("", in("foo") foo);

error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:16:15
   |
   |
LL |         asm!("{:z}", in(reg) foo);
   |               ^^^^   ----------- argument
   |               template modifier
   |
   |
   = note: the `reg` register class supports the following template modifiers: `w`, `x`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:18:15
   |
   |
LL |         asm!("{:r}", in(vreg) foo);
   |               ^^^^   ------------ argument
   |               template modifier
   |
   |
   = note: the `vreg` register class supports the following template modifiers: `b`, `h`, `s`, `d`, `q`, `v`
error: invalid asm template modifier for this register class
  --> /checkout/src/test/ui/asm/aarch64/bad-reg.rs:20:15
   |
   |
LL |         asm!("{:r}", in(vreg_low16) foo);
   |               ^^^^   ------------------ argument
   |               template modifier
   |
   |
   = note: the `vreg_low16` register class supports the following template modifiers: `b`, `h`, `s`, `d`, `q`, `v`

error: asm template modifiers are not allowed for `const` arguments
   |
   |
LL |         asm!("{:a}", const 0);
   |               ^^^^   ------- argument
   |               template modifier


error: asm template modifiers are not allowed for `sym` arguments
   |
   |
LL |         asm!("{:a}", sym main);
   |               ^^^^   -------- argument
   |               template modifier


error: invalid register `x29`: the frame pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("x29") foo);


error: invalid register `sp`: the stack pointer cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("sp") foo);


error: invalid register `xzr`: the zero register cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("xzr") foo);


error: invalid register `x19`: x19 is used internally by LLVM and cannot be used as an operand for inline asm
   |
   |
LL |         asm!("", in("x19") foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("", in("p0") foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", in(preg) foo);


error: register class `preg` can only be used as a clobber, not as an input or output
   |
   |
LL |         asm!("{}", out(preg) _);


error: register `x0` conflicts with register `x0`
   |
   |
LL |         asm!("", in("x0") foo, in("w0") bar);
   |                  ------------  ^^^^^^^^^^^^ register `x0`
   |                  register `x0`


error: register `x0` conflicts with register `x0`
   |
   |
LL |         asm!("", in("x0") foo, out("x0") bar);
   |                  ------------  ^^^^^^^^^^^^^ register `x0`
   |                  register `x0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("x0") foo, out("x0") bar);


error: register `v0` conflicts with register `v0`
   |
   |
LL |         asm!("", in("v0") foo, in("q0") bar);
   |                  ------------  ^^^^^^^^^^^^ register `v0`
   |                  register `v0`


error: register `v0` conflicts with register `v0`
   |
   |
LL |         asm!("", in("v0") foo, out("q0") bar);
   |                  ------------  ^^^^^^^^^^^^^ register `v0`
   |                  register `v0`
   |
   |
help: use `lateout` instead of `out` to avoid conflict
   |
   |
LL |         asm!("", in("v0") foo, out("q0") bar);

error: aborting due to 18 previous errors


---
test result: FAILED. 12324 passed; 1 failed; 154 ignored; 0 measured; 0 filtered out; finished in 147.42s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:20:06
