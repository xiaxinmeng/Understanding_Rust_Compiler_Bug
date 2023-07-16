plain
[00:03:03]       Memory: 8 GB
[00:03:03]       Boot ROM Version: VMW71.00V.7581552.B64.1801142334
[00:03:03]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:03]       SMC Version (system): 2.8f0
[00:03:03]       Serial Number (system): VMXvGxd07JUL
[00:03:03] 
[00:03:03] hw.ncpu: 4
[00:03:03] hw.byteorder: 1234
[00:03:03] hw.memsize: 8589934592
---
[02:07:37] 
[02:07:37] failures:
[02:07:37] 
[02:07:37] ---- /Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::improper_ctypes (line 103) stdout ----
[02:07:37] warning: `extern` block uses type `std::string::String` which is not FFI-safe: this struct has unspecified layout
[02:07:37]   |
[02:07:37] 4 |     static STATIC: String;
[02:07:37]   |                    ^^^^^^
[02:07:37]   |
[02:07:37]   |
[02:07:37]   = note: #[warn(improper_ctypes)] on by default
[02:07:37]   = help: consider adding a #[repr(C)] or #[repr(transparent)] attribute to this struct
[02:07:37] error: linking with `cc` failed: signal: 4
[02:07:37]   |
[02:07:37]   |
[02:07:37]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEdqoAE/rust_out.rust_out.7rcbfp3g-cgu.0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEdqoAE/rust_out.rust_out.7rcbfp3g-cgu.1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEdqoAE/rust_out.rust_out.7rcbfp3g-cgu.2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEdqoAE/rust_out.rust_out.7rcbfp3g-cgu.3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEdqoAE/rust_out.rust_out.7rcbfp3g-cgu.4.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEdqoAE/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctestEdqoAE/rust_out.33dyzt1ekirinwy8.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libstd-9aaf2838fb9495b0.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-ae4d3d70a8daa601.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libbacktrace_sys-bf226fb3bae1b765.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_demangle-0d6ce4945231c787.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libhashbrown-3b39a24aa6c91310.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_alloc-4843d16359df8eb6.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libunwind-886904f3b7bd1b34.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liblibc-d2df2b3768a0bc28.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/liballoc-19702aa80fed9a60.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_std_workspace_core-6538ea0bab19d548.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcore-915d7fe958588090.rlib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-ecd0b9a23795e53a.rlib" "-lSystem" "-lresolv" "-lc" "-lm"
[02:07:37] 
[02:07:37] error: aborting due to previous error
[02:07:37] 
[02:07:37] thread '/Users/travis/build/rust-lang/rust/src/doc/rustc/src/lints/listing/warn-by-default.md - Warn_by_default_lints::improper_ctypes (line 103)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
---
[02:07:37] 
[02:07:37] 
[02:07:37] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap test
[02:07:37] Build completed unsuccessfully in 0:51:38
[02:07:37] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0368c14b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Apr 28 20:41:37 GMT 2019
---
travis_fold:start:after_failure.2
travis_time:start:0229d739
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 1272
drwx------  27 travis  staff    918 Apr 28 20:41 .
-rw-------@  1 travis  staff  13742 Apr 28 20:41 overflow_2019-04-28-204118_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1428 Apr 28 20:40 foo_2019-04-28-204055_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1417 Apr 28 20:40 m4_2019-04-28-204026_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10661 Apr 28 20:40 b_2019-04-28-204015_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1431 Apr 28 20:40 bar_2019-04-28-204016-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   1444 Apr 28 20:40 bar_2019-04-28-204016_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37663 Apr 28 20:06 a_2019-04-28-200632-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  62244 Apr 28 20:06 a_2019-04-28-200632_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  60389 Apr 28 20:06 a_2019-04-28-200623-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  37411 Apr 28 20:06 a_2019-04-28-200623_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10142 Apr 28 20:06 a_2019-04-28-200618_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9873 Apr 28 20:06 a_2019-04-28-200612_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9878 Apr 28 20:06 a_2019-04-28-200604_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9794 Apr 28 20:06 a_2019-04-28-200603_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10034 Apr 28 20:05 a_2019-04-28-200526_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63060 Apr 28 20:05 a_2019-04-28-200516_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  65082 Apr 28 20:05 a_2019-04-28-200513-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  63915 Apr 28 20:05 a_2019-04-28-200513-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  64224 Apr 28 20:05 a_2019-04-28-200513_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  11712 Apr 28 20:03 a_2019-04-28-200300_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff   9895 Apr 28 20:02 a_2019-04-28-200201_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10304 Apr 28 20:00 a_2019-04-28-200047_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr 28 19:59 a_2019-04-28-195946-1_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10491 Apr 28 19:59 a_2019-04-28-195946-2_Traviss-Mac-1044.crash
-rw-------@  1 travis  staff  10212 Apr 28 19:59 a_2019-04-28-195946_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:058eef3a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-28-195946-1_Traviss-Mac-1044.crash
Process:               a [45062]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45049]
Responsible:           a [45062]
User ID:               501
Date/Time:             2019-04-28 19:59:43.354 +0000
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
0   libstd-9aaf2838fb9495b0.dylib  0x0000000107a03a2e std::panicking::rust_panic_with_hook::hc3ab2d58f364c45d + 142
1   a                              0x00000001079d48c5 std::panicking::begin_panic::h93e278867f418319 + 37
2   a                              0x00000001079d23ec _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x00000001079d1ae9 core::ptr::real_drop_in_place::h654ccdbe565444dc + 9
4   a                              0x00000001079d23c3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x00000001079d3539 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x00000001079d18f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hba07b6f8f678dd4f + 6 (rt.rs:64)
7   libstd-9aaf2838fb9495b0.dylib  0x0000000107a034a8 std::panicking::try::do_call::ha1eafece19191e13 + 24
8   libstd-9aaf2838fb9495b0.dylib  0x0000000107a135bf __rust_maybe_catch_panic + 31
9   libstd-9aaf2838fb9495b0.dylib  0x0000000107a03f8e std::rt::lang_start_internal::hfe68f0f2a01a7222 + 542
10  a                              0x00000001079d3d79 main + 41
11  libdyld.dylib                  0x00007fff53ec0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffee822f388  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x0000000107a4bb12  rbp: 0x00007ffee822f480  rsp: 0x00007ffee822f3b0
   r8: 0xffffffff00000100   r9: 0x0000000107a7e9d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x0000000107a4a878  r14: 0x00000001079d6460  r15: 0x00007ffee822f490
  rip: 0x0000000107a03a2e  rfl: 0x0000000000010206  cr2: 0x000000010aae7084
