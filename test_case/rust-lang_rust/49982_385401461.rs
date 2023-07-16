plain
[00:03:46]       Memory: 8 GB
[00:03:46]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:46]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:46]       SMC Version (system): 2.8f0
[00:03:46]       Serial Number (system): VM2KIbmsxLwK
[00:03:46] 
[00:03:46] hw.ncpu: 4
[00:03:46] hw.byteorder: 1234
[00:03:46] hw.memsize: 8589934592
---
[00:53:19] 
[00:53:19] failures:
[00:53:19] 
[00:53:19] ---- builder::__test::dist_with_hosts stdout ----
[00:53:19]  thread 'builder::__test::dist_with_hosts' panicked at 'command did not execute successfully: "git" "rev-parse" "--short=9" "HEAD"
[00:53:19] expected success, got: signal: 4', build_helper/lib.rs:122:9
[00:53:19] 
[00:53:19] 
[00:53:19] failures:
[00:53:19]     builder::__test::dist_with_hosts
---
[00:53:19] 
[00:53:19] 
[00:53:19] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[00:53:19] Build completed unsuccessfully in 0:01:01
[00:53:19] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0666f684
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:087bce62
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 32
drwx------   3 travis  staff    102 Apr 30 13:40 .
-rw-------@  1 travis  staff  14204 Apr 30 13:40 bootstrap-dbf53a170429e672_2018-04-30-134023_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25 19:20 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1346dad0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/bootstrap-dbf53a170429e672_2018-04-30-134023_Traviss-Mac-1044.crash
Process:               bootstrap-dbf53a170429e672 [31327]
Path:                  /Users/USER/*/bootstrap-dbf53a170429e672
Identifier:            bootstrap-dbf53a170429e672
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        bootstrap-dbf53a170429e672 [31150]
Responsible:           bootstrap-dbf53a170429e672 [31327]
User ID:               501
Date/Time:             2018-04-30 13:40:19.026 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Report Version:        12
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 3300 seconds
System Integrity Protection: enabled
Crashed Thread:        0
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Application Specific Information:
crashed on child side of fork pre-exec
BUG IN CLIENT OF LIBPLATFORM: os_once_t is corrupt
Thread 0 Crashed:
0   libsystem_platform.dylib       0x00007fff5e76cad4 _os_once_gate_corruption_abort + 23
1   libsystem_platform.dylib       0x00007fff5e76cb6e _os_once_gate_wait_slow + 129
2   libsystem_platform.dylib       0x00007fff5e768ed4 _os_alloc_once + 42
3   libsystem_notify.dylib         0x00007fff5e76322f _notify_fork_child + 179
4   libSystem.B.dylib              0x00007fff5c265b13 libSystem_atfork_child + 49
5   libsystem_c.dylib              0x00007fff5e54783e fork + 47
6   bootstrap-dbf53a170429e672     0x000000010357ebb0 std::sys::unix::process::process_inner::_$LT$impl$u20$std..sys..unix..process..process_common..Command$GT$::spawn::h6f398f162ba4d9eb + 912 (process_unix.rs:45)
7   bootstrap-dbf53a170429e672     0x000000010359d28c std::process::Command::output::h4459d64179fe8aba + 28 (result.rs:468)
8   bootstrap-dbf53a170429e672     0x0000000103575bd3 build_helper::output::h8dce802bd45620c3 + 131 (lib.rs:114)
9   bootstrap-dbf53a170429e672     0x00000001031a3fce bootstrap::channel::GitInfo::new::hb7b5bdb1aa6f99de + 1326 (channel.rs:63)
10  bootstrap-dbf53a170429e672     0x0000000102d8654c bootstrap::tool::prepare_tool_cargo::h9ad88baaa096ba4d + 1132 (tool.rs:241)
11  bootstrap-dbf53a170429e672     0x0000000102d84fa5 _$LT$bootstrap..tool..ToolBuild$u20$as$u20$bootstrap..builder..Step$GT$::run::hc42f926dcd14b3fa + 517 (tool.rs:112)
12  bootstrap-dbf53a170429e672     0x0000000103405302 bootstrap::builder::Builder::ensure::h97afd8273d394aa1 + 4674 (builder.rs:902)
13  bootstrap-dbf53a170429e672     0x0000000102d89f03 _$LT$bootstrap..tool..RustInstaller$u20$as$u20$bootstrap..builder..Step$GT$::run::h7d26dcb69e12c677 + 227
14  bootstrap-dbf53a170429e672     0x000000010345c20a bootstrap::builder::Builder::ensure::hede3f57b73ef732d + 4602 (builder.rs:902)
15  bootstrap-dbf53a170429e672     0x0000000102d84899 bootstrap::tool::_$LT$impl$u20$bootstrap..builder..Builder$LT$$u27$a$GT$$GT$::tool_exe::hb85d22bf6c0ec904 + 2249 (tool.rs:266)
16  bootstrap-dbf53a170429e672     0x0000000102d83098 bootstrap::tool::_$LT$impl$u20$bootstrap..builder..Builder$LT$$u27$a$GT$$GT$::tool_cmd::h8afa1ab56fd58d3b + 72 (tool.rs:584)
17  bootstrap-dbf53a170429e672     0x000000010307c024 bootstrap::dist::rust_installer::h17183b1369f45d66 + 36 (dist.rs:62)
18  bootstrap-dbf53a170429e672     0x000000010307ca8c _$LT$bootstrap..dist..Docs$u20$as$u20$bootstrap..builder..Step$GT$::run::hd81b1e73d00dac2e + 2412 (dist.rs:107)
19  bootstrap-dbf53a170429e672     0x000000010340172b bootstrap::builder::Builder::ensure::h93d66330862fdb65 + 4603 (builder.rs:902)
20  bootstrap-dbf53a170429e672     0x000000010307c0d6 _$LT$bootstrap..dist..Docs$u20$as$u20$bootstrap..builder..Step$GT$::make_run::he605e9eb56e62bc4 + 54 (dist.rs:79)
21  bootstrap-dbf53a170429e672     0x0000000102d7b909 bootstrap::builder::StepDescription::maybe_run::hca5b528497ad6ffd + 1545 (builder.rs:171)
22  bootstrap-dbf53a170429e672     0x0000000102d7bf6e bootstrap::builder::StepDescription::run::h6368ead7b751026d + 1630 (builder.rs:198)
23  bootstrap-dbf53a170429e672     0x0000000102d7f655 bootstrap::builder::Builder::run_step_descriptions::h6aa2daa07d1b3bd0 + 53 (builder.rs:428)
24  bootstrap-dbf53a170429e672     0x0000000102d17477 bootstrap::builder::__test::dist_with_hosts::hbbda3b25feb50a69 + 247 (builder.rs:1020)
25  bootstrap-dbf53a170429e672     0x00000001031808b1 bootstrap::__test::TESTS::_$u7b$$u7b$closure$u7d$$u7d$::hdf866d4e6a41ece5 + 17 (builder.rs:1017)
26  bootstrap-dbf53a170429e672     0x0000000102d27541 core::ops::function::FnOnce::call_once::h1d3865f2a2a52a83 + 17 (function.rs:223)
27  bootstrap-dbf53a170429e672     0x00000001034776e2 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h8073c60b9240b246 + 18 (boxed.rs:785)
28  bootstrap-dbf53a170429e672     0x00000001035aa9df __rust_maybe_catch_panic + 31 (lib.rs:110)
29  bootstrap-dbf53a170429e672     0x000000010348a356 std::sys_common::backtrace::__rust_begin_short_backtrace::h7b1714457cf551c2 + 358 (lib.rs:1408)
30  bootstrap-dbf53a170429e672     0x0000000103485e28 std::panicking::try::do_call::h9f8bee5affeadf9e + 40 (panicking.rs:308)
31  bootstrap-dbf53a170429e672     0x00000001035aa9df __rust_maybe_catch_panic + 31 (lib.rs:110)
32  bootstrap-dbf53a170429e672     0x000000010349aa15 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h1aae60c98a2ed7ae + 165 (mod.rs:405)
33  bootstrap-dbf53a170429e672     0x000000010359e178 std::sys_common::thread::start_thread::h4499537c830231ab + 136 (thread.rs:25)
34  bootstrap-dbf53a170429e672     0x0000000103581059 std::sys::unix::thread::Thread::new::thread_start::hff4df432fa232e88 + 9 (thread.rs:92)
35  libsystem_pthread.dylib        0x00007fff5e7736c1 _pthread_body + 340
36  libsystem_pthread.dylib        0x00007fff5e77356d _pthread_start + 377
37  libsystem_pthread.dylib        0x00007fff5e772c5d thread_start + 13
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000001303  rbx: 0x0000000000001303  rcx: 0x00007fff5e76ec42  rdx: 0x0000000000000000
  rdi: 0x0000000000001303  rsi: 0x00007fff971fc320  rbp: 0x0000700009cdc580  rsp: 0x0000700009cdc558
   r8: 0x0000700009cdbea8   r9: 0x0000700009cdc140  r10: 0x0000000000000000  r11: 0x0000000000000246
  r12: 0x0000700009cdcfe0  r13: 0x0000700009cdc790  r14: 0x0000000000000203  r15: 0x00007fff971fc320
  rip: 0x00007fff5e76cad4  rfl: 0x0000000000010246  cr2: 0x00007fff9720d2c0
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x102cce000 -        0x103845fd7 +bootstrap-dbf53a170429e672 (0) <3693ECA4-8FAB-3763-AAB8-BE5766E7F866> /Users/USER/*/bootstrap-dbf53a170429e672
       0x10ebf9000 -        0x10ec4398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5bd52000 -     0x7fff5bd85fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5c264000 -     0x7fff5c265ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff5c51a000 -     0x7fff5c570fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff5c571000 -     0x7fff5c595ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff5d8e7000 -     0x7fff5dcd83b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5dda5000 -     0x7fff5ddc1ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5e37f000 -     0x7fff5e383ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5e384000 -     0x7fff5e38eff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5e38f000 -     0x7fff5e396fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5e397000 -     0x7fff5e39fffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5e3a0000 -     0x7fff5e425fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5e4ad000 -     0x7fff5e4e6ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5e4e7000 -     0x7fff5e504ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5e505000 -     0x7fff5e505ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5e513000 -     0x7fff5e513ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5e514000 -     0x7fff5e518ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5e519000 -     0x7fff5e51bff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5e51c000 -     0x7fff5e51dff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5e51e000 -     0x7fff5e535fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5e536000 -     0x7fff5e536fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5e537000 -     0x7fff5e5c0ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5e5c1000 -     0x7fff5e5c4ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5e5c5000 -     0x7fff5e5c8ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5e5c9000 -     0x7fff5e5cafff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5e5cb000 -     0x7fff5e5d1ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5e5d2000 -     0x7fff5e61bff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5e61c000 -     0x7fff5e641ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5e642000 -     0x7fff5e68dfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5e68e000 -     0x7fff5e6adfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5e6ae000 -     0x7fff5e752ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5e753000 -     0x7fff5e75dffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5e75e000 -     0x7fff5e767ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5e768000 -     0x7fff5e76fff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5e770000 -     0x7fff5e77bfff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5e77c000 -     0x7fff5e77fff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5e780000 -     0x7fff5e781ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5e782000 -     0x7fff5e789ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5e78a000 -     0x7fff5e79dff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5e79f000 -     0x7fff5e7a4ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5e7a5000 -     0x7fff5e7d1ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0
  Calls made by all processes on this machine:
    task_for_pid: 2397
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=218.7M resident=0K(0%) swapped_out_or_unallocated=218.7M(100%)
Writable regions: Total=120.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=120.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            36.0M        8 
MALLOC guard page                   16K        5 
STACK GUARD                          4K        2 
Stack                             72.0M        6 
Stack Guard                         16K        5 
VM_ALLOCATE                       20.0M       12 
VM_ALLOCATE (reserved)             640K        2         reserved VM address space (unallocated)
__DATA                            2428K       44 
__LINKEDIT                       198.5M        4 
__TEXT                            20.3M       43 
===========                     =======  ======= 
TOTAL                            349.8M      124 
TOTAL                            349.8M      124 
TOTAL, minus reserved VM space   349.2M      124 
