plain
.................................................................................................... 9400/11719
.................................................................................................... 9500/11719
.............................................................i......i............................... 9600/11719
.................................................................................................... 9700/11719
.......iiiiiii..iiiiii.i............................................................................ 9800/11719
.................................................................................................... 10000/11719
.................................................................................................... 10100/11719
.................................................................................................... 10200/11719
.................................................................................................... 10300/11719
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.113 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.440 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 429 tests
i.FF................F.........F.........i..............F............................................ 100/429
...F............................................................F......FF........................... 200/429
............................F......F..............F................................................. 300/429
.....F.......................................i...................................F.................. 400/429
........................F....
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [rustdoc] rustdoc/assoc-consts-version.rs stdout ----

error: rustdoc failed!
error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts-version" "/checkout/src/test/rustdoc/assoc-consts-version.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("SOME_CONST")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/assoc-consts.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/assoc-consts" "/checkout/src/test/rustdoc/assoc-consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("QUX0")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/const-doc.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-doc/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-doc" "/checkout/src/test/rustdoc/const-doc.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("Any")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/const-generics/const-generics-docs.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-generics-docs" "/checkout/src/test/rustdoc/const-generics/const-generics-docs.rs" "--edition=2018"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("FOO_ASSOC")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/doc-cfg-simplification.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification" "/checkout/src/test/rustdoc/doc-cfg-simplification.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("MAMELLE")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items/auxiliary/assoc-items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/auxiliary/assoc-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("ConstNoDefault")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/intra-link-prim-self.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-prim-self/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-link-prim-self" "/checkout/src/test/rustdoc/intra-link-prim-self.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("MAX")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/intra-doc/self.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/self/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/self" "/checkout/src/test/rustdoc/intra-doc/self.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("ASSO_CONST")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/intra-doc/trait-impl.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/trait-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/trait-impl" "/checkout/src/test/rustdoc/intra-doc/trait-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("ASSO_CONST")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-31808.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-31808/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-31808" "/checkout/src/test/rustdoc/issue-31808.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("BAR")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-33302.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-33302" "/checkout/src/test/rustdoc/issue-33302.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("C")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-36031.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-36031/auxiliary/issue-36031/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-36031" "/checkout/src/test/rustdoc/auxiliary/issue-36031.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("BAR")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/link-assoc-const.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/link-assoc-const/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/link-assoc-const" "/checkout/src/test/rustdoc/link-assoc-const.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("CONST")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/struct-implementations-title.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/struct-implementations-title/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/struct-implementations-title" "/checkout/src/test/rustdoc/struct-implementations-title.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("S")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/visibility.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/visibility/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/visibility" "/checkout/src/test/rustdoc/visibility.rs" "--document-private-items"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'can't make docs for trait item with name Some("CONST")', src/librustdoc/html/render/mod.rs:1607:18

error: internal compiler error: unexpected panic

error: Unrecognized option: 'document-private-items'
---
test result: FAILED. 411 passed; 15 failed; 3 ignored; 0 measured; 0 filtered out; finished in 29.28s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:15
