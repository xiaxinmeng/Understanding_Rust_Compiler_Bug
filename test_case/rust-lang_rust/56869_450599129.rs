plain
[01:52:58] Checking "alias-2.js" ... OK
[01:52:58] Checking "alias-3.js" ... OK
[01:52:58] Checking "alias.js" ... OK
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::string","name":"String"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::ffi","name":"CString"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::ffi","name":"OsString"}'
[01:52:58] ==> Result not found in 'in_args': '{"path":"std::str","name":"eq"}'
[01:52:58] ==> Result not found in 'returned': '{"path":"std::string::String","name":"add"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::f32","name":"is_nan"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::f64","name":"is_nan"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::option::Option","name":"is_none"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::option","name":"Option"}'
[01:52:58] Checking "basic.js" ... Checking "deduplication.js" ... Checking "enum-option.js" ... Checking "filter-crate.js" ... OK
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::mem","name":"forget"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::fmt","name":"format"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::char","name":"from_u32"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::str","name":"from_utf8"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"fn","ty":15}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"fn","ty":21}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"panic","ty":14}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"panic","ty":0}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"print"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"eprint"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"println"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"eprintln"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"str"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"u8"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::ffi","name":"CStr"}'
[01:52:58] Checking "fn-forget.js" ... Checking "from_u.js" ... Checking "keyword.js" ... Checking "macro-check.js" ... Checking "macro-print.js" ... Checking "multi-query.js" ... Checking "never.js" ... OK
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"error"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::fmt","name":"Error"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::io","name":"Error"}'
[01:52:58] ==> Result not found in 'returned': '{"path":"std::fmt::LowerExp","name":"fmt"}'
[01:52:58] Checking "quoted.js" ... Checking "should-fail.js" ... OK
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_lossy"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf16_lossy"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::string::String","name":"from_utf8_unchecked"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::vec","name":"Vec"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::collections","name":"VecDeque"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"debug_assert_eq"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"debug_assert_ne"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std","name":"assert"}'
[01:52:58] FAILED
[01:52:58] ==> Result not found in 'others': '{"path":"std::vec::Vec","name":"new"}'
[01:52:58] ==> Result not found in 'others': '{"path":"std::vec::Vec","name":"ne"}'
[01:52:58] Checking "string-from_ut.js" ... Checking "struct-vec.js" ... Checking "substring.js" ... Checking "vec-new.js" ... 
[01:52:58] 
[01:52:58] command did not execute successfully: "/Users/travis/.nvm/versions/node/v6.12.3/bin/node" "src/tools/rustdoc-js/tester.js" "i686-apple-darwin"
[01:52:58] 
[01:52:58] 
[01:52:58] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:52:58] Build completed unsuccessfully in 0:52:13
[01:52:58] Build completed unsuccessfully in 0:52:13
[01:52:58] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14e5c6e4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 31 01:16:22 GMT 2018
---
travis_fold:start:after_failure.2
travis_time:start:112d1830
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1176
-rw-------@  1 travis  staff   1387 Dec 31 01:15 foo_2018-12-31-011540_Traviss-Mac-1044.crash
drwx------  26 travis  staff    884 Dec 31 01:15 .
-rw-------@  1 travis  staff   1376 Dec 31 01:15 m4_2018-12-31-011510_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9898 Dec 31 01:15 b_2018-12-31-011500_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1403 Dec 31 01:15 bar_2018-12-31-011502_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1391 Dec 31 01:15 bar_2018-12-31-011501_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34790 Dec 31 00:38 a_2018-12-31-003846-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  57364 Dec 31 00:38 a_2018-12-31-003846_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  55583 Dec 31 00:38 a_2018-12-31-003835-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  34698 Dec 31 00:38 a_2018-12-31-003835_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9420 Dec 31 00:38 a_2018-12-31-003831_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9166 Dec 31 00:38 a_2018-12-31-003825_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9171 Dec 31 00:38 a_2018-12-31-003819_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   8937 Dec 31 00:38 a_2018-12-31-003818_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9305 Dec 31 00:37 a_2018-12-31-003742_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  58266 Dec 31 00:37 a_2018-12-31-003735_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60381 Dec 31 00:37 a_2018-12-31-003729-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59104 Dec 31 00:37 a_2018-12-31-003729-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  59540 Dec 31 00:37 a_2018-12-31-003729_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10758 Dec 31 00:35 a_2018-12-31-003524_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9190 Dec 31 00:34 a_2018-12-31-003433_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9551 Dec 31 00:33 a_2018-12-31-003321_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Dec 31 00:32 a_2018-12-31-003231-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9782 Dec 31 00:32 a_2018-12-31-003231_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9484 Dec 31 00:32 a_2018-12-31-003226_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:18f10680
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003226_Traviss-Mac-1044.crash
Process:               a [37250]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [37247]
Responsible:           a [37250]
User ID:               501
Date/Time:             2018-12-31 00:31:59.461 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x00050aae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 46
1   a                              0x0004fefb std::panicking::try::do_call::h597cae53c1593fd1 (.llvm.15024036044453248661) + 11
2   libstd-9485b157ebf30bde.dylib  0x0013ebfd __rust_maybe_catch_panic + 29
3   a                              0x00050d15 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 613
4   a                              0x0004f4bb std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc64f0c43d05738bc + 11
5   libstd-9485b157ebf30bde.dylib  0x0012dc1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
6   libstd-9485b157ebf30bde.dylib  0x001300a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
7   libstd-9485b157ebf30bde.dylib  0x0013ebfd __rust_maybe_catch_panic + 29
8   libstd-9485b157ebf30bde.dylib  0x00130b57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
9   a                              0x0005104c main + 44
10  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x78639120  ebx: 0xbffb1278  ecx: 0x00000000  edx: 0x00000000
  edi: 0x0013ebee  esi: 0x00000000  ebp: 0xbffb1218  esp: 0xbffb1200
   ss: 0x00000023  efl: 0x00010292  eip: 0x00050aae   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00175c50
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x4e000 -    0x51ff3 +a (0) <4E25168C-6FFE-3968-8D6A-01C28D7CBE5D> /Users/USER/*/a
   0x87000 -    0xccfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x110000 -   0x19bff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9328K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.5M      134 
TOTAL                            568.5M      134 
TOTAL, minus reserved VM space   568.4M      134 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003231-1_Traviss-Mac-1044.crash
Process:               a [38070]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38061]
Responsible:           a [38070]
User ID:               501
Date/Time:             2018-12-31 00:32:29.704 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-9485b157ebf30bde.dylib  0x0016a787 std::panicking::rust_panic_with_hook::h41d1202c6b0097fa + 583
1   a                              0x0009cb6f std::panicking::begin_panic::h4f5b916c8b595a8d + 47 (panicking.rs:412)
2   a                              0x0009a5f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 36 (backtrace.rs:24)
3   a                              0x00099f3b core::ptr::real_drop_in_place::h280f13d74e1439c0 + 11
4   a                              0x0009a5c3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x0009b8f8 backtrace::main::hcde7a1a1c3c85e77 + 4600 (backtrace.rs:103)
6   a                              0x00099a7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h132adc36ab358787 + 11 (rt.rs:64)
7   libstd-9485b157ebf30bde.dylib  0x00167c1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
8   libstd-9485b157ebf30bde.dylib  0x0016a0a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
9   libstd-9485b157ebf30bde.dylib  0x00178bfd __rust_maybe_catch_panic + 29
10  libstd-9485b157ebf30bde.dylib  0x0016ab57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
11  a                              0x0009c17c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff68fc8  ebx: 0xbff69010  ecx: 0xbff68eb0  edx: 0xa7702ec6
  edi: 0x001aff1c  esi: 0x0016a54e  ebp: 0xbff69068  esp: 0xbff68fe0
   ss: 0x00000023  efl: 0x00010286  eip: 0x0016a787   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0042a3fd
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0x96000 -    0x9dff7 +a (0) <61CC3534-20B4-3514-8043-3BE1E14F1534> /Users/USER/*/a
   0xc1000 -   0x106fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x14a000 -   0x1d5ff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9596K       10 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.9M      136 
