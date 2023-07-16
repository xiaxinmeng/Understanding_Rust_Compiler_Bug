plain
[00:02:59]       Memory: 8 GB
[00:02:59]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:59]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:59]       SMC Version (system): 2.8f0
[00:02:59]       Serial Number (system): VMbom5jaI7LG
[00:02:59] 
[00:02:59] hw.ncpu: 4
[00:02:59] hw.byteorder: 1234
[00:02:59] hw.memsize: 8589934592
---
[02:03:20] status: exit code: 2
[02:03:20] command: "make"
[02:03:20] stdout:
[02:03:20] ------------------------------------------
[02:03:20] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-address/sanitizer-address:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-address/sanitizer-address -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-address/sanitizer-address  -g -Z sanitizer=address -Z print-link-args -C rpath overflow.rs | "/Users/travis/build/rust-lang/rust/src/etc/cat-and-grep.sh" librustc_asan
[02:03:20] [[[ begin stdout ]]]
[02:03:20] 
[02:03:20] [[[ end stdout ]]]
[02:03:20] Error: cannot match: librustc_asan
[02:03:20] ------------------------------------------
[02:03:20] stderr:
[02:03:20] ------------------------------------------
[02:03:20] ------------------------------------------
[02:03:20] error: AddressSanitizer only works with the `x86_64-unknown-linux-gnu` or `x86_64-apple-darwin` target
[02:03:20] error: aborting due to previous error
[02:03:20] 
[02:03:20] 
[02:03:20] make[1]: *** [all] Error 1
[02:03:20] ------------------------------------------
[02:03:20] 
[02:03:20] 
[02:03:20] ---- [run-make] run-make-fulldeps/sanitizer-invalid-cratetype stdout ----
[02:03:20] ---- [run-make] run-make-fulldeps/sanitizer-invalid-cratetype stdout ----
[02:03:20] 
[02:03:20] error: make failed
[02:03:20] status: exit code: 2
[02:03:20] command: "make"
[02:03:20] stdout:
[02:03:20] ------------------------------------------
[02:03:20] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-invalid-cratetype/sanitizer-invalid-cratetype:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-invalid-cratetype/sanitizer-invalid-cratetype -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/sanitizer-invalid-cratetype/sanitizer-invalid-cratetype  -Z sanitizer=address --crate-type proc-macro --target x86_64-apple-darwin hello.rs 2>&1 | "/Users/travis/build/rust-lang/rust/src/etc/cat-and-grep.sh" '-Z sanitizer'
[02:03:20] [[[ begin stdout ]]]
[02:03:20] error: AddressSanitizer only works with the `x86_64-unknown-linux-gnu` or `x86_64-apple-darwin` target
[02:03:20] error: aborting due to previous error
[02:03:20] 
[02:03:20] 
[02:03:20] 
[02:03:20] [[[ end stdout ]]]
[02:03:20] Error: cannot match: -Z sanitizer
[02:03:20] ------------------------------------------
[02:03:20] stderr:
[02:03:20] ------------------------------------------
[02:03:20] ------------------------------------------
[02:03:20] make[1]: *** [all] Error 1
[02:03:20] ------------------------------------------
[02:03:20] 
[02:03:20] 
[02:03:20] 
---
[02:03:20] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:519:22
[02:03:20] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[02:03:20] 
[02:03:20] 
[02:03:20] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-make-fulldeps" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "run-make" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.36.0-dev\n" "--cc" "/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang" "--cxx" "/Users/travis/build/rust-lang/rust/clang+llvm-7.0.0-x86_64-apple-darwin/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430asmprinter msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option optremarks orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata riscv riscvasmparser riscvasmprinter riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xray" "--llvm-cxxflags" "-I/Users/travis/build/rust-lang/rust/src/llvm-project/llvm/include -I/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/include -std=c++11  -fno-exceptions -fno-rtti -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--llvm-bin-dir" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:03:20] 
[02:03:20] 
[02:03:20] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:03:20] Build completed unsuccessfully in 0:49:00
[02:03:20] Build completed unsuccessfully in 0:49:00
[02:03:20] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13080b5d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat May  4 18:06:48 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:181cf420
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1240
-rw-------@  1 travis  staff   1428 May  4 18:06 foo_2019-05-04-180626_Traviss-Mac-1044.crash
drwx------  26 travis  staff    884 May  4 18:06 .
-rw-------@  1 travis  staff   1417 May  4 18:05 m4_2019-05-04-180554_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 May  4 18:05 bar_2019-05-04-180545_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 May  4 18:05 b_2019-05-04-180544_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 May  4 18:05 bar_2019-05-04-180544_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 May  4 17:33 a_2019-05-04-173340-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 May  4 17:33 a_2019-05-04-173340_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 May  4 17:33 a_2019-05-04-173331-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 May  4 17:33 a_2019-05-04-173331_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 May  4 17:33 a_2019-05-04-173326_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 May  4 17:33 a_2019-05-04-173320_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 May  4 17:33 a_2019-05-04-173313_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9792 May  4 17:33 a_2019-05-04-173312_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 May  4 17:32 a_2019-05-04-173236_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63058 May  4 17:32 a_2019-05-04-173225_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 May  4 17:32 a_2019-05-04-173221-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 May  4 17:32 a_2019-05-04-173221-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 May  4 17:32 a_2019-05-04-173221_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11715 May  4 17:30 a_2019-05-04-173012_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 May  4 17:29 a_2019-05-04-172920_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 May  4 17:28 a_2019-05-04-172809_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 May  4 17:27 a_2019-05-04-172718-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 May  4 17:27 a_2019-05-04-172717-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 May  4 17:27 a_2019-05-04-172717_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:014388ee
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-04-172717-1_Traviss-Mac-1044.crash
Process:               a [45142]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45130]
Responsible:           a [45142]
User ID:               501
Date/Time:             2019-05-04 17:27:11.647 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-eab9b87ea2f8ecdb.dylib  0x0000000108993ade std::panicking::rust_panic_with_hook::h7727ab2ea491e89c + 142
1   a                              0x000000010895f8c5 std::panicking::begin_panic::hb96124c01d149146 + 37
2   a                              0x000000010895d3ec _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x000000010895ce39 core::ptr::real_drop_in_place::ha38bdcda67050f55 + 9
4   a                              0x000000010895d3c3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010895e539 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x000000010895c8f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h500aa6ffbc5e418e + 6 (rt.rs:64)
7   libstd-eab9b87ea2f8ecdb.dylib  0x0000000108993558 std::panicking::try::do_call::h9fa7e6361b2e4d90 + 24
8   libstd-eab9b87ea2f8ecdb.dylib  0x00000001089a360f __rust_maybe_catch_panic + 31
9   libstd-eab9b87ea2f8ecdb.dylib  0x000000010899403e std::rt::lang_start_internal::h954b4be21ff606b2 + 542
10  a                              0x000000010895ed79 main + 41
11  libdyld.dylib                  0x00007fff6bdfa115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee72a42f8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001089dbb62  rbp: 0x00007ffee72a43f0  rsp: 0x00007ffee72a4320
   r8: 0xffffffff00000100   r9: 0x0000000108a0e9d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001089da8c8  r14: 0x0000000108961460  r15: 0x00007ffee72a4400
  rip: 0x0000000108993ade  rfl: 0x0000000000010202  cr2: 0x0000000108c659b4
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x108959000 -        0x108960ff7 +a (0) <974C7262-E663-3C83-A0FC-537C36B6FB90> /Users/USER/*/a
       0x10896f000 -        0x108a06fff +libstd-eab9b87ea2f8ecdb.dylib (0) <CF57B36F-7BEA-3E2A-9DCE-BF16C917A998> /Users/USER/*/libstd-eab9b87ea2f8ecdb.dylib
       0x114cda000 -        0x114d2498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff69664000 -     0x7fff69697fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff69b76000 -     0x7fff69b77ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff69e2c000 -     0x7fff69e82fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff69e83000 -     0x7fff69ea7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6b1f9000 -     0x7fff6b5ea3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6b6b7000 -     0x7fff6b6d3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6bc91000 -     0x7fff6bc95ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6bc96000 -     0x7fff6bca0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6bca1000 -     0x7fff6bca8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6bca9000 -     0x7fff6bcb1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6bcb2000 -     0x7fff6bd37fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6bdbf000 -     0x7fff6bdf8ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6bdf9000 -     0x7fff6be16ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6be17000 -     0x7fff6be17ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6be25000 -     0x7fff6be25ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6be26000 -     0x7fff6be2affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6be2b000 -     0x7fff6be2dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6be2e000 -     0x7fff6be2fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6be30000 -     0x7fff6be47fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6be48000 -     0x7fff6be48fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6be49000 -     0x7fff6bed2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6bed3000 -     0x7fff6bed6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6bed7000 -     0x7fff6bedaffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6bedb000 -     0x7fff6bedcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6bedd000 -     0x7fff6bee3ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6bee4000 -     0x7fff6bf2dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6bf2e000 -     0x7fff6bf53ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6bf54000 -     0x7fff6bf9ffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6bfa0000 -     0x7fff6bfbffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6bfc0000 -     0x7fff6c064ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6c065000 -     0x7fff6c06fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6c070000 -     0x7fff6c079ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6c07a000 -     0x7fff6c081ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6c082000 -     0x7fff6c08dfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6c08e000 -     0x7fff6c091ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6c092000 -     0x7fff6c093ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6c094000 -     0x7fff6c09bff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6c09c000 -     0x7fff6c0afff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6c0b1000 -     0x7fff6c0b6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6c0b7000 -     0x7fff6c0e3ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2376
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=82.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.5M        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4544K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            285.5M      109 
TOTAL                            285.5M      109 
TOTAL, minus reserved VM space   285.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-04-172717_Traviss-Mac-1044.crash
Process:               a [45143]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45130]
Responsible:           a [45143]
User ID:               501
Date/Time:             2019-05-04 17:27:11.659 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-eab9b87ea2f8ecdb.dylib  0x0000000103585ade std::panicking::rust_panic_with_hook::h7727ab2ea491e89c + 142
1   a                              0x00000001035548c5 std::panicking::begin_panic::hb96124c01d149146 + 37
2   a                              0x00000001035523ec _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000103551e39 core::ptr::real_drop_in_place::ha38bdcda67050f55 + 9
4   a                              0x00000001035523c3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000103553539 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x00000001035518f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h500aa6ffbc5e418e + 6 (rt.rs:64)
7   libstd-eab9b87ea2f8ecdb.dylib  0x0000000103585558 std::panicking::try::do_call::h9fa7e6361b2e4d90 + 24
8   libstd-eab9b87ea2f8ecdb.dylib  0x000000010359560f __rust_maybe_catch_panic + 31
9   libstd-eab9b87ea2f8ecdb.dylib  0x000000010358603e std::rt::lang_start_internal::h954b4be21ff606b2 + 542
10  a                              0x0000000103553d79 main + 41
11  libdyld.dylib                  0x00007fff6bdfa115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeec6af2d8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001035cdb62  rbp: 0x00007ffeec6af3d0  rsp: 0x00007ffeec6af300
   r8: 0xffffffff00000100   r9: 0x00000001036009d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001035cc8c8  r14: 0x0000000103556460  r15: 0x00007ffeec6af3e0
  rip: 0x0000000103585ade  rfl: 0x0000000000010206  cr2: 0x00000001012e2000
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10354e000 -        0x103555ff7 +a (0) <974C7262-E663-3C83-A0FC-537C36B6FB90> /Users/USER/*/a
       0x103561000 -        0x1035f8fff +libstd-eab9b87ea2f8ecdb.dylib (0) <CF57B36F-7BEA-3E2A-9DCE-BF16C917A998> /Users/USER/*/libstd-eab9b87ea2f8ecdb.dylib
       0x110b00000 -        0x110b4a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff69664000 -     0x7fff69697fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff69b76000 -     0x7fff69b77ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff69e2c000 -     0x7fff69e82fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff69e83000 -     0x7fff69ea7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6b1f9000 -     0x7fff6b5ea3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6b6b7000 -     0x7fff6b6d3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6bc91000 -     0x7fff6bc95ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6bc96000 -     0x7fff6bca0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6bca1000 -     0x7fff6bca8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6bca9000 -     0x7fff6bcb1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6bcb2000 -     0x7fff6bd37fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6bdbf000 -     0x7fff6bdf8ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6bdf9000 -     0x7fff6be16ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6be17000 -     0x7fff6be17ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6be25000 -     0x7fff6be25ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6be26000 -     0x7fff6be2affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6be2b000 -     0x7fff6be2dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6be2e000 -     0x7fff6be2fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6be30000 -     0x7fff6be47fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6be48000 -     0x7fff6be48fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6be49000 -     0x7fff6bed2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6bed3000 -     0x7fff6bed6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6bed7000 -     0x7fff6bedaffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6bedb000 -     0x7fff6bedcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6bedd000 -     0x7fff6bee3ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6bee4000 -     0x7fff6bf2dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6bf2e000 -     0x7fff6bf53ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6bf54000 -     0x7fff6bf9ffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6bfa0000 -     0x7fff6bfbffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6bfc0000 -     0x7fff6c064ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6c065000 -     0x7fff6c06fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6c070000 -     0x7fff6c079ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6c07a000 -     0x7fff6c081ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6c082000 -     0x7fff6c08dfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6c08e000 -     0x7fff6c091ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6c092000 -     0x7fff6c093ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6c094000 -     0x7fff6c09bff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6c09c000 -     0x7fff6c0afff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6c0b1000 -     0x7fff6c0b6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6c0b7000 -     0x7fff6c0e3ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2376
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9696K        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4544K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            276.5M      110 
TOTAL                            276.5M      110 
TOTAL, minus reserved VM space   276.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-04-172718-1_Traviss-Mac-1044.crash
Process:               a [44322]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44317]
Responsible:           a [44322]
User ID:               501
Date/Time:             2019-05-04 17:26:42.355 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010e6628ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010e661ca9 std::panicking::try::do_call::h45c5cbbab5849d8f (.llvm.16517881753153639607) + 9
2   libstd-eab9b87ea2f8ecdb.dylib  0x000000010e69e60f __rust_maybe_catch_panic + 31
3   a                              0x000000010e662b01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010e6610f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc68263f5212c4ee9 + 6
5   libstd-eab9b87ea2f8ecdb.dylib  0x000000010e68e558 std::panicking::try::do_call::h9fa7e6361b2e4d90 + 24
6   libstd-eab9b87ea2f8ecdb.dylib  0x000000010e69e60f __rust_maybe_catch_panic + 31
7   libstd-eab9b87ea2f8ecdb.dylib  0x000000010e68f03e std::rt::lang_start_internal::h954b4be21ff606b2 + 542
8   a                              0x000000010e662e09 main + 41
9   libdyld.dylib                  0x00007fff6bdfa115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fa945c02bd0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee159cb38  rsi: 0x000000003fffffff  rbp: 0x00007ffee159d590  rsp: 0x00007ffee159d590
   r8: 0x00000000945c02c2   r9: 0x0000000000000004  r10: 0x0000000115d9f8c0  r11: 0x00007fff6c0b196c
  r12: 0x000000010e9a2000  r13: 0x0000000000000000  r14: 0x00007ffee159d6b0  r15: 0x00007ffee159d5f8
  rip: 0x000000010e6628ae  rfl: 0x0000000000010202  cr2: 0x000000010e6d56b8
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10e660000 -        0x10e663ff7 +a (0) <CCE4CAAB-BB94-358E-945E-0225E49E5467> /Users/USER/*/a
       0x10e66a000 -        0x10e701fff +libstd-eab9b87ea2f8ecdb.dylib (0) <CF57B36F-7BEA-3E2A-9DCE-BF16C917A998> /Users/USER/*/libstd-eab9b87ea2f8ecdb.dylib
       0x115d4d000 -        0x115d9798f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff69664000 -     0x7fff69697fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff69b76000 -     0x7fff69b77ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff69e2c000 -     0x7fff69e82fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff69e83000 -     0x7fff69ea7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6b1f9000 -     0x7fff6b5ea3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6b6b7000 -     0x7fff6b6d3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6bc91000 -     0x7fff6bc95ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6bc96000 -     0x7fff6bca0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6bca1000 -     0x7fff6bca8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6bca9000 -     0x7fff6bcb1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6bcb2000 -     0x7fff6bd37fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6bdbf000 -     0x7fff6bdf8ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6bdf9000 -     0x7fff6be16ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6be17000 -     0x7fff6be17ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6be25000 -     0x7fff6be25ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6be26000 -     0x7fff6be2affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6be2b000 -     0x7fff6be2dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6be2e000 -     0x7fff6be2fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6be30000 -     0x7fff6be47fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6be48000 -     0x7fff6be48fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6be49000 -     0x7fff6bed2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6bed3000 -     0x7fff6bed6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6bed7000 -     0x7fff6bedaffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6bedb000 -     0x7fff6bedcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6bedd000 -     0x7fff6bee3ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6bee4000 -     0x7fff6bf2dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6bf2e000 -     0x7fff6bf53ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6bf54000 -     0x7fff6bf9ffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6bfa0000 -     0x7fff6bfbffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6bfc0000 -     0x7fff6c064ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6c065000 -     0x7fff6c06fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6c070000 -     0x7fff6c079ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6c07a000 -     0x7fff6c081ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6c082000 -     0x7fff6c08dfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6c08e000 -     0x7fff6c091ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6c092000 -     0x7fff6c093ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6c094000 -     0x7fff6c09bff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6c09c000 -     0x7fff6c0afff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6c0b1000 -     0x7fff6c0b6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6c0b7000 -     0x7fff6c0e3ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2376
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
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4544K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.0M      109 
TOTAL                            276.0M      109 
TOTAL, minus reserved VM space   275.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-05-04-172809_Traviss-Mac-1044.crash
Process:               a [46831]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46829]
Responsible:           a [46831]
User ID:               501
Date/Time:             2019-05-04 17:28:08.161 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff6bf49e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff6c088150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff6bea6312 abort + 127
3   libstd-eab9b87ea2f8ecdb.dylib  0x00000001035b9af9 std::sys::unix::abort_internal::hadb7421ad5d397a3 + 9
4   libstd-eab9b87ea2f8ecdb.dylib  0x00000001035a9b70 rust_oom + 32
5   libstd-eab9b87ea2f8ecdb.dylib  0x00000001035cae09 alloc::alloc::handle_alloc_error::h2824bdf05f4ed422 + 9
6   a                              0x000000010357e39f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x000000010357dba6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbe0a0621cbf1919f + 6
8   libstd-eab9b87ea2f8ecdb.dylib  0x00000001035aa558 std::panicking::try::do_call::h9fa7e6361b2e4d90 + 24
9   libstd-eab9b87ea2f8ecdb.dylib  0x00000001035ba60f __rust_maybe_catch_panic + 31
10  libstd-eab9b87ea2f8ecdb.dylib  0x00000001035ab03e std::rt::lang_start_internal::h954b4be21ff606b2 + 542
11  a                              0x000000010357e4f9 main + 41
12  libdyld.dylib                  0x00007fff6bdfa115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffa4b20340  rcx: 0x00007ffeec681508  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeec681540  rsp: 0x00007ffeec681508
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff6bf49e3e  rfl: 0x0000000000000206  cr2: 0x00007fffa4afe148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10357c000 -        0x10357eff7 +a (0) <89415638-26CA-3B94-954B-EFC0E7176D1A> /Users/USER/*/a
       0x103586000 -        0x10361dfff +libstd-eab9b87ea2f8ecdb.dylib (0) <CF57B36F-7BEA-3E2A-9DCE-BF16C917A998> /Users/USER/*/libstd-eab9b87ea2f8ecdb.dylib
       0x1065ae000 -        0x1065f898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff69664000 -     0x7fff69697fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff69b76000 -     0x7fff69b77ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff69e2c000 -     0x7fff69e82fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff69e83000 -     0x7fff69ea7ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6b1f9000 -     0x7fff6b5ea3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6b6b7000 -     0x7fff6b6d3ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6bc91000 -     0x7fff6bc95ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6bc96000 -     0x7fff6bca0ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6bca1000 -     0x7fff6bca8fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6bca9000 -     0x7fff6bcb1ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6bcb2000 -     0x7fff6bd37fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6bdbf000 -     0x7fff6bdf8ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6bdf9000 -     0x7fff6be16ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6be17000 -     0x7fff6be17ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6be25000 -     0x7fff6be25ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6be26000 -     0x7fff6be2affb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6be2b000 -     0x7fff6be2dff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6be2e000 -     0x7fff6be2fff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6be30000 -     0x7fff6be47fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6be48000 -     0x7fff6be48fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6be49000 -     0x7fff6bed2ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6bed3000 -     0x7fff6bed6ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6bed7000 -     0x7fff6bedaffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6bedb000 -     0x7fff6bedcfff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6bedd000 -     0x7fff6bee3ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6bee4000 -     0x7fff6bf2dff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6bf2e000 -     0x7fff6bf53ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6bf54000 -     0x7fff6bf9ffcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6bfa0000 -     0x7fff6bfbffff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6bfc0000 -     0x7fff6c064ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6c065000 -     0x7fff6c06fffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6c070000 -     0x7fff6c079ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6c07a000 -     0x7fff6c081ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6c082000 -     0x7fff6c08dfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6c08e000 -     0x7fff6c091ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6c092000 -     0x7fff6c093ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6c094000 -     0x7fff6c09bff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6c09c000 -     0x7fff6c0afff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6c0b1000 -     0x7fff6c0b6ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6c0b7000 -     0x7fff6c0e3ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by all processes on this machine:
