plain
[00:05:06]       Memory: 8 GB
[00:05:06]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:05:06]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:05:06]       SMC Version (system): 2.8f0
[00:05:06]       Serial Number (system): VMV94ohs+4ff
[00:05:06] 
[00:05:06] hw.ncpu: 4
[00:05:06] hw.byteorder: 1234
[00:05:06] hw.memsize: 8589934592
---
[02:17:22] status: exit code: 2
[02:17:22] command: "make"
[02:17:22] stdout:
[02:17:22] ------------------------------------------
[02:17:22] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native -L /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native  foo.rs -C target-cpu=native 2>&1 | tee /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native/out.log
[02:17:22] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib:" /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native/foo
[02:17:22] # Make sure no warnings about "unknown CPU `native`" were emitted
[02:17:22] if [ "$(stat --printf="%s" /Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native/out.log)" = "0" ]; then \
[02:17:22]    echo no warnings generated; \
[02:17:22]  else \
[02:17:22]    exit 1; \
[02:17:22] 
[02:17:22] ------------------------------------------
[02:17:22] stderr:
[02:17:22] ------------------------------------------
[02:17:22] ------------------------------------------
[02:17:22] stat: illegal option -- -
[02:17:22] usage: stat [-FlLnqrsx] [-f format] [-t timefmt] [file ...]
[02:17:22] make[1]: *** [all] Error 1
[02:17:22] ------------------------------------------
[02:17:22] 
[02:17:22] 
[02:17:22] thread '[run-make] run-make-fulldeps/target-cpu-native' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[02:17:22] 
[02:17:22] 
[02:17:22] failures:
[02:17:22]     [run-make] run-make-fulldeps/target-cpu-native
[02:17:22]     [run-make] run-make-fulldeps/target-cpu-native
[02:17:22] 
[02:17:22] test result: FAILED. 188 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[02:17:22] 
[02:17:22] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[02:17:22] 
[02:17:22] 
[02:17:22] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-make-fulldeps" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-i686-apple-darwin" "--mode" "run-make" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "7.0.0\n" "--cc" "/Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang" "--cxx" "/Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata riscv riscvasmparser riscvasmprinter riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils" "--llvm-cxxflags" "-I/Users/travis/build/rust-lang/rust/src/llvm/include -I/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/include -ffunction-sections -fdata-sections -fPIC --target=i686-apple-darwin -stdlib=libc++ -fPIC -fvisibility-inlines-hidden -Werror=date-time -Werror=unguarded-availability-new -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -pedantic -Wno-long-long -Wcovered-switch-default -Wnon-virtual-dtor -Wdelete-non-virtual-dtor -Wstring-conversion -O3 -DNDEBUG  -fno-exceptions -fno-rtti -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[02:17:22] 
[02:17:22] 
[02:17:22] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:17:22] Build completed unsuccessfully in 1:02:43
[02:17:22] Build completed unsuccessfully in 1:02:43
[02:17:22] make: *** [check] Error 1
travis_time:end:1143a3f2:start=1535343111227303000,finish=1535351353525143000,duration=8242297840000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:095a2df8
---
travis_fold:start:after_failure.2
travis_time:start:38225dd0
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
drwx------  26 travis  staff    884 Aug 27 06:28 .
-rw-------@  1 travis  staff   1387 Aug 27 06:28 foo_2018-08-27-062852_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1377 Aug 27 06:28 m4_2018-08-27-062826_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1402 Aug 27 06:28 bar_2018-08-27-062816_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1390 Aug 27 06:28 bar_2018-08-27-062817_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9891 Aug 27 06:28 b_2018-08-27-062816_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34854 Aug 27 05:50 a_2018-08-27-055035_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57428 Aug 27 05:50 a_2018-08-27-055034_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55551 Aug 27 05:50 a_2018-08-27-055028-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34764 Aug 27 05:50 a_2018-08-27-055028_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9412 Aug 27 05:50 a_2018-08-27-055015_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9158 Aug 27 05:50 a_2018-08-27-055009_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9163 Aug 27 05:49 a_2018-08-27-054955_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9013 Aug 27 05:49 a_2018-08-27-054953_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9244 Aug 27 05:49 a_2018-08-27-054913_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58261 Aug 27 05:49 a_2018-08-27-054904_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59070 Aug 27 05:48 a_2018-08-27-054857-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60339 Aug 27 05:48 a_2018-08-27-054857-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59527 Aug 27 05:48 a_2018-08-27-054857_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10913 Aug 27 05:45 a_2018-08-27-054550_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9182 Aug 27 05:44 a_2018-08-27-054437_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9543 Aug 27 05:42 a_2018-08-27-054257_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9767 Aug 27 05:41 a_2018-08-27-054152-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9769 Aug 27 05:41 a_2018-08-27-054151_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9475 Aug 27 05:41 a_2018-08-27-054129_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:18b53b18
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054129_Traviss-Mac-1044.crash
Process:               a [34581]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [34580]
Responsible:           a [34581]
User ID:               501
Date/Time:             2018-08-27 05:41:15.794 +0000
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
0   a                              0x000b29ce abort_on_c_abi::panic_in_ffi::h305168285df224eb + 46
1   a                              0x000b1dfb std::panicking::try::do_call::h325fa2095685ded4 (.llvm.6782527419216690760) + 11
2   libstd-312c1b89c01a0dbc.dylib  0x00238b1d __rust_maybe_catch_panic + 29
3   a                              0x000b2c35 abort_on_c_abi::main::h82c0574e6cdd7b93 + 613
4   a                              0x000b127b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h55ad69be6c72a0c7 + 11
5   libstd-312c1b89c01a0dbc.dylib  0x00227d37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
6   libstd-312c1b89c01a0dbc.dylib  0x00238b1d __rust_maybe_catch_panic + 29
7   libstd-312c1b89c01a0dbc.dylib  0x00227cae std::panicking::try::h808ee7bbea2f45e8 + 62
8   libstd-312c1b89c01a0dbc.dylib  0x0020f5a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
9   a                              0x000b2f6c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00815060  ebx: 0xbff4f588  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00238b0e  esi: 0x00000000  ebp: 0xbff4f528  esp: 0xbff4f510
   ss: 0x00000023  efl: 0x00010292  eip: 0x000b29ce   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x002b04bc
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xb0000 -    0xb3fff +a (0) <762A1236-1A02-3912-BE39-7092BB48AA5C> /Users/USER/*/a
  0x16e000 -   0x1b3fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1f7000 -   0x2c7fd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3336
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3328K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9604K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            572.7M      138 
TOTAL                            572.7M      138 
TOTAL, minus reserved VM space   572.6M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054151_Traviss-Mac-1044.crash
Process:               a [35256]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [35247]
Responsible:           a [35256]
User ID:               501
Date/Time:             2018-08-27 05:41:51.172 +0000
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
0   libstd-312c1b89c01a0dbc.dylib  0x00200216 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 118
1   a                              0x000eeb5f std::panicking::begin_panic::h1dfd76ce48beac77 + 47 (panicking.rs:411)
2   a                              0x000ec5e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 36 (backtrace.rs:34)
3   a                              0x000ebfcb core::ptr::drop_in_place::h39ab9c7eb9f9aa98 + 11
4   a                              0x000ec5b3 backtrace::double::h3a79da7ae181846f + 51
5   a                              0x000ed8c2 backtrace::main::h9751c5dfa7559d82 + 4562 (backtrace.rs:113)
6   a                              0x000ebacb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha211c2951cb49c7d + 11 (rt.rs:74)
7   libstd-312c1b89c01a0dbc.dylib  0x001ffd37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
8   libstd-312c1b89c01a0dbc.dylib  0x00210b1d __rust_maybe_catch_panic + 29
9   libstd-312c1b89c01a0dbc.dylib  0x001ffcae std::panicking::try::h808ee7bbea2f45e8 + 62
10  libstd-312c1b89c01a0dbc.dylib  0x001e75a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
11  a                              0x000ee14c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x002001ae  ecx: 0x00000000  edx: 0xa7702ec6
  edi: 0x002a70b4  esi: 0x0026e9d0  ebp: 0xbff17388  esp: 0xbff17320
   ss: 0x00000023  efl: 0x00010286  eip: 0x00200216   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0095a000
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xe8000 -    0xefffb +a (0) <35A4E37A-4FCB-3551-B8A1-E99B83C28F10> /Users/USER/*/a
  0x146000 -   0x18bfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1cf000 -   0x29ffd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3336
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=85.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=85.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.0M       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       3988K        5 
VM_ALLOCATE                       3988K        5 
VM_ALLOCATE (reserved)             240K        3         reserved VM address space (unallocated)
__DATA                            3328K       44 
__LINKEDIT                        74.2M        5 
__OBJC                              36K        6 
__TEXT                            9620K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            581.7M      140 
TOTAL                            581.7M      140 
TOTAL, minus reserved VM space   581.5M      140 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054152-1_Traviss-Mac-1044.crash
Process:               a [35255]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [35247]
Responsible:           a [35255]
User ID:               501
Date/Time:             2018-08-27 05:41:51.089 +0000
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
0   libstd-312c1b89c01a0dbc.dylib  0x001ce216 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 118
1   a                              0x00018b5f std::panicking::begin_panic::h1dfd76ce48beac77 + 47 (panicking.rs:411)
2   a                              0x000165e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 36 (backtrace.rs:34)
3   a                              0x00015fcb core::ptr::drop_in_place::h39ab9c7eb9f9aa98 + 11
4   a                              0x000165b3 backtrace::double::h3a79da7ae181846f + 51
5   a                              0x000178c2 backtrace::main::h9751c5dfa7559d82 + 4562 (backtrace.rs:113)
6   a                              0x00015acb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha211c2951cb49c7d + 11 (rt.rs:74)
7   libstd-312c1b89c01a0dbc.dylib  0x001cdd37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
8   libstd-312c1b89c01a0dbc.dylib  0x001deb1d __rust_maybe_catch_panic + 29
9   libstd-312c1b89c01a0dbc.dylib  0x001cdcae std::panicking::try::h808ee7bbea2f45e8 + 62
10  libstd-312c1b89c01a0dbc.dylib  0x001b55a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
11  a                              0x0001814c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x001ce1ae  ecx: 0x00000000  edx: 0xa7702ec6
  edi: 0x002750b4  esi: 0x0023c9d0  ebp: 0xbffed398  esp: 0xbffed330
   ss: 0x00000023  efl: 0x00010282  eip: 0x001ce216   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x004c173d
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x12000 -    0x19ffb +a (0) <35A4E37A-4FCB-3551-B8A1-E99B83C28F10> /Users/USER/*/a
  0x114000 -   0x159fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x19d000 -   0x26dfd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3336
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=76.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3328K       44 
__LINKEDIT                        74.2M        5 
__OBJC                              36K        6 
__TEXT                            9620K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            572.7M      138 
TOTAL                            572.7M      138 
TOTAL, minus reserved VM space   572.6M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054257_Traviss-Mac-1044.crash
Process:               a [36647]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [36645]
Responsible:           a [36647]
User ID:               501
Date/Time:             2018-08-27 05:42:56.793 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5600 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x001afd7b std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x001a3ac0 rust_oom + 48
5   libstd-312c1b89c01a0dbc.dylib  0x0020a734 alloc::alloc::handle_alloc_error::h531aecfe0c23f91b + 20
6   a                              0x000be648 default_alloc_error_hook::main::hbf3cf79eecbb97ff + 808
7   a                              0x000bd70b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h8e8fd9e60f90c41f + 11
8   libstd-312c1b89c01a0dbc.dylib  0x001c5d37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
9   libstd-312c1b89c01a0dbc.dylib  0x001d6b1d __rust_maybe_catch_panic + 29
10  libstd-312c1b89c01a0dbc.dylib  0x001c5cae std::panicking::try::h808ee7bbea2f45e8 + 62
11  libstd-312c1b89c01a0dbc.dylib  0x001ad5a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
12  a                              0x000be7bc main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff424ac  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff424d8  esp: 0xbff424ac
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xbd000 -    0xbeff3 +a (0) <1691F384-28E7-3DAF-86F3-C9E89022EA0A> /Users/USER/*/a
  0x10c000 -   0x151fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x195000 -   0x265fd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