TOTAL                            568.9M      136 
TOTAL, minus reserved VM space   568.8M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003231_Traviss-Mac-1044.crash
Process:               a [38069]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [38061]
Responsible:           a [38069]
User ID:               501
Date/Time:             2018-12-31 00:32:29.698 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-9485b157ebf30bde.dylib  0x0022b787 std::panicking::rust_panic_with_hook::h41d1202c6b0097fa + 583
1   a                              0x000e0b6f std::panicking::begin_panic::h4f5b916c8b595a8d + 47 (panicking.rs:412)
2   a                              0x000de5f4 _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 36 (backtrace.rs:24)
3   a                              0x000ddf3b core::ptr::real_drop_in_place::h280f13d74e1439c0 + 11
4   a                              0x000de5c3 backtrace::double::h0c99cc05786c6af0 + 51
5   a                              0x000df8f8 backtrace::main::hcde7a1a1c3c85e77 + 4600 (backtrace.rs:103)
6   a                              0x000dda7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h132adc36ab358787 + 11 (rt.rs:64)
7   libstd-9485b157ebf30bde.dylib  0x00228c1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
8   libstd-9485b157ebf30bde.dylib  0x0022b0a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
9   libstd-9485b157ebf30bde.dylib  0x00239bfd __rust_maybe_catch_panic + 29
10  libstd-9485b157ebf30bde.dylib  0x0022bb57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
11  a                              0x000e017c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff24fd8  ebx: 0xbff25020  ecx: 0xbff24ec0  edx: 0xa7702ec6
  edi: 0x00270f1c  esi: 0x0022b54e  ebp: 0xbff25078  esp: 0xbff24ff0
   ss: 0x00000023  efl: 0x00010286  eip: 0x0022b787   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x004eb3fd
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xda000 -    0xe1ff7 +a (0) <61CC3534-20B4-3514-8043-3BE1E14F1534> /Users/USER/*/a
  0x182000 -   0x1c7fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x20b000 -   0x296ff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.2M resident=0K(0%) swapped_out_or_unallocated=83.2M(100%)
Writable regions: Total=73.6M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.6M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9596K        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9344K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.9M      135 
TOTAL                            568.9M      135 
TOTAL, minus reserved VM space   568.8M      135 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003321_Traviss-Mac-1044.crash
Process:               a [39654]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [39653]
Responsible:           a [39654]
User ID:               501
Date/Time:             2018-12-31 00:33:20.855 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4300 seconds
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
3   libstd-9485b157ebf30bde.dylib  0x001d508b std::sys::unix::abort_internal::hbca6836ae8be931b + 11
4   libstd-9485b157ebf30bde.dylib  0x001c6530 rust_oom + 48
5   libstd-9485b157ebf30bde.dylib  0x001e8564 alloc::alloc::handle_alloc_error::hea871e6d5ad8ada1 + 20
6   a                              0x000e3376 default_alloc_error_hook::main::hbf2d06db626d002e + 790
7   a                              0x000e378b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::ha226d642e934495a + 11
8   libstd-9485b157ebf30bde.dylib  0x001c4c1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
9   libstd-9485b157ebf30bde.dylib  0x001c70a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
10  libstd-9485b157ebf30bde.dylib  0x001d5bfd __rust_maybe_catch_panic + 29
11  libstd-9485b157ebf30bde.dylib  0x001c7b57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
12  a                              0x000e34dc main + 44
13  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff1d18c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff1d1b8  esp: 0xbff1d18c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xe2000 -    0xe3ffb +a (0) <5F2F45AC-63F0-321C-8F69-0743BA1C63D4> /Users/USER/*/a
  0x11e000 -   0x163fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1a7000 -   0x232ff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003729_Traviss-Mac-1044.crash