Logical CPU:     3
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x1079ce000 -        0x1079d5ff7 +a (0) <EEFFEF48-B3E6-3681-9F55-6D1EB341DC59> /Users/USER/*/a
       0x1079df000 -        0x107a76ff7 +libstd-9aaf2838fb9495b0.dylib (0) <640D8139-7E9C-3ACA-93E7-96C301FAEFF3> /Users/USER/*/libstd-9aaf2838fb9495b0.dylib
       0x112393000 -        0x1123dd98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5172a000 -     0x7fff5175dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff51c3c000 -     0x7fff51c3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff51ef2000 -     0x7fff51f48fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff51f49000 -     0x7fff51f6dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff532bf000 -     0x7fff536b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5377d000 -     0x7fff53799ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff53d57000 -     0x7fff53d5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff53d5c000 -     0x7fff53d66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff53d67000 -     0x7fff53d6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff53d6f000 -     0x7fff53d77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff53d78000 -     0x7fff53dfdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff53e85000 -     0x7fff53ebeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff53ebf000 -     0x7fff53edcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff53edd000 -     0x7fff53eddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff53eeb000 -     0x7fff53eebff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff53eec000 -     0x7fff53ef0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff53ef1000 -     0x7fff53ef3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff53ef4000 -     0x7fff53ef5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff53ef6000 -     0x7fff53f0dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff53f0e000 -     0x7fff53f0efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff53f0f000 -     0x7fff53f98ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff53f99000 -     0x7fff53f9cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff53f9d000 -     0x7fff53fa0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff53fa1000 -     0x7fff53fa2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff53fa3000 -     0x7fff53fa9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff53faa000 -     0x7fff53ff3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff53ff4000 -     0x7fff54019ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5401a000 -     0x7fff54065fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff54066000 -     0x7fff54085fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff54086000 -     0x7fff5412aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5412b000 -     0x7fff54135ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff54136000 -     0x7fff5413fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff54140000 -     0x7fff54147ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff54148000 -     0x7fff54153fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff54154000 -     0x7fff54157ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff54158000 -     0x7fff54159ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5415a000 -     0x7fff54161ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff54162000 -     0x7fff54175ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff54177000 -     0x7fff5417cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5417d000 -     0x7fff541a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2368
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=73.8M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=73.8M(100%)
                                VIRTUAL   REGION 
