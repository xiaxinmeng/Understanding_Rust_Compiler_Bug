plain
######################################################################## 100.0%
[00:00:58] extracting /Users/travis/build/rust-lang/rust/build/cache/2019-03-20/rustc-beta-i686-apple-darwin.tar.gz
[00:00:58] downloading https://static.rust-lang.org/dist/2019-03-20/cargo-beta-i686-apple-darwin.tar.gz
[00:00:58] 
[00:00:58] curl: (35) error:14077410:SSL routines:SSL23_GET_SERVER_HELLO:sslv3 alert handshake failure
[00:00:58] spurious failure, trying again
[00:00:58] downloading https://static.rust-lang.org/dist/2019-03-20/cargo-beta-i686-apple-darwin.tar.gz
[00:00:58] 
###################################                                       49.4%
---
[00:02:52]       Memory: 8 GB
[00:02:52]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:02:52]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:52]       SMC Version (system): 2.8f0
[00:02:52]       Serial Number (system): VMKrVjoeM38S
[00:02:52] 
[00:02:53] hw.ncpu: 4
[00:02:53] hw.byteorder: 1234
[00:02:53] hw.memsize: 8589934592
---
[01:29:45] 
[01:29:45] ------------------------------------------
[01:29:45] stderr:
[01:29:45] ------------------------------------------
[01:29:45] thread 'main' panicked at 'found env value "__CF_USER_TEXT_ENCODING" "0x1F5:0x0:0x0"', /Users/travis/build/rust-lang/rust/src/test/run-pass/env-funky-keys.rs:23:13
[01:29:45] 
[01:29:45] ------------------------------------------
[01:29:45] 
[01:29:45] thread '[run-pass] run-pass/env-funky-keys.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3425:9
---
[01:29:45] 
[01:29:45] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:29:45] 
[01:29:45] 
[01:29:45] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib" "--run-lib-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib" "--rustc-path" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/travis/build/rust-lang/rust/src/test/run-pass" "--build-base" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-pass" "--stage-id" "stage2-i686-apple-darwin" "--mode" "run-pass" "--target" "i686-apple-darwin" "--host" "i686-apple-darwin" "--llvm-filecheck" "/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/bin/python2.7" "--lldb-python" "/usr/bin/python" "--lldb-version" "lldb-902.0.73.1\n  Swift-4.1\n" "--lldb-python-dir" "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python" "--llvm-version" "8.0.0-rust-1.35.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:29:45] 
[01:29:45] 
[01:29:45] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:29:45] Build completed unsuccessfully in 0:18:47
[01:29:45] Build completed unsuccessfully in 0:18:47
[01:29:46] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:07f23dd2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 13 16:18:30 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:075404a0
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1880
drwx------  21 travis  staff    714 Apr 13 16:16 .
-rw-------@  1 travis  staff  78234 Apr 13 16:16 a_2019-04-13-161652-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55834 Apr 13 16:16 a_2019-04-13-161652_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55762 Apr 13 16:16 a_2019-04-13-161637-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30292 Apr 13 16:16 a_2019-04-13-161637-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  76453 Apr 13 16:16 a_2019-04-13-161637_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30041 Apr 13 16:16 a_2019-04-13-161627-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30036 Apr 13 16:16 a_2019-04-13-161627-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  29804 Apr 13 16:16 a_2019-04-13-161627_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30174 Apr 13 16:15 a_2019-04-13-161541_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  79121 Apr 13 16:15 a_2019-04-13-161525_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  79974 Apr 13 16:15 a_2019-04-13-161515-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  80386 Apr 13 16:15 a_2019-04-13-161515-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  81242 Apr 13 16:15 a_2019-04-13-161515_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  31748 Apr 13 16:12 a_2019-04-13-161256_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30060 Apr 13 16:12 a_2019-04-13-161208_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30650 Apr 13 16:11 a_2019-04-13-161159-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30654 Apr 13 16:11 a_2019-04-13-161159-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30354 Apr 13 16:11 a_2019-04-13-161159-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  30421 Apr 13 16:11 a_2019-04-13-161159_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0c97fed0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-13-161159-1_Traviss-Mac-1044.crash
Process:               a [40974]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [40962]
Responsible:           a [40974]
User ID:               501
Date/Time:             2019-04-13 16:09:14.770 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-d72d99315c451a4c.dylib  0x0014e1b3 std::panicking::rust_panic_with_hook::h132c3f5bac2c9cae + 115
1   a                              0x00012bff std::panicking::begin_panic::h0e9814b4495c9924 + 47 (panicking.rs:408)
2   a                              0x000106e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 36 (backtrace.rs:24)
3   a                              0x0000ff8b core::ptr::real_drop_in_place::hcb0474b887bab5fa + 11
4   a                              0x000106b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000119c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x0000fbbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb0b7eb89b470acda + 11 (rt.rs:64)
7   libstd-d72d99315c451a4c.dylib  0x0014b14c std::sys_common::backtrace::__rust_begin_short_backtrace::h258c53a673ab9f0e + 12
8   libstd-d72d99315c451a4c.dylib  0x0014dca4 std::panicking::try::do_call::hdeca090a8a640010 + 20
9   libstd-d72d99315c451a4c.dylib  0x0015c3cd __rust_maybe_catch_panic + 29
10  libstd-d72d99315c451a4c.dylib  0x0014e747 std::rt::lang_start_internal::h9bb6eb8f317fc6bd + 631
11  a                              0x0001221c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbfff1468  ebx: 0xbfff14b0  ecx: 0xbfff1360  edx: 0xa7702ec6
  edi: 0x00196008  esi: 0x0014e14e  ebp: 0xbfff1508  esp: 0xbfff1480
   ss: 0x00000023  efl: 0x00210286  eip: 0x0014e1b3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0042dc38
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
    0xd000 -    0x13fff +a (0) <6A646F95-09C2-3D93-971A-8B65D92EC442> /Users/USER/*/a
   0xa3000 -    0xe8fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x12c000 -   0x1bbfff +libstd-d72d99315c451a4c.dylib (0) <DC60EA74-DA2D-3CD0-B0C3-E1E6BA9E45E4> /Users/USER/*/libstd-d72d99315c451a4c.dylib
0x90298000 - 0x90298fff  com.apple.Accelerate (1.11 - Accelerate 1.11) <F4A138F5-290D-3413-AD17-ECD395935FF3> /System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
0x902b0000 - 0x909f1fdf  com.apple.vImage (8.1 - ???) <591C941E-6475-347E-89DA-F541E88F949A> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vImage.framework/Versions/A/vImage
0x909f2000 - 0x90b2dff7  libBLAS.dylib (1211.30.1) <A850E0E2-3A72-3916-9907-AF1E7ECC95F0> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBLAS.dylib
0x90b2e000 - 0x90b5bffb  libBNNS.dylib (37) <C29094A0-5C89-3C5E-AB37-510C28588E2E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBNNS.dylib
0x90b5c000 - 0x90ecffff  libLAPACK.dylib (1211.30.1) <2DDDE838-0FF1-3679-8E62-9C09923ECB7E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLAPACK.dylib
0x90ed0000 - 0x90ee6ffb  libLinearAlgebra.dylib (1211.30.1) <8A120E75-CAF4-3CAE-BBE6-E2F5FAE44DB8> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLinearAlgebra.dylib
0x90ee7000 - 0x90f00ff7  libSparseBLAS.dylib (1211.30.1) <0C5E0EF4-E9A5-3FC4-B7A3-1FE59DB4A2AA> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparseBLAS.dylib
0x90f01000 - 0x9105ffc7  libvDSP.dylib (622.20.8) <C5F16300-061F-3DF0-B91E-8BD0D2173351> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvDSP.dylib
0x91060000 - 0x9113effb  libvMisc.dylib (622.20.8) <1C8D5D80-F32C-3853-8309-57C8A82B7DA5> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvMisc.dylib
0x9113f000 - 0x9113ffff  com.apple.Accelerate.vecLib (3.11 - vecLib 3.11) <7A0D5DD6-C302-390D-9178-0B2EA94BB5ED> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/vecLib
0x92146000 - 0x92146fff  com.apple.ApplicationServices (48 - 50) <BFE7FB45-365B-341F-A8FC-B9483AE87709> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/ApplicationServices
0x92147000 - 0x921adff3  com.apple.ApplicationServices.ATS (377 - 445) <CD3D5685-2BB9-3A7B-AC97-2A74A81CB7CC> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/ATS
0x921b0000 - 0x922d4ff3  libFontParser.dylib (222.1.2) <8F7D388A-299C-3C6D-9864-40EC0914A96B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontParser.dylib
0x922d5000 - 0x92321ffb  libFontRegistry.dylib (221) <8D81FDCF-F05D-3556-AB6D-090F9508C25E> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontRegistry.dylib
0x9240f000 - 0x92414fff  com.apple.ColorSyncLegacy (4.13.0 - 1) <AB5CE7D2-8BE5-35C8-A9D5-ED0FB5BAB7D1> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ColorSyncLegacy.framework/Versions/A/ColorSyncLegacy
0x924be000 - 0x92515ff7  com.apple.HIServices (1.22 - 622) <8544026A-17BE-301D-BA2A-782F3AD864DA> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/HIServices.framework/Versions/A/HIServices
0x92516000 - 0x92525ff7  com.apple.LangAnalysis (1.7.0 - 1.7.0) <E3245701-039B-353F-923D-F81B2242842C> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/LangAnalysis.framework/Versions/A/LangAnalysis
0x92526000 - 0x9257effb  com.apple.print.framework.PrintCore (13 - 503) <FD0F7A18-6F78-34C9-B067-B3AB76C3D4C8> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/PrintCore
0x9257f000 - 0x92615ffb  com.apple.QD (3.12 - 403) <372AFF26-17D1-3C6F-9E47-17C955C2045B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/QD.framework/Versions/A/QD
0x92616000 - 0x92622ff3  com.apple.speech.synthesis.framework (7.4.1 - 7.4.1) <3AE4F801-4A2D-3AB3-AA31-B27F7A7131F4> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/SpeechSynthesis.framework/Versions/A/SpeechSynthesis
0x92623000 - 0x9286ffff  com.apple.audio.toolbox.AudioToolbox (1.14 - 1.14) <E4585EFD-C3B6-327F-88E4-B3BADDA6B08D> /System/Library/Frameworks/AudioToolbox.framework/Versions/A/AudioToolbox
0x92b8f000 - 0x92efcffb  com.apple.CFNetwork (893.13.1 - 893.13.1) <63A5C550-5F0F-3FF1-9061-55E1766B3512> /System/Library/Frameworks/CFNetwork.framework/Versions/A/CFNetwork
0x93426000 - 0x934e5ff7  com.apple.ColorSync (4.13.0 - 546) <DACC5623-8E37-3134-9562-4E8601127F67> /System/Library/Frameworks/ColorSync.framework/Versions/A/ColorSync
0x934e6000 - 0x93581fff  com.apple.audio.CoreAudio (4.3.0 - 4.3.0) <ABA90687-71A6-3431-94A1-0E7E74FE407C> /System/Library/Frameworks/CoreAudio.framework/Versions/A/CoreAudio
0x935e5000 - 0x938c5fff  com.apple.CoreData (120 - 849.2) <B8011F5E-7A2B-349B-AFF1-17EC8D8465AB> /System/Library/Frameworks/CoreData.framework/Versions/A/CoreData
0x938c6000 - 0x938ccff3  com.apple.CoreDisplay (1.0 - 81.7) <E6BFD0F5-A45B-3FE3-BA26-14D2CD970CFF> /System/Library/Frameworks/CoreDisplay.framework/Versions/A/CoreDisplay
0x938cd000 - 0x93d56ff7  com.apple.CoreFoundation (6.9 - 1451) <727B43E3-A1AC-31EC-97A4-F179FE11D04A> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
0x93d58000 - 0x94387ffb  com.apple.CoreGraphics (2.0 - 1129.5) <20752785-E9DA-3CC6-ACDD-5A82AD344209> /System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics
0x94389000 - 0x945ffffb  com.apple.CoreImage (13.0.0 - 579.2.9) <2498F44C-7350-397B-A075-206A91D75ABB> /System/Library/Frameworks/CoreImage.framework/Versions/A/CoreImage
0x947f1000 - 0x947f1fff  com.apple.CoreServices (822.19 - 822.19) <6B5DC8C1-4237-3ADA-B8C8-F926943E6101> /System/Library/Frameworks/CoreServices.framework/Versions/A/CoreServices
0x947f2000 - 0x94864ff3  com.apple.AE (735.1 - 735.1) <3E1B0CED-0AC3-3252-AEDD-5D8F91E3AAA7> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/AE.framework/Versions/A/AE
0x94865000 - 0x94b43ffb  com.apple.CoreServices.CarbonCore (1178.2 - 1178.2) <E61EA71D-294F-3C8B-95BD-0CDBA0FFC907> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/CarbonCore
0x94b44000 - 0x94b78ff3  com.apple.DictionaryServices (1.2 - 284) <56BEF6B8-50D2-38A0-9EF2-D7093E9AAB56> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/DictionaryServices.framework/Versions/A/DictionaryServices
0x94b79000 - 0x94b81fff  com.apple.CoreServices.FSEvents (1239 - 1239) <CABC21F7-E3AB-3954-ACBE-B8066A37516A> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/FSEvents
0x94b82000 - 0x94ce0fff  com.apple.LaunchServices (822.19 - 822.19) <AC40752F-0F99-3EB1-8D25-2C65F9BAC226> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/LaunchServices
0x94ce1000 - 0x94d8dff7  com.apple.Metadata (10.7.0 - 1191.2.6) <6CE69880-6AF3-3A1A-A8E0-F89FC750EE4C> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Metadata
0x94d8e000 - 0x94decff7  com.apple.CoreServices.OSServices (822.19 - 822.19) <38B64F72-5CAE-374A-8BBC-0EAB7C6A6777> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/OSServices.framework/Versions/A/OSServices
0x94ded000 - 0x94e5eff3  com.apple.SearchKit (1.4.0 - 1.4.0) <FAD60011-970B-3889-B6BD-3715CCF599CA> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SearchKit.framework/Versions/A/SearchKit
0x94e5f000 - 0x94e82fff  com.apple.coreservices.SharedFileList (71.4 - 71.4) <CD97E31D-0354-36AB-9997-C4FAF3221D30> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SharedFileList.framework/Versions/A/SharedFileList
0x94e83000 - 0x94fceffb  com.apple.CoreText (352.0 - 578.12) <6389222B-6B26-3F46-93C2-ABE07168227F> /System/Library/Frameworks/CoreText.framework/Versions/A/CoreText
0x94fcf000 - 0x95009ff3  com.apple.CoreVideo (1.8 - 279.2) <0D75C395-3C86-3539-9206-C7A330BE3551> /System/Library/Frameworks/CoreVideo.framework/Versions/A/CoreVideo
0x952e4000 - 0x952edff7  com.apple.DiskArbitration (2.7 - 2.7) <E3552A79-57A4-36AE-8B54-5FE2EB5193DA> /System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration
0x952fe000 - 0x9566dffb  com.apple.Foundation (6.9 - 1451) <E815D5AF-B627-3BF4-8156-4F514FCFD765> /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
0x956ae000 - 0x956ddff3  com.apple.GSS (4.0 - 2.0) <78C94D11-21DF-34C6-B4E8-88564551D67E> /System/Library/Frameworks/GSS.framework/Versions/A/GSS
0x95888000 - 0x95929ffb  com.apple.framework.IOKit (2.0.2 - 1445.40.1) <EDA5B2F5-12B4-39EF-B5EF-899587AACDC5> /System/Library/Frameworks/IOKit.framework/Versions/A/IOKit
0x9592b000 - 0x95932fff  com.apple.IOSurface (209.2.2 - 209.2.2) <0CCA9904-FCBB-3278-96A1-714BDB961D8A> /System/Library/Frameworks/IOSurface.framework/Versions/A/IOSurface
0x95987000 - 0x95b0bff3  com.apple.ImageIO.framework (3.3.0 - 1713) <28D21D61-3526-33ED-92DC-08D4D67445CB> /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
0x95b0c000 - 0x95b10ffb  libGIF.dylib (1713) <47A6BFCC-6651-3AAE-A70C-7BA3717B13BB> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libGIF.dylib
0x95b11000 - 0x95c02fff  libJP2.dylib (1713) <D1075C88-406B-3EAF-9270-9B3762D97803> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJP2.dylib
0x95c03000 - 0x95c25ff7  libJPEG.dylib (1713) <EFD86068-C17E-32AD-B4A5-79C20BC93411> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJPEG.dylib
0x95f06000 - 0x95f2cff7  libPng.dylib (1713) <A10259A1-2581-3642-946D-5B3F101615EC> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libPng.dylib
0x95f2d000 - 0x95f2fffb  libRadiance.dylib (1713) <EE872547-4C9F-397B-B41B-C79FEEE68267> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libRadiance.dylib
0x95f30000 - 0x95f7afff  libTIFF.dylib (1713) <7A56E3C4-9965-3471-8E98-5F6B28D68FBF> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libTIFF.dylib
0x96897000 - 0x968affff  com.apple.Kerberos (3.0 - 1) <8A399DB7-5440-3EC0-A241-3DD10E82DDB2> /System/Library/Frameworks/Kerberos.framework/Versions/A/Kerberos
0x96f27000 - 0x96f9dfff  com.apple.Metal (124.7 - 124.7) <2617CDD0-32C6-358C-A61F-063737F916B3> /System/Library/Frameworks/Metal.framework/Versions/A/Metal
0x96f9f000 - 0x96fabfff  com.apple.NetFS (6.0 - 4.0) <F37A4DA0-AAB6-3F0B-BA18-E322BFA52CC4> /System/Library/Frameworks/NetFS.framework/Versions/A/NetFS
0x99c78000 - 0x99cc3fff  com.apple.opencl (2.8.12 - 2.8.12) <9AC78C72-CBBC-3D29-B553-AACC28014AEC> /System/Library/Frameworks/OpenCL.framework/Versions/A/OpenCL
0x99cc4000 - 0x99ce0fff  com.apple.CFOpenDirectory (10.13 - 207) <255284C6-7BDD-3A0B-A4A2-E43206611B91> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/Frameworks/CFOpenDirectory.framework/Versions/A/CFOpenDirectory
0x99ce1000 - 0x99cecfff  com.apple.OpenDirectory (10.13 - 207) <ECB33DA6-A0A3-3378-AB7A-2C10D395904E> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/OpenDirectory
0x9aef6000 - 0x9aef7fff  libCVMSPluginSupport.dylib (16.4.2) <909D788E-692E-3FF1-AFF0-2AB4609C53D7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCVMSPluginSupport.dylib
0x9aef8000 - 0x9aefcfff  libCoreFSCache.dylib (162.4) <D53B0D41-0774-3C6A-BB59-8BA7DB8A8374> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreFSCache.dylib
0x9aefd000 - 0x9af01fff  libCoreVMClient.dylib (162.4) <19767FEB-6A89-3892-8F18-1F9E73463050> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreVMClient.dylib
0x9af02000 - 0x9af0aff7  libGFXShared.dylib (16.4.2) <F62281F1-9495-3589-A576-75D84880D42D> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGFXShared.dylib
0x9af0b000 - 0x9af17fff  libGL.dylib (16.4.2) <028B909B-DD19-388B-8113-1850DFAD3DCA> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib
0x9af18000 - 0x9af53ffb  libGLImage.dylib (16.4.2) <AE5E3974-656A-3F88-956E-F28192BA98C3> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLImage.dylib
0x9b0cd000 - 0x9b10fff7  libGLU.dylib (16.4.2) <9E1283AA-A7E0-37BA-BDEB-EE5256D677C7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLU.dylib
0x9bab6000 - 0x9bac4fff  com.apple.opengl (16.4.2 - 16.4.2) <40645026-52DC-3CFC-8308-EFA2FA79D5A0> /System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
0x9c829000 - 0x9ca5fff7  com.apple.QuartzCore (1.11 - 584.8.102) <960628B2-C498-36C9-B0D4-F27D49DE029F> /System/Library/Frameworks/QuartzCore.framework/Versions/A/QuartzCore
0x9cef8000 - 0x9d225ff3  com.apple.security (7.0 - 58286.41.2) <F64D64CD-4E80-3384-92F0-56D2A464F7B4> /System/Library/Frameworks/Security.framework/Versions/A/Security
0x9d226000 - 0x9d2adff3  com.apple.securityfoundation (6.0 - 55185.30.4) <632548B9-3E24-32F4-BF50-F6BF71713363> /System/Library/Frameworks/SecurityFoundation.framework/Versions/A/SecurityFoundation
0x9d2d9000 - 0x9d2ddfff  com.apple.xpc.ServiceManagement (1.0 - 1) <B09C1309-46A2-3081-B489-DCE549A8BA46> /System/Library/Frameworks/ServiceManagement.framework/Versions/A/ServiceManagement
0x9d408000 - 0x9d478ff3  com.apple.SystemConfiguration (1.17 - 1.17) <318C287A-BB41-3F72-9095-9DFF0382CB77> /System/Library/Frameworks/SystemConfiguration.framework/Versions/A/SystemConfiguration
0x9f26e000 - 0x9f305ff7  com.apple.APFS (1.0 - 1) <1B119C37-9A4C-3204-9BE2-E4E00E657037> /System/Library/PrivateFrameworks/APFS.framework/Versions/A/APFS
0x9fa4b000 - 0x9fa88ff3  com.apple.AppleJPEG (1.0 - 1) <E87393BB-6140-389C-BF53-36B3985A56D3> /System/Library/PrivateFrameworks/AppleJPEG.framework/Versions/A/AppleJPEG
0x9fbdb000 - 0x9fbe2fff  com.apple.coreservices.BackgroundTaskManagement (1.0 - 57.1) <71FD5EA2-8D0B-3E21-94E4-6D5BD45005E7> /System/Library/PrivateFrameworks/BackgroundTaskManagement.framework/Versions/A/BackgroundTaskManagement
0x9fdb2000 - 0x9fdbbffb  com.apple.CommonAuth (4.0 - 2.0) <424B8D39-396A-3A78-84C1-5161B54A8F5B> /System/Library/PrivateFrameworks/CommonAuth.framework/Versions/A/CommonAuth
0xa022d000 - 0xa023dff7  com.apple.CoreEmoji (1.0 - 69.3) <165A133F-DED4-3B24-A9BF-6EA6F3F7A152> /System/Library/PrivateFrameworks/CoreEmoji.framework/Versions/A/CoreEmoji
0xa0c96000 - 0xa10c6ff7  com.apple.vision.FaceCore (3.3.2 - 3.3.2) <B2288C3D-E67F-3AAE-A652-E920CD19F267> /System/Library/PrivateFrameworks/FaceCore.framework/Versions/A/FaceCore
0xa3a6a000 - 0xa3addff3  com.apple.Heimdal (4.0 - 2.0) <560C8A98-E261-39C9-9862-3340EC6ABC9C> /System/Library/PrivateFrameworks/Heimdal.framework/Versions/A/Heimdal
0xa3d8f000 - 0xa3d95fff  com.apple.IOAccelerator (376.6 - 376.6) <4EFED596-8863-35F6-8EA3-CB11C5D9D157> /System/Library/PrivateFrameworks/IOAccelerator.framework/Versions/A/IOAccelerator
0xa3d96000 - 0xa3daeffb  com.apple.IOPresentment (1.0 - 32.1) <EBD4DB8D-03D3-3136-B431-969A2E5E1B91> /System/Library/PrivateFrameworks/IOPresentment.framework/Versions/A/IOPresentment
0xa3e62000 - 0xa3f56ff7  com.apple.LanguageModeling (1.0 - 159.3.1) <AA880B14-031D-33FB-9B48-0A8AAB7342C6> /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
0xa3f57000 - 0xa3f98ff7  com.apple.Lexicon-framework (1.0 - 33.2) <13FAB8A2-507A-3AEA-A571-27BDDFD96B31> /System/Library/PrivateFrameworks/Lexicon.framework/Versions/A/Lexicon
0xa3f9c000 - 0xa3fa2ff3  com.apple.LinguisticData (1.0 - 238.3) <C47B3EB0-0463-3613-8A09-344F1639EE92> /System/Library/PrivateFrameworks/LinguisticData.framework/Versions/A/LinguisticData
0xa4352000 - 0xa437bffb  com.apple.MultitouchSupport.framework (1204.13 - 1204.13) <01BDF9A5-8C83-3611-A999-F43F9121A173> /System/Library/PrivateFrameworks/MultitouchSupport.framework/Versions/A/MultitouchSupport
0xa449a000 - 0xa44a4fff  com.apple.NetAuth (6.2 - 6.2) <52F67DC1-8C96-3944-8E54-C02DD51FD9FC> /System/Library/PrivateFrameworks/NetAuth.framework/Versions/A/NetAuth
0xa4858000 - 0xa48ddffb  com.apple.SkyLight (1.600.0 - 312.23.4) <27154D6B-3C0F-383F-AA7B-F602C1F397CC> /System/Library/PrivateFrameworks/SkyLight.framework/Versions/A/SkyLight
0xa4cbb000 - 0xa4cc2fff  com.apple.TCC (1.0 - 1) <4B76752A-36A0-3175-87C7-CB42E33CCB5A> /System/Library/PrivateFrameworks/TCC.framework/Versions/A/TCC
0xa545c000 - 0xa545efff  com.apple.loginsupport (1.0 - 1) <086FAE1B-87E2-324A-AE76-E6EC0B5F1517> /System/Library/PrivateFrameworks/login.framework/Versions/A/Frameworks/loginsupport.framework/Versions/A/loginsupport
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa556c000 - 0xa55a3ff3  libCRFSuite.dylib (41) <7F584902-74F1-3362-935D-95F5E735F5E7> /usr/lib/libCRFSuite.dylib
0xa55a4000 - 0xa55aeffb  libChineseTokenizer.dylib (28) <1FF5A32D-E012-3E76-B738-FAC26AD2A39B> /usr/lib/libChineseTokenizer.dylib
0xa564a000 - 0xa564bfff  libDiagnosticMessagesClient.dylib (104) <6829B180-2556-3A7E-A2E6-BD4859DF30A7> /usr/lib/libDiagnosticMessagesClient.dylib
0xa567d000 - 0xa5867ff7  libFosl_dynamic.dylib (17.7) <DBE4D720-8A46-3879-AD2D-F9A8CE3E7476> /usr/lib/libFosl_dynamic.dylib
0xa586f000 - 0xa586ffff  libOpenScriptingUtil.dylib (174) <B7CEDC30-2D17-3896-9EFC-64DB3D11DF00> /usr/lib/libOpenScriptingUtil.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa58ce000 - 0xa58e3ff7  libapple_nghttp2.dylib (1.24) <480C0C04-2533-3D44-8232-006B6CBA7758> /usr/lib/libapple_nghttp2.dylib
0xa58e4000 - 0xa590ffff  libarchive.2.dylib (54) <D55C5F86-251D-3C33-A617-0C623D4F512E> /usr/lib/libarchive.2.dylib
0xa5a63000 - 0xa5a63ff3  libauto.dylib (187) <CE2A78CC-670F-3E07-9539-822DCD2F6084> /usr/lib/libauto.dylib
0xa5a64000 - 0xa5a74fff  libbsm.0.dylib (39) <067E9003-0673-32A3-9B40-492323182C5C> /usr/lib/libbsm.0.dylib
0xa5a75000 - 0xa5a81ff7  libbz2.1.0.dylib (38) <77C24A36-BE84-3702-A786-935C597A0A86> /usr/lib/libbz2.1.0.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa5aff000 - 0xa5b10ff7  libcmph.dylib (6) <EC7664F1-B5A1-37F4-B7DC-F6AC10587E35> /usr/lib/libcmph.dylib
0xa5b11000 - 0xa5b24ff7  libcompression.dylib (47) <F80DDFC1-F96A-3BAD-967D-C1E24253273A> /usr/lib/libcompression.dylib
0xa5b25000 - 0xa5b3cffb  libcoretls.dylib (155) <F66FAEBC-4B6E-31E0-ACA8-C8ACBC7689DD> /usr/lib/libcoretls.dylib
0xa5b3d000 - 0xa5b3efff  libcoretls_cfhelpers.dylib (155) <8B8ABC2C-F251-3C80-9747-88C05A2CBE64> /usr/lib/libcoretls_cfhelpers.dylib
0xa6026000 - 0xa607dfff  libcups.2.dylib (462.1) <0180AE97-A19F-3D49-9838-06995E73F572> /usr/lib/libcups.2.dylib
0xa6193000 - 0xa6193fff  libenergytrace.dylib (16) <34FC43C7-D9B6-3C01-8B65-E49059D31279> /usr/lib/libenergytrace.dylib
0xa61c7000 - 0xa61cbfff  libheimdal-asn1.dylib (520.30.1) <DEA7E913-118F-333E-BE08-5B4F19B33B9A> /usr/lib/libheimdal-asn1.dylib
0xa61f7000 - 0xa62e7ff3  libiconv.2.dylib (51) <FE6D05A5-18DB-3FD8-A52F-B7BADB232C78> /usr/lib/libiconv.2.dylib
0xa62e8000 - 0xa650aff7  libicucore.A.dylib (59152.0.1) <35D52BFF-C74C-3519-AEAC-7371E3C7E4BD> /usr/lib/libicucore.A.dylib
0xa6552000 - 0xa6553fff  liblangid.dylib (128) <120FE992-47E4-3A73-A039-1B401F5696DC> /usr/lib/liblangid.dylib
0xa6554000 - 0xa656cff7  liblzma.5.dylib (10) <8A5C9679-430A-3A19-AF68-9D21BAC442C7> /usr/lib/liblzma.5.dylib
0xa656d000 - 0xa6582fff  libmarisa.dylib (9) <805453EE-B829-3DA5-8DF3-5132D03D5B74> /usr/lib/libmarisa.dylib
0xa6637000 - 0xa6854fff  libmecabra.dylib (779.7.6) <3C7F6A43-B17C-3673-A0AC-14FFE08370D4> /usr/lib/libmecabra.dylib
0xa6a1b000 - 0xa6aeffff  libnetwork.dylib (1229.30.11) <E4008EDE-F873-33FF-BD96-7DB14FA4F364> /usr/lib/libnetwork.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6ed4000 - 0xa6ed7fff  libpam.2.dylib (22) <7106F43C-84DD-3F26-905A-B52780AFEB3E> /usr/lib/libpam.2.dylib
0xa6eda000 - 0xa6f0bfff  libpcap.A.dylib (79.20.1) <154889CF-5F83-3012-953E-0FC8FEE50FF8> /usr/lib/libpcap.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa6faf000 - 0xa7139ff7  libsqlite3.dylib (274.5) <B09AF63F-4F1A-3481-9B61-6EBB64D12EB9> /usr/lib/libsqlite3.dylib
0xa72dd000 - 0xa7317ffb  libusrtcp.dylib (1229.30.11) <39D76669-A48B-3BAC-8F45-1D6CA87E9B4B> /usr/lib/libusrtcp.dylib
0xa7318000 - 0xa731bff7  libutil.dylib (51.20.1) <86BD9675-16A2-345D-9B8D-E8A3397F2365> /usr/lib/libutil.dylib
0xa731c000 - 0xa732aff7  libxar.1.dylib (400) <4B664A7E-EC05-34AD-ACC6-C879B69DBA7C> /usr/lib/libxar.1.dylib
0xa732b000 - 0xa7409ff7  libxml2.2.dylib (31.7) <3E1F9E3D-6C44-3437-AB2B-E5ACE1927F81> /usr/lib/libxml2.2.dylib
0xa740a000 - 0xa7432ff3  libxslt.1.dylib (15.10) <1A3DC7B8-7C92-3D73-BF82-D60E64BC3DF0> /usr/lib/libxslt.1.dylib
0xa7433000 - 0xa7442ff7  libz.1.dylib (70) <588F445F-0065-3D77-8002-BA9411DA1D70> /usr/lib/libz.1.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75c4000 - 0xa75d0ff7  libkxld.dylib (4570.41.2) <C01D2E6F-B29E-3795-9258-55445BF8F933> /usr/lib/system/libkxld.dylib
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
    task_for_pid: 2358
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=168.2M resident=0K(0%) swapped_out_or_unallocated=168.2M(100%)
Writable regions: Total=73.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9636K       14 
MALLOC guard page                   32K        9 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            9332K      164 
__FONT_DATA                          4K        2 
__LINKEDIT                        74.1M        5 
__OBJC                            1252K       46 
__TEXT                            94.2M      167 
mapped file                      316.4M      145 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.1M      553 
TOTAL                            569.1M      553 
TOTAL, minus reserved VM space   568.9M      553 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-13-161159-2_Traviss-Mac-1044.crash
Process:               a [40975]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [40962]
Responsible:           a [40975]
User ID:               501
Date/Time:             2019-04-13 16:09:14.980 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4900 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-d72d99315c451a4c.dylib  0x0011e1b3 std::panicking::rust_panic_with_hook::h132c3f5bac2c9cae + 115
1   a                              0x0003dbff std::panicking::begin_panic::h0e9814b4495c9924 + 47 (panicking.rs:408)
2   a                              0x0003b6e4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 36 (backtrace.rs:24)
3   a                              0x0003af8b core::ptr::real_drop_in_place::hcb0474b887bab5fa + 11
4   a                              0x0003b6b3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x0003c9c8 backtrace::main::hcde7a1a1c3c85e77 + 4568 (backtrace.rs:103)
6   a                              0x0003abbb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb0b7eb89b470acda + 11 (rt.rs:64)
7   libstd-d72d99315c451a4c.dylib  0x0011b14c std::sys_common::backtrace::__rust_begin_short_backtrace::h258c53a673ab9f0e + 12
8   libstd-d72d99315c451a4c.dylib  0x0011dca4 std::panicking::try::do_call::hdeca090a8a640010 + 20
9   libstd-d72d99315c451a4c.dylib  0x0012c3cd __rust_maybe_catch_panic + 29
10  libstd-d72d99315c451a4c.dylib  0x0011e747 std::rt::lang_start_internal::h9bb6eb8f317fc6bd + 631
11  a                              0x0003d21c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbffc6458  ebx: 0xbffc64a0  ecx: 0xbffc6350  edx: 0xa7702ec6
  edi: 0x00166008  esi: 0x0011e14e  ebp: 0xbffc64f8  esp: 0xbffc6470
   ss: 0x00000023  efl: 0x00210282  eip: 0x0011e1b3   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0468f730
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x38000 -    0x3efff +a (0) <6A646F95-09C2-3D93-971A-8B65D92EC442> /Users/USER/*/a
   0x73000 -    0xb8fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
   0xfc000 -   0x18bfff +libstd-d72d99315c451a4c.dylib (0) <DC60EA74-DA2D-3CD0-B0C3-E1E6BA9E45E4> /Users/USER/*/libstd-d72d99315c451a4c.dylib