Process:               a [45962]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [45959]
Responsible:           a [45962]
User ID:               501
Date/Time:             2018-12-31 00:37:29.307 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b065dfec
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb065dfec:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b065d000-00000000b065e000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b065e000-00000000b085f000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-9485b157ebf30bde.dylib  0x001bfbb0 std::sys::unix::thread::Thread::join::he0c849bc88ec0bd3 + 32
4   a                              0x000893f6 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::h5be142c4516d8f20 + 70
5   a                              0x00087c49 out_of_stack::main::hfb05bc1bb33cf0c4 + 233
6   a                              0x0008703b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h20b440df8c6b8fae + 11
7   libstd-9485b157ebf30bde.dylib  0x001afc1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
8   libstd-9485b157ebf30bde.dylib  0x001b20a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
9   libstd-9485b157ebf30bde.dylib  0x001c0bfd __rust_maybe_catch_panic + 29
10  libstd-9485b157ebf30bde.dylib  0x001b2b57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
11  a                              0x0008896c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-9485b157ebf30bde.dylib  0x001c008b std::sys::unix::abort_internal::hbca6836ae8be931b + 11
4   libstd-9485b157ebf30bde.dylib  0x001b0782 std::sys_common::util::abort::h437a0a51084531f1 + 82
5   libstd-9485b157ebf30bde.dylib  0x001bf61b std::sys::unix::stack_overflow::imp::signal_handler::hc8c8913da6aded0e + 955
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-9485b157ebf30bde.dylib  0x001bf260 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h05fc5f7c4ec4ffe1 + 80
9   libstd-9485b157ebf30bde.dylib  0x001a0b97 _$LT$std..io..stdio..StdoutLock$LT$$u27$a$GT$$u20$as$u20$std..io..Write$GT$::write::h8ecea48b307acec7 + 263
10  libstd-9485b157ebf30bde.dylib  0x001a22b7 std::io::Write::write_all::h224cca0d3fb2ed95 + 71
11  libstd-9485b157ebf30bde.dylib  0x001a28d3 _$LT$std..io..Write..write_fmt..Adaptor$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..fmt..Write$GT$::write_str::h24e5662b700fbecf + 35
12  libstd-9485b157ebf30bde.dylib  0x001e0fa4 core::fmt::write::hd10c080826c6c104 + 740
13  libstd-9485b157ebf30bde.dylib  0x001a0966 _$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$::write_fmt::h6592e0ecf7b5b517 + 182
14  libstd-9485b157ebf30bde.dylib  0x001a1c1c std::io::stdio::_print::h014534893a783c8a + 396
15  a                              0x00087b4f out_of_stack::loud_recurse::hcd528ebf130a94fa + 63
16  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
17  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
18  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
19  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
20  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
21  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
22  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
23  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
24  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
25  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
26  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
27  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
28  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
29  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
30  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
31  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
32  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
33  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
34  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
35  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
36  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
37  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
38  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
39  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
40  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
41  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
42  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
43  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
44  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
45  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
46  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
47  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
48  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
49  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
50  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
51  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
52  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
53  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
54  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
55  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
56  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
57  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
58  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
59  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
60  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
61  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
62  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
63  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
64  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
65  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
66  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
67  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
68  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
69  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
70  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
71  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
72  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
73  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
74  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
75  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
76  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
77  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
78  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
79  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
80  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
81  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
82  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
83  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
84  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
85  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
86  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
87  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
88  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
89  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
90  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
91  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
92  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
93  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
94  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
95  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
96  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
97  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
98  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
99  a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
100 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
101 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
102 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
103 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
104 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
105 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
106 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
107 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
108 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
109 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
110 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
111 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
112 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
113 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
114 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
115 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
116 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
117 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
118 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
119 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
120 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
121 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
122 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
123 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
124 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
125 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
126 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
127 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
128 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
129 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
130 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
131 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
132 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
133 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
134 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
135 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
136 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
137 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
138 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
139 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
140 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
141 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
142 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
143 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
144 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
145 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
146 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
147 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
148 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
149 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
150 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
151 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
152 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
153 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
154 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
155 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
156 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
157 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
158 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
159 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
160 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
161 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
162 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
163 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
164 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
165 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
166 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
167 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
168 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
169 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
170 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
171 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
172 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
173 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
174 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
175 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
176 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
177 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
178 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
179 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
180 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
181 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
182 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
183 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
184 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
185 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
186 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
187 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
188 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
189 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
190 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
191 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
192 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
193 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
194 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
195 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
196 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
197 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
198 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
199 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
200 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
201 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
202 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
203 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
204 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
205 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
206 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
207 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
208 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
209 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
210 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
211 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
212 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
213 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
214 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
215 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
216 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
217 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
218 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
219 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
220 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
221 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
222 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
223 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
224 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
225 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
226 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
227 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
228 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
229 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
230 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
231 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
232 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
233 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
234 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
235 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
236 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
237 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
238 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
239 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
240 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
241 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
242 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
243 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
244 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
245 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
246 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
247 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
248 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
249 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
250 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
251 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
252 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
253 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
254 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
255 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
256 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
257 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
258 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
259 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
260 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
261 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
262 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
263 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
264 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
265 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
266 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
267 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
268 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
269 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
270 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
271 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
272 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
273 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
274 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
275 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
276 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
277 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
278 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
279 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
280 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
281 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
282 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
283 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
284 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
285 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
286 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
287 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
288 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
289 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
290 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
291 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
292 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
293 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
294 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
295 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
296 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
297 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
298 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
299 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
300 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
301 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
302 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
303 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
304 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
305 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
306 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
307 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
308 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
309 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
310 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
311 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
312 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
313 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
314 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
315 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
316 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
317 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
318 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
319 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
320 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
321 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
322 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
323 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
324 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
325 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
326 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
327 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
328 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
329 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
330 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
331 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
332 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
333 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
334 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
335 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
336 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
337 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
338 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
339 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
340 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
341 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
342 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
343 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
344 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
345 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
346 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
347 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
348 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
349 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
350 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
351 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
352 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
353 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
354 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
355 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
356 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
357 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
358 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
359 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
360 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
361 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
362 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
363 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
364 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
365 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
366 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
367 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
368 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
369 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
370 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
371 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
372 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
373 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
374 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
375 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
376 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
377 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
378 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
379 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
380 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
381 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
382 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
383 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
384 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
385 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
386 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
387 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
388 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
389 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
390 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
391 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
392 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
393 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
394 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
395 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
396 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
397 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
398 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
399 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
400 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
401 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
402 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
403 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
404 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
405 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
406 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
407 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
408 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
409 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
410 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
411 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
412 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
413 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
414 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
415 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
416 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
417 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
418 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
419 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
420 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
421 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
422 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
423 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
424 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
425 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
426 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
427 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
428 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
429 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
430 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
431 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
432 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
433 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
434 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
435 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
436 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
437 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
438 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
439 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
440 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
441 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
442 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
443 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
444 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
445 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
446 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
447 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
448 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
449 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
450 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
451 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
452 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
453 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
454 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
455 a                              0x00087b54 out_of_stack::loud_recurse::hcd528ebf130a94fa + 68
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003742_Traviss-Mac-1044.crash
Process:               a [46144]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [46142]
Responsible:           a [46144]
User ID:               501
Date/Time:             2018-12-31 00:37:41.695 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
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
3   a                              0x000ec39b panic_abort::__rust_start_panic::abort::hc24e7d609642365c + 11
4   a                              0x000ec38b __rust_start_panic + 11
5   a                              0x000e0c5b rust_panic + 11
6   a                              0x000e07e9 std::panicking::rust_panic_with_hook::h41d1202c6b0097fa + 1321
7   a                              0x000f222a std::panicking::begin_panic::h5b6aaf420146a9c5 + 42
8   a                              0x000df61f lto_abort::main::h9419a0043b6e0505 + 2991
9   a                              0x000f237b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd15b71c2dc06a476 + 11
10  a                              0x000ec20c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
11  a                              0x000dfa04 main + 996
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbff2114c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbff21178  esp: 0xbff2114c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0xde000 -   0x101ff3 +a (0) <382851E0-0029-3A06-AC7B-9B0A7058CFEE> /Users/USER/*/a
  0x207000 -   0x24cfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=82.3M resident=0K(0%) swapped_out_or_unallocated=82.3M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            1336K       43 
__LINKEDIT                        73.7M        4 
__OBJC                              36K        6 
__TEXT                            8896K       43 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            565.6M      131 
TOTAL                            565.6M      131 
TOTAL, minus reserved VM space   565.5M      131 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003818_Traviss-Mac-1044.crash
Process:               a [47163]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47158]
Responsible:           a [47163]
User ID:               501
Date/Time:             2018-12-31 00:38:17.705 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
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
3   libstd-9485b157ebf30bde.dylib  0x001fd08b std::sys::unix::abort_internal::hbca6836ae8be931b + 11
4   libstd-9485b157ebf30bde.dylib  0x001ed782 std::sys_common::util::abort::h437a0a51084531f1 + 82
5   libstd-9485b157ebf30bde.dylib  0x001ef8db rust_panic + 107
6   libstd-9485b157ebf30bde.dylib  0x001ef79b std::panicking::rust_panic_with_hook::h41d1202c6b0097fa + 603
7   a                              0x00044d2f std::panicking::begin_panic::hf6deb8aa5fcc361b + 47
8   a                              0x00045e7c main + 2604
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xa9b3c1c0  ecx: 0xbffbc15c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0xbffbc188  esp: 0xbffbc15c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0xa9b21330
Logical CPU:     0
Error Code:      0x00080148
Trap Number:     132
Binary Images:
   0x43000 -    0x46fff +a (0) <5E3B54CC-9730-309F-A9F1-929F49BFC96A> /Users/USER/*/a
  0x146000 -   0x18bfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1cf000 -   0x25aff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=74.2M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.2M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__OBJC                              36K        6 