---
===========                     =======  ======= 
TOTAL                            572.7M      137 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054857-2_Traviss-Mac-1044.crash
Process:               a [44124]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [44123]
Responsible:           a [44124]
User ID:               501
Date/Time:             2018-08-27 05:48:56.967 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5900 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0662ea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0662ea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0662000-00000000b0663000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0663000-00000000b0864000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-312c1b89c01a0dbc.dylib  0x0018b6d0 std::sys::unix::thread::Thread::join::hd8ab4e4bc9ad521c + 32
4   a                              0x0003a3d6 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::ha782bceda184a50e + 70
5   a                              0x000393d7 out_of_stack::main::hfa8cecea9a60e028 + 2663
6   a                              0x00037e2b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha40f8657ae2241a5 + 11
7   libstd-312c1b89c01a0dbc.dylib  0x001b3d37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
8   libstd-312c1b89c01a0dbc.dylib  0x001c4b1d __rust_maybe_catch_panic + 29
9   libstd-312c1b89c01a0dbc.dylib  0x001b3cae std::panicking::try::h808ee7bbea2f45e8 + 62
10  libstd-312c1b89c01a0dbc.dylib  0x0019b5a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
11  a                              0x0003981c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x0019dd7b std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x001ae388 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x001b9581 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.14668056043756410761) + 161
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-312c1b89c01a0dbc.dylib  0x001b94e0 std::sys::unix::os::exit::hce08a6fea2927a01 + 32
9   a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
10  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
11  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
12  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
13  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
14  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
15  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
16  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
17  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
18  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
19  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
20  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
21  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
22  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
23  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
24  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
25  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
26  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
27  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
28  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
29  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
30  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
31  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
32  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
33  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
34  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
35  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
36  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
37  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
38  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
39  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
40  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
41  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
42  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
43  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
44  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
45  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
46  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
47  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
48  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
49  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
50  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
51  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
52  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
53  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
54  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
55  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
56  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
57  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
58  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
59  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
60  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
61  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
62  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
63  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
64  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
65  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
66  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
67  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
68  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
69  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
70  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
71  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
72  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
73  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
74  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
75  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
76  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
77  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
78  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
79  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
80  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
81  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
82  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
83  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
84  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
85  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
86  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
87  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
88  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
89  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
90  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
91  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
92  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
93  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
94  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
95  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
96  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
97  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
98  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
99  a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
100 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
101 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
102 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
103 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
104 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
105 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
106 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
107 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
108 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
109 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
110 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
111 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
112 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
113 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
114 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
115 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
116 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
117 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
118 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
119 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
120 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
121 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
122 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
123 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
124 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
125 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
126 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
127 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
128 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
129 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
130 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
131 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
132 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
133 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
134 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
135 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
136 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
137 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
138 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
139 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
140 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
141 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
142 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
143 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
144 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
145 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
146 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
147 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
148 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
149 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
150 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
151 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
152 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
153 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
154 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
155 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
156 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
157 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
158 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
159 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
160 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
161 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
162 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
163 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
164 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
165 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
166 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
167 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
168 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
169 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
170 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
171 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
172 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
173 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
174 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
175 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
176 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
177 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
178 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
179 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
180 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
181 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
182 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
183 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
184 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
185 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
186 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
187 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
188 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
189 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
190 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
191 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
192 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
193 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
194 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
195 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
196 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
197 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
198 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
199 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
200 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
201 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
202 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
203 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
204 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
205 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
206 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
207 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
208 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
209 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
210 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
211 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
212 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
213 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
214 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
215 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
216 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
217 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
218 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
219 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
220 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
221 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
222 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
223 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
224 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
225 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
226 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
227 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
228 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
229 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
230 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
231 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
232 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
233 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
234 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
235 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
236 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
237 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
238 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
239 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
240 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
241 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
242 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
243 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
244 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
245 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
246 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
247 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
248 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
249 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
250 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
251 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
252 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
253 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
254 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
255 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
256 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
257 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
258 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
259 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
260 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
261 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
262 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
263 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
264 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
265 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
266 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
267 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
268 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
269 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
270 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
271 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
272 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
273 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
274 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
275 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
276 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
277 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
278 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
279 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
280 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
281 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
282 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
283 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
284 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
285 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
286 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
287 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
288 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
289 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
290 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
291 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
292 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
293 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
294 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
295 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
296 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
297 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
298 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
299 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
300 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
301 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
302 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
303 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
304 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
305 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
306 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
307 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
308 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
309 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
310 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
311 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
312 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
313 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
314 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
315 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
316 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
317 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
318 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
319 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
320 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
321 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
322 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
323 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
324 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
325 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
326 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
327 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
328 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
329 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
330 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
331 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
332 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
333 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
334 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
335 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
336 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
337 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
338 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
339 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
340 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
341 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
342 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
343 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
344 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
345 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
346 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
347 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
348 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
349 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
350 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
351 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
352 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
353 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
354 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
355 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
356 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
357 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
358 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
359 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
360 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
361 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
362 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
363 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
364 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
365 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
366 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
367 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
368 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
369 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
370 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
371 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
372 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
373 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
374 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
375 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
376 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
377 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
378 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
379 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
380 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
381 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
382 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
383 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
384 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
385 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
386 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
387 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
388 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
389 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
390 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
391 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
392 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
393 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
394 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
395 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
396 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
397 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
398 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
399 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
400 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
401 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
402 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
403 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
404 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
405 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
406 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
407 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
408 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
409 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
410 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
411 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
412 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
413 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
414 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
415 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
416 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
417 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
418 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
419 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
420 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
421 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
422 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
423 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
424 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
425 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
426 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
427 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
428 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
429 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
430 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
431 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
432 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
433 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
434 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
435 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
436 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
437 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
438 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
439 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
440 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
441 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
442 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
443 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
444 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
445 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
446 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
447 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
448 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
449 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
450 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
451 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
452 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
453 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
454 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
455 a                              0x00038908 out_of_stack::silent_recurse::hdc68ca807aa784c9 + 40
---
===========                     =======  ======= 
TOTAL                            572.7M      137 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054913_Traviss-Mac-1044.crash
Process:               a [44364]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [44361]
Responsible:           a [44364]
User ID:               501
Date/Time:             2018-08-27 05:49:12.524 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0003f28b panic_abort::__rust_start_panic::abort::h8e92c8e295aa6470 + 11
4   a                              0x0003f27b __rust_start_panic + 11
5   a                              0x0003d921 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 1793
6   a                              0x00043d2a std::panicking::begin_panic::h3adcacae23dc4118 + 42
7   a                              0x000304bd lto_abort::main::h93d7449bf6778972 + 2893
8   a                              0x00043e8b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h15424d6a3ebc0a85 + 11
9   a                              0x0003183c std::sys_common::backtrace::__rust_begin_short_backtrace::hefee785841cfcd1d + 12
10  a                              0x000308b9 main + 1017
11  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffd048c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffd04b8  esp: 0xbffd048c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x2f000 -    0x7dfe7 +a (0) <9F425524-D0AC-3CF9-806C-46E55B85388B> /Users/USER/*/a
  0x153000 -   0x198fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3336
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.6M resident=0K(0%) swapped_out_or_unallocated=82.6M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            1344K       44 
__LINKEDIT                        73.7M        4 
__OBJC                              36K        6 
__TEXT                            9068K       43 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            569.8M      134 
TOTAL                            569.8M      134 
TOTAL, minus reserved VM space   569.7M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054953_Traviss-Mac-1044.crash
Process:               a [45176]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45171]
Responsible:           a [45176]
User ID:               501
Date/Time:             2018-08-27 05:49:53.176 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x0018bd7b std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x0019c388 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x001a24c8 rust_panic.llvm.14011393017470218905 + 104
6   libstd-312c1b89c01a0dbc.dylib  0x001a2389 std::panicking::rust_panic_with_hook::h0589b757ac5dff86 + 489
7   a                              0x000ab97f std::panicking::begin_panic::he072a440972f9cca + 47
8   a                              0x000acbfc main + 2668
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff554dc  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff55508  esp: 0xbff554dc
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xaa000 -    0xadff3 +a (0) <F3024365-AC2B-37F9-AF7D-973D1B125EEA> /Users/USER/*/a
   0xe8000 -   0x12dfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x171000 -   0x241fd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3588
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4096K        3 
__DATA                            3328K       44 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            572.6M      136 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-054955_Traviss-Mac-1044.crash
Process:               a [45204]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45203]
Responsible:           a [45204]
User ID:               501
Date/Time:             2018-08-27 05:49:54.355 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGSEGV)
Exception Codes:       KERN_INVALID_ADDRESS at 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Segmentation fault: 11
Termination Reason:    Namespace SIGNAL, Code 0xb
Terminating Process:   exc handler [0]
VM Regions Near 0:
--> 
    __TEXT                 0000000000046000-0000000000049000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00047f14 segfault_no_out_of_stack::main::h4bc020f9e0c050bb + 2132