REGION TYPE                        SIZE    COUNT (non-coalesced) 
===========                     =======  ======= 
Kernel Alloc Once                    8K        2 
MALLOC                            9696K        9 
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4540K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            276.4M      110 
TOTAL                            276.4M      110 
TOTAL, minus reserved VM space   276.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-28-195946-2_Traviss-Mac-1044.crash
Process:               a [45060]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [45049]
Responsible:           a [45060]
User ID:               501
Date/Time:             2019-04-28 19:59:43.339 +0000
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
0   libstd-9aaf2838fb9495b0.dylib  0x0000000103377a2e std::panicking::rust_panic_with_hook::hc3ab2d58f364c45d + 142
1   a                              0x00000001033458c5 std::panicking::begin_panic::h93e278867f418319 + 37
2   a                              0x00000001033433ec _$LT$backtrace..double..Double$u20$as$u20$core..ops..drop..Drop$GT$::drop::hcc2b5a39c3723dfb + 28
3   a                              0x0000000103342ae9 core::ptr::real_drop_in_place::h654ccdbe565444dc + 9
4   a                              0x00000001033433c3 backtrace::double::h0c99cc05786c6af0 + 35
5   a                              0x0000000103344539 backtrace::main::hcde7a1a1c3c85e77 + 4201 (backtrace.rs:103)
6   a                              0x00000001033428f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hba07b6f8f678dd4f + 6 (rt.rs:64)
7   libstd-9aaf2838fb9495b0.dylib  0x00000001033774a8 std::panicking::try::do_call::ha1eafece19191e13 + 24
8   libstd-9aaf2838fb9495b0.dylib  0x00000001033875bf __rust_maybe_catch_panic + 31
9   libstd-9aaf2838fb9495b0.dylib  0x0000000103377f8e std::rt::lang_start_internal::hfe68f0f2a01a7222 + 542
10  a                              0x0000000103344d79 main + 41
11  libdyld.dylib                  0x00007fff53ec0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007ffeec8be3a8  rbx: 0x0000000000000002  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000002  rsi: 0x00000001033bfb12  rbp: 0x00007ffeec8be4a0  rsp: 0x00007ffeec8be3d0
   r8: 0xffffffff00000100   r9: 0x00000001033f29d0  r10: 0x000000000000002b  r11: 0x0000000000000296
  r12: 0x0000000000000000  r13: 0x00000001033be878  r14: 0x0000000103347460  r15: 0x00007ffeec8be4b0
  rip: 0x0000000103377a2e  rfl: 0x0000000000010206  cr2: 0x00000001036489bc
