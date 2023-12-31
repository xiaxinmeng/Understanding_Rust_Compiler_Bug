plain
.................................................................................................... 9400/11718
.................................................................................................... 9500/11718
............................................................i......i................................ 9600/11718
.................................................................................................... 9700/11718
......iiiiiii..iiiiii.i............................................................................. 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
---
normalized stderr:
warning: an associated function with this name may be added to the standard library in the future
  --> $DIR/macro-expanded.rs:20:19
   |
LL |             false.not();
...
...
LL | m!();
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `not(...)` to keep using the current method
   = help: add `#![feature(bool_not_method)]` to the crate attributes to enable `core::bool::<impl bool>::not`

warning: an associated function with this name may be added to the standard library in the future
  --> $DIR/macro-expanded.rs:36:15
   |
   |
LL |         false.not();
...
...
LL | n!();
   |
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `not(...)` to keep using the current method
   = help: add `#![feature(bool_not_method)]` to the crate attributes to enable `core::bool::<impl bool>::not`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

warning: 2 warnings emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/macro-expanded/macro-expanded.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args underscore-imports/macro-expanded.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/underscore-imports/macro-expanded.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/macro-expanded" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/underscore-imports/macro-expanded/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/underscore-imports/macro-expanded.rs:20:19
   |
LL |             false.not();
...
...
LL | m!();
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `not(...)` to keep using the current method
   = help: add `#![feature(bool_not_method)]` to the crate attributes to enable `core::bool::<impl bool>::not`

warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/underscore-imports/macro-expanded.rs:36:15
   |
   |
LL |         false.not();
...
...
LL | n!();
   |
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `not(...)` to keep using the current method
   = help: add `#![feature(bool_not_method)]` to the crate attributes to enable `core::bool::<impl bool>::not`

warning: 2 warnings emitted


---
test result: FAILED. 11621 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 134.70s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:46