__TEXT                            9328K       44 
mapped file                      408.7M       21 
shared memory                        8K        3 
===========                     =======  ======= 
TOTAL                            569.4M      132 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003819_Traviss-Mac-1044.crash
Process:               a [47194]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47192]
Responsible:           a [47194]
User ID:               501
Date/Time:             2018-12-31 00:38:18.870 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
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
    __TEXT                 00000000000d8000-00000000000db000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000d9efe segfault_no_out_of_stack::main::haec5f9680e9a04a3 + 2110
1   a                              0x000d883b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h35c1e37b58d20fd2 + 11
2   libstd-9485b157ebf30bde.dylib  0x00280c1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
3   libstd-9485b157ebf30bde.dylib  0x002830a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
4   libstd-9485b157ebf30bde.dylib  0x00291bfd __rust_maybe_catch_panic + 29
5   libstd-9485b157ebf30bde.dylib  0x00283b57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
6   a                              0x000da1cc main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x79e0b700  ecx: 0x00000000  edx: 0x00000000
  edi: 0x00291bee  esi: 0xbff272c0  ebp: 0xbff27388  esp: 0xbff271e0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000d9efe   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000000
Logical CPU:     2
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0xd8000 -    0xdaff3 +a (0) <528A4542-DFE1-3CC3-ADFA-A1DAB2C3C7E2> /Users/USER/*/a
  0x1da000 -   0x21ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x263000 -   0x2eeff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003825_Traviss-Mac-1044.crash
Process:               a [47372]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47370]
Responsible:           a [47372]
User ID:               501
Date/Time:             2018-12-31 00:38:25.849 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
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
    __TEXT                 0000000000058000-000000000005b000 [   12K] r-x/rwx SM=COW  /Users/USER/*
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x0005a584 signal_exit_status::main::h013a3960fc5c363d + 436
1   a                              0x0005938b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6657964b7302a4eb + 11
2   libstd-9485b157ebf30bde.dylib  0x00180c1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
3   libstd-9485b157ebf30bde.dylib  0x001830a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
4   libstd-9485b157ebf30bde.dylib  0x00191bfd __rust_maybe_catch_panic + 29
5   libstd-9485b157ebf30bde.dylib  0x00183b57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
6   a                              0x0005a65c main + 44
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000002  ecx: 0x00000000  edx: 0x7a664230
  edi: 0x7a6642c0  esi: 0xbffa7300  ebp: 0xbffa7398  esp: 0xbffa7280
   ss: 0x00000023  efl: 0x00010246  eip: 0x0005a584   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x00000001
Logical CPU:     0
Error Code:      0x00000006
Trap Number:     14
Binary Images:
   0x58000 -    0x5aff7 +a (0) <D72205E0-4B61-38A3-AF9D-3E5A0019B01E> /Users/USER/*/a
   0xda000 -   0x11ffdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x163000 -   0x1eeff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                        132K        3 
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
---
===========                     =======  ======= 
TOTAL                            568.5M      134 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003831_Traviss-Mac-1044.crash
Process:               a [47476]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47466]
Responsible:           a [47476]
User ID:               501
Date/Time:             2018-12-31 00:38:30.458 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000c2bd2 simd_target_feature_mixup::test::id_avx512_512::h052c674ab7f4f4bc + 114
1   a                              0x000c1928 simd_target_feature_mixup::test::main::h379367934b9623dc + 1848
2   a                              0x000c3e59 simd_target_feature_mixup::main::h4f60990077aff357 + 937
3   a                              0x000c0f6b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h0501c808b2e75a13 + 11
4   libstd-9485b157ebf30bde.dylib  0x001efc1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
5   libstd-9485b157ebf30bde.dylib  0x001f20a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
6   libstd-9485b157ebf30bde.dylib  0x00200bfd __rust_maybe_catch_panic + 29
7   libstd-9485b157ebf30bde.dylib  0x001f2b57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
8   a                              0x000c404c main + 44
9   libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0xbff3f000  ebx: 0xbff3ef00  ecx: 0x000c2b6e  edx: 0xbff3ef00
  edi: 0x000c1204  esi: 0x00000000  ebp: 0xbff3eef8  esp: 0xbff3eec0
   ss: 0x00000023  efl: 0x00010246  eip: 0x000c2bd2   cs: 0x0000001b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x000c27b0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
   0xc0000 -    0xc4fcf +a (0) <45B72A82-B664-3284-9C3E-DE467215C97D> /Users/USER/*/a
  0x149000 -   0x18efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x1d2000 -   0x25dff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=73.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9244K        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9332K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            568.5M      133 
TOTAL                            568.5M      133 
TOTAL, minus reserved VM space   568.4M      133 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003835-1_Traviss-Mac-1044.crash
Process:               a [47619]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        a [47617]
Responsible:           a [47619]
User ID:               501
Date/Time:             2018-12-31 00:38:35.659 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000bbf63398
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xbbf63398:
    Stack Guard            00000000bbf62000-00000000bbf63000 [    4K] ---/rwx SM=NUL  
--> VM_ALLOCATE            00000000bbf63000-00000000bbf64000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000bbf64000-00000000bff62000 [ 64.0M] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-9485b157ebf30bde.dylib  0x0021908b std::sys::unix::abort_internal::hbca6836ae8be931b + 11
4   libstd-9485b157ebf30bde.dylib  0x00209782 std::sys_common::util::abort::h437a0a51084531f1 + 82
5   libstd-9485b157ebf30bde.dylib  0x0021861b std::sys::unix::stack_overflow::imp::signal_handler::hc8c8913da6aded0e + 955
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-9485b157ebf30bde.dylib  0x00218260 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h05fc5f7c4ec4ffe1 + 80
9   a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
265 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
266 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
267 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
268 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
269 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
270 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
271 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
272 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
273 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
274 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
275 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
276 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
277 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
278 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
279 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
280 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
281 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
282 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
283 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
284 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
285 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
286 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
287 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
288 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
289 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
290 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
291 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
292 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
293 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
294 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
295 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
296 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
297 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
298 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
299 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
300 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
301 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
302 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
303 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
304 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
305 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
306 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
307 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
308 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
309 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
310 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
311 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
312 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
313 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
314 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
315 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
316 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
317 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
318 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
319 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
320 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
321 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
322 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
323 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
324 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
325 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
326 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
327 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
328 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
329 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
330 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
331 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
332 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
333 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
334 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
335 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
336 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
337 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
338 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
339 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
340 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
341 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
342 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
343 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
344 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
345 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
346 a                              0x000a18c0 stack_probes::recurse::h24283d9484398da0 + 48
---
===========                     =======  ======= 
TOTAL                            568.5M      133 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003835_Traviss-Mac-1044.crash
Process:               a [47620]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47617]
Responsible:           a [47620]
User ID:               501
Date/Time:             2018-12-31 00:38:35.686 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b00a5ea8
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb00a5ea8:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b00a5000-00000000b00a6000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b00a6000-00000000b02a7000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   libstd-9485b157ebf30bde.dylib  0x0029abb0 std::sys::unix::thread::Thread::join::he0c849bc88ec0bd3 + 32
4   a                              0x000fd7a6 _$LT$std..thread..JoinHandle$LT$T$GT$$GT$::join::ha8f17ea308ea714c + 70
5   a                              0x000fc775 stack_probes::main::hc5f49a55fd7e038b + 597
6   a                              0x000fb53b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h05e8ae7443455cc5 + 11
7   libstd-9485b157ebf30bde.dylib  0x0028ac1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
8   libstd-9485b157ebf30bde.dylib  0x0028d0a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
9   libstd-9485b157ebf30bde.dylib  0x0029bbfd __rust_maybe_catch_panic + 29
10  libstd-9485b157ebf30bde.dylib  0x0028db57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
11  a                              0x000fd19c main + 44
12  libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   libstd-9485b157ebf30bde.dylib  0x0029b08b std::sys::unix::abort_internal::hbca6836ae8be931b + 11
4   libstd-9485b157ebf30bde.dylib  0x0028b782 std::sys_common::util::abort::h437a0a51084531f1 + 82
5   libstd-9485b157ebf30bde.dylib  0x0029a61b std::sys::unix::stack_overflow::imp::signal_handler::hc8c8913da6aded0e + 955
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   libstd-9485b157ebf30bde.dylib  0x0029a260 _$LT$std..sys..unix..stack_overflow..Handler$u20$as$u20$core..ops..drop..Drop$GT$::drop::h05fc5f7c4ec4ffe1 + 80
9   a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
10  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
11  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
12  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
13  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
14  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
15  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
16  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
17  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
18  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
19  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
20  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
21  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
22  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
23  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
24  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
25  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
26  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
27  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
28  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
29  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
30  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
31  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
32  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
33  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
34  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
35  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
36  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
37  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
38  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
39  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
40  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
41  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
42  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
43  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
44  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
45  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
46  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
47  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
48  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
49  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
50  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
51  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
52  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
53  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
54  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
55  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
56  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
57  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
58  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
59  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
60  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
61  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
62  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
63  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
64  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
65  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
66  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
67  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
68  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
69  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
70  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
71  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
72  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
73  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
74  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
75  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
76  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
77  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
78  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
79  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
80  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
81  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
82  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
83  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
84  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
85  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
86  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
87  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
88  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
89  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
90  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
91  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
92  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
93  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
94  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
95  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
96  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
97  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
98  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
99  a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
100 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
101 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
102 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
103 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
104 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
105 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
106 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
107 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
108 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
109 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
110 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
111 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
112 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
113 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
114 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
115 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
116 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
117 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
118 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
119 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
120 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
121 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
122 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
123 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
124 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
125 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
126 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
127 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
128 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
129 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
130 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
131 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
132 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
133 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
134 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
135 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
136 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
137 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
138 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
139 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
140 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
141 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
142 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
143 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
144 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
145 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
146 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
147 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
148 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
149 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
150 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
151 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
152 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
153 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
154 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
155 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
156 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
157 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
158 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
159 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
160 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
161 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
162 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
163 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
164 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
165 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
166 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
167 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
168 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
169 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
170 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
171 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
172 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
173 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
174 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
175 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
176 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
177 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
178 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
179 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
180 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
181 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
182 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
183 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
184 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
185 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
186 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
187 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
188 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
189 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
190 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
191 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
192 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
193 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
194 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
195 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
196 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
197 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
198 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
199 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
200 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
201 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
202 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
203 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
204 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
205 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
206 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
207 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
208 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
209 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
210 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
211 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
212 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
213 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
214 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
215 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
216 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
217 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
218 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
219 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
220 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
221 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
222 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
223 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
224 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
225 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
226 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
227 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
228 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
229 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
230 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
231 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
232 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
233 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
234 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
235 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
236 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
237 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
238 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
239 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
240 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
241 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
242 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
243 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
244 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
245 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
246 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
247 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
248 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
249 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
250 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
251 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
252 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
253 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
254 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
255 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
256 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
257 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
258 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
259 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
260 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
261 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
262 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
263 a                              0x000fc8c0 stack_probes::recurse::h24283d9484398da0 + 48
264 a                              0x000fb4cd std::sys_common::backtrace::__rust_begin_short_backtrace::hee50a0aef86c3e1c + 29
265 libstd-9485b157ebf30bde.dylib  0x0029bbfd __rust_maybe_catch_panic + 29
266 a                              0x000fda83 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h52107d37d0132f72 + 131
267 libstd-9485b157ebf30bde.dylib  0x0029aaeb std::sys::unix::thread::Thread::new::thread_start::hea149a74b3a08f40 + 187
268 libsystem_pthread.dylib        0xa782f50d _pthread_body + 347
269 libsystem_pthread.dylib        0xa782f3b2 _pthread_start + 357
270 libsystem_pthread.dylib        0xa782ea8e thread_start + 34
Thread 1 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0xb02a6000  ecx: 0x00150b0c  edx: 0x00000000
  edi: 0xa783236a  esi: 0x0000002d  ebp: 0x00150b38  esp: 0x00150b0c
   ss: 0x00000023  efl: 0x00000206  eip: 0xa7700eae   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0029a6a0
Logical CPU:     0
Error Code:      0x0000014e
Trap Number:     132
Binary Images:
   0xfa000 -    0xfefff +a (0) <C2EBD59E-51A8-3BF2-A6FB-4446928A57FB> /Users/USER/*/a
  0x1e4000 -   0x229fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x26d000 -   0x2f8ff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 2082
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=76.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=76.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          8K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE                        132K        3 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3524K       44 
__LINKEDIT                        74.0M        5 
__OBJC                              36K        6 
__TEXT                            9332K       44 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            571.6M      137 
TOTAL                            571.6M      137 
TOTAL, minus reserved VM space   571.5M      137 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2018-12-31-003846-1_Traviss-Mac-1044.crash
Process:               a [47736]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [47729]
Responsible:           a [47736]
User ID:               501
Date/Time:             2018-12-31 00:38:41.918 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4600 seconds
System Integrity Protection: enabled
Crashed Thread:        1
Exception Type:        EXC_BAD_ACCESS (SIGABRT)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000000b0430f08
Exception Note:        EXC_CORPSE_NOTIFY
VM Regions Near 0xb0430f08:
    mapped file            00000000ae9e4000-00000000aefaf000 [ 5932K] r--/r-- SM=COW  2