Logical CPU:     0
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x10333f000 -        0x103346ff7 +a (0) <EEFFEF48-B3E6-3681-9F55-6D1EB341DC59> /Users/USER/*/a
       0x103353000 -        0x1033eaff7 +libstd-9aaf2838fb9495b0.dylib (0) <640D8139-7E9C-3ACA-93E7-96C301FAEFF3> /Users/USER/*/libstd-9aaf2838fb9495b0.dylib
       0x10508a000 -        0x1050d498f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5172a000 -     0x7fff5175dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff51c3c000 -     0x7fff51c3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff51ef2000 -     0x7fff51f48fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff51f49000 -     0x7fff51f6dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff532bf000 -     0x7fff536b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5377d000 -     0x7fff53799ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff53d57000 -     0x7fff53d5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff53d5c000 -     0x7fff53d66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff53d67000 -     0x7fff53d6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff53d6f000 -     0x7fff53d77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff53d78000 -     0x7fff53dfdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff53e85000 -     0x7fff53ebeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff53ebf000 -     0x7fff53edcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff53edd000 -     0x7fff53eddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff53eeb000 -     0x7fff53eebff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff53eec000 -     0x7fff53ef0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff53ef1000 -     0x7fff53ef3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff53ef4000 -     0x7fff53ef5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff53ef6000 -     0x7fff53f0dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff53f0e000 -     0x7fff53f0efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff53f0f000 -     0x7fff53f98ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff53f99000 -     0x7fff53f9cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff53f9d000 -     0x7fff53fa0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff53fa1000 -     0x7fff53fa2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff53fa3000 -     0x7fff53fa9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff53faa000 -     0x7fff53ff3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff53ff4000 -     0x7fff54019ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5401a000 -     0x7fff54065fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff54066000 -     0x7fff54085fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff54086000 -     0x7fff5412aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5412b000 -     0x7fff54135ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff54136000 -     0x7fff5413fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff54140000 -     0x7fff54147ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff54148000 -     0x7fff54153fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff54154000 -     0x7fff54157ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff54158000 -     0x7fff54159ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5415a000 -     0x7fff54161ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff54162000 -     0x7fff54175ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff54177000 -     0x7fff5417cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5417d000 -     0x7fff541a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2368
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
MALLOC guard page                   16K        5 
Stack Guard                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE                          4K        2 
VM_ALLOCATE (reserved)             128K        2         reserved VM address space (unallocated)
__DATA                            4540K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9640K       44 
===========                     =======  ======= 
TOTAL                            286.4M      110 
TOTAL                            286.4M      110 
TOTAL, minus reserved VM space   286.3M      110 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-28-195946_Traviss-Mac-1044.crash
Process:               a [44266]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [44264]
Responsible:           a [44266]
User ID:               501
Date/Time:             2019-04-28 19:59:15.850 +0000
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
0   a                              0x000000010963b8ae abort_on_c_abi::panic_in_ffi::h5d17554117e8ddd6 + 30
1   a                              0x000000010963aca9 std::panicking::try::do_call::hb7b98e87f296a8b7 (.llvm.783244980944937515) + 9
2   libstd-9aaf2838fb9495b0.dylib  0x00000001096775bf __rust_maybe_catch_panic + 31
3   a                              0x000000010963bb01 abort_on_c_abi::main::ha239c5d4a2ab8e27 + 593
4   a                              0x000000010963a0f6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h90821eb8583e12d6 + 6
5   libstd-9aaf2838fb9495b0.dylib  0x00000001096674a8 std::panicking::try::do_call::ha1eafece19191e13 + 24
6   libstd-9aaf2838fb9495b0.dylib  0x00000001096775bf __rust_maybe_catch_panic + 31
7   libstd-9aaf2838fb9495b0.dylib  0x0000000109667f8e std::rt::lang_start_internal::hfe68f0f2a01a7222 + 542
8   a                              0x000000010963be09 main + 41
9   libdyld.dylib                  0x00007fff53ec0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00007fe556500010  rbx: 0x0000000000000000  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x00007ffee65c3be8  rsi: 0x00000000ffffffc3  rbp: 0x00007ffee65c4640  rsp: 0x00007ffee65c4640
   r8: 0x0000000055650006   r9: 0x0000000000000004  r10: 0x0000000116c858c0  r11: 0x00007fff5417796c
  r12: 0x000000010997a000  r13: 0x0000000000000000  r14: 0x00007ffee65c4760  r15: 0x00007ffee65c46a8
  rip: 0x000000010963b8ae  rfl: 0x0000000000010206  cr2: 0x00000001096ae66c
Logical CPU:     2
Error Code:      0x00000000
Trap Number:     6
Binary Images:
       0x109639000 -        0x10963cfff +a (0) <E77E0678-B2C1-356A-A419-A32B9142374F> /Users/USER/*/a
       0x109643000 -        0x1096daff7 +libstd-9aaf2838fb9495b0.dylib (0) <640D8139-7E9C-3ACA-93E7-96C301FAEFF3> /Users/USER/*/libstd-9aaf2838fb9495b0.dylib
       0x116c33000 -        0x116c7d98f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5172a000 -     0x7fff5175dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff51c3c000 -     0x7fff51c3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff51ef2000 -     0x7fff51f48fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff51f49000 -     0x7fff51f6dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff532bf000 -     0x7fff536b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5377d000 -     0x7fff53799ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff53d57000 -     0x7fff53d5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff53d5c000 -     0x7fff53d66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff53d67000 -     0x7fff53d6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff53d6f000 -     0x7fff53d77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff53d78000 -     0x7fff53dfdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff53e85000 -     0x7fff53ebeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff53ebf000 -     0x7fff53edcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff53edd000 -     0x7fff53eddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff53eeb000 -     0x7fff53eebff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff53eec000 -     0x7fff53ef0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff53ef1000 -     0x7fff53ef3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff53ef4000 -     0x7fff53ef5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff53ef6000 -     0x7fff53f0dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff53f0e000 -     0x7fff53f0efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff53f0f000 -     0x7fff53f98ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff53f99000 -     0x7fff53f9cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff53f9d000 -     0x7fff53fa0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff53fa1000 -     0x7fff53fa2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff53fa3000 -     0x7fff53fa9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff53faa000 -     0x7fff53ff3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff53ff4000 -     0x7fff54019ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5401a000 -     0x7fff54065fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff54066000 -     0x7fff54085fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff54086000 -     0x7fff5412aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5412b000 -     0x7fff54135ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff54136000 -     0x7fff5413fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff54140000 -     0x7fff54147ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff54148000 -     0x7fff54153fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff54154000 -     0x7fff54157ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff54158000 -     0x7fff54159ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5415a000 -     0x7fff54161ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff54162000 -     0x7fff54175ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff54177000 -     0x7fff5417cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5417d000 -     0x7fff541a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
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
    task_for_pid: 2368
    thread_create: 0
