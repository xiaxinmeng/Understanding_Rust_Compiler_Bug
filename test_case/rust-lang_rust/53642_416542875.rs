plain
[00:04:24]       Memory: 8 GB
[00:04:24]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:04:24]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:04:24]       SMC Version (system): 2.8f0
[00:04:24]       Serial Number (system): VM+pE2HOTW57
[00:04:24] 
[00:04:24] hw.ncpu: 4
[00:04:24] hw.byteorder: 1234
[00:04:24] hw.memsize: 8589934592
---
[01:57:59] status: exit code: 2
[01:57:59] command: "make"
[01:57:59] stdout:
[01:57:59] ------------------------------------------
[01:57:59] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib:" '/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc' --out-dir /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native -L /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native  foo.rs -C target-cpu=native 2>&1 | tee /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native/out.log
[01:57:59] DYLD_LIBRARY_PATH="/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native:/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib:" /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native/foo
[01:57:59] # Make sure no warnings about "unknown CPU `native`" were emitted
[01:57:59] if [ "$(wc -c /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps/target-cpu-native/target-cpu-native/out.log | cut -d' ' -f 1)" = "0" ]; then \
[01:57:59]    echo no warnings generated; \
[01:57:59]  else \
[01:57:59]    exit 1; \
[01:57:59] 
[01:57:59] ------------------------------------------
[01:57:59] stderr:
[01:57:59] ------------------------------------------
[01:57:59] ------------------------------------------
[01:57:59] make[1]: *** [all] Error 1
[01:57:59] ------------------------------------------
[01:57:59] 
[01:57:59] 
[01:57:59] thread '[run-make] run-make-fulldeps/target-cpu-native' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:57:59] 
[01:57:59] 
[01:57:59] failures:
[01:57:59]     [run-make] run-make-fulldeps/target-cpu-native
[01:57:59]     [run-make] run-make-fulldeps/target-cpu-native
[01:57:59] 
[01:57:59] test result: FAILED. 189 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:57:59] 
[01:57:59] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[01:57:59] 
[01:57:59] 
[01:57:59] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--rustdoc-path" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustdoc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-make-fulldeps" "--build-base" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-apple-darwin" "--mode" "run-make" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "7.0.0\n" "--cc" "/Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang" "--cxx" "/Users/travis/build/rust-lang/rust/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++" "--cflags" "-ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++" "--llvm-components" "aarch64 aarch64asmparser aarch64asmprinter aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armasmprinter armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter binaryformat bitreader bitwriter codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader libdriver lineeditor linker lto mc mcdisassembler mcjit mcparser mips mipsasmparser mipsasmprinter mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmprinter msp430codegen msp430desc msp430info native nativecodegen nvptx nvptxasmprinter nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes powerpc powerpcasmparser powerpcasmprinter powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata riscv riscvasmparser riscvasmprinter riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport transformutils vectorize webassembly webassemblyasmparser webassemblyasmprinter webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils" "--llvm-cxxflags" "-I/Users/travis/build/rust-lang/rust/src/llvm/include -I/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/llvm/build/include -ffunction-sections -fdata-sections -fPIC --target=x86_64-apple-darwin -stdlib=libc++ -fPIC -fvisibility-inlines-hidden -Werror=date-time -Werror=unguarded-availability-new -std=c++11 -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -pedantic -Wno-long-long -Wcovered-switch-default -Wnon-virtual-dtor -Wdelete-non-virtual-dtor -Wstring-conversion -O3 -DNDEBUG  -fno-exceptions -fno-rtti -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:57:59] 
[01:57:59] 
[01:57:59] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:57:59] Build completed unsuccessfully in 0:54:22
[01:57:59] Build completed unsuccessfully in 0:54:22
[01:57:59] make: *** [check] Error 1
travis_time:end:21c3079c:start=1535446710010717000,finish=1535453788826621000,duration=7078815904000

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1878d7ab
---
travis_fold:start:after_failure.2
travis_time:start:2582ac80
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Aug 28 10:56 .
-rw-------@  1 travis  staff  13741 Aug 28 10:56 overflow_2018-08-28-105628_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Aug 28 10:56 foo_2018-08-28-105607_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Aug 28 10:55 m4_2018-08-28-105540_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Aug 28 10:55 bar_2018-08-28-105531_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10690 Aug 28 10:55 b_2018-08-28-105530_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Aug 28 10:55 bar_2018-08-28-105530_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62273 Aug 28 10:22 a_2018-08-28-102252_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37515 Aug 28 10:22 a_2018-08-28-102251_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37410 Aug 28 10:22 a_2018-08-28-102245-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60415 Aug 28 10:22 a_2018-08-28-102245_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10171 Aug 28 10:22 a_2018-08-28-102233_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9902 Aug 28 10:22 a_2018-08-28-102227_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9907 Aug 28 10:22 a_2018-08-28-102216_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9869 Aug 28 10:22 a_2018-08-28-102215_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9965 Aug 28 10:21 a_2018-08-28-102142_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63133 Aug 28 10:21 a_2018-08-28-102133_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64335 Aug 28 10:21 a_2018-08-28-102128-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65148 Aug 28 10:21 a_2018-08-28-102128-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63943 Aug 28 10:21 a_2018-08-28-102128_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11786 Aug 28 10:18 a_2018-08-28-101836_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9926 Aug 28 10:17 a_2018-08-28-101731_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10333 Aug 28 10:15 a_2018-08-28-101559_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10513 Aug 28 10:15 a_2018-08-28-101503-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10515 Aug 28 10:15 a_2018-08-28-101503-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10242 Aug 28 10:15 a_2018-08-28-101503_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04ad6df2
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-28-101503-1_Traviss-Mac-1044.crash
Process:               a [39191]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        a [39182]
Responsible:           a [39191]
User ID:               501
Date/Time:             2018-08-28 10:14:58.718 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-aea295b28e61a362.dylib  0x000000010644f56b std::panicking::rust_panic_with_hook::h8abd0bf3c79cb330 + 139
1   a                              0x00000001064097d8 std::panicking::begin_panic::h2d70ea80ea04cee7 + 40
2   a                              0x000000010640725c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x0000000106406d99 core::ptr::drop_in_place::hb47aeb4441f77ce0 + 9
4   a                              0x0000000106407233 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x00000001064083ee backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x0000000106406736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6505d7c9a4562e54 + 6 (rt.rs:74)
7   libstd-aea295b28e61a362.dylib  0x000000010644f038 std::panicking::try::do_call::h31a6880baf32957f (.llvm.14111965128192413747) + 24
8   libstd-aea295b28e61a362.dylib  0x000000010646071f __rust_maybe_catch_panic + 31
9   libstd-aea295b28e61a362.dylib  0x0000000106435abd std::rt::lang_start_internal::h9028dc5db1fd95af + 237
10  a                              0x0000000106408c7c main + 44
11  libdyld.dylib                  0x00007fff57173115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee97fbf03  rbx: 0x0000000000000002  rcx: 0x00007fff572c49d2  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001064be405  rbp: 0x00007ffee97fc070  rsp: 0x00007ffee97fbff0
   r8: 0x00000001064be000   r9: 0x0000000106503f20  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001064be000  r14: 0x000000010640b460  r15: 0x00007ffee97fc080
  rip: 0x000000010644f56b  rfl: 0x0000000000010202  cr2: 0x0000000106733a9f
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x106402000 -        0x10640afff +a (0) <FCFAE3C0-EDE1-3229-8969-6260A66FF936> /Users/USER/*/a
       0x10641b000 -        0x1064f9fd7 +libstd-aea295b28e61a362.dylib (0) <30C3398F-08EF-3CF6-AC84-2B9B3E696994> /Users/USER/*/libstd-aea295b28e61a362.dylib
       0x1096ee000 -        0x10973898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff549dd000 -     0x7fff54a10fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff54eef000 -     0x7fff54ef0ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff551a5000 -     0x7fff551fbfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff551fc000 -     0x7fff55220ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff56572000 -     0x7fff569633b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff56a30000 -     0x7fff56a4cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5700a000 -     0x7fff5700eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5700f000 -     0x7fff57019ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5701a000 -     0x7fff57021fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff57022000 -     0x7fff5702affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5702b000 -     0x7fff570b0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff57138000 -     0x7fff57171ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff57172000 -     0x7fff5718fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff57190000 -     0x7fff57190ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5719e000 -     0x7fff5719eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5719f000 -     0x7fff571a3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff571a4000 -     0x7fff571a6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff571a7000 -     0x7fff571a8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff571a9000 -     0x7fff571c0fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff571c1000 -     0x7fff571c1fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff571c2000 -     0x7fff5724bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5724c000 -     0x7fff5724fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff57250000 -     0x7fff57253ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff57254000 -     0x7fff57255fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff57256000 -     0x7fff5725cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5725d000 -     0x7fff572a6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff572a7000 -     0x7fff572ccff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff572cd000 -     0x7fff57318fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff57319000 -     0x7fff57338fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff57339000 -     0x7fff573ddff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff573de000 -     0x7fff573e8ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff573e9000 -     0x7fff573f2ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff573f3000 -     0x7fff573faff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff573fb000 -     0x7fff57406fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff57407000 -     0x7fff5740aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5740b000 -     0x7fff5740cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5740d000 -     0x7fff57414ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff57415000 -     0x7fff57428ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5742a000 -     0x7fff5742fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff57430000 -     0x7fff5745cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2853
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.8M resident=0K(0%) swapped_out_or_unallocated=198.8M(100%)
Writable regions: Total=76.9M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.9M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9928K       44 
===========                     =======  ======= 
TOTAL                            280.2M      113 
TOTAL                            280.2M      113 
TOTAL, minus reserved VM space   280.1M      113 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-28-101503-2_Traviss-Mac-1044.crash
Process:               a [39192]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [39182]
Responsible:           a [39192]
User ID:               501
Date/Time:             2018-08-28 10:14:58.975 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-aea295b28e61a362.dylib  0x0000000106e2756b std::panicking::rust_panic_with_hook::h8abd0bf3c79cb330 + 139
1   a                              0x0000000106de17d8 std::panicking::begin_panic::h2d70ea80ea04cee7 + 40
2   a                              0x0000000106ddf25c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hee8ed3175b25d327 + 28
3   a                              0x0000000106dded99 core::ptr::drop_in_place::hb47aeb4441f77ce0 + 9
4   a                              0x0000000106ddf233 backtrace::double::h3a79da7ae181846f + 35
5   a                              0x0000000106de03ee backtrace::main::h9751c5dfa7559d82 + 4254 (backtrace.rs:113)
6   a                              0x0000000106dde736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6505d7c9a4562e54 + 6 (rt.rs:74)
7   libstd-aea295b28e61a362.dylib  0x0000000106e27038 std::panicking::try::do_call::h31a6880baf32957f (.llvm.14111965128192413747) + 24
8   libstd-aea295b28e61a362.dylib  0x0000000106e3871f __rust_maybe_catch_panic + 31
9   libstd-aea295b28e61a362.dylib  0x0000000106e0dabd std::rt::lang_start_internal::h9028dc5db1fd95af + 237
10  a                              0x0000000106de0c7c main + 44
11  libdyld.dylib                  0x00007fff57173115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee8e23f03  rbx: 0x0000000000000002  rcx: 0x00007fff572c49d2  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000106e96405  rbp: 0x00007ffee8e24060  rsp: 0x00007ffee8e23fe0
   r8: 0x0000000106e96000   r9: 0x0000000106edbf20  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000106e96000  r14: 0x0000000106de3460  r15: 0x00007ffee8e24070
  rip: 0x0000000106e2756b  rfl: 0x0000000000010206  cr2: 0x0000000103289000
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x106dda000 -        0x106de2fff +a (0) <FCFAE3C0-EDE1-3229-8969-6260A66FF936> /Users/USER/*/a
       0x106df3000 -        0x106ed1fd7 +libstd-aea295b28e61a362.dylib (0) <30C3398F-08EF-3CF6-AC84-2B9B3E696994> /Users/USER/*/libstd-aea295b28e61a362.dylib
       0x10fe56000 -        0x10fea098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff549dd000 -     0x7fff54a10fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff54eef000 -     0x7fff54ef0ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff551a5000 -     0x7fff551fbfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff551fc000 -     0x7fff55220ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff56572000 -     0x7fff569633b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff56a30000 -     0x7fff56a4cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5700a000 -     0x7fff5700eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5700f000 -     0x7fff57019ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5701a000 -     0x7fff57021fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff57022000 -     0x7fff5702affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5702b000 -     0x7fff570b0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff57138000 -     0x7fff57171ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff57172000 -     0x7fff5718fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff57190000 -     0x7fff57190ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5719e000 -     0x7fff5719eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5719f000 -     0x7fff571a3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff571a4000 -     0x7fff571a6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff571a7000 -     0x7fff571a8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff571a9000 -     0x7fff571c0fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff571c1000 -     0x7fff571c1fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff571c2000 -     0x7fff5724bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5724c000 -     0x7fff5724fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff57250000 -     0x7fff57253ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff57254000 -     0x7fff57255fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff57256000 -     0x7fff5725cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5725d000 -     0x7fff572a6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff572a7000 -     0x7fff572ccff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff572cd000 -     0x7fff57318fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff57319000 -     0x7fff57338fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff57339000 -     0x7fff573ddff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff573de000 -     0x7fff573e8ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff573e9000 -     0x7fff573f2ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff573f3000 -     0x7fff573faff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff573fb000 -     0x7fff57406fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff57407000 -     0x7fff5740aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5740b000 -     0x7fff5740cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5740d000 -     0x7fff57414ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff57415000 -     0x7fff57428ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5742a000 -     0x7fff5742fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff57430000 -     0x7fff5745cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2853
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.8M resident=0K(0%) swapped_out_or_unallocated=198.8M(100%)
Writable regions: Total=76.9M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.9M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9928K       44 
===========                     =======  ======= 
TOTAL                            280.2M      113 
TOTAL                            280.2M      113 
TOTAL, minus reserved VM space   280.1M      113 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-28-101503_Traviss-Mac-1044.crash
Process:               a [38533]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [38532]
Responsible:           a [38533]
User ID:               501
Date/Time:             2018-08-28 10:14:27.103 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4700 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010a61d74e abort_on_c_abi::panic_in_ffi::h305168285df224eb + 30
1   a                              0x000000010a61cb29 std::panicking::try::do_call::h2497da7791a66fdc (.llvm.7404455901653822252) + 9
2   libstd-aea295b28e61a362.dylib  0x000000010a66a71f __rust_maybe_catch_panic + 31
3   a                              0x000000010a61d9a1 abort_on_c_abi::main::h82c0574e6cdd7b93 + 593
4   a                              0x000000010a61bda6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc31dafc640c2defb + 6
5   libstd-aea295b28e61a362.dylib  0x000000010a659038 std::panicking::try::do_call::h31a6880baf32957f (.llvm.14111965128192413747) + 24
6   libstd-aea295b28e61a362.dylib  0x000000010a66a71f __rust_maybe_catch_panic + 31
7   libstd-aea295b28e61a362.dylib  0x000000010a63fabd std::rt::lang_start_internal::h9028dc5db1fd95af + 237
8   a                              0x000000010a61dcac main + 44
9   libdyld.dylib                  0x00007fff57173115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x000000010ac1c050  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee55e2818  rsi: 0xfffffffffffffce8  rbp: 0x00007ffee55e3220  rsp: 0x00007ffee55e3220
   r8: 0x0000000000000000   r9: 0x000000010a70df20  r10: 0x0000000114caa8d0  r11: 0x00007fff5742a96c
  r12: 0x0000000000000000  r13: 0x0000000000000000  r14: 0x00007ffee55e3340  r15: 0x00007ffee55e3288
  rip: 0x000000010a61d74e  rfl: 0x0000000000010202  cr2: 0x000000010a6c5430
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10a61b000 -        0x10a61efff +a (0) <2DC6D655-56FF-332E-8CF5-B0F718C94476> /Users/USER/*/a
       0x10a625000 -        0x10a703fd7 +libstd-aea295b28e61a362.dylib (0) <30C3398F-08EF-3CF6-AC84-2B9B3E696994> /Users/USER/*/libstd-aea295b28e61a362.dylib
       0x114c58000 -        0x114ca298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff549dd000 -     0x7fff54a10fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff54eef000 -     0x7fff54ef0ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff551a5000 -     0x7fff551fbfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff551fc000 -     0x7fff55220ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff56572000 -     0x7fff569633b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff56a30000 -     0x7fff56a4cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5700a000 -     0x7fff5700eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5700f000 -     0x7fff57019ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5701a000 -     0x7fff57021fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff57022000 -     0x7fff5702affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5702b000 -     0x7fff570b0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff57138000 -     0x7fff57171ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff57172000 -     0x7fff5718fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff57190000 -     0x7fff57190ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5719e000 -     0x7fff5719eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5719f000 -     0x7fff571a3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff571a4000 -     0x7fff571a6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff571a7000 -     0x7fff571a8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff571a9000 -     0x7fff571c0fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff571c1000 -     0x7fff571c1fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff571c2000 -     0x7fff5724bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5724c000 -     0x7fff5724fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff57250000 -     0x7fff57253ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff57254000 -     0x7fff57255fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff57256000 -     0x7fff5725cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5725d000 -     0x7fff572a6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff572a7000 -     0x7fff572ccff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff572cd000 -     0x7fff57318fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff57319000 -     0x7fff57338fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff57339000 -     0x7fff573ddff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff573de000 -     0x7fff573e8ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff573e9000 -     0x7fff573f2ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff573f3000 -     0x7fff573faff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff573fb000 -     0x7fff57406fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff57407000 -     0x7fff5740aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5740b000 -     0x7fff5740cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5740d000 -     0x7fff57414ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff57415000 -     0x7fff57428ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5742a000 -     0x7fff5742fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff57430000 -     0x7fff5745cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2853
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.7M resident=0K(0%) swapped_out_or_unallocated=198.7M(100%)
Writable regions: Total=75.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=75.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE                       4100K        4 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4340K       45 
__LINKEDIT                       189.1M        5 
__TEXT                            9908K       44 
===========                     =======  ======= 
TOTAL                            280.2M      114 
TOTAL                            280.2M      114 
TOTAL, minus reserved VM space   280.1M      114 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-08-28-101559_Traviss-Mac-1044.crash
Process:               a [40597]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [40595]
Responsible:           a [40597]
User ID:               501
Date/Time:             2018-08-28 10:15:59.689 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff572c2e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff57401150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5721f312 abort + 127
3   libstd-aea295b28e61a362.dylib  0x00000001063765f9 std::sys::unix::abort_internal::h8c9172655b389d03 + 9
4   libstd-aea295b28e61a362.dylib  0x0000000106369540 rust_oom + 32
5   libstd-aea295b28e61a362.dylib  0x00000001063d3869 alloc::alloc::handle_alloc_error::h4923d88aaf21ae35 + 9
6   a                              0x000000010635224d default_alloc_error_hook::main::hbf3cf79eecbb97ff + 797
7   a                              0x00000001063525c6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hee90558ecd0802be + 6
8   libstd-aea295b28e61a362.dylib  0x000000010638d038 std::panicking::try::do_call::h31a6880baf32957f (.llvm.14111965128192413747) + 24
9   libstd-aea295b28e61a362.dylib  0x000000010639e71f __rust_maybe_catch_panic + 31
10  libstd-aea295b28e61a362.dylib  0x0000000106373abd std::rt::lang_start_internal::h9028dc5db1fd95af + 237
11  a                              0x00000001063523bc main + 44
12  libdyld.dylib                  0x00007fff57173115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff8fe99340  rcx: 0x00007ffee98ae188  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee98ae1c0  rsp: 0x00007ffee98ae188
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff572c2e3e  rfl: 0x0000000000000206  cr2: 0x00007fff8fe77148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x106350000 -        0x106352fff +a (0) <82529E38-CCC3-3405-8A45-99EF41ECC432> /Users/USER/*/a
       0x106359000 -        0x106437fd7 +libstd-aea295b28e61a362.dylib (0) <30C3398F-08EF-3CF6-AC84-2B9B3E696994> /Users/USER/*/libstd-aea295b28e61a362.dylib
       0x110dc0000 -        0x110e0a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff549dd000 -     0x7fff54a10fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff54eef000 -     0x7fff54ef0ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff551a5000 -     0x7fff551fbfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff551fc000 -     0x7fff55220ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff56572000 -     0x7fff569633b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff56a30000 -     0x7fff56a4cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5700a000 -     0x7fff5700eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5700f000 -     0x7fff57019ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5701a000 -     0x7fff57021fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff57022000 -     0x7fff5702affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5702b000 -     0x7fff570b0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff57138000 -     0x7fff57171ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff57172000 -     0x7fff5718fff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff57190000 -     0x7fff57190ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5719e000 -     0x7fff5719eff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5719f000 -     0x7fff571a3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff571a4000 -     0x7fff571a6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff571a7000 -     0x7fff571a8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff571a9000 -     0x7fff571c0fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff571c1000 -     0x7fff571c1fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff571c2000 -     0x7fff5724bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5724c000 -     0x7fff5724fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff57250000 -     0x7fff57253ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff57254000 -     0x7fff57255fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff57256000 -     0x7fff5725cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5725d000 -     0x7fff572a6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff572a7000 -     0x7fff572ccff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff572cd000 -     0x7fff57318fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff57319000 -     0x7fff57338fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff57339000 -     0x7fff573ddff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff573de000 -     0x7fff573e8ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff573e9000 -     0x7fff573f2ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff573f3000 -     0x7fff573faff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff573fb000 -     0x7fff57406fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff57407000 -     0x7fff5740aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5740b000 -     0x7fff5740cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5740d000 -     0x7fff57414ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff57415000 -     0x7fff57428ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5742a000 -     0x7fff5742fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff57430000 -     0x7fff5745cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