0x90298000 - 0x90298fff  com.apple.Accelerate (1.11 - Accelerate 1.11) <F4A138F5-290D-3413-AD17-ECD395935FF3> /System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
0x902b0000 - 0x909f1fdf  com.apple.vImage (8.1 - ???) <591C941E-6475-347E-89DA-F541E88F949A> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vImage.framework/Versions/A/vImage
0x909f2000 - 0x90b2dff7  libBLAS.dylib (1211.30.1) <A850E0E2-3A72-3916-9907-AF1E7ECC95F0> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBLAS.dylib
0x90b2e000 - 0x90b5bffb  libBNNS.dylib (37) <C29094A0-5C89-3C5E-AB37-510C28588E2E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBNNS.dylib
0x90b5c000 - 0x90ecffff  libLAPACK.dylib (1211.30.1) <2DDDE838-0FF1-3679-8E62-9C09923ECB7E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLAPACK.dylib
0x90ed0000 - 0x90ee6ffb  libLinearAlgebra.dylib (1211.30.1) <8A120E75-CAF4-3CAE-BBE6-E2F5FAE44DB8> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLinearAlgebra.dylib
0x90ee7000 - 0x90f00ff7  libSparseBLAS.dylib (1211.30.1) <0C5E0EF4-E9A5-3FC4-B7A3-1FE59DB4A2AA> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparseBLAS.dylib
0x90f01000 - 0x9105ffc7  libvDSP.dylib (622.20.8) <C5F16300-061F-3DF0-B91E-8BD0D2173351> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvDSP.dylib
0x91060000 - 0x9113effb  libvMisc.dylib (622.20.8) <1C8D5D80-F32C-3853-8309-57C8A82B7DA5> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvMisc.dylib
0x9113f000 - 0x9113ffff  com.apple.Accelerate.vecLib (3.11 - vecLib 3.11) <7A0D5DD6-C302-390D-9178-0B2EA94BB5ED> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/vecLib
0x92146000 - 0x92146fff  com.apple.ApplicationServices (48 - 50) <BFE7FB45-365B-341F-A8FC-B9483AE87709> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/ApplicationServices
0x92147000 - 0x921adff3  com.apple.ApplicationServices.ATS (377 - 445) <CD3D5685-2BB9-3A7B-AC97-2A74A81CB7CC> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/ATS
0x921b0000 - 0x922d4ff3  libFontParser.dylib (222.1.2) <8F7D388A-299C-3C6D-9864-40EC0914A96B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontParser.dylib
0x922d5000 - 0x92321ffb  libFontRegistry.dylib (221) <8D81FDCF-F05D-3556-AB6D-090F9508C25E> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontRegistry.dylib
0x9240f000 - 0x92414fff  com.apple.ColorSyncLegacy (4.13.0 - 1) <AB5CE7D2-8BE5-35C8-A9D5-ED0FB5BAB7D1> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ColorSyncLegacy.framework/Versions/A/ColorSyncLegacy
0x924be000 - 0x92515ff7  com.apple.HIServices (1.22 - 622) <8544026A-17BE-301D-BA2A-782F3AD864DA> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/HIServices.framework/Versions/A/HIServices
0x92516000 - 0x92525ff7  com.apple.LangAnalysis (1.7.0 - 1.7.0) <E3245701-039B-353F-923D-F81B2242842C> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/LangAnalysis.framework/Versions/A/LangAnalysis
0x92526000 - 0x9257effb  com.apple.print.framework.PrintCore (13 - 503) <FD0F7A18-6F78-34C9-B067-B3AB76C3D4C8> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/PrintCore
0x9257f000 - 0x92615ffb  com.apple.QD (3.12 - 403) <372AFF26-17D1-3C6F-9E47-17C955C2045B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/QD.framework/Versions/A/QD
0x92616000 - 0x92622ff3  com.apple.speech.synthesis.framework (7.4.1 - 7.4.1) <3AE4F801-4A2D-3AB3-AA31-B27F7A7131F4> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/SpeechSynthesis.framework/Versions/A/SpeechSynthesis
0x92623000 - 0x9286ffff  com.apple.audio.toolbox.AudioToolbox (1.14 - 1.14) <E4585EFD-C3B6-327F-88E4-B3BADDA6B08D> /System/Library/Frameworks/AudioToolbox.framework/Versions/A/AudioToolbox
0x92b8f000 - 0x92efcffb  com.apple.CFNetwork (893.13.1 - 893.13.1) <63A5C550-5F0F-3FF1-9061-55E1766B3512> /System/Library/Frameworks/CFNetwork.framework/Versions/A/CFNetwork
0x93426000 - 0x934e5ff7  com.apple.ColorSync (4.13.0 - 546) <DACC5623-8E37-3134-9562-4E8601127F67> /System/Library/Frameworks/ColorSync.framework/Versions/A/ColorSync
0x934e6000 - 0x93581fff  com.apple.audio.CoreAudio (4.3.0 - 4.3.0) <ABA90687-71A6-3431-94A1-0E7E74FE407C> /System/Library/Frameworks/CoreAudio.framework/Versions/A/CoreAudio
0x935e5000 - 0x938c5fff  com.apple.CoreData (120 - 849.2) <B8011F5E-7A2B-349B-AFF1-17EC8D8465AB> /System/Library/Frameworks/CoreData.framework/Versions/A/CoreData
0x938c6000 - 0x938ccff3  com.apple.CoreDisplay (1.0 - 81.7) <E6BFD0F5-A45B-3FE3-BA26-14D2CD970CFF> /System/Library/Frameworks/CoreDisplay.framework/Versions/A/CoreDisplay
0x938cd000 - 0x93d56ff7  com.apple.CoreFoundation (6.9 - 1451) <727B43E3-A1AC-31EC-97A4-F179FE11D04A> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
0x93d58000 - 0x94387ffb  com.apple.CoreGraphics (2.0 - 1129.5) <20752785-E9DA-3CC6-ACDD-5A82AD344209> /System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics
0x94389000 - 0x945ffffb  com.apple.CoreImage (13.0.0 - 579.2.9) <2498F44C-7350-397B-A075-206A91D75ABB> /System/Library/Frameworks/CoreImage.framework/Versions/A/CoreImage
0x947f1000 - 0x947f1fff  com.apple.CoreServices (822.19 - 822.19) <6B5DC8C1-4237-3ADA-B8C8-F926943E6101> /System/Library/Frameworks/CoreServices.framework/Versions/A/CoreServices
0x947f2000 - 0x94864ff3  com.apple.AE (735.1 - 735.1) <3E1B0CED-0AC3-3252-AEDD-5D8F91E3AAA7> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/AE.framework/Versions/A/AE
0x94865000 - 0x94b43ffb  com.apple.CoreServices.CarbonCore (1178.2 - 1178.2) <E61EA71D-294F-3C8B-95BD-0CDBA0FFC907> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/CarbonCore
0x94b44000 - 0x94b78ff3  com.apple.DictionaryServices (1.2 - 284) <56BEF6B8-50D2-38A0-9EF2-D7093E9AAB56> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/DictionaryServices.framework/Versions/A/DictionaryServices
0x94b79000 - 0x94b81fff  com.apple.CoreServices.FSEvents (1239 - 1239) <CABC21F7-E3AB-3954-ACBE-B8066A37516A> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/FSEvents
0x94b82000 - 0x94ce0fff  com.apple.LaunchServices (822.19 - 822.19) <AC40752F-0F99-3EB1-8D25-2C65F9BAC226> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/LaunchServices
0x94ce1000 - 0x94d8dff7  com.apple.Metadata (10.7.0 - 1191.2.6) <6CE69880-6AF3-3A1A-A8E0-F89FC750EE4C> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Metadata
0x94d8e000 - 0x94decff7  com.apple.CoreServices.OSServices (822.19 - 822.19) <38B64F72-5CAE-374A-8BBC-0EAB7C6A6777> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/OSServices.framework/Versions/A/OSServices
0x94ded000 - 0x94e5eff3  com.apple.SearchKit (1.4.0 - 1.4.0) <FAD60011-970B-3889-B6BD-3715CCF599CA> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SearchKit.framework/Versions/A/SearchKit
0x94e5f000 - 0x94e82fff  com.apple.coreservices.SharedFileList (71.4 - 71.4) <CD97E31D-0354-36AB-9997-C4FAF3221D30> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SharedFileList.framework/Versions/A/SharedFileList
0x94e83000 - 0x94fceffb  com.apple.CoreText (352.0 - 578.12) <6389222B-6B26-3F46-93C2-ABE07168227F> /System/Library/Frameworks/CoreText.framework/Versions/A/CoreText
0x94fcf000 - 0x95009ff3  com.apple.CoreVideo (1.8 - 279.2) <0D75C395-3C86-3539-9206-C7A330BE3551> /System/Library/Frameworks/CoreVideo.framework/Versions/A/CoreVideo
0x952e4000 - 0x952edff7  com.apple.DiskArbitration (2.7 - 2.7) <E3552A79-57A4-36AE-8B54-5FE2EB5193DA> /System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration
0x952fe000 - 0x9566dffb  com.apple.Foundation (6.9 - 1451) <E815D5AF-B627-3BF4-8156-4F514FCFD765> /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
0x956ae000 - 0x956ddff3  com.apple.GSS (4.0 - 2.0) <78C94D11-21DF-34C6-B4E8-88564551D67E> /System/Library/Frameworks/GSS.framework/Versions/A/GSS
0x95888000 - 0x95929ffb  com.apple.framework.IOKit (2.0.2 - 1445.40.1) <EDA5B2F5-12B4-39EF-B5EF-899587AACDC5> /System/Library/Frameworks/IOKit.framework/Versions/A/IOKit
0x9592b000 - 0x95932fff  com.apple.IOSurface (209.2.2 - 209.2.2) <0CCA9904-FCBB-3278-96A1-714BDB961D8A> /System/Library/Frameworks/IOSurface.framework/Versions/A/IOSurface
0x95987000 - 0x95b0bff3  com.apple.ImageIO.framework (3.3.0 - 1713) <28D21D61-3526-33ED-92DC-08D4D67445CB> /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
0x95b0c000 - 0x95b10ffb  libGIF.dylib (1713) <47A6BFCC-6651-3AAE-A70C-7BA3717B13BB> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libGIF.dylib
0x95b11000 - 0x95c02fff  libJP2.dylib (1713) <D1075C88-406B-3EAF-9270-9B3762D97803> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJP2.dylib
0x95c03000 - 0x95c25ff7  libJPEG.dylib (1713) <EFD86068-C17E-32AD-B4A5-79C20BC93411> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJPEG.dylib
0x95f06000 - 0x95f2cff7  libPng.dylib (1713) <A10259A1-2581-3642-946D-5B3F101615EC> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libPng.dylib
0x95f2d000 - 0x95f2fffb  libRadiance.dylib (1713) <EE872547-4C9F-397B-B41B-C79FEEE68267> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libRadiance.dylib
0x95f30000 - 0x95f7afff  libTIFF.dylib (1713) <7A56E3C4-9965-3471-8E98-5F6B28D68FBF> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libTIFF.dylib
0x96897000 - 0x968affff  com.apple.Kerberos (3.0 - 1) <8A399DB7-5440-3EC0-A241-3DD10E82DDB2> /System/Library/Frameworks/Kerberos.framework/Versions/A/Kerberos
0x96f27000 - 0x96f9dfff  com.apple.Metal (124.7 - 124.7) <2617CDD0-32C6-358C-A61F-063737F916B3> /System/Library/Frameworks/Metal.framework/Versions/A/Metal
0x96f9f000 - 0x96fabfff  com.apple.NetFS (6.0 - 4.0) <F37A4DA0-AAB6-3F0B-BA18-E322BFA52CC4> /System/Library/Frameworks/NetFS.framework/Versions/A/NetFS
0x99c78000 - 0x99cc3fff  com.apple.opencl (2.8.12 - 2.8.12) <9AC78C72-CBBC-3D29-B553-AACC28014AEC> /System/Library/Frameworks/OpenCL.framework/Versions/A/OpenCL
0x99cc4000 - 0x99ce0fff  com.apple.CFOpenDirectory (10.13 - 207) <255284C6-7BDD-3A0B-A4A2-E43206611B91> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/Frameworks/CFOpenDirectory.framework/Versions/A/CFOpenDirectory
0x99ce1000 - 0x99cecfff  com.apple.OpenDirectory (10.13 - 207) <ECB33DA6-A0A3-3378-AB7A-2C10D395904E> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/OpenDirectory
0x9aef6000 - 0x9aef7fff  libCVMSPluginSupport.dylib (16.4.2) <909D788E-692E-3FF1-AFF0-2AB4609C53D7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCVMSPluginSupport.dylib
0x9aef8000 - 0x9aefcfff  libCoreFSCache.dylib (162.4) <D53B0D41-0774-3C6A-BB59-8BA7DB8A8374> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreFSCache.dylib
0x9aefd000 - 0x9af01fff  libCoreVMClient.dylib (162.4) <19767FEB-6A89-3892-8F18-1F9E73463050> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreVMClient.dylib
0x9af02000 - 0x9af0aff7  libGFXShared.dylib (16.4.2) <F62281F1-9495-3589-A576-75D84880D42D> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGFXShared.dylib
0x9af0b000 - 0x9af17fff  libGL.dylib (16.4.2) <028B909B-DD19-388B-8113-1850DFAD3DCA> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib
0x9af18000 - 0x9af53ffb  libGLImage.dylib (16.4.2) <AE5E3974-656A-3F88-956E-F28192BA98C3> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLImage.dylib
0x9b0cd000 - 0x9b10fff7  libGLU.dylib (16.4.2) <9E1283AA-A7E0-37BA-BDEB-EE5256D677C7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLU.dylib
0x9bab6000 - 0x9bac4fff  com.apple.opengl (16.4.2 - 16.4.2) <40645026-52DC-3CFC-8308-EFA2FA79D5A0> /System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
0x9c829000 - 0x9ca5fff7  com.apple.QuartzCore (1.11 - 584.8.102) <960628B2-C498-36C9-B0D4-F27D49DE029F> /System/Library/Frameworks/QuartzCore.framework/Versions/A/QuartzCore
0x9cef8000 - 0x9d225ff3  com.apple.security (7.0 - 58286.41.2) <F64D64CD-4E80-3384-92F0-56D2A464F7B4> /System/Library/Frameworks/Security.framework/Versions/A/Security
0x9d226000 - 0x9d2adff3  com.apple.securityfoundation (6.0 - 55185.30.4) <632548B9-3E24-32F4-BF50-F6BF71713363> /System/Library/Frameworks/SecurityFoundation.framework/Versions/A/SecurityFoundation
0x9d2d9000 - 0x9d2ddfff  com.apple.xpc.ServiceManagement (1.0 - 1) <B09C1309-46A2-3081-B489-DCE549A8BA46> /System/Library/Frameworks/ServiceManagement.framework/Versions/A/ServiceManagement
0x9d408000 - 0x9d478ff3  com.apple.SystemConfiguration (1.17 - 1.17) <318C287A-BB41-3F72-9095-9DFF0382CB77> /System/Library/Frameworks/SystemConfiguration.framework/Versions/A/SystemConfiguration
0x9f26e000 - 0x9f305ff7  com.apple.APFS (1.0 - 1) <1B119C37-9A4C-3204-9BE2-E4E00E657037> /System/Library/PrivateFrameworks/APFS.framework/Versions/A/APFS
0x9fa4b000 - 0x9fa88ff3  com.apple.AppleJPEG (1.0 - 1) <E87393BB-6140-389C-BF53-36B3985A56D3> /System/Library/PrivateFrameworks/AppleJPEG.framework/Versions/A/AppleJPEG
0x9fbdb000 - 0x9fbe2fff  com.apple.coreservices.BackgroundTaskManagement (1.0 - 57.1) <71FD5EA2-8D0B-3E21-94E4-6D5BD45005E7> /System/Library/PrivateFrameworks/BackgroundTaskManagement.framework/Versions/A/BackgroundTaskManagement
0x9fdb2000 - 0x9fdbbffb  com.apple.CommonAuth (4.0 - 2.0) <424B8D39-396A-3A78-84C1-5161B54A8F5B> /System/Library/PrivateFrameworks/CommonAuth.framework/Versions/A/CommonAuth
0xa022d000 - 0xa023dff7  com.apple.CoreEmoji (1.0 - 69.3) <165A133F-DED4-3B24-A9BF-6EA6F3F7A152> /System/Library/PrivateFrameworks/CoreEmoji.framework/Versions/A/CoreEmoji
0xa0c96000 - 0xa10c6ff7  com.apple.vision.FaceCore (3.3.2 - 3.3.2) <B2288C3D-E67F-3AAE-A652-E920CD19F267> /System/Library/PrivateFrameworks/FaceCore.framework/Versions/A/FaceCore
0xa3a6a000 - 0xa3addff3  com.apple.Heimdal (4.0 - 2.0) <560C8A98-E261-39C9-9862-3340EC6ABC9C> /System/Library/PrivateFrameworks/Heimdal.framework/Versions/A/Heimdal
0xa3d8f000 - 0xa3d95fff  com.apple.IOAccelerator (376.6 - 376.6) <4EFED596-8863-35F6-8EA3-CB11C5D9D157> /System/Library/PrivateFrameworks/IOAccelerator.framework/Versions/A/IOAccelerator
0xa3d96000 - 0xa3daeffb  com.apple.IOPresentment (1.0 - 32.1) <EBD4DB8D-03D3-3136-B431-969A2E5E1B91> /System/Library/PrivateFrameworks/IOPresentment.framework/Versions/A/IOPresentment
0xa3e62000 - 0xa3f56ff7  com.apple.LanguageModeling (1.0 - 159.3.1) <AA880B14-031D-33FB-9B48-0A8AAB7342C6> /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
0xa3f57000 - 0xa3f98ff7  com.apple.Lexicon-framework (1.0 - 33.2) <13FAB8A2-507A-3AEA-A571-27BDDFD96B31> /System/Library/PrivateFrameworks/Lexicon.framework/Versions/A/Lexicon
0xa3f9c000 - 0xa3fa2ff3  com.apple.LinguisticData (1.0 - 238.3) <C47B3EB0-0463-3613-8A09-344F1639EE92> /System/Library/PrivateFrameworks/LinguisticData.framework/Versions/A/LinguisticData
0xa4352000 - 0xa437bffb  com.apple.MultitouchSupport.framework (1204.13 - 1204.13) <01BDF9A5-8C83-3611-A999-F43F9121A173> /System/Library/PrivateFrameworks/MultitouchSupport.framework/Versions/A/MultitouchSupport
0xa449a000 - 0xa44a4fff  com.apple.NetAuth (6.2 - 6.2) <52F67DC1-8C96-3944-8E54-C02DD51FD9FC> /System/Library/PrivateFrameworks/NetAuth.framework/Versions/A/NetAuth
0xa4858000 - 0xa48ddffb  com.apple.SkyLight (1.600.0 - 312.23.4) <27154D6B-3C0F-383F-AA7B-F602C1F397CC> /System/Library/PrivateFrameworks/SkyLight.framework/Versions/A/SkyLight
0xa4cbb000 - 0xa4cc2fff  com.apple.TCC (1.0 - 1) <4B76752A-36A0-3175-87C7-CB42E33CCB5A> /System/Library/PrivateFrameworks/TCC.framework/Versions/A/TCC
0xa545c000 - 0xa545efff  com.apple.loginsupport (1.0 - 1) <086FAE1B-87E2-324A-AE76-E6EC0B5F1517> /System/Library/PrivateFrameworks/login.framework/Versions/A/Frameworks/loginsupport.framework/Versions/A/loginsupport
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa556c000 - 0xa55a3ff3  libCRFSuite.dylib (41) <7F584902-74F1-3362-935D-95F5E735F5E7> /usr/lib/libCRFSuite.dylib
0xa55a4000 - 0xa55aeffb  libChineseTokenizer.dylib (28) <1FF5A32D-E012-3E76-B738-FAC26AD2A39B> /usr/lib/libChineseTokenizer.dylib
0xa564a000 - 0xa564bfff  libDiagnosticMessagesClient.dylib (104) <6829B180-2556-3A7E-A2E6-BD4859DF30A7> /usr/lib/libDiagnosticMessagesClient.dylib
0xa567d000 - 0xa5867ff7  libFosl_dynamic.dylib (17.7) <DBE4D720-8A46-3879-AD2D-F9A8CE3E7476> /usr/lib/libFosl_dynamic.dylib
0xa586f000 - 0xa586ffff  libOpenScriptingUtil.dylib (174) <B7CEDC30-2D17-3896-9EFC-64DB3D11DF00> /usr/lib/libOpenScriptingUtil.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa58ce000 - 0xa58e3ff7  libapple_nghttp2.dylib (1.24) <480C0C04-2533-3D44-8232-006B6CBA7758> /usr/lib/libapple_nghttp2.dylib
0xa58e4000 - 0xa590ffff  libarchive.2.dylib (54) <D55C5F86-251D-3C33-A617-0C623D4F512E> /usr/lib/libarchive.2.dylib
0xa5a63000 - 0xa5a63ff3  libauto.dylib (187) <CE2A78CC-670F-3E07-9539-822DCD2F6084> /usr/lib/libauto.dylib
0xa5a64000 - 0xa5a74fff  libbsm.0.dylib (39) <067E9003-0673-32A3-9B40-492323182C5C> /usr/lib/libbsm.0.dylib
0xa5a75000 - 0xa5a81ff7  libbz2.1.0.dylib (38) <77C24A36-BE84-3702-A786-935C597A0A86> /usr/lib/libbz2.1.0.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa5aff000 - 0xa5b10ff7  libcmph.dylib (6) <EC7664F1-B5A1-37F4-B7DC-F6AC10587E35> /usr/lib/libcmph.dylib
0xa5b11000 - 0xa5b24ff7  libcompression.dylib (47) <F80DDFC1-F96A-3BAD-967D-C1E24253273A> /usr/lib/libcompression.dylib
0xa5b25000 - 0xa5b3cffb  libcoretls.dylib (155) <F66FAEBC-4B6E-31E0-ACA8-C8ACBC7689DD> /usr/lib/libcoretls.dylib
0xa5b3d000 - 0xa5b3efff  libcoretls_cfhelpers.dylib (155) <8B8ABC2C-F251-3C80-9747-88C05A2CBE64> /usr/lib/libcoretls_cfhelpers.dylib
0xa6026000 - 0xa607dfff  libcups.2.dylib (462.1) <0180AE97-A19F-3D49-9838-06995E73F572> /usr/lib/libcups.2.dylib
0xa6193000 - 0xa6193fff  libenergytrace.dylib (16) <34FC43C7-D9B6-3C01-8B65-E49059D31279> /usr/lib/libenergytrace.dylib
0xa61c7000 - 0xa61cbfff  libheimdal-asn1.dylib (520.30.1) <DEA7E913-118F-333E-BE08-5B4F19B33B9A> /usr/lib/libheimdal-asn1.dylib
0xa61f7000 - 0xa62e7ff3  libiconv.2.dylib (51) <FE6D05A5-18DB-3FD8-A52F-B7BADB232C78> /usr/lib/libiconv.2.dylib
0xa62e8000 - 0xa650aff7  libicucore.A.dylib (59152.0.1) <35D52BFF-C74C-3519-AEAC-7371E3C7E4BD> /usr/lib/libicucore.A.dylib
0xa6552000 - 0xa6553fff  liblangid.dylib (128) <120FE992-47E4-3A73-A039-1B401F5696DC> /usr/lib/liblangid.dylib
0xa6554000 - 0xa656cff7  liblzma.5.dylib (10) <8A5C9679-430A-3A19-AF68-9D21BAC442C7> /usr/lib/liblzma.5.dylib
0xa656d000 - 0xa6582fff  libmarisa.dylib (9) <805453EE-B829-3DA5-8DF3-5132D03D5B74> /usr/lib/libmarisa.dylib
0xa6637000 - 0xa6854fff  libmecabra.dylib (779.7.6) <3C7F6A43-B17C-3673-A0AC-14FFE08370D4> /usr/lib/libmecabra.dylib
0xa6a1b000 - 0xa6aeffff  libnetwork.dylib (1229.30.11) <E4008EDE-F873-33FF-BD96-7DB14FA4F364> /usr/lib/libnetwork.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6ed4000 - 0xa6ed7fff  libpam.2.dylib (22) <7106F43C-84DD-3F26-905A-B52780AFEB3E> /usr/lib/libpam.2.dylib
0xa6eda000 - 0xa6f0bfff  libpcap.A.dylib (79.20.1) <154889CF-5F83-3012-953E-0FC8FEE50FF8> /usr/lib/libpcap.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa6faf000 - 0xa7139ff7  libsqlite3.dylib (274.5) <B09AF63F-4F1A-3481-9B61-6EBB64D12EB9> /usr/lib/libsqlite3.dylib
0xa72dd000 - 0xa7317ffb  libusrtcp.dylib (1229.30.11) <39D76669-A48B-3BAC-8F45-1D6CA87E9B4B> /usr/lib/libusrtcp.dylib
0xa7318000 - 0xa731bff7  libutil.dylib (51.20.1) <86BD9675-16A2-345D-9B8D-E8A3397F2365> /usr/lib/libutil.dylib
0xa731c000 - 0xa732aff7  libxar.1.dylib (400) <4B664A7E-EC05-34AD-ACC6-C879B69DBA7C> /usr/lib/libxar.1.dylib
0xa732b000 - 0xa7409ff7  libxml2.2.dylib (31.7) <3E1F9E3D-6C44-3437-AB2B-E5ACE1927F81> /usr/lib/libxml2.2.dylib
0xa740a000 - 0xa7432ff3  libxslt.1.dylib (15.10) <1A3DC7B8-7C92-3D73-BF82-D60E64BC3DF0> /usr/lib/libxslt.1.dylib
0xa7433000 - 0xa7442ff7  libz.1.dylib (70) <588F445F-0065-3D77-8002-BA9411DA1D70> /usr/lib/libz.1.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75c4000 - 0xa75d0ff7  libkxld.dylib (4570.41.2) <C01D2E6F-B29E-3795-9258-55445BF8F933> /usr/lib/system/libkxld.dylib
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
---
===========                     =======  ======= 
TOTAL                            569.7M      551 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-13-161627-2_Traviss-Mac-1044.crash
Process:               a [50394]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50391]
Responsible:           a [50394]
User ID:               501
Date/Time:             2019-04-13 16:16:15.683 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
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
    __TEXT                 00000000000d4000-00000000000d7000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000d65d4 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x000d547b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::he15ddd3037a6d325 + 11
