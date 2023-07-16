plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMWQaNgf0k7b
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---

---- [ui] ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs stdout ----
diff of stderr:

1 error: <unknown>:0:0: in function entry_function i32 (i32, i32, i32, i32, i32): secure entry function requires arguments on stack
+ thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
+   left: `LLVMing`,
+   left: `LLVMing`,
+  right: `Codegenning`', /Users/runner/work/rust/rust/compiler/rustc_codegen_ssa/src/back/write.rs:1464:21
+ note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
+ error: internal compiler error: unexpected panic
+ 
+ note: the compiler unexpectedly panicked. this is a bug.
+ 
+ 
+ note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+ 
+ note: rustc 1.59.0-nightly (da34d8956 2021-12-17) running on x86_64-apple-darwin
+ 
+ note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
+ query stack during panic:
+ end of query stack
3 error: aborting due to previous error
4 
---
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: <unknown>:0:0: in function entry_function i32 (i32, i32, i32, i32, i32): secure entry function requires arguments on stack
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
  left: `LLVMing`,
 right: `Codegenning`', /Users/runner/work/rust/rust/compiler/rustc_codegen_ssa/src/back/write.rs:1464:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (da34d8956 2021-12-17) running on x86_64-apple-darwin

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-apple-darwin target=x86_64-apple-darwin


command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/work/rust/rust/src/test/ui" "--build-base" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/ui" "--stage-id" "stage2-x86_64-apple-darwin" "--suite" "ui" "--mode" "ui" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--npm" "/usr/local/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python@3.9/bin/python3.9" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1300.0.32.4\nSwift version 5.5.1-dev\n" "--lldb-python-dir" "/Applications/Xcode_13.1.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 1:33:31
