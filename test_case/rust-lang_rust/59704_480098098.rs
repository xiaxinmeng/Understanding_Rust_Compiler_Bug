plain
[00:03:00]       Memory: 8 GB
[00:03:00]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:00]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:00]       SMC Version (system): 2.8f0
[00:03:00]       Serial Number (system): VMrST5nkVt4p
[00:03:00] 
[00:03:00] hw.ncpu: 4
[00:03:00] hw.byteorder: 1234
[00:03:00] hw.memsize: 8589934592
---
[02:10:24] 
[02:10:24] ---- /Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals::By_value_trait_objects (line 104) stdout ----
[02:10:24] error: linking with `cc` failed: signal: 4
[02:10:24]   |
[02:10:24]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestO3goA1/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-28edbc69fd879800.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-b152b62305f45f50.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-6c9d408d4cf1d58a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-460a4e8729661107.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-f24cc8363760ae6f.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-376cf09404a6e600.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-cff1600e2f9b6c28.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-233abb06723a85b7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-98b24fb5d296bb0b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-51279115ab0d6b82.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:10:24] 
[02:10:24] error: aborting due to previous error
[02:10:24] 
[02:10:24] thread '/Users/travis/build/rust-lang/rust/src/doc/unstable-book/src/language-features/unsized-locals.md - unsized_locals::By_value_trait_objects (line 104)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:10:24] 
[02:10:24] 
[02:10:24] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:10:24] Build completed unsuccessfully in 0:54:51
[02:10:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:009ffcc3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Apr  4 23:24:46 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:2939eb5c
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Apr  4 23:24 .
-rw-------@  1 travis  staff  13742 Apr  4 23:24 overflow_2019-04-04-232434_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Apr  4 23:24 foo_2019-04-04-232407_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Apr  4 23:23 m4_2019-04-04-232338_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Apr  4 23:23 b_2019-04-04-232328_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Apr  4 23:23 bar_2019-04-04-232328-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Apr  4 23:23 bar_2019-04-04-232328_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 Apr  4 22:46 a_2019-04-04-224604-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Apr  4 22:46 a_2019-04-04-224604_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 Apr  4 22:45 a_2019-04-04-224556-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37229 Apr  4 22:45 a_2019-04-04-224556_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Apr  4 22:45 a_2019-04-04-224551_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Apr  4 22:45 a_2019-04-04-224545_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Apr  4 22:45 a_2019-04-04-224538_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9792 Apr  4 22:45 a_2019-04-04-224536_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Apr  4 22:45 a_2019-04-04-224500_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63100 Apr  4 22:44 a_2019-04-04-224448_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Apr  4 22:44 a_2019-04-04-224444-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64264 Apr  4 22:44 a_2019-04-04-224444-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Apr  4 22:44 a_2019-04-04-224444_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11713 Apr  4 22:42 a_2019-04-04-224233_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Apr  4 22:41 a_2019-04-04-224144_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr  4 22:40 a_2019-04-04-224036-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10213 Apr  4 22:40 a_2019-04-04-224036-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr  4 22:40 a_2019-04-04-224036-3_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Apr  4 22:40 a_2019-04-04-224036_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:01980190
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-04-224036-1_Traviss-Mac-1044.crash
Process:               a [45010]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45000]
Responsible:           a [45010]
User ID:               501
Date/Time:             2019-04-04 22:39:25.243 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-28edbc69fd879800.dylib  0x00000001090a10ce std::panicking::rust_panic_with_hook::h18eece72e0e76c42 + 142
1   a                              0x000000010906a8e5 std::panicking::begin_panic::hc3f6f6ee8ad55670 + 37
2   a                              0x000000010906840c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000109067969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x00000001090683e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000109069559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x0000000109067936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf6a1ce72644edbef + 6 (rt.rs:64)
7   libstd-28edbc69fd879800.dylib  0x00000001090a0b48 std::panicking::try::do_call::he44de2c8a6962430 + 24
8   libstd-28edbc69fd879800.dylib  0x00000001090b07bf __rust_maybe_catch_panic + 31
9   libstd-28edbc69fd879800.dylib  0x00000001090a162e std::rt::lang_start_internal::h808ee93b9537043a + 542
10  a                              0x0000000109069d99 main + 41
11  libdyld.dylib                  0x00007fff5abaf115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee6b993d8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001090e8fd2  rbp: 0x00007ffee6b994d0  rsp: 0x00007ffee6b99400
   r8: 0x0000000000000100   r9: 0x000000010911ba30  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001090e7d38  r14: 0x000000010906c460  r15: 0x00007ffee6b994e0
  rip: 0x00000001090a10ce  rfl: 0x0000000000010206  cr2: 0x0000000109389699
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x109064000 -        0x10906bfff +a (0) <1908C991-C11F-3EC9-96D6-64ED62B75912> /Users/USER/*/a
       0x10907c000 -        0x109113fff +libstd-28edbc69fd879800.dylib (0) <196A1199-CAC1-3EBF-AF93-2A85F4933222> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x10c340000 -        0x10c38a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58419000 -     0x7fff5844cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5892b000 -     0x7fff5892cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff58be1000 -     0x7fff58c37fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff58c38000 -     0x7fff58c5cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff59fae000 -     0x7fff5a39f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5a46c000 -     0x7fff5a488ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5aa46000 -     0x7fff5aa4aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5aa4b000 -     0x7fff5aa55ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5aa56000 -     0x7fff5aa5dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5aa5e000 -     0x7fff5aa66ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5aa67000 -     0x7fff5aaecfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5ab74000 -     0x7fff5abadff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5abae000 -     0x7fff5abcbff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5abcc000 -     0x7fff5abccffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5abda000 -     0x7fff5abdaff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5abdb000 -     0x7fff5abdfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5abe0000 -     0x7fff5abe2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5abe3000 -     0x7fff5abe4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5abe5000 -     0x7fff5abfcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5abfd000 -     0x7fff5abfdfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5abfe000 -     0x7fff5ac87ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5ac88000 -     0x7fff5ac8bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5ac8c000 -     0x7fff5ac8fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5ac90000 -     0x7fff5ac91fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5ac92000 -     0x7fff5ac98ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5ac99000 -     0x7fff5ace2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5ace3000 -     0x7fff5ad08ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5ad09000 -     0x7fff5ad54fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5ad55000 -     0x7fff5ad74fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5ad75000 -     0x7fff5ae19ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5ae1a000 -     0x7fff5ae24ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5ae25000 -     0x7fff5ae2eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5ae2f000 -     0x7fff5ae36ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5ae37000 -     0x7fff5ae42fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5ae43000 -     0x7fff5ae46ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5ae47000 -     0x7fff5ae48ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5ae49000 -     0x7fff5ae50ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5ae51000 -     0x7fff5ae64ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5ae66000 -     0x7fff5ae6bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5ae6c000 -     0x7fff5ae98ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2319
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=83.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=83.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.5M       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            286.5M      110 
TOTAL                            286.5M      110 
TOTAL, minus reserved VM space   286.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-04-224036-2_Traviss-Mac-1044.crash
Process:               a [44187]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44185]
Responsible:           a [44187]
User ID:               501
Date/Time:             2019-04-04 22:39:01.494 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5200 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   a                              0x000000010a2458ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010a244ca9 std::panicking::try::do_call::hc7b4d50989e643d9 (.llvm.1328971572518733427) + 9
2   libstd-28edbc69fd879800.dylib  0x000000010a2857bf __rust_maybe_catch_panic + 31
3   a                              0x000000010a245b01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010a2440f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf4f268f28e9a6b3d + 6
5   libstd-28edbc69fd879800.dylib  0x000000010a275b48 std::panicking::try::do_call::he44de2c8a6962430 + 24
6   libstd-28edbc69fd879800.dylib  0x000000010a2857bf __rust_maybe_catch_panic + 31
7   libstd-28edbc69fd879800.dylib  0x000000010a27662e std::rt::lang_start_internal::h808ee93b9537043a + 542
8   a                              0x000000010a245e09 main + 41
9   libdyld.dylib                  0x00007fff5abaf115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007f8b40402be0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee59b9c28  rsi: 0x000000007fffffff  rbp: 0x00007ffee59ba680  rsp: 0x00007ffee59ba680
   r8: 0x00000000b40402c3   r9: 0x0000000000000004  r10: 0x00000001102988c0  r11: 0x00007fff5ae6696c
  r12: 0x000000010a59c000  r13: 0x0000000000000000  r14: 0x00007ffee59ba7a0  r15: 0x00007ffee59ba6e8
  rip: 0x000000010a2458ae  rfl: 0x0000000000010202  cr2: 0x000000010a2bcb28
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10a243000 -        0x10a246ff7 +a (0) <C4703FA1-C66C-3BAE-B8E2-80449D40FC79> /Users/USER/*/a
       0x10a251000 -        0x10a2e8fff +libstd-28edbc69fd879800.dylib (0) <196A1199-CAC1-3EBF-AF93-2A85F4933222> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x110246000 -        0x11029098f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58419000 -     0x7fff5844cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5892b000 -     0x7fff5892cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff58be1000 -     0x7fff58c37fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff58c38000 -     0x7fff58c5cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff59fae000 -     0x7fff5a39f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5a46c000 -     0x7fff5a488ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5aa46000 -     0x7fff5aa4aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5aa4b000 -     0x7fff5aa55ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5aa56000 -     0x7fff5aa5dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5aa5e000 -     0x7fff5aa66ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5aa67000 -     0x7fff5aaecfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5ab74000 -     0x7fff5abadff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5abae000 -     0x7fff5abcbff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5abcc000 -     0x7fff5abccffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5abda000 -     0x7fff5abdaff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5abdb000 -     0x7fff5abdfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5abe0000 -     0x7fff5abe2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5abe3000 -     0x7fff5abe4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5abe5000 -     0x7fff5abfcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5abfd000 -     0x7fff5abfdfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5abfe000 -     0x7fff5ac87ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5ac88000 -     0x7fff5ac8bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5ac8c000 -     0x7fff5ac8fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5ac90000 -     0x7fff5ac91fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5ac92000 -     0x7fff5ac98ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5ac99000 -     0x7fff5ace2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5ace3000 -     0x7fff5ad08ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5ad09000 -     0x7fff5ad54fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5ad55000 -     0x7fff5ad74fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5ad75000 -     0x7fff5ae19ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5ae1a000 -     0x7fff5ae24ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5ae25000 -     0x7fff5ae2eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5ae2f000 -     0x7fff5ae36ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5ae37000 -     0x7fff5ae42fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5ae43000 -     0x7fff5ae46ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5ae47000 -     0x7fff5ae48ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5ae49000 -     0x7fff5ae50ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5ae51000 -     0x7fff5ae64ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5ae66000 -     0x7fff5ae6bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5ae6c000 -     0x7fff5ae98ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2319
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
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.1M      108 
TOTAL                            276.1M      108 
TOTAL, minus reserved VM space   276.0M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-04-224036-3_Traviss-Mac-1044.crash
Process:               a [45011]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45000]
Responsible:           a [45011]
User ID:               501
Date/Time:             2019-04-04 22:39:25.257 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_BAD_INSTRUCTION (SIGILL)
Exception Codes:       0x0000000000000001, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
Termination Signal:    Illegal instruction: 4
Termination Reason:    Namespace SIGNAL, Code 0x4
Terminating Process:   exc handler [0]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libstd-28edbc69fd879800.dylib  0x000000010b6350ce std::panicking::rust_panic_with_hook::h18eece72e0e76c42 + 142
1   a                              0x000000010b5fe8e5 std::panicking::begin_panic::hc3f6f6ee8ad55670 + 37
2   a                              0x000000010b5fc40c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x000000010b5fb969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x000000010b5fc3e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x000000010b5fd559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x000000010b5fb936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf6a1ce72644edbef + 6 (rt.rs:64)
7   libstd-28edbc69fd879800.dylib  0x000000010b634b48 std::panicking::try::do_call::he44de2c8a6962430 + 24
8   libstd-28edbc69fd879800.dylib  0x000000010b6447bf __rust_maybe_catch_panic + 31
9   libstd-28edbc69fd879800.dylib  0x000000010b63562e std::rt::lang_start_internal::h808ee93b9537043a + 542
10  a                              0x000000010b5fdd99 main + 41
11  libdyld.dylib                  0x00007fff5abaf115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee46053c8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x000000010b67cfd2  rbp: 0x00007ffee46054c0  rsp: 0x00007ffee46053f0
   r8: 0x0000000000000100   r9: 0x000000010b6afa30  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010b67bd38  r14: 0x000000010b600460  r15: 0x00007ffee46054d0
  rip: 0x000000010b6350ce  rfl: 0x0000000000010202  cr2: 0x00000001105b503c
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10b5f8000 -        0x10b5fffff +a (0) <1908C991-C11F-3EC9-96D6-64ED62B75912> /Users/USER/*/a
       0x10b610000 -        0x10b6a7fff +libstd-28edbc69fd879800.dylib (0) <196A1199-CAC1-3EBF-AF93-2A85F4933222> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x11a071000 -        0x11a0bb98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58419000 -     0x7fff5844cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5892b000 -     0x7fff5892cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff58be1000 -     0x7fff58c37fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff58c38000 -     0x7fff58c5cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff59fae000 -     0x7fff5a39f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5a46c000 -     0x7fff5a488ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5aa46000 -     0x7fff5aa4aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5aa4b000 -     0x7fff5aa55ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5aa56000 -     0x7fff5aa5dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5aa5e000 -     0x7fff5aa66ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5aa67000 -     0x7fff5aaecfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5ab74000 -     0x7fff5abadff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5abae000 -     0x7fff5abcbff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5abcc000 -     0x7fff5abccffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5abda000 -     0x7fff5abdaff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5abdb000 -     0x7fff5abdfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5abe0000 -     0x7fff5abe2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5abe3000 -     0x7fff5abe4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5abe5000 -     0x7fff5abfcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5abfd000 -     0x7fff5abfdfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5abfe000 -     0x7fff5ac87ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5ac88000 -     0x7fff5ac8bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5ac8c000 -     0x7fff5ac8fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5ac90000 -     0x7fff5ac91fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5ac92000 -     0x7fff5ac98ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5ac99000 -     0x7fff5ace2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5ace3000 -     0x7fff5ad08ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5ad09000 -     0x7fff5ad54fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5ad55000 -     0x7fff5ad74fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5ad75000 -     0x7fff5ae19ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5ae1a000 -     0x7fff5ae24ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5ae25000 -     0x7fff5ae2eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5ae2f000 -     0x7fff5ae36ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5ae37000 -     0x7fff5ae42fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5ae43000 -     0x7fff5ae46ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5ae47000 -     0x7fff5ae48ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5ae49000 -     0x7fff5ae50ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5ae51000 -     0x7fff5ae64ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5ae66000 -     0x7fff5ae6bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5ae6c000 -     0x7fff5ae98ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2319
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9696K       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            276.5M      110 
TOTAL                            276.5M      110 
TOTAL, minus reserved VM space   276.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-04-224036_Traviss-Mac-1044.crash
Process:               a [46683]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46680]
Responsible:           a [46683]
User ID:               501
Date/Time:             2019-04-04 22:40:24.187 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 5300 seconds
System Integrity Protection: enabled
Crashed Thread:        0  Dispatch queue: com.apple.main-thread
Exception Type:        EXC_CRASH (SIGABRT)
Exception Codes:       0x0000000000000000, 0x0000000000000000
Exception Note:        EXC_CORPSE_NOTIFY
abort() called
abort() called
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib         0x00007fff5acfee3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5ae3d150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff5ac5b312 abort + 127
3   libstd-28edbc69fd879800.dylib  0x000000010a53eca9 std::sys::unix::abort_internal::h8137cbab466f581f + 9
4   libstd-28edbc69fd879800.dylib  0x000000010a52f160 rust_oom + 32
5   libstd-28edbc69fd879800.dylib  0x000000010a54ff69 alloc::alloc::handle_alloc_error::h0c82d64670f3f29b + 9
6   a                              0x000000010a50341f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x000000010a502696 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hd8082d12249549da + 6
8   libstd-28edbc69fd879800.dylib  0x000000010a52fb48 std::panicking::try::do_call::he44de2c8a6962430 + 24
9   libstd-28edbc69fd879800.dylib  0x000000010a53f7bf __rust_maybe_catch_panic + 31
10  libstd-28edbc69fd879800.dylib  0x000000010a53062e std::rt::lang_start_internal::h808ee93b9537043a + 542
11  a                              0x000000010a503579 main + 41
12  libdyld.dylib                  0x00007fff5abaf115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff938d5340  rcx: 0x00007ffee56fc5e8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee56fc620  rsp: 0x00007ffee56fc5e8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5acfee3e  rfl: 0x0000000000000206  cr2: 0x00007fff938b3148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10a501000 -        0x10a503ff7 +a (0) <41E90DB3-381D-3F00-963F-675AF220E7E6> /Users/USER/*/a
       0x10a50b000 -        0x10a5a2fff +libstd-28edbc69fd879800.dylib (0) <196A1199-CAC1-3EBF-AF93-2A85F4933222> /Users/USER/*/libstd-28edbc69fd879800.dylib
       0x114858000 -        0x1148a298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff58419000 -     0x7fff5844cfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff5892b000 -     0x7fff5892cff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff58be1000 -     0x7fff58c37fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff58c38000 -     0x7fff58c5cff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff59fae000 -     0x7fff5a39f3b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5a46c000 -     0x7fff5a488ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff5aa46000 -     0x7fff5aa4aff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff5aa4b000 -     0x7fff5aa55ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff5aa56000 -     0x7fff5aa5dfff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff5aa5e000 -     0x7fff5aa66ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff5aa67000 -     0x7fff5aaecfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff5ab74000 -     0x7fff5abadff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff5abae000 -     0x7fff5abcbff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff5abcc000 -     0x7fff5abccffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff5abda000 -     0x7fff5abdaff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff5abdb000 -     0x7fff5abdfffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff5abe0000 -     0x7fff5abe2ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff5abe3000 -     0x7fff5abe4ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff5abe5000 -     0x7fff5abfcfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff5abfd000 -     0x7fff5abfdfff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff5abfe000 -     0x7fff5ac87ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff5ac88000 -     0x7fff5ac8bffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff5ac8c000 -     0x7fff5ac8fffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff5ac90000 -     0x7fff5ac91fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff5ac92000 -     0x7fff5ac98ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff5ac99000 -     0x7fff5ace2ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff5ace3000 -     0x7fff5ad08ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5ad09000 -     0x7fff5ad54fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff5ad55000 -     0x7fff5ad74fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff5ad75000 -     0x7fff5ae19ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5ae1a000 -     0x7fff5ae24ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff5ae25000 -     0x7fff5ae2eff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff5ae2f000 -     0x7fff5ae36ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff5ae37000 -     0x7fff5ae42fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff5ae43000 -     0x7fff5ae46ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff5ae47000 -     0x7fff5ae48ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5ae49000 -     0x7fff5ae50ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff5ae51000 -     0x7fff5ae64ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff5ae66000 -     0x7fff5ae6bff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5ae6c000 -     0x7fff5ae98ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
