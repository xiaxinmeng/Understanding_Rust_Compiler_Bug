plain
[00:02:17]       Memory: 8 GB
[00:02:17]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:17]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:17]       SMC Version (system): 2.8f0
[00:02:17]       Serial Number (system): VMKWpF1JWoSr
[00:02:17] 
[00:02:17] hw.ncpu: 4
[00:02:17] hw.byteorder: 1234
[00:02:17] hw.memsize: 8589934592
---
[01:50:00] Checking "alias-2.js" ... OK
[01:50:00] Checking "alias-3.js" ... OK
[01:50:00] Checking "alias.js" ... OK
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::string","name":"String"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::ffi","name":"CString"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::ffi","name":"OsString"}'
[01:50:00] ==> Result not found in 'in_args': '{"path":"std::str","name":"eq"}'
[01:50:00] ==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::f32","name":"is_nan"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::f64","name":"is_nan"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::option::Option","name":"is_none"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::option","name":"Option"}'
[01:50:00] Checking "basic.js" ... Checking "deduplication.js" ... Checking "enum-option.js" ... Checking "filter-crate.js" ... OK
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::mem","name":"forget"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::fmt","name":"format"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::char","name":"from_u32"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::str","name":"from_utf8"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"fn","ty":15}'
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"fn","ty":21}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"panic","ty":14}'
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"panic","ty":0}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"print"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"eprint"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"println"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"eprintln"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"str"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"u8"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::ffi","name":"CStr"}'
[01:50:00] Checking "fn-forget.js" ... Checking "from_u.js" ... Checking "keyword.js" ... Checking "macro-check.js" ... Checking "macro-print.js" ... Checking "multi-query.js" ... Checking "never.js" ... OK
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std","name":"error"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::fmt","name":"Error"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::io","name":"Error"}'
[01:50:00] ==> Result not found in 'returned': '{"path":"std::fmt::LowerExp","name":"fmt"}'
[01:50:00] Checking "quoted.js" ... Checking "should-fail.js" ... OK
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_lossy"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf16_lossy"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_unchecked"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::vec","name":"Vec"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::collections","name":"VecDeque"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::task","name":"local_waker_from_nonlocal"}'
[01:50:00] ==> Result not found in 'others': '{"path":"alloc::task","name":"local_waker_from_nonlocal"}'
[01:50:00] FAILED
[01:50:00] ==> Result not found in 'others': '{"path":"std::vec::Vec","name":"new"}'
[01:50:00] ==> Result not found in 'others': '{"path":"std::vec::Vec","name":"ne"}'
[01:50:00] Checking "string-from_ut.js" ... Checking "struct-vec.js" ... Checking "substring.js" ... Checking "vec-new.js" ... 
[01:50:00] 
[01:50:00] command did not execute successfully: "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "src/tools/rustdoc-js/tester.js" "x86_64-apple-darwin"
[01:50:00] 
[01:50:00] 
[01:50:00] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:50:00] Build completed unsuccessfully in 0:51:55
[01:50:00] Build completed unsuccessfully in 0:51:55
[01:50:00] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:20773560
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 24 20:45:04 GMT 2018
---
travis_fold:start:after_failure.2
travis_time:start:00c4d757
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Dec 24 20:44 .
-rw-------@  1 travis  staff  13746 Dec 24 20:44 overflow_2018-12-24-204445_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Dec 24 20:44 foo_2018-12-24-204422_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Dec 24 20:43 m4_2018-12-24-204353_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Dec 24 20:43 bar_2018-12-24-204342_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Dec 24 20:43 b_2018-12-24-204341_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Dec 24 20:43 bar_2018-12-24-204341_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Dec 24 20:07 a_2018-12-24-200726_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 Dec 24 20:07 a_2018-12-24-200725_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 Dec 24 20:07 a_2018-12-24-200718-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37238 Dec 24 20:07 a_2018-12-24-200718_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Dec 24 20:07 a_2018-12-24-200713_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Dec 24 20:07 a_2018-12-24-200708_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Dec 24 20:07 a_2018-12-24-200701_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9795 Dec 24 20:07 a_2018-12-24-200700_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Dec 24 20:06 a_2018-12-24-200623_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63075 Dec 24 20:06 a_2018-12-24-200613_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Dec 24 20:06 a_2018-12-24-200610-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64248 Dec 24 20:06 a_2018-12-24-200610-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65091 Dec 24 20:06 a_2018-12-24-200610_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11585 Dec 24 20:04 a_2018-12-24-200406_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Dec 24 20:03 a_2018-12-24-200315_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Dec 24 20:02 a_2018-12-24-200206_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Dec 24 20:01 a_2018-12-24-200125-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Dec 24 20:01 a_2018-12-24-200125_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Dec 24 20:01 a_2018-12-24-200124_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:04b595c0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-24-200124_Traviss-Mac-1044.crash
Process:               a [41917]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41905]
Responsible:           a [41917]
User ID:               501
Date/Time:             2018-12-24 20:01:13.104 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-9704616a50a0e937.dylib  0x000000010887e98c std::panicking::rust_panic_with_hook::h62ca8c89ccbdc3f7 + 668
1   a                              0x000000010884e7f5 std::panicking::begin_panic::h0183cf39c4f26474 + 37
2   a                              0x000000010884c2bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 28
3   a                              0x000000010884b9e9 core::ptr::real_drop_in_place::h0bccc5556cf0dbf4 + 9
4   a                              0x000000010884c293 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010884d42e backtrace::main::hcde7a1a1c3c85e77 + 4238 (backtrace.rs:113)
6   a                              0x000000010884b7b6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h84a8225cc4fa24d8 + 6 (rt.rs:74)
7   libstd-9704616a50a0e937.dylib  0x000000010887e1f8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
8   libstd-9704616a50a0e937.dylib  0x00000001088995ff __rust_maybe_catch_panic + 31
9   libstd-9704616a50a0e937.dylib  0x000000010887eced std::rt::lang_start_internal::h948ce4de2cd5b179 + 541
10  a                              0x000000010884dcb9 main + 41
11  libdyld.dylib                  0x00007fff5f33f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee73b6bd8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001088c3652  rbp: 0x00007ffee73b6cd0  rsp: 0x00007ffee73b6c00
   r8: 0x00000001088c1e70   r9: 0x00000001088f5600  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001088c1e70  r14: 0x0000000108850460  r15: 0x00007ffee73b6ce0
  rip: 0x000000010887e98c  rfl: 0x0000000000010206  cr2: 0x00000001088b7f10
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x108847000 -        0x10884fff7 +a (0) <DB9B9260-1261-3ADA-A5CE-74CF3D93A78C> /Users/USER/*/a
       0x10885b000 -        0x1088edff7 +libstd-9704616a50a0e937.dylib (0) <D1A40CA3-D5F4-329A-A7EE-188BDCDFBAEF> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x11289b000 -        0x1128e598f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5cba9000 -     0x7fff5cbdcfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5d0bb000 -     0x7fff5d0bcff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5d371000 -     0x7fff5d3c7fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5d3c8000 -     0x7fff5d3ecff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5e73e000 -     0x7fff5eb2f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ebfc000 -     0x7fff5ec18ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5f1d6000 -     0x7fff5f1daff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5f1db000 -     0x7fff5f1e5ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5f1e6000 -     0x7fff5f1edfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5f1ee000 -     0x7fff5f1f6ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5f1f7000 -     0x7fff5f27cfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5f304000 -     0x7fff5f33dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5f33e000 -     0x7fff5f35bff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5f35c000 -     0x7fff5f35cffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5f36a000 -     0x7fff5f36aff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5f36b000 -     0x7fff5f36fffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5f370000 -     0x7fff5f372ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5f373000 -     0x7fff5f374ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5f375000 -     0x7fff5f38cfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5f38d000 -     0x7fff5f38dfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5f38e000 -     0x7fff5f417ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5f418000 -     0x7fff5f41bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5f41c000 -     0x7fff5f41fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5f420000 -     0x7fff5f421fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5f422000 -     0x7fff5f428ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5f429000 -     0x7fff5f472ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5f473000 -     0x7fff5f498ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5f499000 -     0x7fff5f4e4fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5f4e5000 -     0x7fff5f504fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5f505000 -     0x7fff5f5a9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5f5aa000 -     0x7fff5f5b4ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5f5b5000 -     0x7fff5f5beff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5f5bf000 -     0x7fff5f5c6ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5f5c7000 -     0x7fff5f5d2fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5f5d3000 -     0x7fff5f5d6ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5f5d7000 -     0x7fff5f5d8ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5f5d9000 -     0x7fff5f5e0ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5f5e1000 -     0x7fff5f5f4ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5f5f6000 -     0x7fff5f5fbff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5f5fc000 -     0x7fff5f628ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 1804
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9684K        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4528K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.4M      109 
TOTAL                            276.4M      109 
TOTAL, minus reserved VM space   276.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-24-200125-1_Traviss-Mac-1044.crash
Process:               a [41914]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41905]
Responsible:           a [41914]
User ID:               501
Date/Time:             2018-12-24 20:01:13.097 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-9704616a50a0e937.dylib  0x00000001031b498c std::panicking::rust_panic_with_hook::h62ca8c89ccbdc3f7 + 668
1   a                              0x00000001031827f5 std::panicking::begin_panic::h0183cf39c4f26474 + 37
2   a                              0x00000001031802bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 28
3   a                              0x000000010317f9e9 core::ptr::real_drop_in_place::h0bccc5556cf0dbf4 + 9
4   a                              0x0000000103180293 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010318142e backtrace::main::hcde7a1a1c3c85e77 + 4238 (backtrace.rs:113)
6   a                              0x000000010317f7b6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h84a8225cc4fa24d8 + 6 (rt.rs:74)
7   libstd-9704616a50a0e937.dylib  0x00000001031b41f8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
8   libstd-9704616a50a0e937.dylib  0x00000001031cf5ff __rust_maybe_catch_panic + 31
9   libstd-9704616a50a0e937.dylib  0x00000001031b4ced std::rt::lang_start_internal::h948ce4de2cd5b179 + 541
10  a                              0x0000000103181cb9 main + 41
11  libdyld.dylib                  0x00007fff5f33f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeeca82be8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001031f9652  rbp: 0x00007ffeeca82ce0  rsp: 0x00007ffeeca82c10
   r8: 0x00000001031f7e70   r9: 0x000000010322b600  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001031f7e70  r14: 0x0000000103184460  r15: 0x00007ffeeca82cf0
  rip: 0x00000001031b498c  rfl: 0x0000000000010206  cr2: 0x000000010347cd25
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10317b000 -        0x103183ff7 +a (0) <DB9B9260-1261-3ADA-A5CE-74CF3D93A78C> /Users/USER/*/a
       0x103191000 -        0x103223ff7 +libstd-9704616a50a0e937.dylib (0) <D1A40CA3-D5F4-329A-A7EE-188BDCDFBAEF> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x10724f000 -        0x10729998f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5cba9000 -     0x7fff5cbdcfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5d0bb000 -     0x7fff5d0bcff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5d371000 -     0x7fff5d3c7fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5d3c8000 -     0x7fff5d3ecff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5e73e000 -     0x7fff5eb2f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ebfc000 -     0x7fff5ec18ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5f1d6000 -     0x7fff5f1daff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5f1db000 -     0x7fff5f1e5ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5f1e6000 -     0x7fff5f1edfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5f1ee000 -     0x7fff5f1f6ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5f1f7000 -     0x7fff5f27cfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5f304000 -     0x7fff5f33dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5f33e000 -     0x7fff5f35bff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5f35c000 -     0x7fff5f35cffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5f36a000 -     0x7fff5f36aff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5f36b000 -     0x7fff5f36fffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5f370000 -     0x7fff5f372ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5f373000 -     0x7fff5f374ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5f375000 -     0x7fff5f38cfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5f38d000 -     0x7fff5f38dfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5f38e000 -     0x7fff5f417ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5f418000 -     0x7fff5f41bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5f41c000 -     0x7fff5f41fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5f420000 -     0x7fff5f421fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5f422000 -     0x7fff5f428ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5f429000 -     0x7fff5f472ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5f473000 -     0x7fff5f498ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5f499000 -     0x7fff5f4e4fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5f4e5000 -     0x7fff5f504fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5f505000 -     0x7fff5f5a9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5f5aa000 -     0x7fff5f5b4ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5f5b5000 -     0x7fff5f5beff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5f5bf000 -     0x7fff5f5c6ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5f5c7000 -     0x7fff5f5d2fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5f5d3000 -     0x7fff5f5d6ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5f5d7000 -     0x7fff5f5d8ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5f5d9000 -     0x7fff5f5e0ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5f5e1000 -     0x7fff5f5f4ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5f5f6000 -     0x7fff5f5fbff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5f5fc000 -     0x7fff5f628ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 1804
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=83.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=83.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.5M        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4528K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            286.4M      108 
TOTAL                            286.4M      108 
TOTAL, minus reserved VM space   286.3M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-24-200125_Traviss-Mac-1044.crash
Process:               a [41127]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41124]
Responsible:           a [41127]
User ID:               501
Date/Time:             2018-12-24 20:00:45.275 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00000001037fa75e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x00000001037f9b49 std::panicking::try::do_call::hcd71044095ad0489 (.llvm.14489980795995569670) + 9
2   libstd-9704616a50a0e937.dylib  0x000000010383e5ff __rust_maybe_catch_panic + 31
3   a                              0x00000001037fa9b1 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x00000001037f8ef6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9b0735fb04836369 + 6
5   libstd-9704616a50a0e937.dylib  0x00000001038231f8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
6   libstd-9704616a50a0e937.dylib  0x000000010383e5ff __rust_maybe_catch_panic + 31
7   libstd-9704616a50a0e937.dylib  0x0000000103823ced std::rt::lang_start_internal::h948ce4de2cd5b179 + 541
8   a                              0x00000001037facb9 main + 41
9   libdyld.dylib                  0x00007fff5f33f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f7f01402ba0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeec405438  rsi: 0x0000000087ffffff  rbp: 0x00007ffeec405e90  rsp: 0x00007ffeec405e90
   r8: 0x00000000f01402bf   r9: 0x0000000000000004  r10: 0x00000001120e58d0  r11: 0x00007fff5f5f696c
  r12: 0x00007ffeec406160  r13: 0x0000000000000000  r14: 0x00007ffeec405fb0  r15: 0x00007ffeec405ef8
  rip: 0x00000001037fa75e  rfl: 0x0000000000010202  cr2: 0x000000010387e668
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1037f8000 -        0x1037fbfff +a (0) <45D331FF-B0CD-31A0-A908-DEC1E93FBD00> /Users/USER/*/a
       0x103800000 -        0x103892ff7 +libstd-9704616a50a0e937.dylib (0) <D1A40CA3-D5F4-329A-A7EE-188BDCDFBAEF> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x112093000 -        0x1120dd98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5cba9000 -     0x7fff5cbdcfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5d0bb000 -     0x7fff5d0bcff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5d371000 -     0x7fff5d3c7fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5d3c8000 -     0x7fff5d3ecff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5e73e000 -     0x7fff5eb2f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ebfc000 -     0x7fff5ec18ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5f1d6000 -     0x7fff5f1daff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5f1db000 -     0x7fff5f1e5ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5f1e6000 -     0x7fff5f1edfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5f1ee000 -     0x7fff5f1f6ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5f1f7000 -     0x7fff5f27cfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5f304000 -     0x7fff5f33dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5f33e000 -     0x7fff5f35bff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5f35c000 -     0x7fff5f35cffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5f36a000 -     0x7fff5f36aff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5f36b000 -     0x7fff5f36fffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5f370000 -     0x7fff5f372ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5f373000 -     0x7fff5f374ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5f375000 -     0x7fff5f38cfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5f38d000 -     0x7fff5f38dfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5f38e000 -     0x7fff5f417ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5f418000 -     0x7fff5f41bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5f41c000 -     0x7fff5f41fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5f420000 -     0x7fff5f421fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5f422000 -     0x7fff5f428ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5f429000 -     0x7fff5f472ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5f473000 -     0x7fff5f498ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5f499000 -     0x7fff5f4e4fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5f4e5000 -     0x7fff5f504fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5f505000 -     0x7fff5f5a9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5f5aa000 -     0x7fff5f5b4ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5f5b5000 -     0x7fff5f5beff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5f5bf000 -     0x7fff5f5c6ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5f5c7000 -     0x7fff5f5d2fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5f5d3000 -     0x7fff5f5d6ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5f5d7000 -     0x7fff5f5d8ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5f5d9000 -     0x7fff5f5e0ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5f5e1000 -     0x7fff5f5f4ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5f5f6000 -     0x7fff5f5fbff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5f5fc000 -     0x7fff5f628ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 1804
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.3M resident=0K(0%) swapped_out_or_unallocated=198.3M(100%)
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
__DATA                            4528K       44 
__LINKEDIT                       188.9M        5 
__TEXT                            9604K       44 
===========                     =======  ======= 
TOTAL                            275.9M      108 
TOTAL                            275.9M      108 
TOTAL, minus reserved VM space   275.8M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-24-200206_Traviss-Mac-1044.crash
Process:               a [43518]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [43517]
Responsible:           a [43518]
User ID:               501
Date/Time:             2018-12-24 20:02:06.347 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4100 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff5f48ee3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5f5cd150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5f3eb312 abort + 127
3   libstd-9704616a50a0e937.dylib  0x000000010a494869 std::sys::unix::abort_internal::haf7423cf66b1806e + 9
4   libstd-9704616a50a0e937.dylib  0x000000010a484550 rust_oom + 32
5   libstd-9704616a50a0e937.dylib  0x000000010a4a64f9 alloc::alloc::handle_alloc_error::h8a946e1e8920f188 + 9
6   a                              0x000000010a45a28d default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x000000010a459426 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h06f7457321a1f120 + 6
8   libstd-9704616a50a0e937.dylib  0x000000010a4851f8 std::panicking::try::do_call::h15f3aa7ff9620fa7 + 24
9   libstd-9704616a50a0e937.dylib  0x000000010a4a05ff __rust_maybe_catch_panic + 31
10  libstd-9704616a50a0e937.dylib  0x000000010a485ced std::rt::lang_start_internal::h948ce4de2cd5b179 + 541
11  a                              0x000000010a45a3f9 main + 41
12  libdyld.dylib                  0x00007fff5f33f115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff98065340  rcx: 0x00007ffee57a5df8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee57a5e30  rsp: 0x00007ffee57a5df8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5f48ee3e  rfl: 0x0000000000000206  cr2: 0x00007fff98043148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10a458000 -        0x10a45aff7 +a (0) <DA5908AD-3987-3DCF-86A8-1D2663703148> /Users/USER/*/a
       0x10a462000 -        0x10a4f4ff7 +libstd-9704616a50a0e937.dylib (0) <D1A40CA3-D5F4-329A-A7EE-188BDCDFBAEF> /Users/USER/*/libstd-9704616a50a0e937.dylib
       0x10cc26000 -        0x10cc7098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5cba9000 -     0x7fff5cbdcfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5d0bb000 -     0x7fff5d0bcff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5d371000 -     0x7fff5d3c7fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5d3c8000 -     0x7fff5d3ecff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5e73e000 -     0x7fff5eb2f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5ebfc000 -     0x7fff5ec18ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5f1d6000 -     0x7fff5f1daff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5f1db000 -     0x7fff5f1e5ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5f1e6000 -     0x7fff5f1edfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5f1ee000 -     0x7fff5f1f6ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5f1f7000 -     0x7fff5f27cfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5f304000 -     0x7fff5f33dff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5f33e000 -     0x7fff5f35bff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5f35c000 -     0x7fff5f35cffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5f36a000 -     0x7fff5f36aff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5f36b000 -     0x7fff5f36fffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5f370000 -     0x7fff5f372ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5f373000 -     0x7fff5f374ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5f375000 -     0x7fff5f38cfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5f38d000 -     0x7fff5f38dfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5f38e000 -     0x7fff5f417ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5f418000 -     0x7fff5f41bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5f41c000 -     0x7fff5f41fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5f420000 -     0x7fff5f421fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5f422000 -     0x7fff5f428ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5f429000 -     0x7fff5f472ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5f473000 -     0x7fff5f498ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5f499000 -     0x7fff5f4e4fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5f4e5000 -     0x7fff5f504fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5f505000 -     0x7fff5f5a9ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5f5aa000 -     0x7fff5f5b4ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5f5b5000 -     0x7fff5f5beff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5f5bf000 -     0x7fff5f5c6ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5f5c7000 -     0x7fff5f5d2fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5f5d3000 -     0x7fff5f5d6ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5f5d7000 -     0x7fff5f5d8ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5f5d9000 -     0x7fff5f5e0ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5f5e1000 -     0x7fff5f5f4ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5f5f6000 -     0x7fff5f5fbff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5f5fc000 -     0x7fff5f628ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