2   libstd-d72d99315c451a4c.dylib  0x0025b14c std::sys_common::backtrace::__rust_begin_short_backtrace::h258c53a673ab9f0e + 12
3   libstd-d72d99315c451a4c.dylib  0x0025dca4 std::panicking::try::do_call::hdeca090a8a640010 + 20
4   libstd-d72d99315c451a4c.dylib  0x0026c3cd __rust_maybe_catch_panic + 29
5   libstd-d72d99315c451a4c.dylib  0x0025e747 std::rt::lang_start_internal::h9bb6eb8f317fc6bd + 631
6   a                              0x000d66ac main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x78675120
  edi: 0x786751b0  esi: 0xbff2a790  ebp: 0xbff2a828  esp: 0xbff2a710
   ss: 0x00000023  efl: 0x00210246  eip: 0x000d65d4   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xd4000 -    0xd6fff +a (0) <EBC1F786-AA58-3E82-AD9A-820B9DA79813> /Users/USER/*/a
  0x1b3000 -   0x1f8fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x23c000 -   0x2cbfff +libstd-d72d99315c451a4c.dylib (0) <DC60EA74-DA2D-3CD0-B0C3-E1E6BA9E45E4> /Users/USER/*/libstd-d72d99315c451a4c.dylib
0x90298000 - 0x90298fff  com.apple.Accelerate (1.11 - Accelerate 1.11) <F4A138F5-290D-3413-AD17-ECD395935FF3> /System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
0x902b0000 - 0x909f1fdf  com.apple.vImage (8.1 - ???) <591C941E-6475-347E-89DA-F541E88F949A> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vImage.framework/Versions/A/vImage
0x909f2000 - 0x90b2dff7  libBLAS.dylib (1211.30.1) <A850E0E2-3A72-3916-9907-AF1E7ECC95F0> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBLAS.dylib
0x90b2e000 - 0x90b5bffb  libBNNS.dylib (37) <C29094A0-5C89-3C5E-AB37-510C28588E2E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBNNS.dylib
0x90b5c000 - 0x90ecffff  libLAPACK.dylib (1211.30.1) <2DDDE838-0FF1-3679-8E62-9C09923ECB7E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLAPACK.dylib
0x90ed0000 - 0x90ee6ffb  libLinearAlgebra.dylib (1211.30.1) <8A120E75-CAF4-3CAE-BBE6-E2F5FAE44DB8> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLinearAlgebra.dylib
0x90ee7000 - 0x90f00ff7  libSparseBLAS.dylib (1211.30.1) <0C5E0EF4-E9A5-3FC4-B7A3-1FE59DB4A2AA> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparseBLAS.dylib
0x90f01000 - 0x9105ffc7  libvDSP.dylib (622.20.8) <C5F16300-061F-3DF0-B91E-8BD0D2173351> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvDSP.dylib
0x91060000 - 0x9113effb  libvMisc.dylib (622.20.8) <1C8D5D80-F32C-3853-8309-57C8A82B7DA5> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvMisc.dylib
0x9113f000 - 0x9113ffff  com.apple.Accelerate.vecLib (3.11 - vecLib 3.11) <7A0D5DD6-C302-390D-9178-0B2EA94BB5ED> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/vecLib
0x92146000 - 0x92146fff  com.apple.ApplicationServices (48 - 50) <BFE7FB45-365B-341F-A8FC-B9483AE87709> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/ApplicationServices
0x92147000 - 0x921adff3  com.apple.ApplicationServices.ATS (377 - 445) <CD3D5685-2BB9-3A7B-AC97-2A74A81CB7CC> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/ATS
0x921b0000 - 0x922d4ff3  libFontParser.dylib (222.1.2) <8F7D388A-299C-3C6D-9864-40EC0914A96B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontParser.dylib
0x922d5000 - 0x92321ffb  libFontRegistry.dylib (221) <8D81FDCF-F05D-3556-AB6D-090F9508C25E> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontRegistry.dylib
0x9240f000 - 0x92414fff  com.apple.ColorSyncLegacy (4.13.0 - 1) <AB5CE7D2-8BE5-35C8-A9D5-ED0FB5BAB7D1> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ColorSyncLegacy.framework/Versions/A/ColorSyncLegacy
0x924be000 - 0x92515ff7  com.apple.HIServices (1.22 - 622) <8544026A-17BE-301D-BA2A-782F3AD864DA> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/HIServices.framework/Versions/A/HIServices
0x92516000 - 0x92525ff7  com.apple.LangAnalysis (1.7.0 - 1.7.0) <E3245701-039B-353F-923D-F81B2242842C> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/LangAnalysis.framework/Versions/A/LangAnalysis
0x92526000 - 0x9257effb  com.apple.print.framework.PrintCore (13 - 503) <FD0F7A18-6F78-34C9-B067-B3AB76C3D4C8> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/PrintCore
0x9257f000 - 0x92615ffb  com.apple.QD (3.12 - 403) <372AFF26-17D1-3C6F-9E47-17C955C2045B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/QD.framework/Versions/A/QD
0x92616000 - 0x92622ff3  com.apple.speech.synthesis.framework (7.4.1 - 7.4.1) <3AE4F801-4A2D-3AB3-AA31-B27F7A7131F4> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/SpeechSynthesis.framework/Versions/A/SpeechSynthesis
0x92623000 - 0x9286ffff  com.apple.audio.toolbox.AudioToolbox (1.14 - 1.14) <E4585EFD-C3B6-327F-88E4-B3BADDA6B08D> /System/Library/Frameworks/AudioToolbox.framework/Versions/A/AudioToolbox
0x92b8f000 - 0x92efcffb  com.apple.CFNetwork (893.13.1 - 893.13.1) <63A5C550-5F0F-3FF1-9061-55E1766B3512> /System/Library/Frameworks/CFNetwork.framework/Versions/A/CFNetwork
0x93426000 - 0x934e5ff7  com.apple.ColorSync (4.13.0 - 546) <DACC5623-8E37-3134-9562-4E8601127F67> /System/Library/Frameworks/ColorSync.framework/Versions/A/ColorSync
0x934e6000 - 0x93581fff  com.apple.audio.CoreAudio (4.3.0 - 4.3.0) <ABA90687-71A6-3431-94A1-0E7E74FE407C> /System/Library/Frameworks/CoreAudio.framework/Versions/A/CoreAudio
0x935e5000 - 0x938c5fff  com.apple.CoreData (120 - 849.2) <B8011F5E-7A2B-349B-AFF1-17EC8D8465AB> /System/Library/Frameworks/CoreData.framework/Versions/A/CoreData
0x938c6000 - 0x938ccff3  com.apple.CoreDisplay (1.0 - 81.7) <E6BFD0F5-A45B-3FE3-BA26-14D2CD970CFF> /System/Library/Frameworks/CoreDisplay.framework/Versions/A/CoreDisplay
0x938cd000 - 0x93d56ff7  com.apple.CoreFoundation (6.9 - 1451) <727B43E3-A1AC-31EC-97A4-F179FE11D04A> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
0x93d58000 - 0x94387ffb  com.apple.CoreGraphics (2.0 - 1129.5) <20752785-E9DA-3CC6-ACDD-5A82AD344209> /System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics
0x94389000 - 0x945ffffb  com.apple.CoreImage (13.0.0 - 579.2.9) <2498F44C-7350-397B-A075-206A91D75ABB> /System/Library/Frameworks/CoreImage.framework/Versions/A/CoreImage
0x947f1000 - 0x947f1fff  com.apple.CoreServices (822.19 - 822.19) <6B5DC8C1-4237-3ADA-B8C8-F926943E6101> /System/Library/Frameworks/CoreServices.framework/Versions/A/CoreServices
0x947f2000 - 0x94864ff3  com.apple.AE (735.1 - 735.1) <3E1B0CED-0AC3-3252-AEDD-5D8F91E3AAA7> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/AE.framework/Versions/A/AE
0x94865000 - 0x94b43ffb  com.apple.CoreServices.CarbonCore (1178.2 - 1178.2) <E61EA71D-294F-3C8B-95BD-0CDBA0FFC907> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/CarbonCore
0x94b44000 - 0x94b78ff3  com.apple.DictionaryServices (1.2 - 284) <56BEF6B8-50D2-38A0-9EF2-D7093E9AAB56> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/DictionaryServices.framework/Versions/A/DictionaryServices
0x94b79000 - 0x94b81fff  com.apple.CoreServices.FSEvents (1239 - 1239) <CABC21F7-E3AB-3954-ACBE-B8066A37516A> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/FSEvents
0x94b82000 - 0x94ce0fff  com.apple.LaunchServices (822.19 - 822.19) <AC40752F-0F99-3EB1-8D25-2C65F9BAC226> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/LaunchServices
0x94ce1000 - 0x94d8dff7  com.apple.Metadata (10.7.0 - 1191.2.6) <6CE69880-6AF3-3A1A-A8E0-F89FC750EE4C> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Metadata
0x94d8e000 - 0x94decff7  com.apple.CoreServices.OSServices (822.19 - 822.19) <38B64F72-5CAE-374A-8BBC-0EAB7C6A6777> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/OSServices.framework/Versions/A/OSServices
0x94ded000 - 0x94e5eff3  com.apple.SearchKit (1.4.0 - 1.4.0) <FAD60011-970B-3889-B6BD-3715CCF599CA> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SearchKit.framework/Versions/A/SearchKit
0x94e5f000 - 0x94e82fff  com.apple.coreservices.SharedFileList (71.4 - 71.4) <CD97E31D-0354-36AB-9997-C4FAF3221D30> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SharedFileList.framework/Versions/A/SharedFileList
0x94e83000 - 0x94fceffb  com.apple.CoreText (352.0 - 578.12) <6389222B-6B26-3F46-93C2-ABE07168227F> /System/Library/Frameworks/CoreText.framework/Versions/A/CoreText
0x94fcf000 - 0x95009ff3  com.apple.CoreVideo (1.8 - 279.2) <0D75C395-3C86-3539-9206-C7A330BE3551> /System/Library/Frameworks/CoreVideo.framework/Versions/A/CoreVideo
0x952e4000 - 0x952edff7  com.apple.DiskArbitration (2.7 - 2.7) <E3552A79-57A4-36AE-8B54-5FE2EB5193DA> /System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration
0x952fe000 - 0x9566dffb  com.apple.Foundation (6.9 - 1451) <E815D5AF-B627-3BF4-8156-4F514FCFD765> /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
0x956ae000 - 0x956ddff3  com.apple.GSS (4.0 - 2.0) <78C94D11-21DF-34C6-B4E8-88564551D67E> /System/Library/Frameworks/GSS.framework/Versions/A/GSS
0x95888000 - 0x95929ffb  com.apple.framework.IOKit (2.0.2 - 1445.40.1) <EDA5B2F5-12B4-39EF-B5EF-899587AACDC5> /System/Library/Frameworks/IOKit.framework/Versions/A/IOKit
0x9592b000 - 0x95932fff  com.apple.IOSurface (209.2.2 - 209.2.2) <0CCA9904-FCBB-3278-96A1-714BDB961D8A> /System/Library/Frameworks/IOSurface.framework/Versions/A/IOSurface
0x95987000 - 0x95b0bff3  com.apple.ImageIO.framework (3.3.0 - 1713) <28D21D61-3526-33ED-92DC-08D4D67445CB> /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
0x95b0c000 - 0x95b10ffb  libGIF.dylib (1713) <47A6BFCC-6651-3AAE-A70C-7BA3717B13BB> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libGIF.dylib
0x95b11000 - 0x95c02fff  libJP2.dylib (1713) <D1075C88-406B-3EAF-9270-9B3762D97803> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJP2.dylib
0x95c03000 - 0x95c25ff7  libJPEG.dylib (1713) <EFD86068-C17E-32AD-B4A5-79C20BC93411> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJPEG.dylib
0x95f06000 - 0x95f2cff7  libPng.dylib (1713) <A10259A1-2581-3642-946D-5B3F101615EC> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libPng.dylib
0x95f2d000 - 0x95f2fffb  libRadiance.dylib (1713) <EE872547-4C9F-397B-B41B-C79FEEE68267> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libRadiance.dylib
0x95f30000 - 0x95f7afff  libTIFF.dylib (1713) <7A56E3C4-9965-3471-8E98-5F6B28D68FBF> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libTIFF.dylib
0x96897000 - 0x968affff  com.apple.Kerberos (3.0 - 1) <8A399DB7-5440-3EC0-A241-3DD10E82DDB2> /System/Library/Frameworks/Kerberos.framework/Versions/A/Kerberos
0x96f27000 - 0x96f9dfff  com.apple.Metal (124.7 - 124.7) <2617CDD0-32C6-358C-A61F-063737F916B3> /System/Library/Frameworks/Metal.framework/Versions/A/Metal
0x96f9f000 - 0x96fabfff  com.apple.NetFS (6.0 - 4.0) <F37A4DA0-AAB6-3F0B-BA18-E322BFA52CC4> /System/Library/Frameworks/NetFS.framework/Versions/A/NetFS
0x99c78000 - 0x99cc3fff  com.apple.opencl (2.8.12 - 2.8.12) <9AC78C72-CBBC-3D29-B553-AACC28014AEC> /System/Library/Frameworks/OpenCL.framework/Versions/A/OpenCL
0x99cc4000 - 0x99ce0fff  com.apple.CFOpenDirectory (10.13 - 207) <255284C6-7BDD-3A0B-A4A2-E43206611B91> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/Frameworks/CFOpenDirectory.framework/Versions/A/CFOpenDirectory
0x99ce1000 - 0x99cecfff  com.apple.OpenDirectory (10.13 - 207) <ECB33DA6-A0A3-3378-AB7A-2C10D395904E> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/OpenDirectory
0x9aef6000 - 0x9aef7fff  libCVMSPluginSupport.dylib (16.4.2) <909D788E-692E-3FF1-AFF0-2AB4609C53D7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCVMSPluginSupport.dylib
0x9aef8000 - 0x9aefcfff  libCoreFSCache.dylib (162.4) <D53B0D41-0774-3C6A-BB59-8BA7DB8A8374> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreFSCache.dylib
0x9aefd000 - 0x9af01fff  libCoreVMClient.dylib (162.4) <19767FEB-6A89-3892-8F18-1F9E73463050> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreVMClient.dylib
0x9af02000 - 0x9af0aff7  libGFXShared.dylib (16.4.2) <F62281F1-9495-3589-A576-75D84880D42D> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGFXShared.dylib
0x9af0b000 - 0x9af17fff  libGL.dylib (16.4.2) <028B909B-DD19-388B-8113-1850DFAD3DCA> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib
0x9af18000 - 0x9af53ffb  libGLImage.dylib (16.4.2) <AE5E3974-656A-3F88-956E-F28192BA98C3> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLImage.dylib
0x9b0cd000 - 0x9b10fff7  libGLU.dylib (16.4.2) <9E1283AA-A7E0-37BA-BDEB-EE5256D677C7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLU.dylib
0x9bab6000 - 0x9bac4fff  com.apple.opengl (16.4.2 - 16.4.2) <40645026-52DC-3CFC-8308-EFA2FA79D5A0> /System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
0x9c829000 - 0x9ca5fff7  com.apple.QuartzCore (1.11 - 584.8.102) <960628B2-C498-36C9-B0D4-F27D49DE029F> /System/Library/Frameworks/QuartzCore.framework/Versions/A/QuartzCore
0x9cef8000 - 0x9d225ff3  com.apple.security (7.0 - 58286.41.2) <F64D64CD-4E80-3384-92F0-56D2A464F7B4> /System/Library/Frameworks/Security.framework/Versions/A/Security
0x9d226000 - 0x9d2adff3  com.apple.securityfoundation (6.0 - 55185.30.4) <632548B9-3E24-32F4-BF50-F6BF71713363> /System/Library/Frameworks/SecurityFoundation.framework/Versions/A/SecurityFoundation
0x9d2d9000 - 0x9d2ddfff  com.apple.xpc.ServiceManagement (1.0 - 1) <B09C1309-46A2-3081-B489-DCE549A8BA46> /System/Library/Frameworks/ServiceManagement.framework/Versions/A/ServiceManagement
0x9d408000 - 0x9d478ff3  com.apple.SystemConfiguration (1.17 - 1.17) <318C287A-BB41-3F72-9095-9DFF0382CB77> /System/Library/Frameworks/SystemConfiguration.framework/Versions/A/SystemConfiguration
0x9f26e000 - 0x9f305ff7  com.apple.APFS (1.0 - 1) <1B119C37-9A4C-3204-9BE2-E4E00E657037> /System/Library/PrivateFrameworks/APFS.framework/Versions/A/APFS
0x9fa4b000 - 0x9fa88ff3  com.apple.AppleJPEG (1.0 - 1) <E87393BB-6140-389C-BF53-36B3985A56D3> /System/Library/PrivateFrameworks/AppleJPEG.framework/Versions/A/AppleJPEG
0x9fbdb000 - 0x9fbe2fff  com.apple.coreservices.BackgroundTaskManagement (1.0 - 57.1) <71FD5EA2-8D0B-3E21-94E4-6D5BD45005E7> /System/Library/PrivateFrameworks/BackgroundTaskManagement.framework/Versions/A/BackgroundTaskManagement
0x9fdb2000 - 0x9fdbbffb  com.apple.CommonAuth (4.0 - 2.0) <424B8D39-396A-3A78-84C1-5161B54A8F5B> /System/Library/PrivateFrameworks/CommonAuth.framework/Versions/A/CommonAuth
0xa022d000 - 0xa023dff7  com.apple.CoreEmoji (1.0 - 69.3) <165A133F-DED4-3B24-A9BF-6EA6F3F7A152> /System/Library/PrivateFrameworks/CoreEmoji.framework/Versions/A/CoreEmoji
0xa0c96000 - 0xa10c6ff7  com.apple.vision.FaceCore (3.3.2 - 3.3.2) <B2288C3D-E67F-3AAE-A652-E920CD19F267> /System/Library/PrivateFrameworks/FaceCore.framework/Versions/A/FaceCore
0xa3a6a000 - 0xa3addff3  com.apple.Heimdal (4.0 - 2.0) <560C8A98-E261-39C9-9862-3340EC6ABC9C> /System/Library/PrivateFrameworks/Heimdal.framework/Versions/A/Heimdal
0xa3d8f000 - 0xa3d95fff  com.apple.IOAccelerator (376.6 - 376.6) <4EFED596-8863-35F6-8EA3-CB11C5D9D157> /System/Library/PrivateFrameworks/IOAccelerator.framework/Versions/A/IOAccelerator
0xa3d96000 - 0xa3daeffb  com.apple.IOPresentment (1.0 - 32.1) <EBD4DB8D-03D3-3136-B431-969A2E5E1B91> /System/Library/PrivateFrameworks/IOPresentment.framework/Versions/A/IOPresentment
0xa3e62000 - 0xa3f56ff7  com.apple.LanguageModeling (1.0 - 159.3.1) <AA880B14-031D-33FB-9B48-0A8AAB7342C6> /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
0xa3f57000 - 0xa3f98ff7  com.apple.Lexicon-framework (1.0 - 33.2) <13FAB8A2-507A-3AEA-A571-27BDDFD96B31> /System/Library/PrivateFrameworks/Lexicon.framework/Versions/A/Lexicon
0xa3f9c000 - 0xa3fa2ff3  com.apple.LinguisticData (1.0 - 238.3) <C47B3EB0-0463-3613-8A09-344F1639EE92> /System/Library/PrivateFrameworks/LinguisticData.framework/Versions/A/LinguisticData
0xa4352000 - 0xa437bffb  com.apple.MultitouchSupport.framework (1204.13 - 1204.13) <01BDF9A5-8C83-3611-A999-F43F9121A173> /System/Library/PrivateFrameworks/MultitouchSupport.framework/Versions/A/MultitouchSupport
0xa449a000 - 0xa44a4fff  com.apple.NetAuth (6.2 - 6.2) <52F67DC1-8C96-3944-8E54-C02DD51FD9FC> /System/Library/PrivateFrameworks/NetAuth.framework/Versions/A/NetAuth
0xa4858000 - 0xa48ddffb  com.apple.SkyLight (1.600.0 - 312.23.4) <27154D6B-3C0F-383F-AA7B-F602C1F397CC> /System/Library/PrivateFrameworks/SkyLight.framework/Versions/A/SkyLight
0xa4cbb000 - 0xa4cc2fff  com.apple.TCC (1.0 - 1) <4B76752A-36A0-3175-87C7-CB42E33CCB5A> /System/Library/PrivateFrameworks/TCC.framework/Versions/A/TCC
0xa545c000 - 0xa545efff  com.apple.loginsupport (1.0 - 1) <086FAE1B-87E2-324A-AE76-E6EC0B5F1517> /System/Library/PrivateFrameworks/login.framework/Versions/A/Frameworks/loginsupport.framework/Versions/A/loginsupport
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa556c000 - 0xa55a3ff3  libCRFSuite.dylib (41) <7F584902-74F1-3362-935D-95F5E735F5E7> /usr/lib/libCRFSuite.dylib
0xa55a4000 - 0xa55aeffb  libChineseTokenizer.dylib (28) <1FF5A32D-E012-3E76-B738-FAC26AD2A39B> /usr/lib/libChineseTokenizer.dylib
0xa564a000 - 0xa564bfff  libDiagnosticMessagesClient.dylib (104) <6829B180-2556-3A7E-A2E6-BD4859DF30A7> /usr/lib/libDiagnosticMessagesClient.dylib
0xa567d000 - 0xa5867ff7  libFosl_dynamic.dylib (17.7) <DBE4D720-8A46-3879-AD2D-F9A8CE3E7476> /usr/lib/libFosl_dynamic.dylib
0xa586f000 - 0xa586ffff  libOpenScriptingUtil.dylib (174) <B7CEDC30-2D17-3896-9EFC-64DB3D11DF00> /usr/lib/libOpenScriptingUtil.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa58ce000 - 0xa58e3ff7  libapple_nghttp2.dylib (1.24) <480C0C04-2533-3D44-8232-006B6CBA7758> /usr/lib/libapple_nghttp2.dylib
0xa58e4000 - 0xa590ffff  libarchive.2.dylib (54) <D55C5F86-251D-3C33-A617-0C623D4F512E> /usr/lib/libarchive.2.dylib
0xa5a63000 - 0xa5a63ff3  libauto.dylib (187) <CE2A78CC-670F-3E07-9539-822DCD2F6084> /usr/lib/libauto.dylib
0xa5a64000 - 0xa5a74fff  libbsm.0.dylib (39) <067E9003-0673-32A3-9B40-492323182C5C> /usr/lib/libbsm.0.dylib
0xa5a75000 - 0xa5a81ff7  libbz2.1.0.dylib (38) <77C24A36-BE84-3702-A786-935C597A0A86> /usr/lib/libbz2.1.0.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa5aff000 - 0xa5b10ff7  libcmph.dylib (6) <EC7664F1-B5A1-37F4-B7DC-F6AC10587E35> /usr/lib/libcmph.dylib
0xa5b11000 - 0xa5b24ff7  libcompression.dylib (47) <F80DDFC1-F96A-3BAD-967D-C1E24253273A> /usr/lib/libcompression.dylib
0xa5b25000 - 0xa5b3cffb  libcoretls.dylib (155) <F66FAEBC-4B6E-31E0-ACA8-C8ACBC7689DD> /usr/lib/libcoretls.dylib
0xa5b3d000 - 0xa5b3efff  libcoretls_cfhelpers.dylib (155) <8B8ABC2C-F251-3C80-9747-88C05A2CBE64> /usr/lib/libcoretls_cfhelpers.dylib
0xa6026000 - 0xa607dfff  libcups.2.dylib (462.1) <0180AE97-A19F-3D49-9838-06995E73F572> /usr/lib/libcups.2.dylib
0xa6193000 - 0xa6193fff  libenergytrace.dylib (16) <34FC43C7-D9B6-3C01-8B65-E49059D31279> /usr/lib/libenergytrace.dylib
0xa61c7000 - 0xa61cbfff  libheimdal-asn1.dylib (520.30.1) <DEA7E913-118F-333E-BE08-5B4F19B33B9A> /usr/lib/libheimdal-asn1.dylib
0xa61f7000 - 0xa62e7ff3  libiconv.2.dylib (51) <FE6D05A5-18DB-3FD8-A52F-B7BADB232C78> /usr/lib/libiconv.2.dylib
0xa62e8000 - 0xa650aff7  libicucore.A.dylib (59152.0.1) <35D52BFF-C74C-3519-AEAC-7371E3C7E4BD> /usr/lib/libicucore.A.dylib
0xa6552000 - 0xa6553fff  liblangid.dylib (128) <120FE992-47E4-3A73-A039-1B401F5696DC> /usr/lib/liblangid.dylib
0xa6554000 - 0xa656cff7  liblzma.5.dylib (10) <8A5C9679-430A-3A19-AF68-9D21BAC442C7> /usr/lib/liblzma.5.dylib
0xa656d000 - 0xa6582fff  libmarisa.dylib (9) <805453EE-B829-3DA5-8DF3-5132D03D5B74> /usr/lib/libmarisa.dylib
0xa6637000 - 0xa6854fff  libmecabra.dylib (779.7.6) <3C7F6A43-B17C-3673-A0AC-14FFE08370D4> /usr/lib/libmecabra.dylib
0xa6a1b000 - 0xa6aeffff  libnetwork.dylib (1229.30.11) <E4008EDE-F873-33FF-BD96-7DB14FA4F364> /usr/lib/libnetwork.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6ed4000 - 0xa6ed7fff  libpam.2.dylib (22) <7106F43C-84DD-3F26-905A-B52780AFEB3E> /usr/lib/libpam.2.dylib
0xa6eda000 - 0xa6f0bfff  libpcap.A.dylib (79.20.1) <154889CF-5F83-3012-953E-0FC8FEE50FF8> /usr/lib/libpcap.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa6faf000 - 0xa7139ff7  libsqlite3.dylib (274.5) <B09AF63F-4F1A-3481-9B61-6EBB64D12EB9> /usr/lib/libsqlite3.dylib
0xa72dd000 - 0xa7317ffb  libusrtcp.dylib (1229.30.11) <39D76669-A48B-3BAC-8F45-1D6CA87E9B4B> /usr/lib/libusrtcp.dylib
0xa7318000 - 0xa731bff7  libutil.dylib (51.20.1) <86BD9675-16A2-345D-9B8D-E8A3397F2365> /usr/lib/libutil.dylib
0xa731c000 - 0xa732aff7  libxar.1.dylib (400) <4B664A7E-EC05-34AD-ACC6-C879B69DBA7C> /usr/lib/libxar.1.dylib
0xa732b000 - 0xa7409ff7  libxml2.2.dylib (31.7) <3E1F9E3D-6C44-3437-AB2B-E5ACE1927F81> /usr/lib/libxml2.2.dylib
0xa740a000 - 0xa7432ff3  libxslt.1.dylib (15.10) <1A3DC7B8-7C92-3D73-BF82-D60E64BC3DF0> /usr/lib/libxslt.1.dylib
0xa7433000 - 0xa7442ff7  libz.1.dylib (70) <588F445F-0065-3D77-8002-BA9411DA1D70> /usr/lib/libz.1.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75c4000 - 0xa75d0ff7  libkxld.dylib (4570.41.2) <C01D2E6F-B29E-3795-9258-55445BF8F933> /usr/lib/system/libkxld.dylib
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
    task_for_pid: 2358
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=168.2M resident=0K(0%) swapped_out_or_unallocated=168.2M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9268K       12 
MALLOC guard page                   32K        9 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            9332K      164 
__FONT_DATA                          4K        2 
---
===========                     =======  ======= 
TOTAL                            568.7M      551 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-13-161627_Traviss-Mac-1044.crash
Process:               a [50188]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [50180]
Responsible:           a [50188]
User ID:               501
Date/Time:             2019-04-13 16:16:06.542 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
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
3   libstd-d72d99315c451a4c.dylib  0x002b17eb std::sys::unix::abort_internal::h549e70aace565765 + 11
4   libstd-d72d99315c451a4c.dylib  0x002a2129 std::sys_common::util::abort::hdcba088e127a7962 + 73
5   libstd-d72d99315c451a4c.dylib  0x002a44c2 rust_panic + 98
6   libstd-d72d99315c451a4c.dylib  0x002a438e std::panicking::rust_panic_with_hook::h132c3f5bac2c9cae + 590
7   a                              0x001009bf std::panicking::begin_panic::h0c787872a300d62e + 47
8   a                              0x00101eec main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbfeff5ec  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbfeff618  esp: 0xbfeff5ec
   ss: 0x00000023  efl: 0x00200206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xff000 -   0x102ff7 +a (0) <339930E4-31B5-3BF8-B4B1-EDEAAAEC38A8> /Users/USER/*/a
  0x1f9000 -   0x23efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x282000 -   0x311fff +libstd-d72d99315c451a4c.dylib (0) <DC60EA74-DA2D-3CD0-B0C3-E1E6BA9E45E4> /Users/USER/*/libstd-d72d99315c451a4c.dylib
