plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMI6qAnOCbsZ
      Hardware UUID: 4203018E-580F-C1B5-9525-B745CECA79EB
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
---
failures:

---- [rustdoc] rustdoc/intra-doc/prim-methods-external-core.rs stdout ----

error: auxiliary build of "/Users/runner/work/rust/rust/src/test/rustdoc/intra-doc/auxiliary/my-core.rs" failed to compile: 
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/rustdoc/intra-doc/auxiliary/my-core.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "-C" "prefer-dynamic" "--out-dir" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/rustdoc/intra-doc/prim-methods-external-core/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/rustdoc/intra-doc/prim-methods-external-core/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-Wl,-exported_symbols_list,/var/folders/24/8k48jl6d249_n_qfxwsl6xvm0000gn/T/rustcg1wStR/list" "-m64" "-arch" "x86_64" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/rustdoc/intra-doc/prim-methods-external-core/auxiliary/my-core.my_core.9f3c60de-cgu.0.rcgu.o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/rustdoc/intra-doc/prim-methods-external-core/auxiliary/my-core.50fu2g9urkmisdsg.rcgu.o" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/rustdoc/intra-doc/prim-methods-external-core/auxiliary" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/rustdoc/intra-doc/prim-methods-external-core/auxiliary/libmy_core.dylib" "-Wl,-dead_strip" "-dynamiclib" "-Wl,-dylib" "-Wl,-install_name" "-Wl,@rpath/libmy_core.dylib" "-nodefaultlibs"
  = note: ld: dynamic main executables must link with libSystem.dylib for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error


---
test result: FAILED. 465 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 295.75s



command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/runner/work/rust/rust/src/test/rustdoc" "--build-base" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/rustdoc" "--stage-id" "stage2-x86_64-apple-darwin" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--npm" "/usr/local/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python@3.9/bin/python3.9" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1300.0.32.2\nSwift version 5.5-dev\n" "--lldb-python-dir" "/Applications/Xcode_13.0.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "13.0.0-rust-1.56.1-stable" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "stable" "--color" "always"


Build completed unsuccessfully in 1:38:05
