plain
.................................................................................................... 9200/11472
.................................................................................................... 9300/11472
.................................................................................................... 9400/11472
.............................i......i............................................................... 9500/11472
....................................................................iiiiiii..iiiiii.i............... 9600/11472
.................................................................................................... 9800/11472
.................................................................................................... 9900/11472
.................................................................................................... 10000/11472
.................................................................................................... 10100/11472
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.075 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.413 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- [rustdoc-json] rustdoc-json/traits/has_body.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/has_body/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/traits/has_body" "/checkout/src/test/rustdoc-json/traits/has_body.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/methods/header.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/methods/header/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/methods/header" "/checkout/src/test/rustdoc-json/methods/header.rs" "--edition=2018" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/structs/plain_empty.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/plain_empty/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/plain_empty" "/checkout/src/test/rustdoc-json/structs/plain_empty.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/fns/header.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/header/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fns/header" "/checkout/src/test/rustdoc-json/fns/header.rs" "--edition=2018" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/method_abi.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/method_abi/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/method_abi" "/checkout/src/test/rustdoc-json/method_abi.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/structs/with_primitives.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/with_primitives/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/with_primitives" "/checkout/src/test/rustdoc-json/structs/with_primitives.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/structs/with_generics.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/with_generics/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/with_generics" "/checkout/src/test/rustdoc-json/structs/with_generics.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/fn_pointer/header.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fn_pointer/header/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/fn_pointer/header" "/checkout/src/test/rustdoc-json/fn_pointer/header.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/structs/unit.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/unit/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/unit" "/checkout/src/test/rustdoc-json/structs/unit.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/unions/union.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/unions/union/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/unions/union" "/checkout/src/test/rustdoc-json/unions/union.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/structs/tuple.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/tuple/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/structs/tuple" "/checkout/src/test/rustdoc-json/structs/tuple.rs" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
---- [rustdoc-json] rustdoc-json/nested.rs stdout ----

error: rustdoc failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/nested/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json/nested" "/checkout/src/test/rustdoc-json/nested.rs" "--edition=2018" "--output-format" "json"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustdoc/json/mod.rs:248:47

error: internal compiler error: unexpected panic

error: Unrecognized option: 'output-format'
---
test result: FAILED. 0 passed; 12 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--jsondocck-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/jsondocck" "--src-base" "/checkout/src/test/rustdoc-json" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-json" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-json" "--mode" "rustdoc-json" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:59
