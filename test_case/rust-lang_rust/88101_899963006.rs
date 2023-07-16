plain
............................................................i.i..................................... 12100/12172
........................................................................
failures:

---- [ui] ui/dylibs/issue-82151-serverctl-prefdyn.rs stdout ----


1 error: linking with `cc` failed: exit status: 1
2    |
-    = note: "cc" "-m64" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/issue-82151-serverctl-prefdyn.issue_82151_serverctl_prefdyn.72d5a704-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-lshared" "-L" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-lbar" "-Wl,--start-group" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-f2409c1bc0340a9e" "-Wl,--end-group" "-Wl,-Bstatic" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-bc417570076b2daa.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/issue-82151-serverctl-prefdyn" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/auxiliary" "-Wl,-rpath,$ORIGIN/../../../../stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
-    = note: /usr/bin/ld: $TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary/libshared.so: undefined reference to `bar::bar'
+    = note: "cc" "-m64" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/issue-82151-serverctl-prefdyn.issue_82151_serverctl_prefdyn.72d5a704-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-lshared" "-L" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-lbar" "-Wl,--start-group" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-979615b1d869d792" "-Wl,--end-group" "-Wl,-Bstatic" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-52fe0581016e3544.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "$BUILD_DIR/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "$TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/issue-82151-serverctl-prefdyn" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/auxiliary" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
+    = note: $TEST_BUILD_DIR/dylibs/issue-82151-serverctl-prefdyn/auxiliary/libshared.so: undefined reference to `bar::bar'
5            collect2: error: ld returned 1 exit status
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6            
7    = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/issue-82151-serverctl-prefdyn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dylibs/issue-82151-serverctl-prefdyn.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dylibs/issue-82151-serverctl-prefdyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "prefer-dynamic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `cc` failed: exit status: 1
   |
   = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/issue-82151-serverctl-prefdyn.issue_82151_serverctl_prefdyn.72d5a704-cgu.0.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-lshared" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/auxiliary" "-lbar" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-979615b1d869d792" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-52fe0581016e3544.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/issue-82151-serverctl-prefdyn" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-Wl,-rpath,$ORIGIN/auxiliary" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dylibs/issue-82151-serverctl-prefdyn/auxiliary/libshared.so: undefined reference to `bar::bar'
           collect2: error: ld returned 1 exit status
           
   = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
   = note: use the `-l` flag to specify native libraries to link
   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
error: aborting due to previous error


------------------------------------------
------------------------------------------



failures:
    [ui] ui/dylibs/issue-82151-serverctl-prefdyn.rs
test result: FAILED. 12069 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 125.03s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:22