1   a                              0x000466fb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd045dd389f3521ab + 11
2   libstd-312c1b89c01a0dbc.dylib  0x00127d37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
3   libstd-312c1b89c01a0dbc.dylib  0x00138b1d __rust_maybe_catch_panic + 29
4   libstd-312c1b89c01a0dbc.dylib  0x00127cae std::panicking::try::h808ee7bbea2f45e8 + 62
5   libstd-312c1b89c01a0dbc.dylib  0x0010f5a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
6   a                              0x000481ec main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00818040  esi: 0xbffb95c8  ebp: 0xbffb9698  esp: 0xbffb94f0
   ss: 0x00000023  efl: 0x00010246  eip: 0x00047f14   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     3
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x46000 -    0x48ff3 +a (0) <5A9B4A31-23B6-333D-8047-EDAB8B547DD2> /Users/USER/*/a
   0x6e000 -    0xb3fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xf7000 -   0x1c7fd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3588
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4228K        5 
__DATA                            3328K       44 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            572.7M      138 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-055009_Traviss-Mac-1044.crash
Process:               a [45362]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45361]
Responsible:           a [45362]
User ID:               501
Date/Time:             2018-08-27 05:50:09.108 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGSEGV)
Exception Codes:       KERN_INVALID_ADDRESS at 0x0000000000000001
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Segmentation fault: 11
Termination Reason:    Namespace SIGNAL, Code 0xb
Terminating Process:   exc handler [0]
VM Regions Near 0x1:
--> 
    __TEXT                 000000000002c000-000000000002f000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0002e594 signal_exit_status::main::h5e838d36792e8f94 + 436
1   a                              0x0002d26b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h4aa733f82e7c1c80 + 11
2   libstd-312c1b89c01a0dbc.dylib  0x001a3d37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
3   libstd-312c1b89c01a0dbc.dylib  0x001b4b1d __rust_maybe_catch_panic + 29
4   libstd-312c1b89c01a0dbc.dylib  0x001a3cae std::panicking::try::h808ee7bbea2f45e8 + 62
5   libstd-312c1b89c01a0dbc.dylib  0x0018b5a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
6   a                              0x0002e66c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x00818020
  edi: 0x00818040  esi: 0xbffd3610  ebp: 0xbffd36a8  esp: 0xbffd3590
   ss: 0x00000023  efl: 0x00010246  eip: 0x0002e594   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x2c000 -    0x2eff7 +a (0) <16D34926-17C7-3E5C-B6C1-BA0683EB0C2E> /Users/USER/*/a
   0xea000 -   0x12ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x173000 -   0x243fd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3588
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4228K        5 
__DATA                            3328K       44 
__LINKEDIT                        74.1M        5 
---
===========                     =======  ======= 
TOTAL                            572.7M      138 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-055015_Traviss-Mac-1044.crash
Process:               a [45458]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45455]
Responsible:           a [45458]
User ID:               501
Date/Time:             2018-08-27 05:50:15.378 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00005710 simd_target_feature_mixup::test::id_avx512_512::h96f843e8b3f4d2b1 + 112
1   a                              0x00004468 simd_target_feature_mixup::test::main::h1209a6983816d331 + 1848
2   a                              0x00006999 simd_target_feature_mixup::main::h9cd2e2449b07bb04 + 937
3   a                              0x00006bab std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h398bc92ea1b82c97 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x000e5d37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
5   libstd-312c1b89c01a0dbc.dylib  0x000f6b1d __rust_maybe_catch_panic + 29
6   libstd-312c1b89c01a0dbc.dylib  0x000e5cae std::panicking::try::h808ee7bbea2f45e8 + 62
7   libstd-312c1b89c01a0dbc.dylib  0x000cd5a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
8   a                              0x00006b8c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x000056ac  ebx: 0xbfffc240  ecx: 0xbfffc340  edx: 0xbfffc240
  edi: 0x00003d44  esi: 0x00000000  ebp: 0xbfffc238  esp: 0xbfffc200
   ss: 0x00000023  efl: 0x00010246  eip: 0x00005710   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000052f0
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
    0x3000 -     0x7fc7 +a (0) <F50AA27F-14EC-3BCB-B90A-4A2914F6A94F> /Users/USER/*/a
   0x2c000 -    0x71fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xb5000 -   0x185fd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3588
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=75.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3328K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9608K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            572.7M      138 
TOTAL                            572.7M      138 
TOTAL, minus reserved VM space   572.6M      138 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-055028-1_Traviss-Mac-1044.crash
Process:               a [45676]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45673]
Responsible:           a [45676]
User ID:               501
Date/Time:             2018-08-27 05:50:28.036 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf386a8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf386a8:
    Stack Guard            00000000bbf37000-00000000bbf38000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf38000-00000000bbf39000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf39000-00000000bff37000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x001bad7b std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x001cb388 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x001d6581 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.14668056043756410761) + 161
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-312c1b89c01a0dbc.dylib  0x001d64e0 std::sys::unix::os::exit::hce08a6fea2927a01 + 32
9   a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
10  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
11  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
12  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
13  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
14  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
15  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
16  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
17  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
18  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
19  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
20  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
21  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
22  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
23  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
24  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
25  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
26  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
27  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
28  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
29  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
30  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
31  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
32  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
33  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
34  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
35  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
36  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
37  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
38  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
39  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
40  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
41  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
42  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
43  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
44  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
45  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
46  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
47  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
48  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
49  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
50  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
51  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
52  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
53  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
54  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
55  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
56  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
57  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
58  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
59  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
60  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
61  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
62  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
63  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
64  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
65  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
66  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
67  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
68  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
69  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
70  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
71  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
72  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
73  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
74  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
75  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
76  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
77  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
78  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
79  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
80  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
81  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
82  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
83  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
84  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
85  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
86  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
87  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
88  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
89  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
90  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
91  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
92  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
93  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
94  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
95  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
96  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
97  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
98  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
99  a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
100 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
101 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
102 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
103 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
104 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
105 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
106 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
107 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
108 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
109 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
110 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
111 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
112 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
113 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
114 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
115 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
116 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
117 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
118 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
119 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
120 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
121 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
122 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
123 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
124 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
125 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
126 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
127 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
128 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
129 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
130 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
131 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
132 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
133 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
134 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
135 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
136 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
137 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
138 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
139 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
140 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
141 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
142 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
143 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
144 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
145 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
146 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
147 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
148 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
149 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
150 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
151 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
152 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
153 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
154 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
155 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
156 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
157 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
158 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
159 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
160 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
161 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
162 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
163 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
164 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
165 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
166 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
167 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
168 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
169 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
170 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
171 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
172 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
173 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
174 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
175 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
176 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
177 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
178 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
179 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
180 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
181 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
182 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
183 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
184 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
185 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
186 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
187 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
188 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
189 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
190 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
191 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
192 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
193 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
194 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
195 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
196 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
197 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
198 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
199 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
200 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
201 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
202 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
203 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
204 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
205 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
206 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
207 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
208 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
209 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
210 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
211 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
212 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
213 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
214 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
215 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
216 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
217 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
218 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
219 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
220 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
221 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
222 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
223 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
224 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
225 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
226 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
227 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
228 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
229 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
230 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
231 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
232 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
233 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
234 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
235 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
236 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
237 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
238 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
239 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
240 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
241 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
242 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
243 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
244 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
245 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
246 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
247 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
248 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
249 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
250 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
251 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
252 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
253 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
254 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
255 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
256 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
257 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
258 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
259 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
260 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
261 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
262 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
263 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
264 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
265 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
266 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
267 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
268 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
269 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
270 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
271 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
272 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
273 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
274 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
275 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
276 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
277 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
278 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
279 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
280 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
281 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
282 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
283 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
284 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
285 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
286 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
287 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
288 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
289 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
290 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
291 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
292 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
293 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
294 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
295 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
296 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
297 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
298 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
299 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
300 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
301 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
302 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
303 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
304 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
305 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
306 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
307 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
308 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
309 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
310 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
311 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
312 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
313 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
314 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
315 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
316 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
317 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
318 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
319 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
320 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
321 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
322 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
323 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
324 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
325 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
326 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
327 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
328 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
329 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
330 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
331 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
332 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
333 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
334 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
335 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
336 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
337 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
338 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
339 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
340 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
341 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
342 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
343 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
344 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
345 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
346 a                              0x000cc840 stack_probes::recurse::h5b19fb02dc377c0b + 48
---
===========                     =======  ======= 
TOTAL                            572.7M      137 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-055028_Traviss-Mac-1044.crash
Process:               a [45679]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45673]
Responsible:           a [45679]
User ID:               501
Date/Time:             2018-08-27 05:50:28.043 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b03bbe98
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb03bbe98:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b03bb000-00000000b03bc000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b03bc000-00000000b05bd000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-312c1b89c01a0dbc.dylib  0x001c56d0 std::sys::unix::thread::Thread::join::hd8ab4e4bc9ad521c + 32
4   a                              0x00090786 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::hafc79c397df6dd89 + 70
5   a                              0x0008f6f5 stack_probes::main::h521fd0a37334d32c + 597
6   a                              0x0008e34b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h1ad31c5e175b52d3 + 11
7   libstd-312c1b89c01a0dbc.dylib  0x001edd37 std::panicking::try::do_call::h9314641171ec181d (.llvm.14011393017470218905) + 23
8   libstd-312c1b89c01a0dbc.dylib  0x001feb1d __rust_maybe_catch_panic + 29
9   libstd-312c1b89c01a0dbc.dylib  0x001edcae std::panicking::try::h808ee7bbea2f45e8 + 62
10  libstd-312c1b89c01a0dbc.dylib  0x001d55a5 std::rt::lang_start_internal::ha462b0a2effb85ff + 197
11  a                              0x0009016c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-312c1b89c01a0dbc.dylib  0x001d7d7b std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   libstd-312c1b89c01a0dbc.dylib  0x001e8388 std::sys_common::util::abort::heae8a1f2fd58dc08 + 88
5   libstd-312c1b89c01a0dbc.dylib  0x001f3581 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.14668056043756410761) + 161
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-312c1b89c01a0dbc.dylib  0x001f34e0 std::sys::unix::os::exit::hce08a6fea2927a01 + 32
9   a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
10  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
11  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
12  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
13  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
14  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
15  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
16  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
17  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
18  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
19  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
20  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
21  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
22  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
23  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
24  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
25  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
26  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
27  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
28  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
29  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
30  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
31  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
32  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
33  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
34  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
35  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
36  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
37  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
38  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
39  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
40  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
41  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
42  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
43  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
44  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
45  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
46  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
47  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
48  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
49  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
50  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
51  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
52  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
53  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
54  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
55  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
56  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
57  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
58  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
59  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
60  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
61  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
62  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
63  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
64  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
65  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
66  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
67  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
68  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
69  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
70  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
71  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
72  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
73  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
74  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
75  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
76  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
77  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
78  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
79  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
80  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
81  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
82  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
83  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
84  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
85  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
86  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
87  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
88  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
89  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
90  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
91  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
92  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
93  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
94  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
95  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
96  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
97  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
98  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
99  a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
100 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
101 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
102 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
103 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
104 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
105 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
106 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
107 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
108 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
109 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
110 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
111 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
112 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
113 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
114 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
115 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
116 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
117 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
118 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
119 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
120 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
121 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
122 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
123 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
124 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
125 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
126 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
127 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
128 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
129 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
130 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
131 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
132 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
133 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
134 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
135 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
136 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
137 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
138 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
139 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
140 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
141 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
142 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
143 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
144 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
145 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
146 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
147 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
148 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
149 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
150 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
151 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
152 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
153 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
154 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
155 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
156 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
157 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
158 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
159 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
160 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
161 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
162 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
163 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
164 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
165 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
166 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
167 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
168 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
169 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
170 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
171 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
172 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
173 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
174 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
175 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
176 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
177 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
178 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
179 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
180 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
181 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
182 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
183 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
184 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
185 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
186 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
187 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
188 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
189 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
190 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
191 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
192 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
193 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
194 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
195 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
196 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
197 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
198 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
199 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
200 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
201 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
202 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
203 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
204 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
205 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
206 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
207 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
208 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
209 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
210 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
211 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
212 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
213 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
214 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
215 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
216 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
217 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
218 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
219 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
220 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
221 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
222 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
223 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
224 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
225 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
226 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
227 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
228 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
229 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
230 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
231 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
232 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
233 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
234 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
235 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
236 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
237 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
238 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
239 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
240 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
241 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
242 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
243 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
244 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
245 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
246 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
247 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
248 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
249 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
250 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
251 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
252 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
253 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
254 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
255 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
256 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
257 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
258 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
259 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
260 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
261 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
262 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
263 a                              0x0008f840 stack_probes::recurse::h5b19fb02dc377c0b + 48
264 a                              0x0008e3dd std::sys_common::backtrace::__rust_begin_short_backtrace::h75043aa63ec0db1c + 29
265 libstd-312c1b89c01a0dbc.dylib  0x001feb1d __rust_maybe_catch_panic + 29
266 a                              0x00090a63 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h81a86fd07c13df25 + 131
267 libstd-312c1b89c01a0dbc.dylib  0x001f25ab std::sys_common::thread::start_thread::h52dad84f78503b62 + 187
268 libstd-312c1b89c01a0dbc.dylib  0x001c5561 std::sys::unix::thread::Thread::new::thread_start::h7a084032ca4cb5ba + 17
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb05bc000  ecx: 0x000e5b4c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x000e5b78  esp: 0x000e5b4c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x001c51c0
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x8d000 -    0x91ff3 +a (0) <FDBA1E7C-CB2C-3E6D-974C-F79240BEA871> /Users/USER/*/a
  0x134000 -   0x179fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1bd000 -   0x28dfd7 +libstd-312c1b89c01a0dbc.dylib (0) <F96F5A86-C087-3CD4-A19F-2907A1F81098> /Users/USER/*/libstd-312c1b89c01a0dbc.dylib
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75d1000 - 0xa75d1fff  liblaunch.dylib (1205.30.29) <0F3BF17D-FCFA-3692-8A6E-FDE5C58DB3B7> /usr/lib/system/liblaunch.dylib
0xa75d2000 - 0xa75d7fff  libmacho.dylib (900.0.1) <F1F0BC1D-A2D9-39F9-9A11-263F8392CB3B> /usr/lib/system/libmacho.dylib
0xa75d8000 - 0xa75dafff  libquarantine.dylib (86) <68DE2EB2-7911-304D-89D6-1517CA689001> /usr/lib/system/libquarantine.dylib
0xa75db000 - 0xa75dcfff  libremovefile.dylib (45) <BEF76B44-53EA-3970-AB50-2296DC7F097F> /usr/lib/system/libremovefile.dylib
0xa75dd000 - 0xa75f4ff7  libsystem_asl.dylib (356.1.1) <F96973B5-C36B-3037-8AEC-3BF7147D79E2> /usr/lib/system/libsystem_asl.dylib
0xa75f5000 - 0xa75f5fff  libsystem_blocks.dylib (67) <32CE9BB7-E047-3568-981E-4EA94D3DCBB5> /usr/lib/system/libsystem_blocks.dylib
0xa75f6000 - 0xa7682fff  libsystem_c.dylib (1244.30.3) <8BCBF89D-5CE7-3950-884A-86E37DBF2660> /usr/lib/system/libsystem_c.dylib
0xa7683000 - 0xa7686fff  libsystem_configuration.dylib (963.30.1) <0F30DC5A-F39F-32C9-BA01-05AAC699713A> /usr/lib/system/libsystem_configuration.dylib
0xa7687000 - 0xa768afff  libsystem_coreservices.dylib (51) <C3D75760-EED5-3C5C-8245-FBD3D9FD60FD> /usr/lib/system/libsystem_coreservices.dylib
0xa768b000 - 0xa768cfff  libsystem_darwin.dylib (1244.30.3) <83B1D06A-9FA5-3F0B-A223-0659F4248E51> /usr/lib/system/libsystem_darwin.dylib
0xa768d000 - 0xa7693ff3  libsystem_dnssd.dylib (878.30.4) <3C4400C4-C531-3653-872B-E22892D7B29A> /usr/lib/system/libsystem_dnssd.dylib
0xa7694000 - 0xa76e3ffb  libsystem_info.dylib (517.30.1) <A8E62937-32F9-373C-8150-B6B442227226> /usr/lib/system/libsystem_info.dylib
0xa76e4000 - 0xa7707ff7  libsystem_kernel.dylib (4570.41.2) <649BB7E7-6378-3D2C-BBC6-ED2577E551B9> /usr/lib/system/libsystem_kernel.dylib
0xa7708000 - 0xa7757fdb  libsystem_m.dylib (3146) <DBE0AACD-3665-3EEB-BADA-A435E591C54B> /usr/lib/system/libsystem_m.dylib
0xa7758000 - 0xa7772fff  libsystem_malloc.dylib (140.40.1) <968EF31E-50B0-3BFD-96B8-8FD513BDCB3C> /usr/lib/system/libsystem_malloc.dylib
0xa7773000 - 0xa7810ffb  libsystem_network.dylib (1229.30.11) <84B584A7-7583-31E7-8A39-64B4A5AD643B> /usr/lib/system/libsystem_network.dylib
0xa7811000 - 0xa781bfff  libsystem_networkextension.dylib (767.40.1) <EB81FB6B-A725-3F87-991A-DD55F0ED540A> /usr/lib/system/libsystem_networkextension.dylib
0xa781c000 - 0xa7824ff3  libsystem_notify.dylib (172) <63E3CF8C-4F15-3D45-84A6-1218352031E9> /usr/lib/system/libsystem_notify.dylib
0xa7825000 - 0xa782bffb  libsystem_platform.dylib (161.20.1) <82782E0E-7264-3CB4-AE69-571EDB5E7A24> /usr/lib/system/libsystem_platform.dylib
0xa782c000 - 0xa7836ff3  libsystem_pthread.dylib (301.30.1) <7409C1E5-F3BA-3AB3-ADC1-9DCD356C6C13> /usr/lib/system/libsystem_pthread.dylib
0xa7837000 - 0xa783aff3  libsystem_sandbox.dylib (765.40.2) <07D6B9E4-5A9D-300F-8D65-E218EDE85477> /usr/lib/system/libsystem_sandbox.dylib
0xa783b000 - 0xa783dfff  libsystem_secinit.dylib (30) <CE2C90DE-27A4-3546-8A05-96B743861DD0> /usr/lib/system/libsystem_secinit.dylib
0xa783e000 - 0xa7846ff7  libsystem_symptoms.dylib (820.30.7) <0283BDB3-16A2-371E-A25C-A26F076C24A5> /usr/lib/system/libsystem_symptoms.dylib
0xa7847000 - 0xa7859fff  libsystem_trace.dylib (829.30.14) <79446DE0-89F6-3979-B14C-7B463AD70351> /usr/lib/system/libsystem_trace.dylib
0xa785b000 - 0xa7861fff  libunwind.dylib (35.3) <642BBA03-0411-3FAA-A421-D79A9156AB1C> /usr/lib/system/libunwind.dylib
0xa7862000 - 0xa788aff7  libxpc.dylib (1205.30.29) <CD44097B-4B65-3F75-AB7F-52228229FFB5> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 3588
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.5M resident=0K(0%) swapped_out_or_unallocated=83.5M(100%)
Writable regions: Total=77.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=77.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9252K       10 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                       6276K        7 
VM_ALLOCATE                       6276K        7 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3328K       44 
__LINKEDIT                        74.1M        5 
__OBJC                              36K        6 
__TEXT                            9608K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            576.8M      143 
TOTAL                            576.8M      143 
TOTAL, minus reserved VM space   576.7M      143 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-055034_Traviss-Mac-1044.crash
Process:               a [45749]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45748]
Responsible:           a [45749]
User ID:               501
Date/Time:             2018-08-27 05:50:32.853 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf32738
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf32738:
    Stack Guard            00000000bbf31000-00000000bbf32000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf32000-00000000bbf33000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf33000-00000000bff31000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000dd13b std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   a                              0x000e0d32 std::sys_common::util::abort::heae8a1f2fd58dc08 + 82
5   a                              0x000e54f1 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.14668056043756410761) + 593
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000e52a0 std::sys::unix::os::error_string::h385ab9f8bc14b0c6 + 240
9   a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
10  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
11  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
12  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
13  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
14  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
15  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
16  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
17  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
18  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
19  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
20  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
21  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
22  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
23  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
24  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
25  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
26  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
27  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
28  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
29  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
30  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
31  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
32  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
33  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
34  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
35  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
36  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
37  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
38  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
39  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
40  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
41  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
42  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
43  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
44  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
45  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
46  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
47  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
48  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
49  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
50  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
51  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
52  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
53  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
54  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
55  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
56  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
57  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
58  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
59  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
60  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
61  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
62  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
63  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
64  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
65  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
66  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
67  a                              0x000d20c8 stack_probes_lto::recurse::h6895313952b89569 + 40
---
===========                     =======  ======= 
TOTAL                            569.9M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-27-055035_Traviss-Mac-1044.crash
Process:               a [45750]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [45748]
Responsible:           a [45750]
User ID:               501
Date/Time:             2018-08-27 05:50:32.903 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6000 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0098ef8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0098ef8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0098000-00000000b0099000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0099000-00000000b029a000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x00094ba5 stack_probes_lto::main::h46e0341d7865a006 + 2709
4   a                              0x000aebcb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc85022b643f5e2ea + 11
5   a                              0x000974dc std::sys_common::backtrace::__rust_begin_short_backtrace::hefee785841cfcd1d + 12
6   a                              0x00095ec6 main + 582
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x000a013b std::sys::unix::abort_internal::h88a26145a8b97680 + 11
4   a                              0x000a3d32 std::sys_common::util::abort::heae8a1f2fd58dc08 + 82
5   a                              0x000a84f1 std::sys::unix::stack_overflow::imp::signal_handler::hf542bf7dd946dc68 (.llvm.14668056043756410761) + 593
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000a82a0 std::sys::unix::os::error_string::h385ab9f8bc14b0c6 + 240
9   a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
10  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
11  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
12  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
13  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
14  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
15  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
16  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
17  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
18  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
19  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
20  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
21  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
22  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
23  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
24  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
25  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
26  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
27  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
28  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
29  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
30  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
31  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
32  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
33  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
34  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
35  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
36  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
37  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
38  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
39  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
40  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
41  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
42  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
43  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
44  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
45  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
46  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
47  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
48  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
49  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
50  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
51  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
52  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
53  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
54  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
55  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
56  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
57  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
58  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
59  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
60  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
61  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
62  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
63  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
64  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
65  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
66  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
67  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
68  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
69  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
70  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
71  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
72  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
73  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
74  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
75  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
76  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
77  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
78  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
79  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
80  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
81  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
82  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
83  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
84  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
85  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
86  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
87  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
88  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
89  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
90  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
91  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
92  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
93  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
94  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
95  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
96  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
97  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
98  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
99  a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
100 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
101 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
102 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
103 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
104 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
105 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
106 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
107 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
108 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
109 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
110 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
111 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
112 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
113 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
114 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
115 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
116 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
117 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
118 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
119 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
120 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
121 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
122 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
123 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
124 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
125 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
126 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
127 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
128 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
129 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
130 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
131 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
132 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
133 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
134 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
135 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
136 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
137 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
138 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
139 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
140 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
141 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
142 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
143 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
144 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
145 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
146 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
147 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
148 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
149 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
150 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
151 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
152 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
153 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
154 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
155 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
156 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
157 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
158 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
159 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
160 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
161 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
162 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
163 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
164 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
165 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
166 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
167 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
168 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
169 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
170 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
171 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
172 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
173 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
174 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
175 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
176 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
177 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
178 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
179 a                              0x000950c8 stack_probes_lto::recurse::h6895313952b89569 + 40