--> Stack Guard            00000000b0430000-00000000b0431000 [    4K] ---/rwx SM=NUL  
    Stack                  00000000b0431000-00000000b0632000 [ 2052K] rw-/rwx SM=COW  
abort() called
abort() called
Thread 0:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0xa7701126 __semwait_signal + 10
1   libsystem_pthread.dylib        0xa7833d4a _pthread_join + 574
2   libsystem_pthread.dylib        0xa78354f9 pthread_join$UNIX2003 + 85
3   a                              0x00037949 stack_probes_lto::main::h7f9d4bbc5a99dd1a + 2553
4   a                              0x0005012b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hfd859b20684db5c8 + 11
5   a                              0x00048bac std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
6   a                              0x00038fdd main + 589
7   libdyld.dylib                  0xa75a66e1 start + 1
Thread 1 Crashed:
0   libsystem_kernel.dylib         0xa7700eae __pthread_kill + 10
1   libsystem_pthread.dylib        0xa78324c7 pthread_kill + 363
2   libsystem_c.dylib              0xa7650afe abort + 133
3   a                              0x0003a42b std::sys::unix::abort_internal::hbca6836ae8be931b + 11
4   a                              0x0003a412 std::sys_common::util::abort::h437a0a51084531f1 + 82
5   a                              0x00048b04 std::sys::unix::stack_overflow::imp::signal_handler::hc8c8913da6aded0e + 868
6   libsystem_platform.dylib       0xa782702b _sigtramp + 43
7   ???                            0xffffffff 0 + 4294967295
8   a                              0x000487a0 rust_begin_unwind + 16
9   a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
10  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
11  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
12  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
13  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
14  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
15  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
16  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
17  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
18  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
19  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
20  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
21  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
22  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
23  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
24  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
25  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
26  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
27  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
28  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
29  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
30  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
31  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
32  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
33  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
34  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
35  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
36  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
37  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
38  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
39  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
40  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
41  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
42  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
43  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
44  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
45  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
46  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
47  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
48  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
49  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
50  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
51  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
52  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
53  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
54  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
55  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
56  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
57  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
58  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
59  a                              0x00037dc8 stack_probes_lto::recurse::h907252696a8f0ddd + 40
---
===========                     =======  ======= 
TOTAL                            566.6M      130 
travis_fold:end:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/b_2018-12-31-011500_Traviss-Mac-1044.crash
Process:               b [65838]
Path:                  /Users/USER/*/b
Identifier:            b
Version:               0
Code Type:             X86 (Native)
Parent Process:        sh [65837]
Responsible:           b [65838]
User ID:               501
Date/Time:             2018-12-31 01:14:59.949 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6800 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Reason:    DYLD, [0x4] Symbol missing
Dyld Error Message:
  Symbol not found: __ZN1a3foo17hc61ef93bf551e6c1E
  Referenced from: /Users/USER/*/b
  Expected in: /Users/USER/*/liba.dylib
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   dyld                           0x0013147a __abort_with_payload + 10
1   dyld                           0x00130f60 abort_with_payload_wrapper_internal + 64
2   dyld                           0x00130fac abort_with_payload + 38
3   dyld                           0x0010353b dyld::halt(char const*) + 326
4   dyld                           0x00103693 dyld::fastBindLazySymbol(ImageLoader**, unsigned long) + 184
5   libdyld.dylib                  0xa75a694c dyld_stub_binder_ + 20
6   b                              0x00076c02 b::main::hc99712a88efc0959 + 18
7   b                              0x00076d7b std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hffee4b0a856fee2e + 11
8   libstd-9485b157ebf30bde.dylib  0x001a5c1c std::sys_common::backtrace::__rust_begin_short_backtrace::hd249725879bb30eb + 12
9   libstd-9485b157ebf30bde.dylib  0x001a80a4 std::panicking::try::do_call::hd9734058df00e3a9 + 20
10  libstd-9485b157ebf30bde.dylib  0x001b6bfd __rust_maybe_catch_panic + 29
11  libstd-9485b157ebf30bde.dylib  0x001a8b57 std::rt::lang_start_internal::h793facf83c37c8cb + 631
12  b                              0x00076d57 std::rt::lang_start::h277dc38db5064f97 + 55
13  b                              0x00076c38 main + 40
14  libdyld.dylib                  0xa75a66e1 start + 1
Thread 0 crashed with X86 Thread State (32-bit):
  eax: 0x00000000  ebx: 0x00000000  ecx: 0xbff8765c  edx: 0x00000000
  edi: 0x00000004  esi: 0x00000006  ebp: 0xbff87698  esp: 0xbff8765c
   ss: 0x00000023  efl: 0x00000282  eip: 0x0013147a   cs: 0x0000000b
   ds: 0x00000023   es: 0x00000023   fs: 0x00000000   gs: 0x0000000f
  cr2: 0x0013c0e8
Logical CPU:     0
Error Code:      0x00000209
Trap Number:     132
Binary Images:
   0x76000 -    0x76fff +b (0) <F3BE73F7-E3DA-3FEC-AB23-84F4083D5A50> /Users/USER/*/b
   0x7c000 -    0x7cff3 +liba.dylib (0) <AE8810CA-E0F7-3C4E-9BF9-73B7D46432D5> /Users/USER/*/liba.dylib
   0xff000 -   0x144fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
  0x188000 -   0x213ff3 +libstd-9485b157ebf30bde.dylib (0) <843A6C08-965F-3A78-9A23-3C7A7954166F> /Users/USER/*/libstd-9485b157ebf30bde.dylib
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
    task_for_pid: 3376
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=83.1M resident=0K(0%) swapped_out_or_unallocated=83.1M(100%)
Writable regions: Total=74.3M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.3M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            3528K       45 
__LINKEDIT                        74.0M        6 
__OBJC                              36K        6 
__TEXT                            9320K       45 
mapped file                      408.7M       21 
===========                     =======  ======= 
TOTAL                            569.5M      136 
TOTAL                            569.5M      136 
TOTAL, minus reserved VM space   569.4M      136 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/bar_2018-12-31-011501_Traviss-Mac-1044.crash
Process:               bar [65987]
Path:                  /Users/USER/*/bar
Identifier:            bar
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [65986]
Responsible:           bar [65987]
User ID:               501
Date/Time:             2018-12-31 01:15:01.310 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6800 seconds
System Integrity Protection: enabled
Crashed Thread:        0
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Reason:    DYLD, [0x1] Library missing
Application Specific Information:
dyld: launch, loading dependent libraries
DYLD_LIBRARY_PATH=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/c-static-dylib/c-static-dylib:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib:
Dyld Error Message:
  Library not loaded: /Users/USER/*/libfoo.dylib
  Referenced from: /Users/USER/*/bar
Binary Images:
Binary Images:
   0x3b000 -    0x3bff7 +bar (0) <BFAC8703-87C6-33E7-B135-86653EF15597> /Users/USER/*/bar
   0xff000 -   0x144fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/bar_2018-12-31-011502_Traviss-Mac-1044.crash
