plain
diff of stderr:

8    |      ^^^^^
9    |
10    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
11    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error[E0369]: binary operation `!=` cannot be applied to type `Error`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
20    |      ^^^^^
21    |
21    |
22    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
23    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/derives-span-PartialEq-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs:9:6
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
...
LL |      Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `!=` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs:9:6
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
...
LL |      Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0369`.

---
test result: FAILED. 11987 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 130.20s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:32
