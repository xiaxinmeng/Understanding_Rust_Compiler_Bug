plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 486 tests
i................................F...................i.FFFF...F........................F............ 100/486
.................................................................................................... 200/486
......................FF.F.................................................FF.....F........F........ 300/486
...........F.............i............................F..............................F.F...i.....F.. 400/486
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----
---- [rustdoc] rustdoc/const-generics/const-impl.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/const-generics/const-impl" "/checkout/src/test/rustdoc/const-generics/const-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
27: @has check failed
 `XPATH PATTERN` did not match
 // @has foo/struct.VSet.html '//div[@id="impl-1"]/h3[@class="code-header in-band"]' 'impl<T> VSet<T, { Order::Unsorted }>'
Encountered 1 errors

------------------------------------------



---- [rustdoc] rustdoc/deref-const-fn.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-const-fn/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-const-fn" "--deny" "warnings" "/checkout/src/test/rustdoc/deref-const-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/deref-typedef.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-typedef" "--deny" "warnings" "/checkout/src/test/rustdoc/deref-typedef.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/deref-mut-methods.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-mut-methods/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-mut-methods" "--deny" "warnings" "/checkout/src/test/rustdoc/deref-mut-methods.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/deref-recursive-pathbuf.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive-pathbuf" "--deny" "warnings" "/checkout/src/test/rustdoc/deref-recursive-pathbuf.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/deref-recursive.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deref-recursive" "--deny" "warnings" "/checkout/src/test/rustdoc/deref-recursive.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/escape-deref-methods.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/escape-deref-methods" "--deny" "warnings" "/checkout/src/test/rustdoc/escape-deref-methods.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-19190-2.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-2" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-19190-2.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-19190.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-19190.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-19190-3.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-19190-3" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-19190-3.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-35169-2.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169-2" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-35169-2.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-35169.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-35169" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-35169.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-43701.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43701/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43701" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-43701.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/intra-doc/libstd-re-export.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/libstd-re-export.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/issue-56701.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56701/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-56701" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-56701.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/masked.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/masked/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/masked" "--deny" "warnings" "/checkout/src/test/rustdoc/masked.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/recursive-deref-sidebar.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursive-deref-sidebar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursive-deref-sidebar" "--deny" "warnings" "/checkout/src/test/rustdoc/recursive-deref-sidebar.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/recursive-deref.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursive-deref/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursive-deref" "--deny" "warnings" "/checkout/src/test/rustdoc/recursive-deref.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
---- [rustdoc] rustdoc/reexport-check.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/reexport-check/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/reexport-check" "--deny" "warnings" "/checkout/src/test/rustdoc/reexport-check.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Deref section without derived id', src/librustdoc/html/render/mod.rs:2155:45

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
test result: FAILED. 463 passed; 19 failed; 4 ignored; 0 measured; 0 filtered out; finished in 56.42s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:17:27
