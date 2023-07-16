plain
[00:02:54]       Memory: 8 GB
[00:02:54]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:02:54]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:02:54]       SMC Version (system): 2.8f0
[00:02:54]       Serial Number (system): VMThdk5BjR6G
[00:02:54] 
[00:02:54] hw.ncpu: 4
[00:02:54] hw.byteorder: 1234
[00:02:54] hw.memsize: 8589934592
---
[00:04:01] * pattern                          lib      unstable     None    
[00:04:01] * pattern_parentheses              lang     stable       1.31.0  
[00:04:01] * peek                             lib      stable       1.18.0  
[00:04:01] * pin                              lib      stable       1.33.0  
[00:04:01] * pin_partialeq_partialord_impl_applicability lib      stable       1.34.0  
[00:04:01] * plugin                           lang     unstable     1.0.0   
[00:04:01] * plugin_registrar                 lang     unstable     1.0.0   
[00:04:01] * pointer_methods                  lib      stable       1.26.0  
[00:04:01] * powerpc_target_feature           lang     unstable     1.27.0  
---
[01:13:23] * pattern                          lib      unstable     None    
[01:13:23] * pattern_parentheses              lang     stable       1.31.0  
[01:13:23] * peek                             lib      stable       1.18.0  
[01:13:23] * pin                              lib      stable       1.33.0  
[01:13:23] * pin_partialeq_partialord_impl_applicability lib      stable       1.34.0  
[01:13:23] * plugin                           lang     unstable     1.0.0   
[01:13:23] * plugin_registrar                 lang     unstable     1.0.0   
[01:13:23] * pointer_methods                  lib      stable       1.26.0  
[01:13:23] * powerpc_target_feature           lang     unstable     1.27.0  
---
[01:52:00] 
[01:52:00] ---- alloc.rs - alloc::alloc_zeroed (line 126) stdout ----
[01:52:00] error: linking with `cc` failed: signal: 4
[01:52:00]   |
[01:52:00]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.10.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.8.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.rust_out.7rcbfp3g-cgu.9.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestn01aAx/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/x86_64-apple-darwin/release/build/compiler_builtins-22e11b6d0e739d2e/out" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1-std/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libstd-d5e594e720c390d6.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-f8e431478626c027.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-f554ac0c08bb33ee.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libunwind-b8e993c77d021ba0.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-5b6ea2d7d5d6740f.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/liblibc-33c512cbec99717c.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/liballoc-f4a1614a7a8a6f86.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-8c58198746621e91.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libcore-4a71e986e661e449.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-c63a82f6c0200b58.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[01:52:00] 
[01:52:00] thread 'alloc.rs - alloc::alloc_zeroed (line 126)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:319:13
[01:52:00] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:52:00] 
---
[01:52:00] 
[01:52:00] 
[01:52:00] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[01:52:00] Build completed unsuccessfully in 0:38:48
[01:52:00] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c3958a4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 18 16:17:02 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:0a0390f8
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1184
drwx------  21 travis  staff    714 Jan 18 15:55 .
-rw-------@  1 travis  staff  62244 Jan 18 15:55 a_2019-01-18-155503_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37481 Jan 18 15:55 a_2019-01-18-155502_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60387 Jan 18 15:54 a_2019-01-18-155454-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37238 Jan 18 15:54 a_2019-01-18-155454_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Jan 18 15:54 a_2019-01-18-155451_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Jan 18 15:54 a_2019-01-18-155444_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Jan 18 15:54 a_2019-01-18-155438_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9795 Jan 18 15:54 a_2019-01-18-155436_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Jan 18 15:53 a_2019-01-18-155358_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63128 Jan 18 15:53 a_2019-01-18-155347_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Jan 18 15:53 a_2019-01-18-155345-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64248 Jan 18 15:53 a_2019-01-18-155345-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65091 Jan 18 15:53 a_2019-01-18-155345_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11585 Jan 18 15:51 a_2019-01-18-155141_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Jan 18 15:50 a_2019-01-18-155041_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Jan 18 15:49 a_2019-01-18-154918_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Jan 18 15:48 a_2019-01-18-154852_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jan 18 15:48 a_2019-01-18-154853-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Jan 18 15:48 a_2019-01-18-154853-2_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:101b4892
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-18-154852_Traviss-Mac-1044.crash
Process:               a [40780]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [40779]
Responsible:           a [40780]
User ID:               501
Date/Time:             2019-01-18 15:47:28.325 +0000
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
0   a                              0x000000010d9bc75e abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010d9bbb49 std::panicking::try::do_call::h95d8531d3f294881 (.llvm.11246735667575966273) + 9
2   libstd-d5e594e720c390d6.dylib  0x000000010d9fa84f __rust_maybe_catch_panic + 31
3   a                              0x000000010d9bc9b1 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010d9baef6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h759769a5798fd3c9 + 6
5   libstd-d5e594e720c390d6.dylib  0x000000010d9ea6c8 std::panicking::try::do_call::h01d564bb08a29534 + 24
6   libstd-d5e594e720c390d6.dylib  0x000000010d9fa84f __rust_maybe_catch_panic + 31
7   libstd-d5e594e720c390d6.dylib  0x000000010d9eb1bd std::rt::lang_start_internal::hc268a56a67b71744 + 541
8   a                              0x000000010d9bccb9 main + 41
9   libdyld.dylib                  0x00007fff6eea7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ff1d2c02ba0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee22430e8  rsi: 0x0000000087ffffff  rbp: 0x00007ffee2243b40  rsp: 0x00007ffee2243b40
   r8: 0x000000001d2c02bf   r9: 0x0000000000000004  r10: 0x0000000119cf38d0  r11: 0x00007fff6f15e96c
  r12: 0x00007ffee2243e10  r13: 0x0000000000000000  r14: 0x00007ffee2243c60  r15: 0x00007ffee2243ba8
  rip: 0x000000010d9bc75e  rfl: 0x0000000000010206  cr2: 0x000000010da46000
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10d9ba000 -        0x10d9bdff7 +a (0) <4B9689A1-66BF-33A1-A711-854847AFF7AD> /Users/USER/*/a
       0x10d9c7000 -        0x10da5afff +libstd-d5e594e720c390d6.dylib (0) <7D166126-C70F-300E-B8CC-1DE07AD86007> /Users/USER/*/libstd-d5e594e720c390d6.dylib
       0x119ca1000 -        0x119ceb98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6c711000 -     0x7fff6c744fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6cc23000 -     0x7fff6cc24ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6ced9000 -     0x7fff6cf2ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6cf30000 -     0x7fff6cf54ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6e2a6000 -     0x7fff6e6973b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6e764000 -     0x7fff6e780ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ed3e000 -     0x7fff6ed42ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ed43000 -     0x7fff6ed4dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ed4e000 -     0x7fff6ed55fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ed56000 -     0x7fff6ed5effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ed5f000 -     0x7fff6ede4fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6ee6c000 -     0x7fff6eea5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6eea6000 -     0x7fff6eec3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6eec4000 -     0x7fff6eec4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6eed2000 -     0x7fff6eed2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6eed3000 -     0x7fff6eed7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6eed8000 -     0x7fff6eedaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6eedb000 -     0x7fff6eedcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6eedd000 -     0x7fff6eef4fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6eef5000 -     0x7fff6eef5fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6eef6000 -     0x7fff6ef7fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6ef80000 -     0x7fff6ef83ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6ef84000 -     0x7fff6ef87ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6ef88000 -     0x7fff6ef89fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6ef8a000 -     0x7fff6ef90ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6ef91000 -     0x7fff6efdaff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6efdb000 -     0x7fff6f000ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6f001000 -     0x7fff6f04cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6f04d000 -     0x7fff6f06cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6f06d000 -     0x7fff6f111ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6f112000 -     0x7fff6f11cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6f11d000 -     0x7fff6f126ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6f127000 -     0x7fff6f12eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6f12f000 -     0x7fff6f13afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6f13b000 -     0x7fff6f13eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6f13f000 -     0x7fff6f140ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6f141000 -     0x7fff6f148ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6f149000 -     0x7fff6f15cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6f15e000 -     0x7fff6f163ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6f164000 -     0x7fff6f190ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2330
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
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4536K       44 
__LINKEDIT                       188.9M        5 
__TEXT                            9608K       44 
===========                     =======  ======= 
TOTAL                            275.9M      107 
TOTAL                            275.9M      107 
TOTAL, minus reserved VM space   275.8M      107 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-18-154853-1_Traviss-Mac-1044.crash
Process:               a [41569]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41564]
Responsible:           a [41569]
User ID:               501
Date/Time:             2019-01-18 15:48:02.057 +0000
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
0   libstd-d5e594e720c390d6.dylib  0x0000000103cace5c std::panicking::rust_panic_with_hook::hd0f9f7cd98942c98 + 668
1   a                              0x0000000103c777f5 std::panicking::begin_panic::ha6378659dfc19abe + 37
2   a                              0x0000000103c752bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 28
3   a                              0x0000000103c749e9 core::ptr::real_drop_in_place::h0bccc5556cf0dbf4 + 9
4   a                              0x0000000103c75293 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000103c7642e backtrace::main::hcde7a1a1c3c85e77 + 4238 (backtrace.rs:103)
6   a                              0x0000000103c747b6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6c1004053bce8eab + 6 (rt.rs:64)
7   libstd-d5e594e720c390d6.dylib  0x0000000103cac6c8 std::panicking::try::do_call::h01d564bb08a29534 + 24
8   libstd-d5e594e720c390d6.dylib  0x0000000103cbc84f __rust_maybe_catch_panic + 31
9   libstd-d5e594e720c390d6.dylib  0x0000000103cad1bd std::rt::lang_start_internal::hc268a56a67b71744 + 541
10  a                              0x0000000103c76cb9 main + 41
11  libdyld.dylib                  0x00007fff6eea7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeebf8d888  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000103cf24e2  rbp: 0x00007ffeebf8d980  rsp: 0x00007ffeebf8d8b0
   r8: 0x0000000103cf0d00   r9: 0x0000000103d24600  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000103cf0d00  r14: 0x0000000103c79460  r15: 0x00007ffeebf8d990
  rip: 0x0000000103cace5c  rfl: 0x0000000000010206  cr2: 0x0000000103c8eae0
