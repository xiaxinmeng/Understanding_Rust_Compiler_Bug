plain
[00:02:37]       Memory: 8 GB
[00:02:37]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:37]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:37]       SMC Version (system): 2.8f0
[00:02:37]       Serial Number (system): VMhuqU+NZXqT
[00:02:37] 
[00:02:38] hw.ncpu: 4
[00:02:38] hw.byteorder: 1234
[00:02:38] hw.memsize: 8589934592
---
[00:53:34] 
[00:53:34] failures:
[00:53:34] 
[00:53:34] ---- builder::__test::dist_with_same_targets_and_hosts stdout ----
[00:53:34]  thread 'builder::__test::dist_with_same_targets_and_hosts' panicked at 'command did not execute successfully: "git" "log" "-1" "--date=short" "--pretty=format:%cd"
[00:53:34] expected success, got: signal: 4', build_helper/lib.rs:122:9
[00:53:34] 
[00:53:34] 
[00:53:34] failures:
[00:53:34]     builder::__test::dist_with_same_targets_and_hosts
---
[00:53:34] 
[00:53:34] 
[00:53:34] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[00:53:34] Build completed unsuccessfully in 0:01:03
[00:53:34] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01db93a4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:0319e404
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 24
drwx------   3 travis  staff    102 May  6 12:43 .
-rw-------@  1 travis  staff  12211 May  6 12:43 bootstrap-dbf53a170429e672_2018-05-06-124321_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25 19:20 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0a23785c
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/bootstrap-dbf53a170429e672_2018-05-06-124321_Traviss-Mac-1044.crash
Process:               bootstrap-dbf53a170429e672 [31299]
Path:                  /Users/USER/*/bootstrap-dbf53a170429e672
Identifier:            bootstrap-dbf53a170429e672
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        bootstrap-dbf53a170429e672 [31128]
Responsible:           bootstrap-dbf53a170429e672 [31299]
User ID:               501
Date/Time:             2018-05-06 12:43:17.691 +0000
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
0   libsystem_platform.dylib       0x00007fff6f9b5ad4 _os_once_gate_corruption_abort + 23
1   libsystem_platform.dylib       0x00007fff6f9b5b6e _os_once_gate_wait_slow + 129
2   libsystem_platform.dylib       0x00007fff6f9b1ed4 _os_alloc_once + 42
3   libsystem_notify.dylib         0x00007fff6f9ac22f _notify_fork_child + 179
4   libSystem.B.dylib              0x00007fff6d4aeb13 libSystem_atfork_child + 49
5   libsystem_c.dylib              0x00007fff6f79083e fork + 47
6   bootstrap-dbf53a170429e672     0x000000010abfd3c0 std::sys::unix::process::process_inner::_$LT$impl$u20$std..sys..unix..process..process_common..Command$GT$::spawn::h6f398f162ba4d9eb + 912 (process_unix.rs:45)
7   bootstrap-dbf53a170429e672     0x000000010ac1bb2c std::process::Command::output::h4459d64179fe8aba + 28 (result.rs:468)
8   bootstrap-dbf53a170429e672     0x000000010abf43e3 build_helper::output::h8dce802bd45620c3 + 131 (lib.rs:114)
9   bootstrap-dbf53a170429e672     0x000000010a821fc7 bootstrap::channel::GitInfo::new::hb7b5bdb1aa6f99de + 887 (channel.rs:57)
10  bootstrap-dbf53a170429e672     0x000000010a73d672 bootstrap::Build::new::h1206ee0cf8c7def9 + 818 (lib.rs:341)
11  bootstrap-dbf53a170429e672     0x000000010a3902ff bootstrap::builder::__test::dist_with_same_targets_and_hosts::hb413633df6dbdb1b + 95
12  bootstrap-dbf53a170429e672     0x000000010a7fe5f1 bootstrap::__test::TESTS::_$u7b$$u7b$closure$u7d$$u7d$::h8c1f5fe7d65c145c + 17 (builder.rs:1138)
13  bootstrap-dbf53a170429e672     0x000000010a39ef51 core::ops::function::FnOnce::call_once::h99f8d7616a8db4a6 + 17 (function.rs:223)
14  bootstrap-dbf53a170429e672     0x000000010aaf5ef2 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h8073c60b9240b246 + 18 (boxed.rs:785)
15  bootstrap-dbf53a170429e672     0x000000010ac2927f __rust_maybe_catch_panic + 31 (lib.rs:110)
16  bootstrap-dbf53a170429e672     0x000000010ab08b66 std::sys_common::backtrace::__rust_begin_short_backtrace::h7b1714457cf551c2 + 358 (lib.rs:1408)
17  bootstrap-dbf53a170429e672     0x000000010ab04638 std::panicking::try::do_call::h9f8bee5affeadf9e + 40 (panicking.rs:308)
18  bootstrap-dbf53a170429e672     0x000000010ac2927f __rust_maybe_catch_panic + 31 (lib.rs:110)
19  bootstrap-dbf53a170429e672     0x000000010ab19225 _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h1aae60c98a2ed7ae + 165 (mod.rs:405)
20  bootstrap-dbf53a170429e672     0x000000010ac1ca18 std::sys_common::thread::start_thread::h4499537c830231ab + 136 (thread.rs:25)
21  bootstrap-dbf53a170429e672     0x000000010abff869 std::sys::unix::thread::Thread::new::thread_start::hff4df432fa232e88 + 9 (thread.rs:92)
22  libsystem_pthread.dylib        0x00007fff6f9bc6c1 _pthread_body + 340
23  libsystem_pthread.dylib        0x00007fff6f9bc56d _pthread_start + 377
24  libsystem_pthread.dylib        0x00007fff6f9bbc5d thread_start + 13
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000001103  rbx: 0x0000000000001103  rcx: 0x00007fff6f9b7c42  rdx: 0x0000000000000000
  rdi: 0x0000000000001103  rsi: 0x00007fffa8445320  rbp: 0x000070000a66e770  rsp: 0x000070000a66e748
   r8: 0x000070000a66e098   r9: 0x000070000a66e330  r10: 0x0000000000000000  r11: 0x0000000000000246
  r12: 0x000070000a66f030  r13: 0x000070000a66e980  r14: 0x0000000000000203  r15: 0x00007fffa8445320
  rip: 0x00007fff6f9b5ad4  rfl: 0x0000000000010246  cr2: 0x00007fffa84562c0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10a342000 -        0x10aec6fff +bootstrap-dbf53a170429e672 (0) <58058868-0068-35A9-9774-5612BC747B8F> /Users/USER/*/bootstrap-dbf53a170429e672
       0x10d682000 -        0x10d6cc98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6cf9b000 -     0x7fff6cfcefff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6d4ad000 -     0x7fff6d4aeff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6d763000 -     0x7fff6d7b9fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6d7ba000 -     0x7fff6d7deff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6eb30000 -     0x7fff6ef213b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6efee000 -     0x7fff6f00affb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6f5c8000 -     0x7fff6f5ccff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6f5cd000 -     0x7fff6f5d7ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6f5d8000 -     0x7fff6f5dffff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6f5e0000 -     0x7fff6f5e8ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6f5e9000 -     0x7fff6f66efff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6f6f6000 -     0x7fff6f72fff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6f730000 -     0x7fff6f74dff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6f74e000 -     0x7fff6f74effb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6f75c000 -     0x7fff6f75cff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6f75d000 -     0x7fff6f761ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6f762000 -     0x7fff6f764ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6f765000 -     0x7fff6f766ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6f767000 -     0x7fff6f77efff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6f77f000 -     0x7fff6f77ffff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6f780000 -     0x7fff6f809ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6f80a000 -     0x7fff6f80dffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6f80e000 -     0x7fff6f811ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6f812000 -     0x7fff6f813fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6f814000 -     0x7fff6f81aff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6f81b000 -     0x7fff6f864ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6f865000 -     0x7fff6f88aff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6f88b000 -     0x7fff6f8d6fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6f8d7000 -     0x7fff6f8f6fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6f8f7000 -     0x7fff6f99bff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6f99c000 -     0x7fff6f9a6ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6f9a7000 -     0x7fff6f9b0ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6f9b1000 -     0x7fff6f9b8ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6f9b9000 -     0x7fff6f9c4fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6f9c5000 -     0x7fff6f9c8ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6f9c9000 -     0x7fff6f9caff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6f9cb000 -     0x7fff6f9d2ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6f9d3000 -     0x7fff6f9e6ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6f9e8000 -     0x7fff6f9edff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6f9ee000 -     0x7fff6fa1aff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2417
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=218.8M resident=0K(0%) swapped_out_or_unallocated=218.8M(100%)
Writable regions: Total=120.7M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=120.7M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            36.0M        8 
MALLOC guard page                   16K        5 
STACK GUARD                          4K        2 
Stack                             72.0M        6 
Stack Guard                         16K        5 
VM_ALLOCATE                       20.0M       14 
VM_ALLOCATE (reserved)             640K        2         reserved VM address space (unallocated)
__DATA                            2428K       44 
__LINKEDIT                       198.5M        4 
__TEXT                            20.3M       43 
===========                     =======  ======= 
TOTAL                            349.9M      126 
TOTAL                            349.9M      126 
TOTAL, minus reserved VM space   349.3M      126 
travis_time:end:0a23785c:start=1525610605805442000,finish=1525610605869323000,duration=63881000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00ead9e0
travis_time:start:00ead9e0
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.4

Done. Your build exited with 1.
