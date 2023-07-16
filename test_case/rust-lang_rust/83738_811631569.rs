plain
.................................................................................................... 9400/11730
.................................................................................................... 9500/11730
.......................................................................i......i..................... 9600/11730
.................................................................................................... 9700/11730
.................iiiiiii..iiiiii.i.................................................................. 9800/11730
.................................................................................................... 10000/11730
.................................................................................................... 10100/11730
.................................................................................................... 10200/11730
.................................................................................................... 10300/11730
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.099 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.437 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Finished release [optimized] target(s) in 1.73s
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 222 tests
..........iiii...ii.................F.....................................................F......... 100/222
.............i..................iiiiii.......i..........F...F.iiiFF.........ii...................... 200/222
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [run-make] run-make-fulldeps/exit-code stdout ----
---- [run-make] run-make-fulldeps/exit-code stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code  success.rs; [ $? -eq 0 ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code  --invalid-arg-foo; [ $? -eq 1 ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code  compile-error.rs; [ $? -eq 1 ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code  -Ztreat-err-as-bug compile-error.rs; [ $? -eq 101 ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/exit-code/exit-code/exit-code success.rs; [ $? -eq 0 ]
Makefile:4: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: Unrecognized option: 'invalid-arg-foo'

error: kaboom
  |
  |
2 |     compile_error!("kaboom");

error: aborting due to previous error


error: kaboom
  |
  |
2 |     compile_error!("kaboom");


thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1026:27

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
---
note: compiler flags: -Z treat-err-as-bug

query stack during panic:
end of query stack
error: Option 'output' given more than once

make: *** [all] Error 1
------------------------------------------



---- [run-make] run-make-fulldeps/issue-38237 stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237  foo.rs; LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237  bar.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237 -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib baz.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237 -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-38237/issue-38237
Makefile:4: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: Option 'output' given more than once

make: *** [all] Error 1
------------------------------------------



---- [run-make] run-make-fulldeps/rustdoc-determinism stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-determinism/rustdoc-determinism:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-determinism/rustdoc-determinism -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib foo.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-determinism/rustdoc-determinism/foo_first
Makefile:10: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: Option 'output' given more than once

make: *** [all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/rustdoc-map-file stdout ----
---- [run-make] run-make-fulldeps/rustdoc-map-file stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-map-file/rustdoc-map-file:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-map-file/rustdoc-map-file -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -Z unstable-options --generate-redirect-map foo.rs -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-map-file/rustdoc-map-file/out"
Makefile:4: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: Option 'output' given more than once

make: *** [all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/rustdoc-output-path stdout ----
---- [run-make] run-make-fulldeps/rustdoc-output-path stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-output-path/rustdoc-output-path:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-output-path/rustdoc-output-path -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-output-path/rustdoc-output-path/foo/bar/doc" foo.rs
Makefile:4: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: Option 'output' given more than once

make: *** [all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/rustdoc-themes stdout ----
---- [run-make] run-make-fulldeps/rustdoc-themes stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
cp /checkout/src/librustdoc/html/static/themes/light.css /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes/test.css
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' --output /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes/rustdoc-themes" foo.rs --theme /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/rustdoc-themes/rustdoc-themes/test.css
Makefile:8: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: Option 'output' given more than once

make: *** [all] Error 1
------------------------------------------



---
test result: FAILED. 195 passed; 6 failed; 21 ignored; 0 measured; 0 filtered out; finished in 21.87s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:26
