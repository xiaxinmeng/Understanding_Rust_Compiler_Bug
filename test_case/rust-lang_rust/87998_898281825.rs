plain
failures:

---- [ui] ui/moves/issue-72649-uninit-in-loop.rs stdout ----
normalized stderr:
error[E0382]: use of moved value: `value`
   |
LL |         let value = NonCopy{};
LL |         let value = NonCopy{};
   |             ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
LL |
LL |         let _used = value;
   |                     ----- value moved here
LL |
LL |         let _used2 = value;
   |                      ^^^^^ value used here after move

error[E0382]: use of moved value: `value`
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     let value = NonCopy{};
LL |     let value = NonCopy{};
   |         ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used = value;
   |                     ----- value moved here
...
LL |             let _used2 = value;
   |                          ^^^^^ value used here after move

error[E0382]: use of moved value: `value`
   |
LL |     let value = NonCopy{};
LL |     let value = NonCopy{};
   |         ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used = value;
   |                     ^^^^^ value moved here, in previous iteration of loop

error[E0382]: use of moved value: `value`
   |
LL |     let mut value = NonCopy{};
LL |     let mut value = NonCopy{};
   |         --------- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used2 = value;
   |                      ^^^^^ value moved here, in previous iteration of loop

error[E0381]: use of possibly-uninitialized variable: `value`
   |
   |
LL |         let _used = value;
   |                     ^^^^^ use of possibly-uninitialized `value`

error[E0381]: use of possibly-uninitialized variable: `value`
   |
   |
LL |         let _used = value;
   |                     ^^^^^ use of possibly-uninitialized `value`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0381, E0382.
For more information about an error, try `rustc --explain E0381`.
---
To only update this specific test, also pass `--test-args moves/issue-72649-uninit-in-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/moves/issue-72649-uninit-in-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/issue-72649-uninit-in-loop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/moves/issue-72649-uninit-in-loop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value: `value`
   |
LL |         let value = NonCopy{};
LL |         let value = NonCopy{};
   |             ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
LL |         //~^ NOTE move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
LL |         let _used = value;
   |                     ----- value moved here
LL |         //~^ NOTE value moved here
LL |         let _used2 = value; //~ ERROR use of moved value: `value`
   |                      ^^^^^ value used here after move

error[E0382]: use of moved value: `value`
   |
LL |     let value = NonCopy{};
LL |     let value = NonCopy{};
   |         ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used = value;
   |                     ----- value moved here
...
LL |             let _used2 = value; //~ ERROR use of moved value: `value`
   |                          ^^^^^ value used here after move

error[E0382]: use of moved value: `value`
   |
LL |     let value = NonCopy{};
LL |     let value = NonCopy{};
   |         ----- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used = value; //~ ERROR use of moved value: `value`
   |                     ^^^^^ value moved here, in previous iteration of loop

error[E0382]: use of moved value: `value`
   |
LL |     let mut value = NonCopy{};
LL |     let mut value = NonCopy{};
   |         --------- move occurs because `value` has type `NonCopy`, which does not implement the `Copy` trait
...
LL |         let _used2 = value; //~ ERROR use of moved value: `value`
   |                      ^^^^^ value moved here, in previous iteration of loop

error[E0381]: use of possibly-uninitialized variable: `value`
   |
   |
LL |         let _used = value; //~ ERROR use of possibly-uninitialized variable: `value`
   |                     ^^^^^ use of possibly-uninitialized `value`

error[E0381]: use of possibly-uninitialized variable: `value`
   |
   |
LL |         let _used = value; //~ ERROR use of possibly-uninitialized variable: `value`
   |                     ^^^^^ use of possibly-uninitialized `value`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0381, E0382.
For more information about an error, try `rustc --explain E0381`.
---
test result: FAILED. 12029 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 101.47s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:04
