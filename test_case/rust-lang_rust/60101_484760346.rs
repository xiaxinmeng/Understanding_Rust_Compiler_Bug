plain
[00:03:01]       Memory: 8 GB
[00:03:01]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:01]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:01]       SMC Version (system): 2.8f0
[00:03:01]       Serial Number (system): VMfGnUf/h91Z
[00:03:01] 
[00:03:01] hw.ncpu: 4
[00:03:01] hw.byteorder: 1234
[00:03:01] hw.memsize: 8589934592
---
[02:14:40] status: exit code: 2
[02:14:40] command: "make"
[02:14:40] stdout:
[02:14:40] ------------------------------------------
[02:14:40] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-cdylib-link/sanitizer-cdylib-link:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-cdylib-link/sanitizer-cdylib-link -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-cdylib-link/sanitizer-cdylib-link  -g -Z sanitizer=address --crate-type cdylib --target x86_64-apple-darwin  library.rs
[02:14:40] ------------------------------------------
[02:14:40] stderr:
[02:14:40] ------------------------------------------
[02:14:40] ------------------------------------------
[02:14:40] error: Only executables and rlibs can be compiled with `-Z sanitizer`
[02:14:40] error: aborting due to previous error
[02:14:40] 
[02:14:40] 
[02:14:40] make[1]: *** [all] Error 1
[02:14:40] ------------------------------------------
[02:14:40] 
[02:14:40] 
[02:14:40] ---- [run-make] run-make-fulldeps/sanitizer-dylib-link stdout ----
[02:14:40] ---- [run-make] run-make-fulldeps/sanitizer-dylib-link stdout ----
[02:14:40] 
[02:14:40] error: make failed
[02:14:40] status: exit code: 2
[02:14:40] command: "make"
[02:14:40] stdout:
[02:14:40] ------------------------------------------
[02:14:40] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-dylib-link/sanitizer-dylib-link:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-dylib-link/sanitizer-dylib-link -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-dylib-link/sanitizer-dylib-link  -g -Z sanitizer=address --crate-type dylib --target x86_64-apple-darwin  library.rs
[02:14:40] ------------------------------------------
[02:14:40] stderr:
[02:14:40] ------------------------------------------
[02:14:40] ------------------------------------------
[02:14:40] error: Only executables and rlibs can be compiled with `-Z sanitizer`
[02:14:40] error: aborting due to previous error
[02:14:40] 
[02:14:40] 
[02:14:40] make[1]: *** [all] Error 1
[02:14:40] ------------------------------------------
[02:14:40] 
[02:14:40] 
[02:14:40] ---- [run-make] run-make-fulldeps/sanitizer-staticlib-link stdout ----
[02:14:40] ---- [run-make] run-make-fulldeps/sanitizer-staticlib-link stdout ----
[02:14:40] 
[02:14:40] error: make failed
[02:14:40] status: exit code: 2
[02:14:40] command: "make"
[02:14:40] stdout:
[02:14:40] ------------------------------------------
[02:14:40] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-staticlib-link/sanitizer-staticlib-link:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-staticlib-link/sanitizer-staticlib-link -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-staticlib-link/sanitizer-staticlib-link  -g -Z sanitizer=address --crate-type staticlib --target x86_64-apple-darwin library.rs
[02:14:40] ------------------------------------------
[02:14:40] stderr:
[02:14:40] ------------------------------------------
[02:14:40] ------------------------------------------
[02:14:40] error: Only executables and rlibs can be compiled with `-Z sanitizer`
[02:14:40] error: aborting due to previous error
[02:14:40] 
[02:14:40] 
[02:14:40] make[1]: *** [all] Error 1
[02:14:40] ------------------------------------------
[02:14:40] 
[02:14:40] 
[02:14:40] 
---
[02:14:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[02:14:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[02:14:40] 
[02:14:40] 
[02:14:40] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-make-fulldeps" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "run-make" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang" "--cxx" "/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata riscv riscvasmparser riscvasmprinter riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/Users/travis/build/rust-lang/rust/src/llvm-project/llvm/include -I/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/include -std=c++11  -fno-exceptions -fno-rtti -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:14:40] 
[02:14:40] 
[02:14:40] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:14:40] Build completed unsuccessfully in 0:55:29
[02:14:40] Build completed unsuccessfully in 0:55:29
[02:14:40] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:012fe204
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 19 03:57:41 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:123d276b
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Apr 19 03:57 .
-rw-------@  1 travis  staff  13742 Apr 19 03:57 overflow_2019-04-19-035740_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Apr 19 03:57 foo_2019-04-19-035716_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1417 Apr 19 03:56 m4_2019-04-19-035645_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Apr 19 03:56 bar_2019-04-19-035636_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Apr 19 03:56 b_2019-04-19-035636_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Apr 19 03:56 bar_2019-04-19-035636-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Apr 19 03:18 a_2019-04-19-031849-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 Apr 19 03:18 a_2019-04-19-031849_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Apr 19 03:18 a_2019-04-19-031840-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 Apr 19 03:18 a_2019-04-19-031840_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Apr 19 03:18 a_2019-04-19-031834_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Apr 19 03:18 a_2019-04-19-031829_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Apr 19 03:18 a_2019-04-19-031822_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Apr 19 03:18 a_2019-04-19-031820_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Apr 19 03:17 a_2019-04-19-031744_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63113 Apr 19 03:17 a_2019-04-19-031735_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 Apr 19 03:17 a_2019-04-19-031731-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Apr 19 03:17 a_2019-04-19-031731-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Apr 19 03:17 a_2019-04-19-031731_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Apr 19 03:15 a_2019-04-19-031523_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Apr 19 03:14 a_2019-04-19-031430_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Apr 19 03:13 a_2019-04-19-031324-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr 19 03:13 a_2019-04-19-031324-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr 19 03:13 a_2019-04-19-031324-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Apr 19 03:13 a_2019-04-19-031324_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1eff4c4d
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-19-031324-1_Traviss-Mac-1044.crash
Process:               a [44637]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44636]
Responsible:           a [44637]
User ID:               501
Date/Time:             2019-04-19 03:11:47.609 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010e7fc8ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010e7fbca9 std::panicking::try::do_call::h1648e185d38e160c (.llvm.16739154691400472136) + 9
2   libstd-6ca0c946281cbb44.dylib  0x000000010e83a67f __rust_maybe_catch_panic + 31
3   a                              0x000000010e7fcb01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010e7fb0f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc66c9a54bc245d28 + 6
5   libstd-6ca0c946281cbb44.dylib  0x000000010e82a8f8 std::panicking::try::do_call::hd6a765e9161f4b58 + 24
6   libstd-6ca0c946281cbb44.dylib  0x000000010e83a67f __rust_maybe_catch_panic + 31
7   libstd-6ca0c946281cbb44.dylib  0x000000010e82b3de std::rt::lang_start_internal::he3a3d0277bd24308 + 542
8   a                              0x000000010e7fce09 main + 41
9   libdyld.dylib                  0x00007fff79fc0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fce6dc02c00  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee1402938  rsi: 0x00000000ffffffe1  rbp: 0x00007ffee1403390  rsp: 0x00007ffee1403390
   r8: 0x00000000e6dc02c5   r9: 0x0000000000000004  r10: 0x00000001160328c0  r11: 0x00007fff7a27796c
  r12: 0x000000010eb54000  r13: 0x0000000000000000  r14: 0x00007ffee14034b0  r15: 0x00007ffee14033f8
  rip: 0x000000010e7fc8ae  rfl: 0x0000000000010202  cr2: 0x000000010e8716d8
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10e7fa000 -        0x10e7fdfff +a (0) <1C76FBA5-8818-3750-BCC5-865CD694509A> /Users/USER/*/a
       0x10e806000 -        0x10e89dfff +libstd-6ca0c946281cbb44.dylib (0) <286690BB-F3BE-34E2-950D-1AD25EF84620> /Users/USER/*/libstd-6ca0c946281cbb44.dylib
       0x115fe0000 -        0x11602a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7782a000 -     0x7fff7785dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff77d3c000 -     0x7fff77d3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff77ff2000 -     0x7fff78048fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78049000 -     0x7fff7806dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff793bf000 -     0x7fff797b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7987d000 -     0x7fff79899ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff79e57000 -     0x7fff79e5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff79e5c000 -     0x7fff79e66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff79e67000 -     0x7fff79e6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff79e6f000 -     0x7fff79e77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff79e78000 -     0x7fff79efdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79f85000 -     0x7fff79fbeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff79fbf000 -     0x7fff79fdcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff79fdd000 -     0x7fff79fddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff79feb000 -     0x7fff79febff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff79fec000 -     0x7fff79ff0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff79ff1000 -     0x7fff79ff3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff79ff4000 -     0x7fff79ff5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff79ff6000 -     0x7fff7a00dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7a00e000 -     0x7fff7a00efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7a00f000 -     0x7fff7a098ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7a099000 -     0x7fff7a09cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7a09d000 -     0x7fff7a0a0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7a0a1000 -     0x7fff7a0a2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7a0a3000 -     0x7fff7a0a9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7a0aa000 -     0x7fff7a0f3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7a0f4000 -     0x7fff7a119ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7a11a000 -     0x7fff7a165fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7a166000 -     0x7fff7a185fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7a186000 -     0x7fff7a22aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7a22b000 -     0x7fff7a235ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7a236000 -     0x7fff7a23fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7a240000 -     0x7fff7a247ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7a248000 -     0x7fff7a253fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7a254000 -     0x7fff7a257ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7a258000 -     0x7fff7a259ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7a25a000 -     0x7fff7a261ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7a262000 -     0x7fff7a275ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7a277000 -     0x7fff7a27cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7a27d000 -     0x7fff7a2a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by all processes on this machine:
  Calls made by all processes on this machine:
    task_for_pid: 2619
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9260K        8 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4640K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.1M      108 
TOTAL                            276.1M      108 
TOTAL, minus reserved VM space   276.0M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-19-031324-2_Traviss-Mac-1044.crash
Process:               a [45458]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45453]
Responsible:           a [45458]
User ID:               501
Date/Time:             2019-04-19 03:12:17.885 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-6ca0c946281cbb44.dylib  0x0000000103019e7e std::panicking::rust_panic_with_hook::h16d72fd6ab3ad5bf + 142
1   a                              0x0000000102fe88c5 std::panicking::begin_panic::h75a10df8f23f26de + 37
2   a                              0x0000000102fe63ec _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000102fe5d49 core::ptr::real_drop_in_place::h654ccdbe565444dc + 9
4   a                              0x0000000102fe63c3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000102fe7539 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x0000000102fe58f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha7a11fd58527110e + 6 (rt.rs:64)
7   libstd-6ca0c946281cbb44.dylib  0x00000001030198f8 std::panicking::try::do_call::hd6a765e9161f4b58 + 24
8   libstd-6ca0c946281cbb44.dylib  0x000000010302967f __rust_maybe_catch_panic + 31
9   libstd-6ca0c946281cbb44.dylib  0x000000010301a3de std::rt::lang_start_internal::he3a3d0277bd24308 + 542
10  a                              0x0000000102fe7d79 main + 41
11  libdyld.dylib                  0x00007fff79fc0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeecc1b0f8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000103061b82  rbp: 0x00007ffeecc1b1f0  rsp: 0x00007ffeecc1b120
   r8: 0x0000000000000100   r9: 0x00000001030949d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001030608e8  r14: 0x0000000102fea460  r15: 0x00007ffeecc1b200
  rip: 0x0000000103019e7e  rfl: 0x0000000000010202  cr2: 0x0000000103303944
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x102fe2000 -        0x102fe9ff7 +a (0) <A963215E-9477-3C3D-B8B0-8DDA564D64F8> /Users/USER/*/a
       0x102ff5000 -        0x10308cfff +libstd-6ca0c946281cbb44.dylib (0) <286690BB-F3BE-34E2-950D-1AD25EF84620> /Users/USER/*/libstd-6ca0c946281cbb44.dylib
       0x10960d000 -        0x10965798f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7782a000 -     0x7fff7785dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff77d3c000 -     0x7fff77d3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff77ff2000 -     0x7fff78048fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78049000 -     0x7fff7806dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff793bf000 -     0x7fff797b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7987d000 -     0x7fff79899ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff79e57000 -     0x7fff79e5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff79e5c000 -     0x7fff79e66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff79e67000 -     0x7fff79e6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff79e6f000 -     0x7fff79e77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff79e78000 -     0x7fff79efdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79f85000 -     0x7fff79fbeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff79fbf000 -     0x7fff79fdcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff79fdd000 -     0x7fff79fddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff79feb000 -     0x7fff79febff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff79fec000 -     0x7fff79ff0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff79ff1000 -     0x7fff79ff3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff79ff4000 -     0x7fff79ff5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff79ff6000 -     0x7fff7a00dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7a00e000 -     0x7fff7a00efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7a00f000 -     0x7fff7a098ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7a099000 -     0x7fff7a09cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7a09d000 -     0x7fff7a0a0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7a0a1000 -     0x7fff7a0a2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7a0a3000 -     0x7fff7a0a9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7a0aa000 -     0x7fff7a0f3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7a0f4000 -     0x7fff7a119ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7a11a000 -     0x7fff7a165fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7a166000 -     0x7fff7a185fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7a186000 -     0x7fff7a22aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7a22b000 -     0x7fff7a235ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7a236000 -     0x7fff7a23fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7a240000 -     0x7fff7a247ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7a248000 -     0x7fff7a253fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7a254000 -     0x7fff7a257ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7a258000 -     0x7fff7a259ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7a25a000 -     0x7fff7a261ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7a262000 -     0x7fff7a275ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7a277000 -     0x7fff7a27cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7a27d000 -     0x7fff7a2a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by all processes on this machine:
  Calls made by all processes on this machine:
    task_for_pid: 2619
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=74.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4640K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            277.6M      110 
TOTAL                            277.6M      110 
TOTAL, minus reserved VM space   277.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-19-031324-3_Traviss-Mac-1044.crash
Process:               a [45460]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45453]
Responsible:           a [45460]
User ID:               501
Date/Time:             2019-04-19 03:12:17.886 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-6ca0c946281cbb44.dylib  0x00000001024b1e7e std::panicking::rust_panic_with_hook::h16d72fd6ab3ad5bf + 142
1   a                              0x000000010247d8c5 std::panicking::begin_panic::h75a10df8f23f26de + 37
2   a                              0x000000010247b3ec _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x000000010247ad49 core::ptr::real_drop_in_place::h654ccdbe565444dc + 9
4   a                              0x000000010247b3c3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010247c539 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x000000010247a8f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha7a11fd58527110e + 6 (rt.rs:64)
7   libstd-6ca0c946281cbb44.dylib  0x00000001024b18f8 std::panicking::try::do_call::hd6a765e9161f4b58 + 24
8   libstd-6ca0c946281cbb44.dylib  0x00000001024c167f __rust_maybe_catch_panic + 31
9   libstd-6ca0c946281cbb44.dylib  0x00000001024b23de std::rt::lang_start_internal::he3a3d0277bd24308 + 542
10  a                              0x000000010247cd79 main + 41
11  libdyld.dylib                  0x00007fff79fc0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeed7860d8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001024f9b82  rbp: 0x00007ffeed7861d0  rsp: 0x00007ffeed786100
   r8: 0x0000000000000100   r9: 0x000000010252c9d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001024f88e8  r14: 0x000000010247f460  r15: 0x00007ffeed7861e0
  rip: 0x00000001024b1e7e  rfl: 0x0000000000010206  cr2: 0x00000001025f7000
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x102477000 -        0x10247eff7 +a (0) <A963215E-9477-3C3D-B8B0-8DDA564D64F8> /Users/USER/*/a
       0x10248d000 -        0x102524fff +libstd-6ca0c946281cbb44.dylib (0) <286690BB-F3BE-34E2-950D-1AD25EF84620> /Users/USER/*/libstd-6ca0c946281cbb44.dylib
       0x108727000 -        0x10877198f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7782a000 -     0x7fff7785dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff77d3c000 -     0x7fff77d3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff77ff2000 -     0x7fff78048fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78049000 -     0x7fff7806dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff793bf000 -     0x7fff797b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7987d000 -     0x7fff79899ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff79e57000 -     0x7fff79e5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff79e5c000 -     0x7fff79e66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff79e67000 -     0x7fff79e6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff79e6f000 -     0x7fff79e77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff79e78000 -     0x7fff79efdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79f85000 -     0x7fff79fbeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff79fbf000 -     0x7fff79fdcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff79fdd000 -     0x7fff79fddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff79feb000 -     0x7fff79febff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff79fec000 -     0x7fff79ff0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff79ff1000 -     0x7fff79ff3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff79ff4000 -     0x7fff79ff5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff79ff6000 -     0x7fff7a00dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7a00e000 -     0x7fff7a00efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7a00f000 -     0x7fff7a098ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7a099000 -     0x7fff7a09cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7a09d000 -     0x7fff7a0a0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7a0a1000 -     0x7fff7a0a2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7a0a3000 -     0x7fff7a0a9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7a0aa000 -     0x7fff7a0f3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7a0f4000 -     0x7fff7a119ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7a11a000 -     0x7fff7a165fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7a166000 -     0x7fff7a185fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7a186000 -     0x7fff7a22aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7a22b000 -     0x7fff7a235ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7a236000 -     0x7fff7a23fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7a240000 -     0x7fff7a247ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7a248000 -     0x7fff7a253fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7a254000 -     0x7fff7a257ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7a258000 -     0x7fff7a259ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7a25a000 -     0x7fff7a261ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7a262000 -     0x7fff7a275ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7a277000 -     0x7fff7a27cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7a27d000 -     0x7fff7a2a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by all processes on this machine:
  Calls made by all processes on this machine:
    task_for_pid: 2619
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=91.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=91.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            27.5M        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4640K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            294.6M      109 
TOTAL                            294.6M      109 
TOTAL, minus reserved VM space   294.4M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-19-031324_Traviss-Mac-1044.crash
Process:               a [47139]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [47138]
Responsible:           a [47139]
User ID:               501
Date/Time:             2019-04-19 03:13:15.038 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5500 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff7a10fe3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff7a24e150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff7a06c312 abort + 127
3   libstd-6ca0c946281cbb44.dylib  0x0000000102991b69 std::sys::unix::abort_internal::hd7f17a900090ea96 + 9
4   libstd-6ca0c946281cbb44.dylib  0x0000000102981f10 rust_oom + 32
5   libstd-6ca0c946281cbb44.dylib  0x00000001029a2e29 alloc::alloc::handle_alloc_error::h1d1b1e63977f437a + 9
6   a                              0x000000010295609f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x00000001029556d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h2b7f33c85905198a + 6
8   libstd-6ca0c946281cbb44.dylib  0x00000001029828f8 std::panicking::try::do_call::hd6a765e9161f4b58 + 24
9   libstd-6ca0c946281cbb44.dylib  0x000000010299267f __rust_maybe_catch_panic + 31
10  libstd-6ca0c946281cbb44.dylib  0x00000001029833de std::rt::lang_start_internal::he3a3d0277bd24308 + 542
11  a                              0x00000001029561f9 main + 41
12  libdyld.dylib                  0x00007fff79fc0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffb2ce6340  rcx: 0x00007ffeed2a92f8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeed2a9330  rsp: 0x00007ffeed2a92f8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff7a10fe3e  rfl: 0x0000000000000206  cr2: 0x00007fffb2cc4148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x102954000 -        0x102956ff7 +a (0) <CFA0C263-39F2-3931-A677-CC8BA611EFBA> /Users/USER/*/a
       0x10295e000 -        0x1029f5fff +libstd-6ca0c946281cbb44.dylib (0) <286690BB-F3BE-34E2-950D-1AD25EF84620> /Users/USER/*/libstd-6ca0c946281cbb44.dylib
       0x10fdfb000 -        0x10fe4598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7782a000 -     0x7fff7785dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff77d3c000 -     0x7fff77d3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff77ff2000 -     0x7fff78048fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff78049000 -     0x7fff7806dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff793bf000 -     0x7fff797b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff7987d000 -     0x7fff79899ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff79e57000 -     0x7fff79e5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff79e5c000 -     0x7fff79e66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff79e67000 -     0x7fff79e6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff79e6f000 -     0x7fff79e77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff79e78000 -     0x7fff79efdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79f85000 -     0x7fff79fbeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff79fbf000 -     0x7fff79fdcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff79fdd000 -     0x7fff79fddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff79feb000 -     0x7fff79febff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff79fec000 -     0x7fff79ff0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff79ff1000 -     0x7fff79ff3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff79ff4000 -     0x7fff79ff5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff79ff6000 -     0x7fff7a00dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff7a00e000 -     0x7fff7a00efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff7a00f000 -     0x7fff7a098ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7a099000 -     0x7fff7a09cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff7a09d000 -     0x7fff7a0a0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff7a0a1000 -     0x7fff7a0a2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff7a0a3000 -     0x7fff7a0a9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7a0aa000 -     0x7fff7a0f3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff7a0f4000 -     0x7fff7a119ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7a11a000 -     0x7fff7a165fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff7a166000 -     0x7fff7a185fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff7a186000 -     0x7fff7a22aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7a22b000 -     0x7fff7a235ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff7a236000 -     0x7fff7a23fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff7a240000 -     0x7fff7a247ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7a248000 -     0x7fff7a253fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff7a254000 -     0x7fff7a257ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7a258000 -     0x7fff7a259ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7a25a000 -     0x7fff7a261ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff7a262000 -     0x7fff7a275ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7a277000 -     0x7fff7a27cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff7a27d000 -     0x7fff7a2a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