Logical CPU:     1
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x103c70000 -        0x103c78fff +a (0) <754C2A81-86D9-3E25-996C-C96D558346A1> /Users/USER/*/a
       0x103c89000 -        0x103d1cfff +libstd-d5e594e720c390d6.dylib (0) <7D166126-C70F-300E-B8CC-1DE07AD86007> /Users/USER/*/libstd-d5e594e720c390d6.dylib
       0x113a7e000 -        0x113ac898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6c711000 -     0x7fff6c744fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6cc23000 -     0x7fff6cc24ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6ced9000 -     0x7fff6cf2ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6cf30000 -     0x7fff6cf54ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6e2a6000 -     0x7fff6e6973b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6e764000 -     0x7fff6e780ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ed3e000 -     0x7fff6ed42ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ed43000 -     0x7fff6ed4dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ed4e000 -     0x7fff6ed55fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ed56000 -     0x7fff6ed5effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ed5f000 -     0x7fff6ede4fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6ee6c000 -     0x7fff6eea5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6eea6000 -     0x7fff6eec3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6eec4000 -     0x7fff6eec4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6eed2000 -     0x7fff6eed2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6eed3000 -     0x7fff6eed7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6eed8000 -     0x7fff6eedaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6eedb000 -     0x7fff6eedcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6eedd000 -     0x7fff6eef4fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6eef5000 -     0x7fff6eef5fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6eef6000 -     0x7fff6ef7fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6ef80000 -     0x7fff6ef83ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6ef84000 -     0x7fff6ef87ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6ef88000 -     0x7fff6ef89fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6ef8a000 -     0x7fff6ef90ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6ef91000 -     0x7fff6efdaff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6efdb000 -     0x7fff6f000ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6f001000 -     0x7fff6f04cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6f04d000 -     0x7fff6f06cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6f06d000 -     0x7fff6f111ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6f112000 -     0x7fff6f11cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6f11d000 -     0x7fff6f126ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6f127000 -     0x7fff6f12eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6f12f000 -     0x7fff6f13afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6f13b000 -     0x7fff6f13eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6f13f000 -     0x7fff6f140ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6f141000 -     0x7fff6f148ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6f149000 -     0x7fff6f15cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6f15e000 -     0x7fff6f163ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6f164000 -     0x7fff6f190ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2330
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=83.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=83.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            19.4M       10 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4536K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9628K       44 
===========                     =======  ======= 
TOTAL                            286.4M      109 
TOTAL                            286.4M      109 
TOTAL, minus reserved VM space   286.3M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-18-154853-2_Traviss-Mac-1044.crash
Process:               a [41568]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [41564]
Responsible:           a [41568]
User ID:               501
Date/Time:             2019-01-18 15:48:02.030 +0000
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
0   libstd-d5e594e720c390d6.dylib  0x00000001064fbe5c std::panicking::rust_panic_with_hook::hd0f9f7cd98942c98 + 668
1   a                              0x00000001064c97f5 std::panicking::begin_panic::ha6378659dfc19abe + 37
2   a                              0x00000001064c72bc _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hbddd8cc943232966 + 28
3   a                              0x00000001064c69e9 core::ptr::real_drop_in_place::h0bccc5556cf0dbf4 + 9
4   a                              0x00000001064c7293 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x00000001064c842e backtrace::main::hcde7a1a1c3c85e77 + 4238 (backtrace.rs:103)
6   a                              0x00000001064c67b6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h6c1004053bce8eab + 6 (rt.rs:64)
7   libstd-d5e594e720c390d6.dylib  0x00000001064fb6c8 std::panicking::try::do_call::h01d564bb08a29534 + 24
8   libstd-d5e594e720c390d6.dylib  0x000000010650b84f __rust_maybe_catch_panic + 31
9   libstd-d5e594e720c390d6.dylib  0x00000001064fc1bd std::rt::lang_start_internal::hc268a56a67b71744 + 541
10  a                              0x00000001064c8cb9 main + 41
11  libdyld.dylib                  0x00007fff6eea7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee973b8a8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001065414e2  rbp: 0x00007ffee973b9a0  rsp: 0x00007ffee973b8d0
   r8: 0x000000010653fd00   r9: 0x0000000106573600  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010653fd00  r14: 0x00000001064cb460  r15: 0x00007ffee973b9b0
  rip: 0x00000001064fbe5c  rfl: 0x0000000000010206  cr2: 0x00000001067c6e85
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1064c2000 -        0x1064cafff +a (0) <754C2A81-86D9-3E25-996C-C96D558346A1> /Users/USER/*/a
       0x1064d8000 -        0x10656bfff +libstd-d5e594e720c390d6.dylib (0) <7D166126-C70F-300E-B8CC-1DE07AD86007> /Users/USER/*/libstd-d5e594e720c390d6.dylib
       0x10f0c9000 -        0x10f11398f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6c711000 -     0x7fff6c744fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6cc23000 -     0x7fff6cc24ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6ced9000 -     0x7fff6cf2ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6cf30000 -     0x7fff6cf54ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6e2a6000 -     0x7fff6e6973b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6e764000 -     0x7fff6e780ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ed3e000 -     0x7fff6ed42ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ed43000 -     0x7fff6ed4dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ed4e000 -     0x7fff6ed55fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ed56000 -     0x7fff6ed5effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ed5f000 -     0x7fff6ede4fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6ee6c000 -     0x7fff6eea5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6eea6000 -     0x7fff6eec3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6eec4000 -     0x7fff6eec4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6eed2000 -     0x7fff6eed2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6eed3000 -     0x7fff6eed7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6eed8000 -     0x7fff6eedaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6eedb000 -     0x7fff6eedcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6eedd000 -     0x7fff6eef4fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6eef5000 -     0x7fff6eef5fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6eef6000 -     0x7fff6ef7fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6ef80000 -     0x7fff6ef83ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6ef84000 -     0x7fff6ef87ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6ef88000 -     0x7fff6ef89fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6ef8a000 -     0x7fff6ef90ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6ef91000 -     0x7fff6efdaff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6efdb000 -     0x7fff6f000ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6f001000 -     0x7fff6f04cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6f04d000 -     0x7fff6f06cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6f06d000 -     0x7fff6f111ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6f112000 -     0x7fff6f11cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6f11d000 -     0x7fff6f126ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6f127000 -     0x7fff6f12eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6f12f000 -     0x7fff6f13afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6f13b000 -     0x7fff6f13eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6f13f000 -     0x7fff6f140ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6f141000 -     0x7fff6f148ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6f149000 -     0x7fff6f15cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6f15e000 -     0x7fff6f163ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6f164000 -     0x7fff6f190ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2330
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=82.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=82.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            18.4M        9 
MALLOC guard page                   16K        4 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4536K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9628K       44 
===========                     =======  ======= 
TOTAL                            285.4M      108 
TOTAL                            285.4M      108 
TOTAL, minus reserved VM space   285.3M      108 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-01-18-154918_Traviss-Mac-1044.crash
Process:               a [43260]
Path:                  /Users/USER/*/a
Identifier:            a
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [43258]
Responsible:           a [43260]
User ID:               501
Date/Time:             2019-01-18 15:49:15.812 +0000
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
0   libsystem_kernel.dylib         0x00007fff6eff6e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff6f135150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff6ef53312 abort + 127
3   libstd-d5e594e720c390d6.dylib  0x0000000106d2cd39 std::sys::unix::abort_internal::h6e91f6c84650c2c8 + 9
4   libstd-d5e594e720c390d6.dylib  0x0000000106d1ca20 rust_oom + 32
5   libstd-d5e594e720c390d6.dylib  0x0000000106d3e9e9 alloc::alloc::handle_alloc_error::h470e6c8457bb9cb9 + 9
6   a                              0x0000000106cf2c2d default_alloc_error_hook::main::hbf2d06db626d002e + 781
7   a                              0x0000000106cf2426 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h9bec864c85691aa7 + 6
8   libstd-d5e594e720c390d6.dylib  0x0000000106d1d6c8 std::panicking::try::do_call::h01d564bb08a29534 + 24
9   libstd-d5e594e720c390d6.dylib  0x0000000106d2d84f __rust_maybe_catch_panic + 31
10  libstd-d5e594e720c390d6.dylib  0x0000000106d1e1bd std::rt::lang_start_internal::hc268a56a67b71744 + 541
11  a                              0x0000000106cf2d99 main + 41
12  libdyld.dylib                  0x00007fff6eea7115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffa7bcd340  rcx: 0x00007ffee8f0cab8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee8f0caf0  rsp: 0x00007ffee8f0cab8
   r8: 0x0000000000000002   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff6eff6e3e  rfl: 0x0000000000000206  cr2: 0x00007fffa7bab148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x106cf1000 -        0x106cf3ff7 +a (0) <9CC607B3-1873-34A1-A5AB-C8E1B00F8318> /Users/USER/*/a
       0x106cfa000 -        0x106d8dfff +libstd-d5e594e720c390d6.dylib (0) <7D166126-C70F-300E-B8CC-1DE07AD86007> /Users/USER/*/libstd-d5e594e720c390d6.dylib
       0x1099e0000 -        0x109a2a98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff6c711000 -     0x7fff6c744fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff6cc23000 -     0x7fff6cc24ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff6ced9000 -     0x7fff6cf2ffff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff6cf30000 -     0x7fff6cf54ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff6e2a6000 -     0x7fff6e6973b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff6e764000 -     0x7fff6e780ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff6ed3e000 -     0x7fff6ed42ff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff6ed43000 -     0x7fff6ed4dff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff6ed4e000 -     0x7fff6ed55fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff6ed56000 -     0x7fff6ed5effb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff6ed5f000 -     0x7fff6ede4fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff6ee6c000 -     0x7fff6eea5ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff6eea6000 -     0x7fff6eec3ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff6eec4000 -     0x7fff6eec4ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff6eed2000 -     0x7fff6eed2ff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff6eed3000 -     0x7fff6eed7ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff6eed8000 -     0x7fff6eedaff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff6eedb000 -     0x7fff6eedcff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff6eedd000 -     0x7fff6eef4fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff6eef5000 -     0x7fff6eef5fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff6eef6000 -     0x7fff6ef7fff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff6ef80000 -     0x7fff6ef83ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff6ef84000 -     0x7fff6ef87ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff6ef88000 -     0x7fff6ef89fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff6ef8a000 -     0x7fff6ef90ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff6ef91000 -     0x7fff6efdaff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff6efdb000 -     0x7fff6f000ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff6f001000 -     0x7fff6f04cfcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff6f04d000 -     0x7fff6f06cfff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff6f06d000 -     0x7fff6f111ff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff6f112000 -     0x7fff6f11cffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff6f11d000 -     0x7fff6f126ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff6f127000 -     0x7fff6f12eff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff6f12f000 -     0x7fff6f13afff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff6f13b000 -     0x7fff6f13eff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff6f13f000 -     0x7fff6f140ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff6f141000 -     0x7fff6f148ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff6f149000 -     0x7fff6f15cff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff6f15e000 -     0x7fff6f163ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff6f164000 -     0x7fff6f190ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2330
    thread_create: 0
VM Region Summary:
VM Region Summary:
