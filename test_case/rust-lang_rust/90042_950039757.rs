plain
Successfully built 79823caa861d
Successfully tagged rust-ci:latest
Built container sha256:79823caa861d70fcbd0c2c2cb3e43aaa8a3b17ccf8446ecc24a920efd0ada0e7
Uploading finished image to https://ci-caches.rust-lang.org/docker/2b0ad7d278cb390aa31a8527da7dea1cd13001059913f8b600a26c048df13e13523fe6bd677cd78072a76b3390505d6be5a5fd4bacd37d78731d213d86e83d6a
upload failed: - to s3://rust-lang-ci-sccache2/docker/2b0ad7d278cb390aa31a8527da7dea1cd13001059913f8b600a26c048df13e13523fe6bd677cd78072a76b3390505d6be5a5fd4bacd37d78731d213d86e83d6a Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
[CI_JOB_NAME=x86_64-gnu-llvm-10]
/checkout/src/ci/run.sh: line 54: [: 1: unary operator expected
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 39 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
ii.iii.iii..iiiiiii.iiii.F.............

---- [run-make] run-make/fmt-write-bloat stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat  main.rs -O
nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/fmt-write-bloat/fmt-write-bloat/main | "/checkout/src/etc/cat-and-grep.sh" -v panic_bounds_check pad_integral Display Debug
[[[ begin stdout ]]]
0000000000201de0 d _DYNAMIC
0000000000201fa0 d _GLOBAL_OFFSET_TABLE_
                 w _ITM_deregisterTMCloneTable
                 w _ITM_registerTMCloneTable
0000000000000b80 T _ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17hbd2354d8169a20fdE
00000000000008d0 T _ZN4core3fmt5write17h27134d752146251dE
00000000000008b0 T _ZN4core3ops8function6FnOnce9call_once17h0cf227818ad1a4d7E.llvm.16535721751928471754
0000000000000b90 t _ZN4core3ptr152drop_in_place$LT$$RF$core..option..Option$LT$core..iter..adapters..flatten..Flatten$LT$core..option..IntoIter$LT$core..char..EscapeDebug$GT$$GT$$GT$$GT$17h64ecf004cee6c2b9E
0000000000000740 t _ZN4core3ptr44drop_in_place$LT$$RF$mut$u20$main..Dummy$GT$17h52940fd3dcd72bf7E
0000000000000ba0 T _ZN4core9panicking5panic17hcd29df677569ef29E
0000000000000bf0 T _ZN4core9panicking9panic_fmt17h291f2877311c86aaE
0000000000000750 t _ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h5a345a2b1158699eE
0000000000000760 t _ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h3e5ccc4ae4fe8e8eE
00000000000007b0 t _ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h58d4cdd3698bf368E
0000000000000f84 r __FRAME_END__
0000000000000cc8 r __GNU_EH_FRAME_HDR
0000000000202008 D __TMC_END__
0000000000202008 B __bss_start
                 w __cxa_finalize@@GLIBC_2.2.5
00000000000006f0 t __do_global_dtors_aux
0000000000201d48 t __do_global_dtors_aux_fini_array_entry
0000000000202000 D __dso_handle
0000000000201d40 t __frame_dummy_init_array_entry
                 w __gmon_start__
0000000000201d48 t __init_array_end
0000000000201d40 t __init_array_start
00000000000008a0 T __libc_csu_fini
0000000000000830 T __libc_csu_init
                 U __libc_start_main@@GLIBC_2.2.5
0000000000202008 D _edata
0000000000202010 B _end
0000000000000c24 T _fini
00000000000005f0 T _init
0000000000000630 T _start
0000000000202008 b completed.7698
0000000000000660 t deregister_tm_clones
0000000000000730 t frame_dummy
00000000000007d0 T main
00000000000006a0 t register_tm_clones
00000000000007c0 T rust_begin_unwind

[[[ end stdout ]]]
Error: should not match: Debug
Makefile:22: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
make: *** [all] Error 1
------------------------------------------




failures:
    [run-make] run-make/fmt-write-bloat

test result: FAILED. 19 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out; finished in 3.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:31:34