0x90298000 - 0x90298fff  com.apple.Accelerate (1.11 - Accelerate 1.11) <F4A138F5-290D-3413-AD17-ECD395935FF3> /System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
0x902b0000 - 0x909f1fdf  com.apple.vImage (8.1 - ???) <591C941E-6475-347E-89DA-F541E88F949A> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vImage.framework/Versions/A/vImage
0x909f2000 - 0x90b2dff7  libBLAS.dylib (1211.30.1) <A850E0E2-3A72-3916-9907-AF1E7ECC95F0> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBLAS.dylib
0x90b2e000 - 0x90b5bffb  libBNNS.dylib (37) <C29094A0-5C89-3C5E-AB37-510C28588E2E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBNNS.dylib
0x90b5c000 - 0x90ecffff  libLAPACK.dylib (1211.30.1) <2DDDE838-0FF1-3679-8E62-9C09923ECB7E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLAPACK.dylib
0x90ed0000 - 0x90ee6ffb  libLinearAlgebra.dylib (1211.30.1) <8A120E75-CAF4-3CAE-BBE6-E2F5FAE44DB8> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLinearAlgebra.dylib
0x90ee7000 - 0x90f00ff7  libSparseBLAS.dylib (1211.30.1) <0C5E0EF4-E9A5-3FC4-B7A3-1FE59DB4A2AA> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparseBLAS.dylib
0x90f01000 - 0x9105ffc7  libvDSP.dylib (622.20.8) <C5F16300-061F-3DF0-B91E-8BD0D2173351> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvDSP.dylib
0x91060000 - 0x9113effb  libvMisc.dylib (622.20.8) <1C8D5D80-F32C-3853-8309-57C8A82B7DA5> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvMisc.dylib
0x9113f000 - 0x9113ffff  com.apple.Accelerate.vecLib (3.11 - vecLib 3.11) <7A0D5DD6-C302-390D-9178-0B2EA94BB5ED> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/vecLib
0x92146000 - 0x92146fff  com.apple.ApplicationServices (48 - 50) <BFE7FB45-365B-341F-A8FC-B9483AE87709> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/ApplicationServices
0x92147000 - 0x921adff3  com.apple.ApplicationServices.ATS (377 - 445) <CD3D5685-2BB9-3A7B-AC97-2A74A81CB7CC> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/ATS
0x921b0000 - 0x922d4ff3  libFontParser.dylib (222.1.2) <8F7D388A-299C-3C6D-9864-40EC0914A96B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontParser.dylib
0x922d5000 - 0x92321ffb  libFontRegistry.dylib (221) <8D81FDCF-F05D-3556-AB6D-090F9508C25E> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontRegistry.dylib
0x9240f000 - 0x92414fff  com.apple.ColorSyncLegacy (4.13.0 - 1) <AB5CE7D2-8BE5-35C8-A9D5-ED0FB5BAB7D1> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ColorSyncLegacy.framework/Versions/A/ColorSyncLegacy
0x924be000 - 0x92515ff7  com.apple.HIServices (1.22 - 622) <8544026A-17BE-301D-BA2A-782F3AD864DA> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/HIServices.framework/Versions/A/HIServices
0x92516000 - 0x92525ff7  com.apple.LangAnalysis (1.7.0 - 1.7.0) <E3245701-039B-353F-923D-F81B2242842C> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/LangAnalysis.framework/Versions/A/LangAnalysis
0x92526000 - 0x9257effb  com.apple.print.framework.PrintCore (13 - 503) <FD0F7A18-6F78-34C9-B067-B3AB76C3D4C8> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/PrintCore
0x9257f000 - 0x92615ffb  com.apple.QD (3.12 - 403) <372AFF26-17D1-3C6F-9E47-17C955C2045B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/QD.framework/Versions/A/QD
0x92616000 - 0x92622ff3  com.apple.speech.synthesis.framework (7.4.1 - 7.4.1) <3AE4F801-4A2D-3AB3-AA31-B27F7A7131F4> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/SpeechSynthesis.framework/Versions/A/SpeechSynthesis
0x92623000 - 0x9286ffff  com.apple.audio.toolbox.AudioToolbox (1.14 - 1.14) <E4585EFD-C3B6-327F-88E4-B3BADDA6B08D> /System/Library/Frameworks/AudioToolbox.framework/Versions/A/AudioToolbox
0x92b8f000 - 0x92efcffb  com.apple.CFNetwork (893.13.1 - 893.13.1) <63A5C550-5F0F-3FF1-9061-55E1766B3512> /System/Library/Frameworks/CFNetwork.framework/Versions/A/CFNetwork
0x93426000 - 0x934e5ff7  com.apple.ColorSync (4.13.0 - 546) <DACC5623-8E37-3134-9562-4E8601127F67> /System/Library/Frameworks/ColorSync.framework/Versions/A/ColorSync
0x934e6000 - 0x93581fff  com.apple.audio.CoreAudio (4.3.0 - 4.3.0) <ABA90687-71A6-3431-94A1-0E7E74FE407C> /System/Library/Frameworks/CoreAudio.framework/Versions/A/CoreAudio
0x935e5000 - 0x938c5fff  com.apple.CoreData (120 - 849.2) <B8011F5E-7A2B-349B-AFF1-17EC8D8465AB> /System/Library/Frameworks/CoreData.framework/Versions/A/CoreData
0x938c6000 - 0x938ccff3  com.apple.CoreDisplay (1.0 - 81.7) <E6BFD0F5-A45B-3FE3-BA26-14D2CD970CFF> /System/Library/Frameworks/CoreDisplay.framework/Versions/A/CoreDisplay
0x938cd000 - 0x93d56ff7  com.apple.CoreFoundation (6.9 - 1451) <727B43E3-A1AC-31EC-97A4-F179FE11D04A> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
0x93d58000 - 0x94387ffb  com.apple.CoreGraphics (2.0 - 1129.5) <20752785-E9DA-3CC6-ACDD-5A82AD344209> /System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics
0x94389000 - 0x945ffffb  com.apple.CoreImage (13.0.0 - 579.2.9) <2498F44C-7350-397B-A075-206A91D75ABB> /System/Library/Frameworks/CoreImage.framework/Versions/A/CoreImage
0x947f1000 - 0x947f1fff  com.apple.CoreServices (822.19 - 822.19) <6B5DC8C1-4237-3ADA-B8C8-F926943E6101> /System/Library/Frameworks/CoreServices.framework/Versions/A/CoreServices
0x947f2000 - 0x94864ff3  com.apple.AE (735.1 - 735.1) <3E1B0CED-0AC3-3252-AEDD-5D8F91E3AAA7> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/AE.framework/Versions/A/AE
0x94865000 - 0x94b43ffb  com.apple.CoreServices.CarbonCore (1178.2 - 1178.2) <E61EA71D-294F-3C8B-95BD-0CDBA0FFC907> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/CarbonCore
0x94b44000 - 0x94b78ff3  com.apple.DictionaryServices (1.2 - 284) <56BEF6B8-50D2-38A0-9EF2-D7093E9AAB56> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/DictionaryServices.framework/Versions/A/DictionaryServices
0x94b79000 - 0x94b81fff  com.apple.CoreServices.FSEvents (1239 - 1239) <CABC21F7-E3AB-3954-ACBE-B8066A37516A> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/FSEvents
0x94b82000 - 0x94ce0fff  com.apple.LaunchServices (822.19 - 822.19) <AC40752F-0F99-3EB1-8D25-2C65F9BAC226> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/LaunchServices
0x94ce1000 - 0x94d8dff7  com.apple.Metadata (10.7.0 - 1191.2.6) <6CE69880-6AF3-3A1A-A8E0-F89FC750EE4C> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Metadata
0x94d8e000 - 0x94decff7  com.apple.CoreServices.OSServices (822.19 - 822.19) <38B64F72-5CAE-374A-8BBC-0EAB7C6A6777> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/OSServices.framework/Versions/A/OSServices
0x94ded000 - 0x94e5eff3  com.apple.SearchKit (1.4.0 - 1.4.0) <FAD60011-970B-3889-B6BD-3715CCF599CA> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SearchKit.framework/Versions/A/SearchKit
0x94e5f000 - 0x94e82fff  com.apple.coreservices.SharedFileList (71.4 - 71.4) <CD97E31D-0354-36AB-9997-C4FAF3221D30> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SharedFileList.framework/Versions/A/SharedFileList
0x94e83000 - 0x94fceffb  com.apple.CoreText (352.0 - 578.12) <6389222B-6B26-3F46-93C2-ABE07168227F> /System/Library/Frameworks/CoreText.framework/Versions/A/CoreText
0x94fcf000 - 0x95009ff3  com.apple.CoreVideo (1.8 - 279.2) <0D75C395-3C86-3539-9206-C7A330BE3551> /System/Library/Frameworks/CoreVideo.framework/Versions/A/CoreVideo
0x952e4000 - 0x952edff7  com.apple.DiskArbitration (2.7 - 2.7) <E3552A79-57A4-36AE-8B54-5FE2EB5193DA> /System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration
0x952fe000 - 0x9566dffb  com.apple.Foundation (6.9 - 1451) <E815D5AF-B627-3BF4-8156-4F514FCFD765> /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
0x956ae000 - 0x956ddff3  com.apple.GSS (4.0 - 2.0) <78C94D11-21DF-34C6-B4E8-88564551D67E> /System/Library/Frameworks/GSS.framework/Versions/A/GSS
0x95888000 - 0x95929ffb  com.apple.framework.IOKit (2.0.2 - 1445.40.1) <EDA5B2F5-12B4-39EF-B5EF-899587AACDC5> /System/Library/Frameworks/IOKit.framework/Versions/A/IOKit
0x9592b000 - 0x95932fff  com.apple.IOSurface (209.2.2 - 209.2.2) <0CCA9904-FCBB-3278-96A1-714BDB961D8A> /System/Library/Frameworks/IOSurface.framework/Versions/A/IOSurface
0x95987000 - 0x95b0bff3  com.apple.ImageIO.framework (3.3.0 - 1713) <28D21D61-3526-33ED-92DC-08D4D67445CB> /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
0x95b0c000 - 0x95b10ffb  libGIF.dylib (1713) <47A6BFCC-6651-3AAE-A70C-7BA3717B13BB> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libGIF.dylib
0x95b11000 - 0x95c02fff  libJP2.dylib (1713) <D1075C88-406B-3EAF-9270-9B3762D97803> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJP2.dylib
0x95c03000 - 0x95c25ff7  libJPEG.dylib (1713) <EFD86068-C17E-32AD-B4A5-79C20BC93411> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJPEG.dylib
0x95f06000 - 0x95f2cff7  libPng.dylib (1713) <A10259A1-2581-3642-946D-5B3F101615EC> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libPng.dylib
0x95f2d000 - 0x95f2fffb  libRadiance.dylib (1713) <EE872547-4C9F-397B-B41B-C79FEEE68267> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libRadiance.dylib
0x95f30000 - 0x95f7afff  libTIFF.dylib (1713) <7A56E3C4-9965-3471-8E98-5F6B28D68FBF> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libTIFF.dylib
0x96897000 - 0x968affff  com.apple.Kerberos (3.0 - 1) <8A399DB7-5440-3EC0-A241-3DD10E82DDB2> /System/Library/Frameworks/Kerberos.framework/Versions/A/Kerberos
0x96f27000 - 0x96f9dfff  com.apple.Metal (124.7 - 124.7) <2617CDD0-32C6-358C-A61F-063737F916B3> /System/Library/Frameworks/Metal.framework/Versions/A/Metal
0x96f9f000 - 0x96fabfff  com.apple.NetFS (6.0 - 4.0) <F37A4DA0-AAB6-3F0B-BA18-E322BFA52CC4> /System/Library/Frameworks/NetFS.framework/Versions/A/NetFS
0x99c78000 - 0x99cc3fff  com.apple.opencl (2.8.12 - 2.8.12) <9AC78C72-CBBC-3D29-B553-AACC28014AEC> /System/Library/Frameworks/OpenCL.framework/Versions/A/OpenCL
0x99cc4000 - 0x99ce0fff  com.apple.CFOpenDirectory (10.13 - 207) <255284C6-7BDD-3A0B-A4A2-E43206611B91> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/Frameworks/CFOpenDirectory.framework/Versions/A/CFOpenDirectory
0x99ce1000 - 0x99cecfff  com.apple.OpenDirectory (10.13 - 207) <ECB33DA6-A0A3-3378-AB7A-2C10D395904E> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/OpenDirectory
0x9aef6000 - 0x9aef7fff  libCVMSPluginSupport.dylib (16.4.2) <909D788E-692E-3FF1-AFF0-2AB4609C53D7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCVMSPluginSupport.dylib
0x9aef8000 - 0x9aefcfff  libCoreFSCache.dylib (162.4) <D53B0D41-0774-3C6A-BB59-8BA7DB8A8374> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreFSCache.dylib
0x9aefd000 - 0x9af01fff  libCoreVMClient.dylib (162.4) <19767FEB-6A89-3892-8F18-1F9E73463050> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreVMClient.dylib
0x9af02000 - 0x9af0aff7  libGFXShared.dylib (16.4.2) <F62281F1-9495-3589-A576-75D84880D42D> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGFXShared.dylib
0x9af0b000 - 0x9af17fff  libGL.dylib (16.4.2) <028B909B-DD19-388B-8113-1850DFAD3DCA> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib
0x9af18000 - 0x9af53ffb  libGLImage.dylib (16.4.2) <AE5E3974-656A-3F88-956E-F28192BA98C3> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLImage.dylib
0x9b0cd000 - 0x9b10fff7  libGLU.dylib (16.4.2) <9E1283AA-A7E0-37BA-BDEB-EE5256D677C7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLU.dylib
0x9bab6000 - 0x9bac4fff  com.apple.opengl (16.4.2 - 16.4.2) <40645026-52DC-3CFC-8308-EFA2FA79D5A0> /System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
0x9c829000 - 0x9ca5fff7  com.apple.QuartzCore (1.11 - 584.8.102) <960628B2-C498-36C9-B0D4-F27D49DE029F> /System/Library/Frameworks/QuartzCore.framework/Versions/A/QuartzCore
0x9cef8000 - 0x9d225ff3  com.apple.security (7.0 - 58286.41.2) <F64D64CD-4E80-3384-92F0-56D2A464F7B4> /System/Library/Frameworks/Security.framework/Versions/A/Security
0x9d226000 - 0x9d2adff3  com.apple.securityfoundation (6.0 - 55185.30.4) <632548B9-3E24-32F4-BF50-F6BF71713363> /System/Library/Frameworks/SecurityFoundation.framework/Versions/A/SecurityFoundation
0x9d2d9000 - 0x9d2ddfff  com.apple.xpc.ServiceManagement (1.0 - 1) <B09C1309-46A2-3081-B489-DCE549A8BA46> /System/Library/Frameworks/ServiceManagement.framework/Versions/A/ServiceManagement
0x9d408000 - 0x9d478ff3  com.apple.SystemConfiguration (1.17 - 1.17) <318C287A-BB41-3F72-9095-9DFF0382CB77> /System/Library/Frameworks/SystemConfiguration.framework/Versions/A/SystemConfiguration
0x9f26e000 - 0x9f305ff7  com.apple.APFS (1.0 - 1) <1B119C37-9A4C-3204-9BE2-E4E00E657037> /System/Library/PrivateFrameworks/APFS.framework/Versions/A/APFS
0x9fa4b000 - 0x9fa88ff3  com.apple.AppleJPEG (1.0 - 1) <E87393BB-6140-389C-BF53-36B3985A56D3> /System/Library/PrivateFrameworks/AppleJPEG.framework/Versions/A/AppleJPEG
0x9fbdb000 - 0x9fbe2fff  com.apple.coreservices.BackgroundTaskManagement (1.0 - 57.1) <71FD5EA2-8D0B-3E21-94E4-6D5BD45005E7> /System/Library/PrivateFrameworks/BackgroundTaskManagement.framework/Versions/A/BackgroundTaskManagement
0x9fdb2000 - 0x9fdbbffb  com.apple.CommonAuth (4.0 - 2.0) <424B8D39-396A-3A78-84C1-5161B54A8F5B> /System/Library/PrivateFrameworks/CommonAuth.framework/Versions/A/CommonAuth
0xa022d000 - 0xa023dff7  com.apple.CoreEmoji (1.0 - 69.3) <165A133F-DED4-3B24-A9BF-6EA6F3F7A152> /System/Library/PrivateFrameworks/CoreEmoji.framework/Versions/A/CoreEmoji
0xa0c96000 - 0xa10c6ff7  com.apple.vision.FaceCore (3.3.2 - 3.3.2) <B2288C3D-E67F-3AAE-A652-E920CD19F267> /System/Library/PrivateFrameworks/FaceCore.framework/Versions/A/FaceCore
0xa3a6a000 - 0xa3addff3  com.apple.Heimdal (4.0 - 2.0) <560C8A98-E261-39C9-9862-3340EC6ABC9C> /System/Library/PrivateFrameworks/Heimdal.framework/Versions/A/Heimdal
0xa3d8f000 - 0xa3d95fff  com.apple.IOAccelerator (376.6 - 376.6) <4EFED596-8863-35F6-8EA3-CB11C5D9D157> /System/Library/PrivateFrameworks/IOAccelerator.framework/Versions/A/IOAccelerator
0xa3d96000 - 0xa3daeffb  com.apple.IOPresentment (1.0 - 32.1) <EBD4DB8D-03D3-3136-B431-969A2E5E1B91> /System/Library/PrivateFrameworks/IOPresentment.framework/Versions/A/IOPresentment
0xa3e62000 - 0xa3f56ff7  com.apple.LanguageModeling (1.0 - 159.3.1) <AA880B14-031D-33FB-9B48-0A8AAB7342C6> /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
0xa3f57000 - 0xa3f98ff7  com.apple.Lexicon-framework (1.0 - 33.2) <13FAB8A2-507A-3AEA-A571-27BDDFD96B31> /System/Library/PrivateFrameworks/Lexicon.framework/Versions/A/Lexicon
0xa3f9c000 - 0xa3fa2ff3  com.apple.LinguisticData (1.0 - 238.3) <C47B3EB0-0463-3613-8A09-344F1639EE92> /System/Library/PrivateFrameworks/LinguisticData.framework/Versions/A/LinguisticData
0xa4352000 - 0xa437bffb  com.apple.MultitouchSupport.framework (1204.13 - 1204.13) <01BDF9A5-8C83-3611-A999-F43F9121A173> /System/Library/PrivateFrameworks/MultitouchSupport.framework/Versions/A/MultitouchSupport
0xa449a000 - 0xa44a4fff  com.apple.NetAuth (6.2 - 6.2) <52F67DC1-8C96-3944-8E54-C02DD51FD9FC> /System/Library/PrivateFrameworks/NetAuth.framework/Versions/A/NetAuth
0xa4858000 - 0xa48ddffb  com.apple.SkyLight (1.600.0 - 312.23.4) <27154D6B-3C0F-383F-AA7B-F602C1F397CC> /System/Library/PrivateFrameworks/SkyLight.framework/Versions/A/SkyLight
0xa4cbb000 - 0xa4cc2fff  com.apple.TCC (1.0 - 1) <4B76752A-36A0-3175-87C7-CB42E33CCB5A> /System/Library/PrivateFrameworks/TCC.framework/Versions/A/TCC
0xa545c000 - 0xa545efff  com.apple.loginsupport (1.0 - 1) <086FAE1B-87E2-324A-AE76-E6EC0B5F1517> /System/Library/PrivateFrameworks/login.framework/Versions/A/Frameworks/loginsupport.framework/Versions/A/loginsupport
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa556c000 - 0xa55a3ff3  libCRFSuite.dylib (41) <7F584902-74F1-3362-935D-95F5E735F5E7> /usr/lib/libCRFSuite.dylib
0xa55a4000 - 0xa55aeffb  libChineseTokenizer.dylib (28) <1FF5A32D-E012-3E76-B738-FAC26AD2A39B> /usr/lib/libChineseTokenizer.dylib
0xa564a000 - 0xa564bfff  libDiagnosticMessagesClient.dylib (104) <6829B180-2556-3A7E-A2E6-BD4859DF30A7> /usr/lib/libDiagnosticMessagesClient.dylib
0xa567d000 - 0xa5867ff7  libFosl_dynamic.dylib (17.7) <DBE4D720-8A46-3879-AD2D-F9A8CE3E7476> /usr/lib/libFosl_dynamic.dylib
0xa586f000 - 0xa586ffff  libOpenScriptingUtil.dylib (174) <B7CEDC30-2D17-3896-9EFC-64DB3D11DF00> /usr/lib/libOpenScriptingUtil.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa58ce000 - 0xa58e3ff7  libapple_nghttp2.dylib (1.24) <480C0C04-2533-3D44-8232-006B6CBA7758> /usr/lib/libapple_nghttp2.dylib
0xa58e4000 - 0xa590ffff  libarchive.2.dylib (54) <D55C5F86-251D-3C33-A617-0C623D4F512E> /usr/lib/libarchive.2.dylib
0xa5a63000 - 0xa5a63ff3  libauto.dylib (187) <CE2A78CC-670F-3E07-9539-822DCD2F6084> /usr/lib/libauto.dylib
0xa5a64000 - 0xa5a74fff  libbsm.0.dylib (39) <067E9003-0673-32A3-9B40-492323182C5C> /usr/lib/libbsm.0.dylib
0xa5a75000 - 0xa5a81ff7  libbz2.1.0.dylib (38) <77C24A36-BE84-3702-A786-935C597A0A86> /usr/lib/libbz2.1.0.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa5aff000 - 0xa5b10ff7  libcmph.dylib (6) <EC7664F1-B5A1-37F4-B7DC-F6AC10587E35> /usr/lib/libcmph.dylib
0xa5b11000 - 0xa5b24ff7  libcompression.dylib (47) <F80DDFC1-F96A-3BAD-967D-C1E24253273A> /usr/lib/libcompression.dylib
0xa5b25000 - 0xa5b3cffb  libcoretls.dylib (155) <F66FAEBC-4B6E-31E0-ACA8-C8ACBC7689DD> /usr/lib/libcoretls.dylib
0xa5b3d000 - 0xa5b3efff  libcoretls_cfhelpers.dylib (155) <8B8ABC2C-F251-3C80-9747-88C05A2CBE64> /usr/lib/libcoretls_cfhelpers.dylib
0xa6026000 - 0xa607dfff  libcups.2.dylib (462.1) <0180AE97-A19F-3D49-9838-06995E73F572> /usr/lib/libcups.2.dylib
0xa6193000 - 0xa6193fff  libenergytrace.dylib (16) <34FC43C7-D9B6-3C01-8B65-E49059D31279> /usr/lib/libenergytrace.dylib
0xa61c7000 - 0xa61cbfff  libheimdal-asn1.dylib (520.30.1) <DEA7E913-118F-333E-BE08-5B4F19B33B9A> /usr/lib/libheimdal-asn1.dylib
0xa61f7000 - 0xa62e7ff3  libiconv.2.dylib (51) <FE6D05A5-18DB-3FD8-A52F-B7BADB232C78> /usr/lib/libiconv.2.dylib
0xa62e8000 - 0xa650aff7  libicucore.A.dylib (59152.0.1) <35D52BFF-C74C-3519-AEAC-7371E3C7E4BD> /usr/lib/libicucore.A.dylib
0xa6552000 - 0xa6553fff  liblangid.dylib (128) <120FE992-47E4-3A73-A039-1B401F5696DC> /usr/lib/liblangid.dylib
0xa6554000 - 0xa656cff7  liblzma.5.dylib (10) <8A5C9679-430A-3A19-AF68-9D21BAC442C7> /usr/lib/liblzma.5.dylib
0xa656d000 - 0xa6582fff  libmarisa.dylib (9) <805453EE-B829-3DA5-8DF3-5132D03D5B74> /usr/lib/libmarisa.dylib
0xa6637000 - 0xa6854fff  libmecabra.dylib (779.7.6) <3C7F6A43-B17C-3673-A0AC-14FFE08370D4> /usr/lib/libmecabra.dylib
0xa6a1b000 - 0xa6aeffff  libnetwork.dylib (1229.30.11) <E4008EDE-F873-33FF-BD96-7DB14FA4F364> /usr/lib/libnetwork.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6ed4000 - 0xa6ed7fff  libpam.2.dylib (22) <7106F43C-84DD-3F26-905A-B52780AFEB3E> /usr/lib/libpam.2.dylib
0xa6eda000 - 0xa6f0bfff  libpcap.A.dylib (79.20.1) <154889CF-5F83-3012-953E-0FC8FEE50FF8> /usr/lib/libpcap.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa6faf000 - 0xa7139ff7  libsqlite3.dylib (274.5) <B09AF63F-4F1A-3481-9B61-6EBB64D12EB9> /usr/lib/libsqlite3.dylib
0xa72dd000 - 0xa7317ffb  libusrtcp.dylib (1229.30.11) <39D76669-A48B-3BAC-8F45-1D6CA87E9B4B> /usr/lib/libusrtcp.dylib
0xa7318000 - 0xa731bff7  libutil.dylib (51.20.1) <86BD9675-16A2-345D-9B8D-E8A3397F2365> /usr/lib/libutil.dylib
0xa731c000 - 0xa732aff7  libxar.1.dylib (400) <4B664A7E-EC05-34AD-ACC6-C879B69DBA7C> /usr/lib/libxar.1.dylib
0xa732b000 - 0xa7409ff7  libxml2.2.dylib (31.7) <3E1F9E3D-6C44-3437-AB2B-E5ACE1927F81> /usr/lib/libxml2.2.dylib
0xa740a000 - 0xa7432ff3  libxslt.1.dylib (15.10) <1A3DC7B8-7C92-3D73-BF82-D60E64BC3DF0> /usr/lib/libxslt.1.dylib
0xa7433000 - 0xa7442ff7  libz.1.dylib (70) <588F445F-0065-3D77-8002-BA9411DA1D70> /usr/lib/libz.1.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75c4000 - 0xa75d0ff7  libkxld.dylib (4570.41.2) <C01D2E6F-B29E-3795-9258-55445BF8F933> /usr/lib/system/libkxld.dylib
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
    task_for_pid: 2358
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=168.2M resident=0K(0%) swapped_out_or_unallocated=168.2M(100%)
Writable regions: Total=92.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=92.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            28.1M       14 
MALLOC guard page                   32K        9 
Stack Guard                          4K        2 
__DATA                            9332K      164 
__FONT_DATA                          4K        2 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            587.6M      551 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-13-161637-1_Traviss-Mac-1044.crash
Process:               a [50641]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [50632]
Responsible:           a [50641]
User ID:               501
Date/Time:             2019-04-13 16:16:27.588 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5400 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b06cbe68
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb06cbe68:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b06cb000-00000000b06cc000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b06cc000-00000000b08cd000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-d72d99315c451a4c.dylib  0x00179ff0 std::sys::unix::thread::Thread::join::h574b8a0b5ce79d79 + 32
4   a                              0x0002b646 std::thread::JoinHandle$LT$T$GT$::join::hab5c6f60cb1543af + 70
5   a                              0x0002a865 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x0002967b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h97d012a7cecd90df + 11
7   libstd-d72d99315c451a4c.dylib  0x0016a14c std::sys_common::backtrace::__rust_begin_short_backtrace::h258c53a673ab9f0e + 12
8   libstd-d72d99315c451a4c.dylib  0x0016cca4 std::panicking::try::do_call::hdeca090a8a640010 + 20
9   libstd-d72d99315c451a4c.dylib  0x0017b3cd __rust_maybe_catch_panic + 29
10  libstd-d72d99315c451a4c.dylib  0x0016d747 std::rt::lang_start_internal::h9bb6eb8f317fc6bd + 631
11  a                              0x0002b28c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-d72d99315c451a4c.dylib  0x0017a7eb std::sys::unix::abort_internal::h549e70aace565765 + 11
4   libstd-d72d99315c451a4c.dylib  0x0016b129 std::sys_common::util::abort::hdcba088e127a7962 + 73
5   libstd-d72d99315c451a4c.dylib  0x00179a88 std::sys::unix::stack_overflow::imp::signal_handler::h6683de7f5209c2e9 + 952
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-d72d99315c451a4c.dylib  0x001796d0 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::hfa9e09a96917c8fc + 80
9   a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x0002a9b0 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x000296dd std::sys_common::backtrace::__rust_begin_short_backtrace::h394204c0f175d3b4 (.llvm.13704876518820267383) + 29
265 libstd-d72d99315c451a4c.dylib  0x0017b3cd __rust_maybe_catch_panic + 29
266 a                              0x0002b9a3 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h21b8353081c32e65 + 131
267 libstd-d72d99315c451a4c.dylib  0x00151601 _$LT$alloc..boxed..Box$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$A$GT$$GT$::call_once::h2ffc491806af3ead + 65
268 libstd-d72d99315c451a4c.dylib  0x00179f28 std::sys::unix::thread::Thread::new::thread_start::h8464f3a1bf5e8822 + 184
269 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
270 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
271 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb08cc000  ecx: 0x00088b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00088b38  esp: 0x00088b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00179b10
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0x28000 -    0x2cff3 +a (0) <9DE3FA69-79AB-31D6-8B94-D299E4180B5B> /Users/USER/*/a
   0xc2000 -   0x107fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x14b000 -   0x1dafff +libstd-d72d99315c451a4c.dylib (0) <DC60EA74-DA2D-3CD0-B0C3-E1E6BA9E45E4> /Users/USER/*/libstd-d72d99315c451a4c.dylib
