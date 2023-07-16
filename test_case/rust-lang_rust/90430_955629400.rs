plain
failures:

---- [ui] ui/rust-2018/uniform-paths/redundant.rs stdout ----
normalized stderr:
warning: unused return value of `stdout` that must be used
   |
LL |     io::stdout();
   |     ^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(unused_must_use)]` on by default

warning: unused return value of `stdout` that must be used
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     self::std::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused return value of `stdout` that must be used
   |
   |
LL |     foo::my_std::io::stdout();


warning: unused return value of `stdout` that must be used
   |
LL |     bar::std::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/redundant.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/redundant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/redundant/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/redundant/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused return value of `stdout` that must be used
   |
LL |     io::stdout();
   |     ^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(unused_must_use)]` on by default

warning: unused return value of `stdout` that must be used
   |
LL |     self::std::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^


warning: unused return value of `stdout` that must be used
   |
   |
LL |     foo::my_std::io::stdout();


warning: unused return value of `stdout` that must be used
   |
LL |     bar::std::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^^^^^^

---


---- [ui] ui/uniform-paths/basic.rs stdout ----
normalized stderr:
warning: unused return value of `stdout` that must be used
   |
LL |     std_io::stdout();
   |     ^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(unused_must_use)]` on by default

warning: unused return value of `stdout` that must be used
   |
LL |         stdout();
   |         ^^^^^^^^^

---
To only update this specific test, also pass `--test-args uniform-paths/basic.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uniform-paths/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/basic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/basic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused return value of `stdout` that must be used
   |
LL |     std_io::stdout();
   |     ^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(unused_must_use)]` on by default

warning: unused return value of `stdout` that must be used
   |
LL |         stdout();
   |         ^^^^^^^^^

---


---- [ui] ui/uniform-paths/macros-nested.rs stdout ----
normalized stderr:
warning: unused return value of `stdout` that must be used
   |
   |
LL |     foo::std_io::stdout();
   |
   = note: `#[warn(unused_must_use)]` on by default


warning: unused return value of `stdout` that must be used
   |
LL |     io::stdout();
   |     ^^^^^^^^^^^^^


warning: unused return value of `stdout` that must be used
   |
LL |     bar::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args uniform-paths/macros-nested.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uniform-paths/macros-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/macros-nested/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/macros-nested/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused return value of `stdout` that must be used
   |
   |
LL |     foo::std_io::stdout();
   |
   = note: `#[warn(unused_must_use)]` on by default


warning: unused return value of `stdout` that must be used
   |
LL |     io::stdout();
   |     ^^^^^^^^^^^^^


warning: unused return value of `stdout` that must be used
   |
LL |     bar::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^

---


---- [ui] ui/uniform-paths/basic-nested.rs stdout ----
normalized stderr:
warning: unused return value of `stdout` that must be used
   |
   |
LL |     foo::std_io::stdout();
   |
   = note: `#[warn(unused_must_use)]` on by default


warning: unused return value of `stdout` that must be used
   |
LL |     io::stdout();
   |     ^^^^^^^^^^^^^


warning: unused return value of `stdout` that must be used
   |
LL |     bar::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^


warning: unused return value of `stdout` that must be used
   |
LL |         stdout();
   |         ^^^^^^^^^

---
To only update this specific test, also pass `--test-args uniform-paths/basic-nested.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uniform-paths/basic-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/basic-nested/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/basic-nested/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused return value of `stdout` that must be used
   |
   |
LL |     foo::std_io::stdout();
   |
   = note: `#[warn(unused_must_use)]` on by default


warning: unused return value of `stdout` that must be used
   |
LL |     io::stdout();
   |     ^^^^^^^^^^^^^


warning: unused return value of `stdout` that must be used
   |
LL |     bar::io::stdout();
   |     ^^^^^^^^^^^^^^^^^^


warning: unused return value of `stdout` that must be used
   |
LL |         stdout();
   |         ^^^^^^^^^

---


---- [ui] ui/uniform-paths/macros.rs stdout ----
normalized stderr:
warning: unused return value of `stdout` that must be used
   |
LL |     std_io::stdout();
   |     ^^^^^^^^^^^^^^^^^
   |
---
To only update this specific test, also pass `--test-args uniform-paths/macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uniform-paths/macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/macros/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uniform-paths/macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused return value of `stdout` that must be used
   |
LL |     std_io::stdout();
   |     ^^^^^^^^^^^^^^^^^
   |
---
test result: FAILED. 12254 passed; 5 failed; 110 ignored; 0 measured; 0 filtered out; finished in 136.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:05
