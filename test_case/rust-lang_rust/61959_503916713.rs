plain
[00:03:10]       Memory: 8 GB
[00:03:10]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:10]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:10]       SMC Version (system): 2.8f0
[00:03:10]       Serial Number (system): VMKnjuhSApYI
[00:03:10] 
[00:03:10] hw.ncpu: 4
[00:03:10] hw.byteorder: 1234
[00:03:10] hw.memsize: 8589934592
---
[01:59:42] 
[01:59:42] ---- /Users/travis/build/rust-lang/rust/src/doc/rustdoc/src/documentation-tests.md - Documentation_tests::Documenting_macros (line 276) stdout ----
[01:59:42] error: linking with `cc` failed: signal: 4
[01:59:42]   |
[01:59:42]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestAlKL6V/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestAlKL6V/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestAlKL6V/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestAlKL6V/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestAlKL6V/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestAlKL6V/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestAlKL6V/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-329ef83dbc1dd86d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-ddf1f863d2eaf653.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace-a26391313b65d7dc.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-0ad3fa8cdd0b9704.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-4d45b60ed1186ef3.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-b5491b1ffbbc1406.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-4228aae7223b6103.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-99ea320c6f5b5eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcfg_if-6595b4030f6ddb0e.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d930cd0bdea4797b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-8bde4cd3b4c45eea.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6450bce6cc789177.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-992f5c26c2d9c590.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-0f189b415a69de4e.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:59:42] 
[01:59:42] error: aborting due to previous error
[01:59:42] 
[01:59:42] Couldn't compile the test.
---
travis_fold:start:after_failure.2
travis_time:start:225639c4
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Jun 20 07:45 .
-rw-------@  1 travis  staff  13742 Jun 20 07:45 overflow_2019-06-20-074535_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Jun 20 07:45 foo_2019-06-20-074513_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Jun 20 07:44 m4_2019-06-20-074439_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Jun 20 07:44 bar_2019-06-20-074430_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Jun 20 07:44 b_2019-06-20-074429_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Jun 20 07:44 bar_2019-06-20-074429_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Jun 20 06:59 a_2019-06-20-065937-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Jun 20 06:59 a_2019-06-20-065937_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37415 Jun 20 06:59 a_2019-06-20-065929-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60388 Jun 20 06:59 a_2019-06-20-065929_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jun 20 06:59 a_2019-06-20-065922_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jun 20 06:59 a_2019-06-20-065918_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jun 20 06:59 a_2019-06-20-065914-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9792 Jun 20 06:59 a_2019-06-20-065914_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10033 Jun 20 06:58 a_2019-06-20-065837_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63059 Jun 20 06:58 a_2019-06-20-065823-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65081 Jun 20 06:58 a_2019-06-20-065823-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64276 Jun 20 06:58 a_2019-06-20-065823-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63914 Jun 20 06:58 a_2019-06-20-065823_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Jun 20 06:56 a_2019-06-20-065609_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Jun 20 06:55 a_2019-06-20-065517_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jun 20 06:54 a_2019-06-20-065419-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 20 06:54 a_2019-06-20-065419-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jun 20 06:54 a_2019-06-20-065419-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jun 20 06:54 a_2019-06-20-065419_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:2d10181a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-20-065419-1_Traviss-Mac-1044.crash
Process:               a [44069]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44068]
Responsible:           a [44069]
User ID:               501
Date/Time:             2019-06-20 06:54:03.252 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 4200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff79202e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff79341150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff7915f312 abort + 127
3   libstd-329ef83dbc1dd86d.dylib  0x000000010c472579 std::sys::unix::abort_internal::hb0d87c939ffeb797 + 9
4   libstd-329ef83dbc1dd86d.dylib  0x000000010c462900 rust_oom + 32
5   libstd-329ef83dbc1dd86d.dylib  0x000000010c488539 alloc::alloc::handle_alloc_error::h4044521f3c8241d3 + 9
6   a                              0x000000010c43707f default_alloc_error_hook::main::h0fe124586986ad5d + 767
7   a                              0x000000010c437736 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h417afde3001e038b + 6
8   libstd-329ef83dbc1dd86d.dylib  0x000000010c463458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
9   libstd-329ef83dbc1dd86d.dylib  0x000000010c47303f __rust_maybe_catch_panic + 31
10  libstd-329ef83dbc1dd86d.dylib  0x000000010c463f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
11  a                              0x000000010c4371d9 main + 41
12  libdyld.dylib                  0x00007fff790b3115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffb1dd9340  rcx: 0x00007ffee37c8618  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee37c8650  rsp: 0x00007ffee37c8618
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff79202e3e  rfl: 0x0000000000000206  cr2: 0x00007fffb1db7148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10c435000 -        0x10c437fff +a (0) <0A4F8E0E-5AEE-386A-AFF2-393BFC812577> /Users/USER/*/a
       0x10c440000 -        0x10c4dffff +libstd-329ef83dbc1dd86d.dylib (0) <C5003865-3170-3C67-96C0-AE40A4464E4D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x117360000 -        0x1173aa98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7691d000 -     0x7fff76950fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff76e2f000 -     0x7fff76e30ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff770e5000 -     0x7fff7713bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7713c000 -     0x7fff77160ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff784b2000 -     0x7fff788a33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff78970000 -     0x7fff7898cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff78f4a000 -     0x7fff78f4eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff78f4f000 -     0x7fff78f59ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff78f5a000 -     0x7fff78f61fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff78f62000 -     0x7fff78f6affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff78f6b000 -     0x7fff78ff0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79078000 -     0x7fff790b1ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff790b2000 -     0x7fff790cfff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff790d0000 -     0x7fff790d0ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff790de000 -     0x7fff790deff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff790df000 -     0x7fff790e3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff790e4000 -     0x7fff790e6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff790e7000 -     0x7fff790e8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff790e9000 -     0x7fff79100fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff79101000 -     0x7fff79101fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff79102000 -     0x7fff7918bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7918c000 -     0x7fff7918fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff79190000 -     0x7fff79193ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff79194000 -     0x7fff79195fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff79196000 -     0x7fff7919cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7919d000 -     0x7fff791e6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff791e7000 -     0x7fff7920cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7920d000 -     0x7fff79258fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff79259000 -     0x7fff79278fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff79279000 -     0x7fff7931dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7931e000 -     0x7fff79328ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff79329000 -     0x7fff79332ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff79333000 -     0x7fff7933aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7933b000 -     0x7fff79346fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff79347000 -     0x7fff7934aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7934b000 -     0x7fff7934cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7934d000 -     0x7fff79354ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff79355000 -     0x7fff79368ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7936a000 -     0x7fff7936fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff79370000 -     0x7fff7939cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2124
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=17.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9260K        8 
MALLOC guard page                   16K        4 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4556K       45 
__LINKEDIT                       189.0M        5 
__TEXT                            9652K       44 
===========                     =======  ======= 
TOTAL                            276.1M      109 
TOTAL                            276.1M      109 
TOTAL, minus reserved VM space   275.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-20-065419-2_Traviss-Mac-1044.crash
Process:               a [42347]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42339]
Responsible:           a [42347]
User ID:               501
Date/Time:             2019-06-20 06:53:03.369 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x00000001004179de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x00000001003e36c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x00000001003e0d6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x00000001003e0589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x00000001003e0d43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x00000001003e22aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x00000001003e0276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x0000000100417458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x000000010042703f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x0000000100417f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x00000001003e2bf9 main + 41
11  libdyld.dylib                  0x00007fff790b3115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeef8213d8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000100467e42  rbp: 0x00007ffeef8214d0  rsp: 0x00007ffeef821400
   r8: 0xffffffff00000100   r9: 0x000000010049bae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000100466c38  r14: 0x00000001003e5480  r15: 0x00007ffeef8214e0
  rip: 0x00000001004179de  rfl: 0x0000000000010206  cr2: 0x00007fe4dcc560d0
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1003dc000 -        0x1003e4ff7 +a (0) <16143907-4FA3-35D4-88F2-CCC000904811> /Users/USER/*/a
       0x1003f4000 -        0x100493fff +libstd-329ef83dbc1dd86d.dylib (0) <C5003865-3170-3C67-96C0-AE40A4464E4D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x1090a4000 -        0x1090ee98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7691d000 -     0x7fff76950fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff76e2f000 -     0x7fff76e30ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff770e5000 -     0x7fff7713bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7713c000 -     0x7fff77160ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff784b2000 -     0x7fff788a33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff78970000 -     0x7fff7898cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff78f4a000 -     0x7fff78f4eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff78f4f000 -     0x7fff78f59ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff78f5a000 -     0x7fff78f61fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff78f62000 -     0x7fff78f6affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff78f6b000 -     0x7fff78ff0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79078000 -     0x7fff790b1ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff790b2000 -     0x7fff790cfff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff790d0000 -     0x7fff790d0ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff790de000 -     0x7fff790deff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff790df000 -     0x7fff790e3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff790e4000 -     0x7fff790e6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff790e7000 -     0x7fff790e8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff790e9000 -     0x7fff79100fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff79101000 -     0x7fff79101fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff79102000 -     0x7fff7918bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7918c000 -     0x7fff7918fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff79190000 -     0x7fff79193ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff79194000 -     0x7fff79195fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff79196000 -     0x7fff7919cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7919d000 -     0x7fff791e6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff791e7000 -     0x7fff7920cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7920d000 -     0x7fff79258fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff79259000 -     0x7fff79278fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff79279000 -     0x7fff7931dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7931e000 -     0x7fff79328ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff79329000 -     0x7fff79332ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff79333000 -     0x7fff7933aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7933b000 -     0x7fff79346fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff79347000 -     0x7fff7934aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7934b000 -     0x7fff7934cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7934d000 -     0x7fff79354ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff79355000 -     0x7fff79368ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7936a000 -     0x7fff7936fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff79370000 -     0x7fff7939cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2124
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=17.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=17.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9724K       10 
MALLOC guard page                   16K        4 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4556K       45 
__LINKEDIT                       189.0M        5 
__TEXT                            9676K       44 
===========                     =======  ======= 
TOTAL                            276.6M      111 
TOTAL                            276.6M      111 
TOTAL, minus reserved VM space   276.4M      111 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-20-065419-3_Traviss-Mac-1044.crash
Process:               a [41578]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41575]
Responsible:           a [41578]
User ID:               501
Date/Time:             2019-06-20 06:52:36.550 +0000
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
0   a                              0x000000010d53a7fe abort_on_c_abi::panic_in_ffi::h8a291139e67b5975 + 30
1   a                              0x000000010d539bf9 std::panicking::try::do_call::h279475168e6f9cd6 (.llvm.15361624866771944484) + 9
2   libstd-329ef83dbc1dd86d.dylib  0x000000010d57503f __rust_maybe_catch_panic + 31
3   a                              0x000000010d53aa51 abort_on_c_abi::main::he771bf881fc862e3 + 593
4   a                              0x000000010d5390d6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hbc930dad328663fe + 6
5   libstd-329ef83dbc1dd86d.dylib  0x000000010d565458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
6   libstd-329ef83dbc1dd86d.dylib  0x000000010d57503f __rust_maybe_catch_panic + 31
7   libstd-329ef83dbc1dd86d.dylib  0x000000010d565f3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
8   a                              0x000000010d53ad59 main + 41
9   libdyld.dylib                  0x00007fff790b3115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fc57f500010  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee26c4c58  rsi: 0x00000000ffffffc3  rbp: 0x00007ffee26c56b0  rsp: 0x00007ffee26c56b0
   r8: 0x0000000057f50006   r9: 0x0000000000000004  r10: 0x0000000116ff48c0  r11: 0x00007fff7936a96c
  r12: 0x000000010d88b000  r13: 0x0000000000000000  r14: 0x00007ffee26c57d0  r15: 0x00007ffee26c5718
  rip: 0x000000010d53a7fe  rfl: 0x0000000000010206  cr2: 0x000000010d5b4998
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10d538000 -        0x10d53bfff +a (0) <B9B7BC2A-8B83-375C-A5CD-F87771AAF509> /Users/USER/*/a
       0x10d542000 -        0x10d5e1fff +libstd-329ef83dbc1dd86d.dylib (0) <C5003865-3170-3C67-96C0-AE40A4464E4D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x116fa2000 -        0x116fec98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7691d000 -     0x7fff76950fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff76e2f000 -     0x7fff76e30ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff770e5000 -     0x7fff7713bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7713c000 -     0x7fff77160ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff784b2000 -     0x7fff788a33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff78970000 -     0x7fff7898cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff78f4a000 -     0x7fff78f4eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff78f4f000 -     0x7fff78f59ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff78f5a000 -     0x7fff78f61fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff78f62000 -     0x7fff78f6affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff78f6b000 -     0x7fff78ff0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79078000 -     0x7fff790b1ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff790b2000 -     0x7fff790cfff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff790d0000 -     0x7fff790d0ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff790de000 -     0x7fff790deff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff790df000 -     0x7fff790e3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff790e4000 -     0x7fff790e6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff790e7000 -     0x7fff790e8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff790e9000 -     0x7fff79100fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff79101000 -     0x7fff79101fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff79102000 -     0x7fff7918bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7918c000 -     0x7fff7918fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff79190000 -     0x7fff79193ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff79194000 -     0x7fff79195fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff79196000 -     0x7fff7919cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7919d000 -     0x7fff791e6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff791e7000 -     0x7fff7920cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7920d000 -     0x7fff79258fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff79259000 -     0x7fff79278fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff79279000 -     0x7fff7931dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7931e000 -     0x7fff79328ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff79329000 -     0x7fff79332ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff79333000 -     0x7fff7933aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7933b000 -     0x7fff79346fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff79347000 -     0x7fff7934aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7934b000 -     0x7fff7934cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7934d000 -     0x7fff79354ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff79355000 -     0x7fff79368ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7936a000 -     0x7fff7936fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff79370000 -     0x7fff7939cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2124
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=18.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=18.4M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.0M        8 
MALLOC guard page                   16K        5 
Stack Guard                       56.0M        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4556K       45 
__LINKEDIT                       189.0M        5 
__TEXT                            9656K       44 
===========                     =======  ======= 
TOTAL                            277.1M      110 
TOTAL                            277.1M      110 
TOTAL, minus reserved VM space   276.9M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-06-20-065419_Traviss-Mac-1044.crash
Process:               a [42348]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [42339]
Responsible:           a [42348]
User ID:               501
Date/Time:             2019-06-20 06:53:03.467 +0000
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
0   libstd-329ef83dbc1dd86d.dylib  0x00000001032aa9de std::panicking::rust_panic_with_hook::h756d0bba11076be5 + 142
1   a                              0x000000010327b6c5 std::panicking::begin_panic::haff5b930812bcdc4 + 37
2   a                              0x0000000103278d6c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hb0a79f427bc4332a + 28
3   a                              0x0000000103278589 core::ptr::real_drop_in_place::h40492c2c00bdb879 + 9
4   a                              0x0000000103278d43 backtrace::double::h35adec2a6f63ef6c + 35
5   a                              0x000000010327a2aa backtrace::main::hc9a5bc8fc93ded64 + 5210 (backtrace.rs:119)
6   a                              0x0000000103278276 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hc80788536e97b081 + 6 (rt.rs:64)
7   libstd-329ef83dbc1dd86d.dylib  0x00000001032aa458 std::panicking::try::do_call::hf4a9ae9eb738ad83 + 24
8   libstd-329ef83dbc1dd86d.dylib  0x00000001032ba03f __rust_maybe_catch_panic + 31
9   libstd-329ef83dbc1dd86d.dylib  0x00000001032aaf3e std::rt::lang_start_internal::hb2916a8a31799db8 + 542
10  a                              0x000000010327abf9 main + 41
11  libdyld.dylib                  0x00007fff790b3115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeec9893c8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001032fae42  rbp: 0x00007ffeec9894c0  rsp: 0x00007ffeec9893f0
   r8: 0xffffffff00000100   r9: 0x000000010332eae0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001032f9c38  r14: 0x000000010327d480  r15: 0x00007ffeec9894d0
  rip: 0x00000001032aa9de  rfl: 0x0000000000010202  cr2: 0x000000010fb10000
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x103274000 -        0x10327cff7 +a (0) <16143907-4FA3-35D4-88F2-CCC000904811> /Users/USER/*/a
       0x103287000 -        0x103326fff +libstd-329ef83dbc1dd86d.dylib (0) <C5003865-3170-3C67-96C0-AE40A4464E4D> /Users/USER/*/libstd-329ef83dbc1dd86d.dylib
       0x108d81000 -        0x108dcb98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7691d000 -     0x7fff76950fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff76e2f000 -     0x7fff76e30ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff770e5000 -     0x7fff7713bfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff7713c000 -     0x7fff77160ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff784b2000 -     0x7fff788a33b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff78970000 -     0x7fff7898cffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff78f4a000 -     0x7fff78f4eff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff78f4f000 -     0x7fff78f59ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff78f5a000 -     0x7fff78f61fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff78f62000 -     0x7fff78f6affb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff78f6b000 -     0x7fff78ff0fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff79078000 -     0x7fff790b1ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff790b2000 -     0x7fff790cfff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff790d0000 -     0x7fff790d0ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff790de000 -     0x7fff790deff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff790df000 -     0x7fff790e3ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff790e4000 -     0x7fff790e6ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff790e7000 -     0x7fff790e8ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff790e9000 -     0x7fff79100fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff79101000 -     0x7fff79101fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff79102000 -     0x7fff7918bff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff7918c000 -     0x7fff7918fffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff79190000 -     0x7fff79193ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff79194000 -     0x7fff79195fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff79196000 -     0x7fff7919cff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff7919d000 -     0x7fff791e6ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff791e7000 -     0x7fff7920cff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff7920d000 -     0x7fff79258fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff79259000 -     0x7fff79278fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff79279000 -     0x7fff7931dff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff7931e000 -     0x7fff79328ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff79329000 -     0x7fff79332ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff79333000 -     0x7fff7933aff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff7933b000 -     0x7fff79346fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff79347000 -     0x7fff7934aff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff7934b000 -     0x7fff7934cff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff7934d000 -     0x7fff79354ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff79355000 -     0x7fff79368ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff7936a000 -     0x7fff7936fff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff79370000 -     0x7fff7939cff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
