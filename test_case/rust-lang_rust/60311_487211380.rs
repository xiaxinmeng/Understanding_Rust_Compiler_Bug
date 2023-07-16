plain
[00:03:52]       Memory: 8 GB
[00:03:52]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:52]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:52]       SMC Version (system): 2.8f0
[00:03:52]       Serial Number (system): VMMvW3WzlTyU
[00:03:52] 
[00:03:52] hw.ncpu: 4
[00:03:52] hw.byteorder: 1234
[00:03:52] hw.memsize: 8589934592
---
[02:13:16]   |
[02:13:16] 6 |     5.0 => {},
[02:13:16]   |     ^^^
[02:13:16]   |
[02:13:16]   = note: #[warn(illegal_floating_point_literal_pattern)] on by default
[02:13:16]   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[02:13:16] 
[02:13:16] warning: floating-point types cannot be used in patterns
[02:13:16]  --> /Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md:79:5
[02:13:16]   |
---
[02:13:16]   = note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
[02:13:16] 
[02:13:16] error: linking with `cc` failed: signal: 4
[02:13:16]   |
[02:13:16]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest0q79Am/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest0q79Am/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest0q79Am/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest0q79Am/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest0q79Am/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest0q79Am/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest0q79Am/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-f23ea46703c918bb.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-9594c37878cc9962.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-15eb3ddee796149a.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-e069d4823e178d08.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-276abfca01d24267.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-376cf09404a6e600.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-c8ecb1e61d78c34d.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-233abb06723a85b7.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-98b24fb5d296bb0b.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-43ba55b0f02ba456.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:13:16] 
[02:13:16] error: aborting due to previous error
[02:13:16] 
[02:13:16] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::illegal_floating_point_literal_pattern (line 75)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:13:16] 
[02:13:16] 
[02:13:16] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:13:16] Build completed unsuccessfully in 0:56:28
[02:13:16] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06a0ee25
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Apr 26 21:44:21 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:03a1e16d
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Apr 26 21:44 .
-rw-------@  1 travis  staff  13742 Apr 26 21:44 overflow_2019-04-26-214403_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Apr 26 21:43 foo_2019-04-26-214339_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1418 Apr 26 21:43 m4_2019-04-26-214310_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Apr 26 21:43 b_2019-04-26-214300_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1432 Apr 26 21:43 bar_2019-04-26-214300-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Apr 26 21:43 bar_2019-04-26-214300_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Apr 26 21:04 a_2019-04-26-210437-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62246 Apr 26 21:04 a_2019-04-26-210437_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Apr 26 21:04 a_2019-04-26-210425-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 Apr 26 21:04 a_2019-04-26-210425_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Apr 26 21:04 a_2019-04-26-210418_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Apr 26 21:04 a_2019-04-26-210413_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Apr 26 21:04 a_2019-04-26-210404-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Apr 26 21:04 a_2019-04-26-210404_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Apr 26 21:03 a_2019-04-26-210326_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63098 Apr 26 21:03 a_2019-04-26-210315_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Apr 26 21:03 a_2019-04-26-210313_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64211 Apr 26 21:03 a_2019-04-26-210312-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Apr 26 21:03 a_2019-04-26-210312_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11714 Apr 26 21:01 a_2019-04-26-210110_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9897 Apr 26 21:00 a_2019-04-26-210007_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Apr 26 20:58 a_2019-04-26-205848_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr 26 20:57 a_2019-04-26-205744-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr 26 20:57 a_2019-04-26-205744_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10214 Apr 26 20:57 a_2019-04-26-205741_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:089045b8
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-26-205741_Traviss-Mac-1044.crash
Process:               a [44206]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44204]
Responsible:           a [44206]
User ID:               501
Date/Time:             2019-04-26 20:57:11.534 +0000
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
0   a                              0x0000000100d448ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x0000000100d43ca9 std::panicking::try::do_call::h5c81075da2dcccf9 (.llvm.12675958016211414747) + 9
2   libstd-f23ea46703c918bb.dylib  0x0000000100d80a3f __rust_maybe_catch_panic + 31
3   a                              0x0000000100d44b01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x0000000100d430f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf20099c8e9367143 + 6
5   libstd-f23ea46703c918bb.dylib  0x0000000100d70dc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
6   libstd-f23ea46703c918bb.dylib  0x0000000100d80a3f __rust_maybe_catch_panic + 31
7   libstd-f23ea46703c918bb.dylib  0x0000000100d718ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
8   a                              0x0000000100d44e09 main + 41
9   libdyld.dylib                  0x00007fff77ce4115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ff915c02be0  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffeeeebaad8  rsi: 0x000000007fffffff  rbp: 0x00007ffeeeebb530  rsp: 0x00007ffeeeebb530
   r8: 0x00000000915c02c3   r9: 0x0000000000000004  r10: 0x000000010bf408c0  r11: 0x00007fff77f9b96c
  r12: 0x000000010109a000  r13: 0x0000000000000000  r14: 0x00007ffeeeebb650  r15: 0x00007ffeeeebb598
  rip: 0x0000000100d448ae  rfl: 0x0000000000010202  cr2: 0x0000000100db7a28
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x100d42000 -        0x100d45fff +a (0) <B3EF0B91-4C5A-3DA3-899B-20741C0D8C51> /Users/USER/*/a
       0x100d4c000 -        0x100de3ff7 +libstd-f23ea46703c918bb.dylib (0) <DF994744-4750-38DA-8C37-CB53A328E813> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x10beee000 -        0x10bf3898f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7554e000 -     0x7fff75581fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff75a60000 -     0x7fff75a61ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff75d16000 -     0x7fff75d6cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff75d6d000 -     0x7fff75d91ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff770e3000 -     0x7fff774d43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff775a1000 -     0x7fff775bdffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff77b7b000 -     0x7fff77b7fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff77b80000 -     0x7fff77b8aff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff77b8b000 -     0x7fff77b92fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff77b93000 -     0x7fff77b9bffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff77b9c000 -     0x7fff77c21fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff77ca9000 -     0x7fff77ce2ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff77ce3000 -     0x7fff77d00ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff77d01000 -     0x7fff77d01ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff77d0f000 -     0x7fff77d0fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff77d10000 -     0x7fff77d14ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff77d15000 -     0x7fff77d17ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff77d18000 -     0x7fff77d19ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff77d1a000 -     0x7fff77d31fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff77d32000 -     0x7fff77d32fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff77d33000 -     0x7fff77dbcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff77dbd000 -     0x7fff77dc0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff77dc1000 -     0x7fff77dc4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff77dc5000 -     0x7fff77dc6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff77dc7000 -     0x7fff77dcdff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff77dce000 -     0x7fff77e17ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff77e18000 -     0x7fff77e3dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff77e3e000 -     0x7fff77e89fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff77e8a000 -     0x7fff77ea9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff77eaa000 -     0x7fff77f4eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff77f4f000 -     0x7fff77f59ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff77f5a000 -     0x7fff77f63ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff77f64000 -     0x7fff77f6bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff77f6c000 -     0x7fff77f77fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff77f78000 -     0x7fff77f7bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff77f7c000 -     0x7fff77f7dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff77f7e000 -     0x7fff77f85ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff77f86000 -     0x7fff77f99ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff77f9b000 -     0x7fff77fa0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff77fa1000 -     0x7fff77fcdff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2364
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
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            276.1M      109 
TOTAL                            276.1M      109 
TOTAL, minus reserved VM space   276.0M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-26-205744-1_Traviss-Mac-1044.crash
Process:               a [45022]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45017]
Responsible:           a [45022]
User ID:               501
Date/Time:             2019-04-26 20:57:43.661 +0000
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
0   libstd-f23ea46703c918bb.dylib  0x0000000100fe934e std::panicking::rust_panic_with_hook::h59ed2c15bfb73246 + 142
1   a                              0x0000000100fb88e5 std::panicking::begin_panic::h55d7cb6258d03287 + 37
2   a                              0x0000000100fb640c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000100fb5969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x0000000100fb63e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000100fb7559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x0000000100fb5936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb4bf0af51319d429 + 6 (rt.rs:64)
7   libstd-f23ea46703c918bb.dylib  0x0000000100fe8dc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
8   libstd-f23ea46703c918bb.dylib  0x0000000100ff8a3f __rust_maybe_catch_panic + 31
9   libstd-f23ea46703c918bb.dylib  0x0000000100fe98ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
10  a                              0x0000000100fb7d99 main + 41
11  libdyld.dylib                  0x00007fff77ce4115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeeec4b278  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000101030ed2  rbp: 0x00007ffeeec4b370  rsp: 0x00007ffeeec4b2a0
   r8: 0xffffffff00000100   r9: 0x00000001010639f0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x000000010102fc38  r14: 0x0000000100fba460  r15: 0x00007ffeeec4b380
  rip: 0x0000000100fe934e  rfl: 0x0000000000010206  cr2: 0x00000001122d2170
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x100fb2000 -        0x100fb9fff +a (0) <2193879B-D85C-3F79-B7C8-D52628682CFD> /Users/USER/*/a
       0x100fc4000 -        0x10105bff7 +libstd-f23ea46703c918bb.dylib (0) <DF994744-4750-38DA-8C37-CB53A328E813> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x10f5a8000 -        0x10f5f298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7554e000 -     0x7fff75581fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff75a60000 -     0x7fff75a61ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff75d16000 -     0x7fff75d6cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff75d6d000 -     0x7fff75d91ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff770e3000 -     0x7fff774d43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff775a1000 -     0x7fff775bdffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff77b7b000 -     0x7fff77b7fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff77b80000 -     0x7fff77b8aff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff77b8b000 -     0x7fff77b92fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff77b93000 -     0x7fff77b9bffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff77b9c000 -     0x7fff77c21fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff77ca9000 -     0x7fff77ce2ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff77ce3000 -     0x7fff77d00ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff77d01000 -     0x7fff77d01ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff77d0f000 -     0x7fff77d0fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff77d10000 -     0x7fff77d14ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff77d15000 -     0x7fff77d17ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff77d18000 -     0x7fff77d19ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff77d1a000 -     0x7fff77d31fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff77d32000 -     0x7fff77d32fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff77d33000 -     0x7fff77dbcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff77dbd000 -     0x7fff77dc0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff77dc1000 -     0x7fff77dc4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff77dc5000 -     0x7fff77dc6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff77dc7000 -     0x7fff77dcdff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff77dce000 -     0x7fff77e17ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff77e18000 -     0x7fff77e3dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff77e3e000 -     0x7fff77e89fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff77e8a000 -     0x7fff77ea9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff77eaa000 -     0x7fff77f4eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff77f4f000 -     0x7fff77f59ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff77f5a000 -     0x7fff77f63ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff77f64000 -     0x7fff77f6bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff77f6c000 -     0x7fff77f77fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff77f78000 -     0x7fff77f7bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff77f7c000 -     0x7fff77f7dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff77f7e000 -     0x7fff77f85ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff77f86000 -     0x7fff77f99ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff77f9b000 -     0x7fff77fa0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff77fa1000 -     0x7fff77fcdff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2364
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=74.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            10.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            277.5M      110 
TOTAL                            277.5M      110 
TOTAL, minus reserved VM space   277.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-26-205744_Traviss-Mac-1044.crash
Process:               a [45021]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45017]
Responsible:           a [45021]
User ID:               501
Date/Time:             2019-04-26 20:57:43.649 +0000
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
0   libstd-f23ea46703c918bb.dylib  0x000000010630d34e std::panicking::rust_panic_with_hook::h59ed2c15bfb73246 + 142
1   a                              0x00000001062db8e5 std::panicking::begin_panic::h55d7cb6258d03287 + 37
2   a                              0x00000001062d940c _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x00000001062d8969 core::ptr::real_drop_in_place::h08174337b74568a1 + 9
4   a                              0x00000001062d93e3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x00000001062da559 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x00000001062d8936 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hb4bf0af51319d429 + 6 (rt.rs:64)
7   libstd-f23ea46703c918bb.dylib  0x000000010630cdc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
8   libstd-f23ea46703c918bb.dylib  0x000000010631ca3f __rust_maybe_catch_panic + 31
9   libstd-f23ea46703c918bb.dylib  0x000000010630d8ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
10  a                              0x00000001062dad99 main + 41
11  libdyld.dylib                  0x00007fff77ce4115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee9928298  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000106354ed2  rbp: 0x00007ffee9928390  rsp: 0x00007ffee99282c0
   r8: 0xffffffff00000100   r9: 0x00000001063879f0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000106353c38  r14: 0x00000001062dd460  r15: 0x00007ffee99283a0
  rip: 0x000000010630d34e  rfl: 0x0000000000010202  cr2: 0x00000001065f5434
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1062d5000 -        0x1062dcfff +a (0) <2193879B-D85C-3F79-B7C8-D52628682CFD> /Users/USER/*/a
       0x1062e8000 -        0x10637fff7 +libstd-f23ea46703c918bb.dylib (0) <DF994744-4750-38DA-8C37-CB53A328E813> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x10be7f000 -        0x10bec998f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7554e000 -     0x7fff75581fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff75a60000 -     0x7fff75a61ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff75d16000 -     0x7fff75d6cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff75d6d000 -     0x7fff75d91ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff770e3000 -     0x7fff774d43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff775a1000 -     0x7fff775bdffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff77b7b000 -     0x7fff77b7fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff77b80000 -     0x7fff77b8aff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff77b8b000 -     0x7fff77b92fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff77b93000 -     0x7fff77b9bffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff77b9c000 -     0x7fff77c21fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff77ca9000 -     0x7fff77ce2ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff77ce3000 -     0x7fff77d00ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff77d01000 -     0x7fff77d01ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff77d0f000 -     0x7fff77d0fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff77d10000 -     0x7fff77d14ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff77d15000 -     0x7fff77d17ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff77d18000 -     0x7fff77d19ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff77d1a000 -     0x7fff77d31fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff77d32000 -     0x7fff77d32fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff77d33000 -     0x7fff77dbcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff77dbd000 -     0x7fff77dc0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff77dc1000 -     0x7fff77dc4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff77dc5000 -     0x7fff77dc6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff77dc7000 -     0x7fff77dcdff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff77dce000 -     0x7fff77e17ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff77e18000 -     0x7fff77e3dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff77e3e000 -     0x7fff77e89fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff77e8a000 -     0x7fff77ea9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff77eaa000 -     0x7fff77f4eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff77f4f000 -     0x7fff77f59ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff77f5a000 -     0x7fff77f63ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff77f64000 -     0x7fff77f6bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff77f6c000 -     0x7fff77f77fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff77f78000 -     0x7fff77f7bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff77f7c000 -     0x7fff77f7dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff77f7e000 -     0x7fff77f85ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff77f86000 -     0x7fff77f99ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff77f9b000 -     0x7fff77fa0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff77fa1000 -     0x7fff77fcdff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2364
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=91.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=91.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            27.5M        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4636K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            294.5M      110 
TOTAL                            294.5M      110 
TOTAL, minus reserved VM space   294.4M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-26-205848_Traviss-Mac-1044.crash
Process:               a [46698]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46697]
Responsible:           a [46698]
User ID:               501
Date/Time:             2019-04-26 20:58:47.014 +0000
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
0   libsystem_kernel.dylib         0x00007fff77e33e3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff77f72150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff77d90312 abort + 127
3   libstd-f23ea46703c918bb.dylib  0x000000010590af29 std::sys::unix::abort_internal::hfa6963a1e064fb02 + 9
4   libstd-f23ea46703c918bb.dylib  0x00000001058fb3e0 rust_oom + 32
5   libstd-f23ea46703c918bb.dylib  0x000000010591c1e9 alloc::alloc::handle_alloc_error::h85e2cdc83e93f20e + 9
6   a                              0x00000001058d040f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x00000001058cf686 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hdfc862f94f5529c1 + 6
8   libstd-f23ea46703c918bb.dylib  0x00000001058fbdc8 std::panicking::try::do_call::h5758c52bb7fcd96d + 24
9   libstd-f23ea46703c918bb.dylib  0x000000010590ba3f __rust_maybe_catch_panic + 31
10  libstd-f23ea46703c918bb.dylib  0x00000001058fc8ae std::rt::lang_start_internal::ha0d5ff149ddc1e03 + 542
11  a                              0x00000001058d0569 main + 41
12  libdyld.dylib                  0x00007fff77ce4115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fffb0a0a340  rcx: 0x00007ffeea32f498  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffeea32f4d0  rsp: 0x00007ffeea32f498
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff77e33e3e  rfl: 0x0000000000000206  cr2: 0x00007fffb09e8148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x1058ce000 -        0x1058d0ff7 +a (0) <45E49D60-88E2-3840-ABD7-591175213979> /Users/USER/*/a
       0x1058d7000 -        0x10596eff7 +libstd-f23ea46703c918bb.dylib (0) <DF994744-4750-38DA-8C37-CB53A328E813> /Users/USER/*/libstd-f23ea46703c918bb.dylib
       0x1105c1000 -        0x11060b98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff7554e000 -     0x7fff75581fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff75a60000 -     0x7fff75a61ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff75d16000 -     0x7fff75d6cfff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff75d6d000 -     0x7fff75d91ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff770e3000 -     0x7fff774d43b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff775a1000 -     0x7fff775bdffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff77b7b000 -     0x7fff77b7fff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff77b80000 -     0x7fff77b8aff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff77b8b000 -     0x7fff77b92fff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff77b93000 -     0x7fff77b9bffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff77b9c000 -     0x7fff77c21fff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff77ca9000 -     0x7fff77ce2ff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff77ce3000 -     0x7fff77d00ff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff77d01000 -     0x7fff77d01ffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff77d0f000 -     0x7fff77d0fff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff77d10000 -     0x7fff77d14ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff77d15000 -     0x7fff77d17ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff77d18000 -     0x7fff77d19ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff77d1a000 -     0x7fff77d31fff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff77d32000 -     0x7fff77d32fff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff77d33000 -     0x7fff77dbcff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff77dbd000 -     0x7fff77dc0ffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff77dc1000 -     0x7fff77dc4ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff77dc5000 -     0x7fff77dc6fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff77dc7000 -     0x7fff77dcdff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff77dce000 -     0x7fff77e17ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff77e18000 -     0x7fff77e3dff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff77e3e000 -     0x7fff77e89fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff77e8a000 -     0x7fff77ea9fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff77eaa000 -     0x7fff77f4eff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff77f4f000 -     0x7fff77f59ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff77f5a000 -     0x7fff77f63ff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff77f64000 -     0x7fff77f6bff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff77f6c000 -     0x7fff77f77fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff77f78000 -     0x7fff77f7bff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff77f7c000 -     0x7fff77f7dff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff77f7e000 -     0x7fff77f85ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff77f86000 -     0x7fff77f99ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff77f9b000 -     0x7fff77fa0ff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff77fa1000 -     0x7fff77fcdff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