0x90298000 - 0x90298fff  com.apple.Accelerate (1.11 - Accelerate 1.11) <F4A138F5-290D-3413-AD17-ECD395935FF3> /System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
0x902b0000 - 0x909f1fdf  com.apple.vImage (8.1 - ???) <591C941E-6475-347E-89DA-F541E88F949A> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vImage.framework/Versions/A/vImage
0x909f2000 - 0x90b2dff7  libBLAS.dylib (1211.30.1) <A850E0E2-3A72-3916-9907-AF1E7ECC95F0> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBLAS.dylib
0x90b2e000 - 0x90b5bffb  libBNNS.dylib (37) <C29094A0-5C89-3C5E-AB37-510C28588E2E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBNNS.dylib
0x90b5c000 - 0x90ecffff  libLAPACK.dylib (1211.30.1) <2DDDE838-0FF1-3679-8E62-9C09923ECB7E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLAPACK.dylib
0x90ed0000 - 0x90ee6ffb  libLinearAlgebra.dylib (1211.30.1) <8A120E75-CAF4-3CAE-BBE6-E2F5FAE44DB8> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLinearAlgebra.dylib
0x90ee7000 - 0x90f00ff7  libSparseBLAS.dylib (1211.30.1) <0C5E0EF4-E9A5-3FC4-B7A3-1FE59DB4A2AA> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparseBLAS.dylib
0x90f01000 - 0x9105ffc7  libvDSP.dylib (622.20.8) <C5F16300-061F-3DF0-B91E-8BD0D2173351> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvDSP.dylib
0x91060000 - 0x9113effb  libvMisc.dylib (622.20.8) <1C8D5D80-F32C-3853-8309-57C8A82B7DA5> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvMisc.dylib
0x9113f000 - 0x9113ffff  com.apple.Accelerate.vecLib (3.11 - vecLib 3.11) <7A0D5DD6-C302-390D-9178-0B2EA94BB5ED> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/vecLib
0x92146000 - 0x92146fff  com.apple.ApplicationServices (48 - 50) <BFE7FB45-365B-341F-A8FC-B9483AE87709> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/ApplicationServices
0x92147000 - 0x921adff3  com.apple.ApplicationServices.ATS (377 - 445) <CD3D5685-2BB9-3A7B-AC97-2A74A81CB7CC> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/ATS
0x921b0000 - 0x922d4ff3  libFontParser.dylib (222.1.2) <8F7D388A-299C-3C6D-9864-40EC0914A96B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontParser.dylib
0x922d5000 - 0x92321ffb  libFontRegistry.dylib (221) <8D81FDCF-F05D-3556-AB6D-090F9508C25E> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontRegistry.dylib
0x9240f000 - 0x92414fff  com.apple.ColorSyncLegacy (4.13.0 - 1) <AB5CE7D2-8BE5-35C8-A9D5-ED0FB5BAB7D1> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ColorSyncLegacy.framework/Versions/A/ColorSyncLegacy
0x924be000 - 0x92515ff7  com.apple.HIServices (1.22 - 622) <8544026A-17BE-301D-BA2A-782F3AD864DA> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/HIServices.framework/Versions/A/HIServices
0x92516000 - 0x92525ff7  com.apple.LangAnalysis (1.7.0 - 1.7.0) <E3245701-039B-353F-923D-F81B2242842C> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/LangAnalysis.framework/Versions/A/LangAnalysis
0x92526000 - 0x9257effb  com.apple.print.framework.PrintCore (13 - 503) <FD0F7A18-6F78-34C9-B067-B3AB76C3D4C8> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/PrintCore
0x9257f000 - 0x92615ffb  com.apple.QD (3.12 - 403) <372AFF26-17D1-3C6F-9E47-17C955C2045B> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/QD.framework/Versions/A/QD
0x92616000 - 0x92622ff3  com.apple.speech.synthesis.framework (7.4.1 - 7.4.1) <3AE4F801-4A2D-3AB3-AA31-B27F7A7131F4> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/SpeechSynthesis.framework/Versions/A/SpeechSynthesis
0x92623000 - 0x9286ffff  com.apple.audio.toolbox.AudioToolbox (1.14 - 1.14) <E4585EFD-C3B6-327F-88E4-B3BADDA6B08D> /System/Library/Frameworks/AudioToolbox.framework/Versions/A/AudioToolbox
0x92b8f000 - 0x92efcffb  com.apple.CFNetwork (893.13.1 - 893.13.1) <63A5C550-5F0F-3FF1-9061-55E1766B3512> /System/Library/Frameworks/CFNetwork.framework/Versions/A/CFNetwork
0x93426000 - 0x934e5ff7  com.apple.ColorSync (4.13.0 - 546) <DACC5623-8E37-3134-9562-4E8601127F67> /System/Library/Frameworks/ColorSync.framework/Versions/A/ColorSync
0x934e6000 - 0x93581fff  com.apple.audio.CoreAudio (4.3.0 - 4.3.0) <ABA90687-71A6-3431-94A1-0E7E74FE407C> /System/Library/Frameworks/CoreAudio.framework/Versions/A/CoreAudio
0x935e5000 - 0x938c5fff  com.apple.CoreData (120 - 849.2) <B8011F5E-7A2B-349B-AFF1-17EC8D8465AB> /System/Library/Frameworks/CoreData.framework/Versions/A/CoreData
0x938c6000 - 0x938ccff3  com.apple.CoreDisplay (1.0 - 81.7) <E6BFD0F5-A45B-3FE3-BA26-14D2CD970CFF> /System/Library/Frameworks/CoreDisplay.framework/Versions/A/CoreDisplay
0x938cd000 - 0x93d56ff7  com.apple.CoreFoundation (6.9 - 1451) <727B43E3-A1AC-31EC-97A4-F179FE11D04A> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
0x93d58000 - 0x94387ffb  com.apple.CoreGraphics (2.0 - 1129.5) <20752785-E9DA-3CC6-ACDD-5A82AD344209> /System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics
0x94389000 - 0x945ffffb  com.apple.CoreImage (13.0.0 - 579.2.9) <2498F44C-7350-397B-A075-206A91D75ABB> /System/Library/Frameworks/CoreImage.framework/Versions/A/CoreImage
0x947f1000 - 0x947f1fff  com.apple.CoreServices (822.19 - 822.19) <6B5DC8C1-4237-3ADA-B8C8-F926943E6101> /System/Library/Frameworks/CoreServices.framework/Versions/A/CoreServices
0x947f2000 - 0x94864ff3  com.apple.AE (735.1 - 735.1) <3E1B0CED-0AC3-3252-AEDD-5D8F91E3AAA7> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/AE.framework/Versions/A/AE
0x94865000 - 0x94b43ffb  com.apple.CoreServices.CarbonCore (1178.2 - 1178.2) <E61EA71D-294F-3C8B-95BD-0CDBA0FFC907> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/CarbonCore
0x94b44000 - 0x94b78ff3  com.apple.DictionaryServices (1.2 - 284) <56BEF6B8-50D2-38A0-9EF2-D7093E9AAB56> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/DictionaryServices.framework/Versions/A/DictionaryServices
0x94b79000 - 0x94b81fff  com.apple.CoreServices.FSEvents (1239 - 1239) <CABC21F7-E3AB-3954-ACBE-B8066A37516A> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/FSEvents
0x94b82000 - 0x94ce0fff  com.apple.LaunchServices (822.19 - 822.19) <AC40752F-0F99-3EB1-8D25-2C65F9BAC226> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/LaunchServices
0x94ce1000 - 0x94d8dff7  com.apple.Metadata (10.7.0 - 1191.2.6) <6CE69880-6AF3-3A1A-A8E0-F89FC750EE4C> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Metadata
0x94d8e000 - 0x94decff7  com.apple.CoreServices.OSServices (822.19 - 822.19) <38B64F72-5CAE-374A-8BBC-0EAB7C6A6777> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/OSServices.framework/Versions/A/OSServices
0x94ded000 - 0x94e5eff3  com.apple.SearchKit (1.4.0 - 1.4.0) <FAD60011-970B-3889-B6BD-3715CCF599CA> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SearchKit.framework/Versions/A/SearchKit
0x94e5f000 - 0x94e82fff  com.apple.coreservices.SharedFileList (71.4 - 71.4) <CD97E31D-0354-36AB-9997-C4FAF3221D30> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SharedFileList.framework/Versions/A/SharedFileList
0x94e83000 - 0x94fceffb  com.apple.CoreText (352.0 - 578.12) <6389222B-6B26-3F46-93C2-ABE07168227F> /System/Library/Frameworks/CoreText.framework/Versions/A/CoreText
0x94fcf000 - 0x95009ff3  com.apple.CoreVideo (1.8 - 279.2) <0D75C395-3C86-3539-9206-C7A330BE3551> /System/Library/Frameworks/CoreVideo.framework/Versions/A/CoreVideo
0x952e4000 - 0x952edff7  com.apple.DiskArbitration (2.7 - 2.7) <E3552A79-57A4-36AE-8B54-5FE2EB5193DA> /System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration
0x952fe000 - 0x9566dffb  com.apple.Foundation (6.9 - 1451) <E815D5AF-B627-3BF4-8156-4F514FCFD765> /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
0x956ae000 - 0x956ddff3  com.apple.GSS (4.0 - 2.0) <78C94D11-21DF-34C6-B4E8-88564551D67E> /System/Library/Frameworks/GSS.framework/Versions/A/GSS
0x95888000 - 0x95929ffb  com.apple.framework.IOKit (2.0.2 - 1445.40.1) <EDA5B2F5-12B4-39EF-B5EF-899587AACDC5> /System/Library/Frameworks/IOKit.framework/Versions/A/IOKit
0x9592b000 - 0x95932fff  com.apple.IOSurface (209.2.2 - 209.2.2) <0CCA9904-FCBB-3278-96A1-714BDB961D8A> /System/Library/Frameworks/IOSurface.framework/Versions/A/IOSurface
0x95987000 - 0x95b0bff3  com.apple.ImageIO.framework (3.3.0 - 1713) <28D21D61-3526-33ED-92DC-08D4D67445CB> /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
0x95b0c000 - 0x95b10ffb  libGIF.dylib (1713) <47A6BFCC-6651-3AAE-A70C-7BA3717B13BB> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libGIF.dylib
0x95b11000 - 0x95c02fff  libJP2.dylib (1713) <D1075C88-406B-3EAF-9270-9B3762D97803> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJP2.dylib
0x95c03000 - 0x95c25ff7  libJPEG.dylib (1713) <EFD86068-C17E-32AD-B4A5-79C20BC93411> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJPEG.dylib
0x95f06000 - 0x95f2cff7  libPng.dylib (1713) <A10259A1-2581-3642-946D-5B3F101615EC> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libPng.dylib
0x95f2d000 - 0x95f2fffb  libRadiance.dylib (1713) <EE872547-4C9F-397B-B41B-C79FEEE68267> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libRadiance.dylib
0x95f30000 - 0x95f7afff  libTIFF.dylib (1713) <7A56E3C4-9965-3471-8E98-5F6B28D68FBF> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libTIFF.dylib
0x96897000 - 0x968affff  com.apple.Kerberos (3.0 - 1) <8A399DB7-5440-3EC0-A241-3DD10E82DDB2> /System/Library/Frameworks/Kerberos.framework/Versions/A/Kerberos
0x96f27000 - 0x96f9dfff  com.apple.Metal (124.7 - 124.7) <2617CDD0-32C6-358C-A61F-063737F916B3> /System/Library/Frameworks/Metal.framework/Versions/A/Metal
0x96f9f000 - 0x96fabfff  com.apple.NetFS (6.0 - 4.0) <F37A4DA0-AAB6-3F0B-BA18-E322BFA52CC4> /System/Library/Frameworks/NetFS.framework/Versions/A/NetFS
0x99c78000 - 0x99cc3fff  com.apple.opencl (2.8.12 - 2.8.12) <9AC78C72-CBBC-3D29-B553-AACC28014AEC> /System/Library/Frameworks/OpenCL.framework/Versions/A/OpenCL
0x99cc4000 - 0x99ce0fff  com.apple.CFOpenDirectory (10.13 - 207) <255284C6-7BDD-3A0B-A4A2-E43206611B91> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/Frameworks/CFOpenDirectory.framework/Versions/A/CFOpenDirectory
0x99ce1000 - 0x99cecfff  com.apple.OpenDirectory (10.13 - 207) <ECB33DA6-A0A3-3378-AB7A-2C10D395904E> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/OpenDirectory
0x9aef6000 - 0x9aef7fff  libCVMSPluginSupport.dylib (16.4.2) <909D788E-692E-3FF1-AFF0-2AB4609C53D7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCVMSPluginSupport.dylib
0x9aef8000 - 0x9aefcfff  libCoreFSCache.dylib (162.4) <D53B0D41-0774-3C6A-BB59-8BA7DB8A8374> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreFSCache.dylib
0x9aefd000 - 0x9af01fff  libCoreVMClient.dylib (162.4) <19767FEB-6A89-3892-8F18-1F9E73463050> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreVMClient.dylib
0x9af02000 - 0x9af0aff7  libGFXShared.dylib (16.4.2) <F62281F1-9495-3589-A576-75D84880D42D> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGFXShared.dylib
0x9af0b000 - 0x9af17fff  libGL.dylib (16.4.2) <028B909B-DD19-388B-8113-1850DFAD3DCA> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib
0x9af18000 - 0x9af53ffb  libGLImage.dylib (16.4.2) <AE5E3974-656A-3F88-956E-F28192BA98C3> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLImage.dylib
0x9b0cd000 - 0x9b10fff7  libGLU.dylib (16.4.2) <9E1283AA-A7E0-37BA-BDEB-EE5256D677C7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLU.dylib
0x9bab6000 - 0x9bac4fff  com.apple.opengl (16.4.2 - 16.4.2) <40645026-52DC-3CFC-8308-EFA2FA79D5A0> /System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
0x9c829000 - 0x9ca5fff7  com.apple.QuartzCore (1.11 - 584.8.102) <960628B2-C498-36C9-B0D4-F27D49DE029F> /System/Library/Frameworks/QuartzCore.framework/Versions/A/QuartzCore
0x9cef8000 - 0x9d225ff3  com.apple.security (7.0 - 58286.41.2) <F64D64CD-4E80-3384-92F0-56D2A464F7B4> /System/Library/Frameworks/Security.framework/Versions/A/Security
0x9d226000 - 0x9d2adff3  com.apple.securityfoundation (6.0 - 55185.30.4) <632548B9-3E24-32F4-BF50-F6BF71713363> /System/Library/Frameworks/SecurityFoundation.framework/Versions/A/SecurityFoundation
0x9d2d9000 - 0x9d2ddfff  com.apple.xpc.ServiceManagement (1.0 - 1) <B09C1309-46A2-3081-B489-DCE549A8BA46> /System/Library/Frameworks/ServiceManagement.framework/Versions/A/ServiceManagement
0x9d408000 - 0x9d478ff3  com.apple.SystemConfiguration (1.17 - 1.17) <318C287A-BB41-3F72-9095-9DFF0382CB77> /System/Library/Frameworks/SystemConfiguration.framework/Versions/A/SystemConfiguration
0x9f26e000 - 0x9f305ff7  com.apple.APFS (1.0 - 1) <1B119C37-9A4C-3204-9BE2-E4E00E657037> /System/Library/PrivateFrameworks/APFS.framework/Versions/A/APFS
0x9fa4b000 - 0x9fa88ff3  com.apple.AppleJPEG (1.0 - 1) <E87393BB-6140-389C-BF53-36B3985A56D3> /System/Library/PrivateFrameworks/AppleJPEG.framework/Versions/A/AppleJPEG
0x9fbdb000 - 0x9fbe2fff  com.apple.coreservices.BackgroundTaskManagement (1.0 - 57.1) <71FD5EA2-8D0B-3E21-94E4-6D5BD45005E7> /System/Library/PrivateFrameworks/BackgroundTaskManagement.framework/Versions/A/BackgroundTaskManagement
0x9fdb2000 - 0x9fdbbffb  com.apple.CommonAuth (4.0 - 2.0) <424B8D39-396A-3A78-84C1-5161B54A8F5B> /System/Library/PrivateFrameworks/CommonAuth.framework/Versions/A/CommonAuth
0xa022d000 - 0xa023dff7  com.apple.CoreEmoji (1.0 - 69.3) <165A133F-DED4-3B24-A9BF-6EA6F3F7A152> /System/Library/PrivateFrameworks/CoreEmoji.framework/Versions/A/CoreEmoji
0xa0c96000 - 0xa10c6ff7  com.apple.vision.FaceCore (3.3.2 - 3.3.2) <B2288C3D-E67F-3AAE-A652-E920CD19F267> /System/Library/PrivateFrameworks/FaceCore.framework/Versions/A/FaceCore
0xa3a6a000 - 0xa3addff3  com.apple.Heimdal (4.0 - 2.0) <560C8A98-E261-39C9-9862-3340EC6ABC9C> /System/Library/PrivateFrameworks/Heimdal.framework/Versions/A/Heimdal
0xa3d8f000 - 0xa3d95fff  com.apple.IOAccelerator (376.6 - 376.6) <4EFED596-8863-35F6-8EA3-CB11C5D9D157> /System/Library/PrivateFrameworks/IOAccelerator.framework/Versions/A/IOAccelerator
0xa3d96000 - 0xa3daeffb  com.apple.IOPresentment (1.0 - 32.1) <EBD4DB8D-03D3-3136-B431-969A2E5E1B91> /System/Library/PrivateFrameworks/IOPresentment.framework/Versions/A/IOPresentment
0xa3e62000 - 0xa3f56ff7  com.apple.LanguageModeling (1.0 - 159.3.1) <AA880B14-031D-33FB-9B48-0A8AAB7342C6> /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
0xa3f57000 - 0xa3f98ff7  com.apple.Lexicon-framework (1.0 - 33.2) <13FAB8A2-507A-3AEA-A571-27BDDFD96B31> /System/Library/PrivateFrameworks/Lexicon.framework/Versions/A/Lexicon
0xa3f9c000 - 0xa3fa2ff3  com.apple.LinguisticData (1.0 - 238.3) <C47B3EB0-0463-3613-8A09-344F1639EE92> /System/Library/PrivateFrameworks/LinguisticData.framework/Versions/A/LinguisticData
0xa4352000 - 0xa437bffb  com.apple.MultitouchSupport.framework (1204.13 - 1204.13) <01BDF9A5-8C83-3611-A999-F43F9121A173> /System/Library/PrivateFrameworks/MultitouchSupport.framework/Versions/A/MultitouchSupport
0xa449a000 - 0xa44a4fff  com.apple.NetAuth (6.2 - 6.2) <52F67DC1-8C96-3944-8E54-C02DD51FD9FC> /System/Library/PrivateFrameworks/NetAuth.framework/Versions/A/NetAuth
0xa4858000 - 0xa48ddffb  com.apple.SkyLight (1.600.0 - 312.23.4) <27154D6B-3C0F-383F-AA7B-F602C1F397CC> /System/Library/PrivateFrameworks/SkyLight.framework/Versions/A/SkyLight
0xa4cbb000 - 0xa4cc2fff  com.apple.TCC (1.0 - 1) <4B76752A-36A0-3175-87C7-CB42E33CCB5A> /System/Library/PrivateFrameworks/TCC.framework/Versions/A/TCC
0xa545c000 - 0xa545efff  com.apple.loginsupport (1.0 - 1) <086FAE1B-87E2-324A-AE76-E6EC0B5F1517> /System/Library/PrivateFrameworks/login.framework/Versions/A/Frameworks/loginsupport.framework/Versions/A/loginsupport
0xa54e9000 - 0xa551cff7  libclosured.dylib (519.2.2) <E0E52FC3-51A9-385F-953D-23A7CA8D5666> /usr/lib/closure/libclosured.dylib
0xa556c000 - 0xa55a3ff3  libCRFSuite.dylib (41) <7F584902-74F1-3362-935D-95F5E735F5E7> /usr/lib/libCRFSuite.dylib
0xa55a4000 - 0xa55aeffb  libChineseTokenizer.dylib (28) <1FF5A32D-E012-3E76-B738-FAC26AD2A39B> /usr/lib/libChineseTokenizer.dylib
0xa564a000 - 0xa564bfff  libDiagnosticMessagesClient.dylib (104) <6829B180-2556-3A7E-A2E6-BD4859DF30A7> /usr/lib/libDiagnosticMessagesClient.dylib
0xa567d000 - 0xa5867ff7  libFosl_dynamic.dylib (17.7) <DBE4D720-8A46-3879-AD2D-F9A8CE3E7476> /usr/lib/libFosl_dynamic.dylib
0xa586f000 - 0xa586ffff  libOpenScriptingUtil.dylib (174) <B7CEDC30-2D17-3896-9EFC-64DB3D11DF00> /usr/lib/libOpenScriptingUtil.dylib
0xa58be000 - 0xa58bffff  libSystem.B.dylib (1252) <D7139382-C03A-377B-9F91-DAC2C5296343> /usr/lib/libSystem.B.dylib
0xa58ce000 - 0xa58e3ff7  libapple_nghttp2.dylib (1.24) <480C0C04-2533-3D44-8232-006B6CBA7758> /usr/lib/libapple_nghttp2.dylib
0xa58e4000 - 0xa590ffff  libarchive.2.dylib (54) <D55C5F86-251D-3C33-A617-0C623D4F512E> /usr/lib/libarchive.2.dylib
0xa5a63000 - 0xa5a63ff3  libauto.dylib (187) <CE2A78CC-670F-3E07-9539-822DCD2F6084> /usr/lib/libauto.dylib
0xa5a64000 - 0xa5a74fff  libbsm.0.dylib (39) <067E9003-0673-32A3-9B40-492323182C5C> /usr/lib/libbsm.0.dylib
0xa5a75000 - 0xa5a81ff7  libbz2.1.0.dylib (38) <77C24A36-BE84-3702-A786-935C597A0A86> /usr/lib/libbz2.1.0.dylib
0xa5a82000 - 0xa5adbffb  libc++.1.dylib (400.9) <AD612EEF-6CE3-315D-82C2-58248BE13431> /usr/lib/libc++.1.dylib
0xa5adc000 - 0xa5afdfff  libc++abi.dylib (400.7) <41323E53-C7EA-3E9A-BD30-38E82399F843> /usr/lib/libc++abi.dylib
0xa5aff000 - 0xa5b10ff7  libcmph.dylib (6) <EC7664F1-B5A1-37F4-B7DC-F6AC10587E35> /usr/lib/libcmph.dylib
0xa5b11000 - 0xa5b24ff7  libcompression.dylib (47) <F80DDFC1-F96A-3BAD-967D-C1E24253273A> /usr/lib/libcompression.dylib
0xa5b25000 - 0xa5b3cffb  libcoretls.dylib (155) <F66FAEBC-4B6E-31E0-ACA8-C8ACBC7689DD> /usr/lib/libcoretls.dylib
0xa5b3d000 - 0xa5b3efff  libcoretls_cfhelpers.dylib (155) <8B8ABC2C-F251-3C80-9747-88C05A2CBE64> /usr/lib/libcoretls_cfhelpers.dylib
0xa6026000 - 0xa607dfff  libcups.2.dylib (462.1) <0180AE97-A19F-3D49-9838-06995E73F572> /usr/lib/libcups.2.dylib
0xa6193000 - 0xa6193fff  libenergytrace.dylib (16) <34FC43C7-D9B6-3C01-8B65-E49059D31279> /usr/lib/libenergytrace.dylib
0xa61c7000 - 0xa61cbfff  libheimdal-asn1.dylib (520.30.1) <DEA7E913-118F-333E-BE08-5B4F19B33B9A> /usr/lib/libheimdal-asn1.dylib
0xa61f7000 - 0xa62e7ff3  libiconv.2.dylib (51) <FE6D05A5-18DB-3FD8-A52F-B7BADB232C78> /usr/lib/libiconv.2.dylib
0xa62e8000 - 0xa650aff7  libicucore.A.dylib (59152.0.1) <35D52BFF-C74C-3519-AEAC-7371E3C7E4BD> /usr/lib/libicucore.A.dylib
0xa6552000 - 0xa6553fff  liblangid.dylib (128) <120FE992-47E4-3A73-A039-1B401F5696DC> /usr/lib/liblangid.dylib
0xa6554000 - 0xa656cff7  liblzma.5.dylib (10) <8A5C9679-430A-3A19-AF68-9D21BAC442C7> /usr/lib/liblzma.5.dylib
0xa656d000 - 0xa6582fff  libmarisa.dylib (9) <805453EE-B829-3DA5-8DF3-5132D03D5B74> /usr/lib/libmarisa.dylib
0xa6637000 - 0xa6854fff  libmecabra.dylib (779.7.6) <3C7F6A43-B17C-3673-A0AC-14FFE08370D4> /usr/lib/libmecabra.dylib
0xa6a1b000 - 0xa6aeffff  libnetwork.dylib (1229.30.11) <E4008EDE-F873-33FF-BD96-7DB14FA4F364> /usr/lib/libnetwork.dylib
0xa6af0000 - 0xa6ed00fb  libobjc.A.dylib (723) <4AF346B8-C1A8-3199-B1BB-ADEDD64D5C1A> /usr/lib/libobjc.A.dylib
0xa6ed4000 - 0xa6ed7fff  libpam.2.dylib (22) <7106F43C-84DD-3F26-905A-B52780AFEB3E> /usr/lib/libpam.2.dylib
0xa6eda000 - 0xa6f0bfff  libpcap.A.dylib (79.20.1) <154889CF-5F83-3012-953E-0FC8FEE50FF8> /usr/lib/libpcap.A.dylib
0xa6f49000 - 0xa6f64ffb  libresolv.9.dylib (65) <65A43F5B-CF88-3948-AE5C-D7CA02D814A1> /usr/lib/libresolv.9.dylib
0xa6faf000 - 0xa7139ff7  libsqlite3.dylib (274.5) <B09AF63F-4F1A-3481-9B61-6EBB64D12EB9> /usr/lib/libsqlite3.dylib
0xa72dd000 - 0xa7317ffb  libusrtcp.dylib (1229.30.11) <39D76669-A48B-3BAC-8F45-1D6CA87E9B4B> /usr/lib/libusrtcp.dylib
0xa7318000 - 0xa731bff7  libutil.dylib (51.20.1) <86BD9675-16A2-345D-9B8D-E8A3397F2365> /usr/lib/libutil.dylib
0xa731c000 - 0xa732aff7  libxar.1.dylib (400) <4B664A7E-EC05-34AD-ACC6-C879B69DBA7C> /usr/lib/libxar.1.dylib
0xa732b000 - 0xa7409ff7  libxml2.2.dylib (31.7) <3E1F9E3D-6C44-3437-AB2B-E5ACE1927F81> /usr/lib/libxml2.2.dylib
0xa740a000 - 0xa7432ff3  libxslt.1.dylib (15.10) <1A3DC7B8-7C92-3D73-BF82-D60E64BC3DF0> /usr/lib/libxslt.1.dylib
0xa7433000 - 0xa7442ff7  libz.1.dylib (70) <588F445F-0065-3D77-8002-BA9411DA1D70> /usr/lib/libz.1.dylib
0xa747c000 - 0xa7480fff  libcache.dylib (80) <5D011637-CADA-38A1-A695-CE049657FD9D> /usr/lib/system/libcache.dylib
0xa7481000 - 0xa748bfff  libcommonCrypto.dylib (60118.30.2) <38B2C15B-D27F-3106-A337-F72F29844825> /usr/lib/system/libcommonCrypto.dylib
0xa748c000 - 0xa7491fff  libcompiler_rt.dylib (62) <FA07FEE2-CEFE-3CC0-A82F-E601AA2CCB36> /usr/lib/system/libcompiler_rt.dylib
0xa7492000 - 0xa749bff3  libcopyfile.dylib (146.30.2) <F3A05833-AD1C-3E3A-8100-847297C882FC> /usr/lib/system/libcopyfile.dylib
0xa749c000 - 0xa7504ff7  libcorecrypto.dylib (562.30.10) <0D8A61F8-2D7D-31F1-93AB-0597D80CCA85> /usr/lib/system/libcorecrypto.dylib
0xa756f000 - 0xa75a4fff  libdispatch.dylib (913.30.4) <D1812254-DE85-3A5B-AD7B-5CE23BB8C9E1> /usr/lib/system/libdispatch.dylib
0xa75a5000 - 0xa75c2fff  libdyld.dylib (519.2.2) <A79B6A65-DDAA-31C5-B66B-95FB343125BE> /usr/lib/system/libdyld.dylib
0xa75c3000 - 0xa75c3fff  libkeymgr.dylib (28) <C448ACFC-DD1B-3F08-B4C3-D2B69D1210B1> /usr/lib/system/libkeymgr.dylib
0xa75c4000 - 0xa75d0ff7  libkxld.dylib (4570.41.2) <C01D2E6F-B29E-3795-9258-55445BF8F933> /usr/lib/system/libkxld.dylib
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