Process:               bar [65921]
Path:                  /Users/USER/*/bar
Identifier:            bar
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [65920]
Responsible:           bar [65921]
User ID:               501
Date/Time:             2018-12-31 01:15:00.586 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6800 seconds
System Integrity Protection: enabled
Crashed Thread:        0
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Reason:    DYLD, [0x1] Library missing
Application Specific Information:
dyld: launch, loading dependent libraries
DYLD_LIBRARY_PATH=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/c-link-to-rust-dylib/c-link-to-rust-dylib:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib:
Dyld Error Message:
  Library not loaded: /Users/USER/*/libfoo.dylib
  Referenced from: /Users/USER/*/bar
Binary Images:
Binary Images:
   0xa2000 -    0xa2ff3 +bar (0) <FC04F2A6-F67B-35DD-9C89-FBD389BAA519> /Users/USER/*/bar
  0x11b000 -   0x160fdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/foo_2018-12-31-011540_Traviss-Mac-1044.crash
Process:               foo [69179]
Path:                  /Users/USER/*/foo
Identifier:            foo
Version:               0
Code Type:             X86 (Native)
Parent Process:        ??? [69178]
Responsible:           foo [69179]
User ID:               501
Date/Time:             2018-12-31 01:15:40.922 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6900 seconds
System Integrity Protection: enabled
Crashed Thread:        0
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Reason:    DYLD, [0x1] Library missing
Application Specific Information:
dyld: launch, loading dependent libraries
DYLD_LIBRARY_PATH=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/prefer-dylib/prefer-dylib:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib:
Dyld Error Message:
  Library not loaded: /Users/USER/*/libbar.dylib
  Referenced from: /Users/USER/*/foo
Binary Images:
Binary Images:
   0xf2000 -    0xf2ff7 +foo (0) <FD6A94F7-0618-32D5-9E26-708B01254739> /Users/USER/*/foo
  0x186000 -   0x1cbfdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/m4_2018-12-31-011510_Traviss-Mac-1044.crash
