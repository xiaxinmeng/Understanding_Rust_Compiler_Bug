plain
---- [run-make] tests/run-make/libtest-json stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/libtest-json" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json  --test f.rs
RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json/libtest-json-output-default.json || true
RUST_BACKTRACE=0 LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json/f -Z unstable-options --test-threads=1 --format=json --show-output > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json/libtest-json-output-stdout-success.json || true
cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json/libtest-json-output-default.json | "/usr/bin/python3" validate_json.py
cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json/libtest-json-output-stdout-success.json | "/usr/bin/python3" validate_json.py
# Normalize the actual output and compare to expected output file
cat /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/libtest-json/libtest-json/libtest-json-output-default.json | sed 's/"exec_time": [0-9.]*/"exec_time": $TIME/' | diff output-default.json -
3c3
< { "type": "test", "name": "a", "event": "ok" }
---
> { "type": "test", "name": "a", "event": "ok"}
5c5
< { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'b' panicked at 'assertion failed: false', f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
---
> { "type": "test", "name": "b", "event": "failed", "stdout": "thread 'b' panicked at 'assertion failed: false', f.rs:9:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"}
7c7
< { "type": "test", "name": "c", "event": "ok" }
---
> { "type": "test", "name": "c", "event": "ok"}
9,10c9,10
< { "type": "test", "name": "d", "event": "ignored", "message": "msg" }
< { "type": "suite", "event": "failed", "passed": 2, "failed": 1, "ignored": 1, "measured": 0, "filtered_out": 0, "exec_time": $TIME }
---
> { "type": "test", "name": "d", "event": "ignored", "message": "msg"}
> { "type": "suite", "event": "failed", "passed": 2, "failed": 1, "ignored": 1, "measured": 0, "filtered_out": 0, "exec_time": $TIME}
--- stderr -------------------------------
make: *** [Makefile:16: all] Error 1
------------------------------------------

