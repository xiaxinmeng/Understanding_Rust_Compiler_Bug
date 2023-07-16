plain
.................................................................................................... 1600/11266
.................................................................................................... 1700/11266
......................................i..............................................F.............. 1800/11266
.................................................................................................... 1900/11266
...F......................F......................................................................... 2000/11266
.................................................................................................... 2200/11266
.................................................................................................... 2300/11266
.................................................................................................... 2400/11266
.................................................................................................... 2500/11266
---
failures:

---- [ui] ui/const-generics/issues/issue-62579-no-match.rs#full stdout ----

error in revision `full`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-62579-no-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match.full/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_symbol_mangling/src/v0.rs:538:17: symbol_names: unsupported constant of type `NoMatch` (Const { ty: NoMatch, val: Value(ByRef { alloc: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) })
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (208817d75 2021-01-17) running on x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `foo::<NoMatch>`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack


------------------------------------------



---- [ui] ui/const-generics/slice-const-param.rs#full stdout ----

error in revision `full`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/slice-const-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/slice-const-param.full/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/slice-const-param.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_symbol_mangling/src/v0.rs:538:17: symbol_names: unsupported constant of type `&str` (Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [226, 132, 135, 227, 135, 136, 226, 134, 166], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [511], len: Size { raw: 9 } }, size: Size { raw: 9 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 9 }) })
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (208817d75 2021-01-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `function_with_str::<"ℇ㇈↦">`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack


------------------------------------------



---- [ui] ui/const-generics/type-dependent/issue-71348.rs#full stdout ----

error in revision `full`: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/type-dependent/issue-71348.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/type-dependent/issue-71348.full/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/type-dependent/issue-71348.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_symbol_mangling/src/v0.rs:538:17: symbol_names: unsupported constant of type `&str` (Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 }) })
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (208817d75 2021-01-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [symbol_name] computing the symbol for `<Foo as Get<"int">>::get`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack


------------------------------------------



---- [ui] ui/panics/issue-47429-short-backtraces.rs stdout ----
diff of run.stderr:

1 thread 'main' panicked at 'explicit panic', $DIR/issue-47429-short-backtraces.rs:16:5
2 stack backtrace:
-    0: std::panicking::begin_panic
+    0: std::panicking::begin_panic::<&str>
4    1: issue_47429_short_backtraces::main
5 note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.



The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces/issue-47429-short-backtraces.run.stderr
error: 1 errors occurred comparing run output.
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces/a"
stdout:
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:16:5
stack backtrace:
   0: std::panicking::begin_panic::<&str>
   1: issue_47429_short_backtraces::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
------------------------------------------



---
test result: FAILED. 11175 passed; 4 failed; 87 ignored; 0 measured; 0 filtered out; finished in 135.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:25