Process:               m4 [66888]
Path:                  /Users/USER/*/m4
Identifier:            m4
Version:               0
Code Type:             X86 (Native)
Parent Process:        sh [66887]
Responsible:           m4 [66888]
User ID:               501
Date/Time:             2018-12-31 01:15:10.670 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 6800 seconds
System Integrity Protection: enabled
Crashed Thread:        0
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Reason:    DYLD, [0x1] Library missing
Application Specific Information:
dyld: launch, loading dependent libraries
DYLD_LIBRARY_PATH=/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/test/run-make-fulldeps/dylib-chain/dylib-chain:/Users/travis/build/rust-lang/rust/build/i686-apple-darwin/stage2/lib/rustlib/i686-apple-darwin/lib:
Dyld Error Message:
  Library not loaded: /Users/USER/*/libm3.dylib
  Referenced from: /Users/USER/*/m4
Binary Images:
Binary Images:
   0xad000 -    0xadfff +m4 (0) <2CBA9932-7FE7-3478-9B3A-7A94F2C792D0> /Users/USER/*/m4
  0x159000 -   0x19efdf  dyld (519.2.2) <7B7B05B7-204A-38FF-BD32-4CBB51752DD4> /usr/lib/dyld
travis_time:end:18f10680:start=1546218987425705000,finish=1546218996648124000,duration=9222419000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15508d88
travis_time:start:15508d88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00574512
travis_time:start:00574512
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0401a5bb
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