VM Region Summary:
VM Region Summary:
ReadOnly portion of Libraries: Total=198.4M resident=0K(0%) swapped_out_or_unallocated=198.4M(100%)
Writable regions: Total=74.4M written=0K(0%) resident=0K(0%) swapped_out=0K(0%) unallocated=74.4M(100%)
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
__DATA                            4540K       44 
__LINKEDIT                       189.0M        5 
__TEXT                            9624K       44 
===========                     =======  ======= 
TOTAL                            277.0M      109 
TOTAL                            277.0M      109 
TOTAL, minus reserved VM space   276.9M      109 
travis_fold:start:crashlog
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/a_2019-04-28-200047_Traviss-Mac-1044.crash
Process:               a [46751]
Path:                  /Users/USER/*/a
Version:               0
Version:               0
Code Type:             X86-64 (Native)
Parent Process:        ??? [46749]
Responsible:           a [46751]
User ID:               501
Date/Time:             2019-04-28 20:00:45.998 +0000
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
0   libsystem_kernel.dylib         0x00007fff5400fe3e __pthread_kill + 10
1   libsystem_pthread.dylib        0x00007fff5414e150 pthread_kill + 333
2   libsystem_c.dylib              0x00007fff53f6c312 abort + 127
3   libstd-9aaf2838fb9495b0.dylib  0x000000010c2d3aa9 std::sys::unix::abort_internal::hc559c3068df06db1 + 9
4   libstd-9aaf2838fb9495b0.dylib  0x000000010c2c3ac0 rust_oom + 32
5   libstd-9aaf2838fb9495b0.dylib  0x000000010c2e4db9 alloc::alloc::handle_alloc_error::h1d1b1e63977f437a + 9
6   a                              0x000000010c29607f default_alloc_error_hook::main::hbf2d06db626d002e + 767
7   a                              0x000000010c2961e6 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::hf5c5186fb8c843bc + 6
8   libstd-9aaf2838fb9495b0.dylib  0x000000010c2c44a8 std::panicking::try::do_call::ha1eafece19191e13 + 24
9   libstd-9aaf2838fb9495b0.dylib  0x000000010c2d45bf __rust_maybe_catch_panic + 31
10  libstd-9aaf2838fb9495b0.dylib  0x000000010c2c4f8e std::rt::lang_start_internal::hfe68f0f2a01a7222 + 542
11  a                              0x000000010c2961d9 main + 41
12  libdyld.dylib                  0x00007fff53ec0115 start + 1
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x0000000000000000  rbx: 0x00007fff8cbe6340  rcx: 0x00007ffee39695a8  rdx: 0x0000000000000000
  rdi: 0x0000000000000307  rsi: 0x0000000000000006  rbp: 0x00007ffee39695e0  rsp: 0x00007ffee39695a8
   r8: 0x0000000000000000   r9: 0x0000000000000002  r10: 0x0000000000000000  r11: 0x0000000000000206
  r12: 0x0000000000000307  r13: 0x0000000000000000  r14: 0x0000000000000006  r15: 0x000000000000002d
  rip: 0x00007fff5400fe3e  rfl: 0x0000000000000206  cr2: 0x00007fff8cbc4148
Logical CPU:     0
Error Code:      0x02000148
Trap Number:     133
Binary Images:
       0x10c294000 -        0x10c296fff +a (0) <8BCF8071-1CC7-33EA-AF37-0A9EF39AF18C> /Users/USER/*/a
       0x10c2a0000 -        0x10c337ff7 +libstd-9aaf2838fb9495b0.dylib (0) <640D8139-7E9C-3ACA-93E7-96C301FAEFF3> /Users/USER/*/libstd-9aaf2838fb9495b0.dylib
       0x11ad48000 -        0x11ad9298f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff5172a000 -     0x7fff5175dfff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff51c3c000 -     0x7fff51c3dff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff51ef2000 -     0x7fff51f48fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff51f49000 -     0x7fff51f6dff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff532bf000 -     0x7fff536b03b7  libobjc.A.dylib (723) <37A7D77E-952C-3F5D-970B-3CDE349B2322> /usr/lib/libobjc.A.dylib
    0x7fff5377d000 -     0x7fff53799ffb  libresolv.9.dylib (65) <E8F3415B-4472-3202-8901-41FD87981DB2> /usr/lib/libresolv.9.dylib
    0x7fff53d57000 -     0x7fff53d5bff7  libcache.dylib (80) <354F3B7D-404E-3398-9EBF-65CA2CE65211> /usr/lib/system/libcache.dylib
    0x7fff53d5c000 -     0x7fff53d66ff3  libcommonCrypto.dylib (60118.30.2) <674286D3-7744-36A3-9AAA-49DFCD97A986> /usr/lib/system/libcommonCrypto.dylib
    0x7fff53d67000 -     0x7fff53d6efff  libcompiler_rt.dylib (62) <4487CFBA-A5D7-3282-9E6B-94CAD7BE507E> /usr/lib/system/libcompiler_rt.dylib
    0x7fff53d6f000 -     0x7fff53d77ffb  libcopyfile.dylib (146.30.2) <2C7C67D7-562B-3FFA-973D-BACF4C10E1EC> /usr/lib/system/libcopyfile.dylib
    0x7fff53d78000 -     0x7fff53dfdfff  libcorecrypto.dylib (562.30.10) <8A53EFE1-AFCA-3676-BEE1-FA5ED9F0E222> /usr/lib/system/libcorecrypto.dylib
    0x7fff53e85000 -     0x7fff53ebeff7  libdispatch.dylib (913.30.4) <7D0E3183-282B-3FEE-A734-2C0ADC092084> /usr/lib/system/libdispatch.dylib
    0x7fff53ebf000 -     0x7fff53edcff7  libdyld.dylib (519.2.2) <C50D02BC-A333-3313-B787-02F255A6783F> /usr/lib/system/libdyld.dylib
    0x7fff53edd000 -     0x7fff53eddffb  libkeymgr.dylib (28) <6D84A96F-C65B-38EC-BDB5-21FD2C97E7B2> /usr/lib/system/libkeymgr.dylib
    0x7fff53eeb000 -     0x7fff53eebff7  liblaunch.dylib (1205.30.29) <E66F58ED-C15E-3DFB-BC22-A861E13918C6> /usr/lib/system/liblaunch.dylib
    0x7fff53eec000 -     0x7fff53ef0ffb  libmacho.dylib (900.0.1) <756F2553-07B6-3B42-ACEA-2F0F1A5E8D0F> /usr/lib/system/libmacho.dylib
    0x7fff53ef1000 -     0x7fff53ef3ff3  libquarantine.dylib (86) <6AC8773F-3817-3D82-99C2-01BABB9C3CBB> /usr/lib/system/libquarantine.dylib
    0x7fff53ef4000 -     0x7fff53ef5ff3  libremovefile.dylib (45) <912FA211-DD8C-3C92-8424-21B89F8B10FD> /usr/lib/system/libremovefile.dylib
    0x7fff53ef6000 -     0x7fff53f0dfff  libsystem_asl.dylib (356.1.1) <94972913-9DF0-3C78-847C-43E58919E3DA> /usr/lib/system/libsystem_asl.dylib
    0x7fff53f0e000 -     0x7fff53f0efff  libsystem_blocks.dylib (67) <F2493BB5-B1C6-3C4D-9F1F-1B402E0F1DB7> /usr/lib/system/libsystem_blocks.dylib
    0x7fff53f0f000 -     0x7fff53f98ff7  libsystem_c.dylib (1244.30.3) <E0136C71-0648-36F0-9F84-82EA2748A8D7> /usr/lib/system/libsystem_c.dylib
    0x7fff53f99000 -     0x7fff53f9cffb  libsystem_configuration.dylib (963.30.1) <0F8D0B76-4F7D-34EC-AB6C-50F9465809DA> /usr/lib/system/libsystem_configuration.dylib
    0x7fff53f9d000 -     0x7fff53fa0ffb  libsystem_coreservices.dylib (51) <21A488D0-2D07-344E-8631-CC8B2A246F35> /usr/lib/system/libsystem_coreservices.dylib
    0x7fff53fa1000 -     0x7fff53fa2fff  libsystem_darwin.dylib (1244.30.3) <2F750CB1-BC26-3FA3-AE59-553EE30D451B> /usr/lib/system/libsystem_darwin.dylib
    0x7fff53fa3000 -     0x7fff53fa9ff7  libsystem_dnssd.dylib (878.30.4) <EB9BB165-45A4-367C-B33A-688D4F383A95> /usr/lib/system/libsystem_dnssd.dylib
    0x7fff53faa000 -     0x7fff53ff3ff7  libsystem_info.dylib (517.30.1) <7D79E167-4B5C-3833-81EE-3AF3FB53616D> /usr/lib/system/libsystem_info.dylib
    0x7fff53ff4000 -     0x7fff54019ff7  libsystem_kernel.dylib (4570.41.2) <5155A4C3-825B-3178-AC51-0D2D2F2A6618> /usr/lib/system/libsystem_kernel.dylib
    0x7fff5401a000 -     0x7fff54065fcb  libsystem_m.dylib (3146) <ABB1B85F-9FFE-31B8-AD4F-E39A30794A93> /usr/lib/system/libsystem_m.dylib
    0x7fff54066000 -     0x7fff54085fff  libsystem_malloc.dylib (140.40.1) <36B22C99-D772-3039-9A4C-AA31389965E1> /usr/lib/system/libsystem_malloc.dylib
    0x7fff54086000 -     0x7fff5412aff3  libsystem_network.dylib (1229.30.11) <40BAD301-8744-3AD8-A688-E7925C587B00> /usr/lib/system/libsystem_network.dylib
    0x7fff5412b000 -     0x7fff54135ffb  libsystem_networkextension.dylib (767.40.1) <CEDC330D-28F0-3902-BEB0-10B92ACEC69F> /usr/lib/system/libsystem_networkextension.dylib
    0x7fff54136000 -     0x7fff5413fff3  libsystem_notify.dylib (172) <98EA3D62-7C86-30DE-8261-D020D2F1EFF3> /usr/lib/system/libsystem_notify.dylib
    0x7fff54140000 -     0x7fff54147ff7  libsystem_platform.dylib (161.20.1) <C049250F-8C35-314D-810F-4E28AEAED983> /usr/lib/system/libsystem_platform.dylib
    0x7fff54148000 -     0x7fff54153fff  libsystem_pthread.dylib (301.30.1) <ABA848E1-6978-3B42-A3A7-608B2C36FA93> /usr/lib/system/libsystem_pthread.dylib
    0x7fff54154000 -     0x7fff54157ff3  libsystem_sandbox.dylib (765.40.2) <922D3D15-AB4C-3F1A-A94F-39214AF1ADB3> /usr/lib/system/libsystem_sandbox.dylib
    0x7fff54158000 -     0x7fff54159ff3  libsystem_secinit.dylib (30) <F06ADB8F-9E94-34A7-B3C9-2C22FDD14BAD> /usr/lib/system/libsystem_secinit.dylib
    0x7fff5415a000 -     0x7fff54161ff7  libsystem_symptoms.dylib (820.30.7) <DC3586C2-AA56-3419-88D3-FC0DBF08E3C0> /usr/lib/system/libsystem_symptoms.dylib
    0x7fff54162000 -     0x7fff54175ff7  libsystem_trace.dylib (829.30.14) <69EBF017-D40F-30D7-9B0B-BFC862D761A5> /usr/lib/system/libsystem_trace.dylib
    0x7fff54177000 -     0x7fff5417cff7  libunwind.dylib (35.3) <6D4FCD49-D2A9-3233-95C7-A7635CE265F2> /usr/lib/system/libunwind.dylib
    0x7fff5417d000 -     0x7fff541a9ff7  libxpc.dylib (1205.30.29) <F7E5F1BC-614B-39CB-B6CE-92A9C7B7EC0B> /usr/lib/system/libxpc.dylib
External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
  Calls made by this process:
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
