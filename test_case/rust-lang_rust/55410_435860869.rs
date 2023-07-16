plain
[00:03:19]       Memory: 8 GB
[00:03:19]       Boot ROM Version: VMW71.00V.0.B64.1704110547
[00:03:19]       Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
[00:03:19]       SMC Version (system): 2.8f0
[00:03:19]       Serial Number (system): VMhltNEBRPAU
[00:03:19] 
[00:03:19] hw.ncpu: 4
[00:03:19] hw.byteorder: 1234
[00:03:19] hw.memsize: 8589934592
---
[00:05:16] [RUSTC-TIMING] rustc_tsan test:false 0.251
[00:05:16] [RUSTC-TIMING] panic_unwind test:false 0.471
[00:05:40] error: linking with `cc` failed: exit code: 1
[00:05:40]   |
[00:05:40]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/std-488eca94fe82da1a.std.9wz3hm6n-cgu.0.rcgu.o" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/libstd-488eca94fe82da1a.dylib" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/std-488eca94fe82da1a.br81sexl84qnwlo.rcgu.o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps/std-488eca94fe82da1a.51qntoo1q8ednmi2.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/release/deps" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/libbacktrace/" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/libbacktrace" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-ed49fae7907d47e0/out" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/profiler_builtins-636e7d7c2c55400f/out" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib" "-Wl,-force_load" "-Wl,/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/libbacktrace/libbacktrace.a" "-lSystem" "-lresolv" "-Wl,-force_load" "-Wl,/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustcnZzlLu/libpanic_unwind-c4e11d7e6b740860.rlib" "-Wl,-force_load" "-Wl,/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustcnZzlLu/libunwind-e8415ff2f9099cea.rlib" "-Wl,-force_load" "-Wl,/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustcnZzlLu/liballoc_system-9cae6dfa1c647f41.rlib" "-Wl,-force_load" "-Wl,/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustcnZzlLu/liblibc-bac6380900a2df98.rlib" "-Wl,-force_load" "-Wl,/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustcnZzlLu/liballoc-a9d61c075edcd554.rlib" "-Wl,-force_load" "-Wl,/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustcnZzlLu/libcore-e7e6742debafea8b.rlib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustcnZzlLu/libcompiler_builtins-fc5f2eca929aefb5.rlib" "-lc" "-lm" "-dynamiclib" "-Wl,-dylib" "-Wl,-install_name" "-Wl,@rpath/libstd-488eca94fe82da1a.dylib" "-Wl,-rpath,@loader_path/../lib"
[00:05:40]   = note: ld: warning: directory not found for option '-L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib'
[00:05:40]           ld: warning: directory not found for option '-L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib'
[00:05:40]           Undefined symbols for architecture x86_64:
[00:05:40]             "___atomic_load", referenced from:
[00:05:40]                 _$LT$core..sync..atomic..AtomicI128$u20$as$u20$core..fmt..Debug$GT$::fmt::h0efa719038b1f39b in libcore-e7e6742debafea8b.rlib(core-e7e6742debafea8b.core.7wi5exqu-cgu.0.rcgu.o)
[00:05:40]                 _$LT$core..sync..atomic..AtomicU128$u20$as$u20$core..fmt..Debug$GT$::fmt::h7efa51815e2f2d4d in libcore-e7e6742debafea8b.rlib(core-e7e6742debafea8b.core.7wi5exqu-cgu.0.rcgu.o)
[00:05:40]           ld: symbol(s) not found for architecture x86_64
[00:05:40]           
[00:05:40] 
[00:05:40] error: aborting due to previous error
[00:05:40] 
[00:05:40] 
[00:05:40] [RUSTC-TIMING] std test:false 23.999
[00:05:40] error: Could not compile `std`.
[00:05:40] 
[00:05:40] To learn more, run the command again with --verbose.
[00:05:40] command did not execute successfully: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler" "--manifest-path" "/Users/travis/build/rust-lang/rust/src/libstd/Cargo.toml" "--message-format" "json"
[00:05:40] expected success, got: exit code: 101
[00:05:40] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:05:40] travis_fold:end:stage0-std

[00:05:40] travis_time:end:stage0-std:start=1541421743642462000,finish=1541421817744152000,duration=74101690000


[00:05:40] failed to run: /Users/travis/build/rust-lang/rust/build/bootstrap/debug/bootstrap build
[00:05:40] Build completed unsuccessfully in 0:01:16
[00:05:40] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2bd929c3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_fold:start:after_failure.2
travis_time:start:0048f218
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
total 184
drwx------   3 travis  staff    102 Nov  5 12:34 .
-rw-------@  1 travis  staff  91743 Nov  5 12:34 CalendarAgent_2018-11-05-123432_Traviss-Mac-1044.crash
drwx------+ 15 travis  staff    510 Jan 25  2018 ..
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:034893d0
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
travis_fold:start:crashlog
/Users/travis/Library/Logs/DiagnosticReports/CalendarAgent_2018-11-05-123432_Traviss-Mac-1044.crash
Process:               CalendarAgent [285]
Path:                  /System/Library/PrivateFrameworks/CalendarAgent.framework/Executables/CalendarAgent
Identifier:            CalendarAgent
Version:               8.0 (399.2.2)
Code Type:             X86-64 (Native)
Parent Process:        ??? [1]
Responsible:           CalendarAgent [285]
User ID:               501
Date/Time:             2018-11-05 12:34:17.917 +0000
OS Version:            Mac OS X 10.13.3 (17D47)
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7
Time Awake Since Boot: 27 seconds
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
    __TEXT                 000000010cb37000-000000010cb39000 [    8K] r-x/rwx SM=COW  & [/System/Library/PrivateFrameworks/CalendarAgent.framework/Executables/CalendarAgent]
Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_pthread.dylib        0x00007fff509b13d5 pthread_mutex_lock + 0
1   com.apple.corelocation         0x00007fff29ab12a8 0x7fff29aa8000 + 37544
2   com.apple.corelocation         0x00007fff29aac2be 0x7fff29aa8000 + 17086
3   com.apple.corelocation         0x00007fff29aab2da 0x7fff29aa8000 + 13018
4   com.apple.CoreFoundation       0x00007fff28d7b454 __CFRUNLOOP_IS_CALLING_OUT_TO_A_TIMER_CALLBACK_FUNCTION__ + 20
5   com.apple.CoreFoundation       0x00007fff28d7b0d4 __CFRunLoopDoTimer + 1108
6   com.apple.CoreFoundation       0x00007fff28d7abca __CFRunLoopDoTimers + 346
7   com.apple.CoreFoundation       0x00007fff28d7239b __CFRunLoopRun + 2427
8   com.apple.CoreFoundation       0x00007fff28d71787 CFRunLoopRunSpecific + 487
9   CalendarAgent                  0x000000010cb387e4 0x10cb37000 + 6116
10  libdyld.dylib                  0x00007fff50728114 0x7fff50727000 + 4372
Thread 1:: Dispatch queue: com.apple.root.user-initiated-qos
0   com.apple.CalendarPersistence  0x00007fff39d17571 -[CalNetworkChangeNotifier init] + 0
1   com.apple.CalendarPersistence  0x00007fff39d16b90 +[CalNetworkChangeNotifier enableNotifications] + 46
2   com.apple.CalendarPersistence  0x00007fff39d16b55 +[CalNetworkChangeNotifier sharedNotifier] + 17
3   com.apple.CalendarAgent        0x00007fff39b13ead -[CalAgent _setupNetworkChangeNotificationListener] + 37
4   com.apple.CalendarAgent        0x00007fff39b126e4 -[CalAgent start] + 456
5   CalendarAgent                  0x000000010cb3891d 0x10cb37000 + 6429
6   libdispatch.dylib              0x00007fff506f6591 _dispatch_call_block_and_release + 12
7   libdispatch.dylib              0x00007fff506eed50 _dispatch_client_callout + 8
8   libdispatch.dylib              0x00007fff506f0ac4 _dispatch_root_queue_drain + 902
9   libdispatch.dylib              0x00007fff506f06ed _dispatch_worker_thread3 + 101
10  libsystem_pthread.dylib        0x00007fff509b31ca _pthread_wqthread + 1387
11  libsystem_pthread.dylib        0x00007fff509b2c4d start_wqthread + 13
Thread 2:
0   libsystem_pthread.dylib        0x00007fff509b2c40 start_wqthread + 0
Thread 3:: Dispatch queue: com.apple.root.default-qos.overcommit
0   libsystem_kernel.dylib         0x00007fff5086e7c2 mach_msg_trap + 10
1   libsystem_kernel.dylib         0x00007fff5086dcdc mach_msg + 60
2   libxpc.dylib                   0x00007fff509ea3ea xpc_pipe_routine + 302
3   libxpc.dylib                   0x00007fff509ea21e _xpc_interface_routine + 163
4   libxpc.dylib                   0x00007fff509f16ae _xpc_activate_endpoint + 143
5   libxpc.dylib                   0x00007fff509ed7fa _xpc_connection_init + 866
6   libxpc.dylib                   0x00007fff509ed488 _xpc_connection_resume_init + 14
7   libdispatch.dylib              0x00007fff506eed50 _dispatch_client_callout + 8
8   libdispatch.dylib              0x00007fff506fbc61 _dispatch_queue_override_invoke + 880
9   libdispatch.dylib              0x00007fff506f0941 _dispatch_root_queue_drain + 515
10  libdispatch.dylib              0x00007fff506f06ed _dispatch_worker_thread3 + 101
11  libsystem_pthread.dylib        0x00007fff509b31ca _pthread_wqthread + 1387
12  libsystem_pthread.dylib        0x00007fff509b2c4d start_wqthread + 13
Thread 4:
0   libsystem_pthread.dylib        0x00007fff509b2c40 start_wqthread + 0
1   ???                            0x7265666572504643 0 + 8243107278867154499
Thread 5:
0   libsystem_pthread.dylib        0x00007fff509b2c40 start_wqthread + 0
1   ???                            0x614465726f43534e 0 + 7008838462262694734
Thread 6:: Dispatch queue: com.apple.root.default-qos.overcommit
0   libsystem_kernel.dylib         0x00007fff5086e7c2 mach_msg_trap + 10
1   libsystem_kernel.dylib         0x00007fff5086dcdc mach_msg + 60
2   libxpc.dylib                   0x00007fff509ea3ea xpc_pipe_routine + 302
3   libxpc.dylib                   0x00007fff509ea21e _xpc_interface_routine + 163
4   libxpc.dylib                   0x00007fff509f16ae _xpc_activate_endpoint + 143
5   libxpc.dylib                   0x00007fff509ed7fa _xpc_connection_init + 866
6   libxpc.dylib                   0x00007fff509ed488 _xpc_connection_resume_init + 14
7   libdispatch.dylib              0x00007fff506eed50 _dispatch_client_callout + 8
8   libdispatch.dylib              0x00007fff506fbc61 _dispatch_queue_override_invoke + 880
9   libdispatch.dylib              0x00007fff506f0941 _dispatch_root_queue_drain + 515
10  libdispatch.dylib              0x00007fff506f06ed _dispatch_worker_thread3 + 101
11  libsystem_pthread.dylib        0x00007fff509b31ca _pthread_wqthread + 1387
12  libsystem_pthread.dylib        0x00007fff509b2c4d start_wqthread + 13
Thread 7:: Dispatch queue: SQLQueue 0x7f9561c12c70 for Calendar Cache
0   libsystem_malloc.dylib         0x00007fff508d031f szone_malloc_should_clear + 194
1   libsystem_malloc.dylib         0x00007fff508d0201 malloc_zone_malloc + 103
2   libsqlite3.dylib               0x00007fff50055536 sqlite3MemMalloc + 38
3   libsqlite3.dylib               0x00007fff5010f5e1 dbMallocRawFinish + 17
4   libsqlite3.dylib               0x00007fff5009b651 sqlite3VdbeMemGrow + 497
5   libsqlite3.dylib               0x00007fff5005634a sqlite3VdbeMemSetStr + 394
6   libsqlite3.dylib               0x00007fff5009b3a1 generateColumnNames + 2065
7   libsqlite3.dylib               0x00007fff50089ff7 sqlite3Select + 27863
8   libsqlite3.dylib               0x00007fff50061422 yy_reduce + 2178
9   libsqlite3.dylib               0x00007fff5005f8ca sqlite3RunParser + 538
10  libsqlite3.dylib               0x00007fff5005e7b2 sqlite3Prepare + 850
11  libsqlite3.dylib               0x00007fff5005e3b4 sqlite3LockAndPrepare + 292
12  libsqlite3.dylib               0x00007fff500ce7be sqlite3_prepare_v2 + 254
13  com.apple.CoreData             0x00007fff288d7062 -[NSSQLiteConnection prepareSQLStatement:] + 498
14  com.apple.CoreData             0x00007fff289ed4b5 -[NSSQLiteConnection selectRowsWithStatement:cached:] + 53
15  com.apple.CoreData             0x00007fff28b13a9d _newRawRowsForXPCRequest + 301
16  com.apple.CoreData             0x00007fff28b13f15 _rawResultSetForForXPCRequest + 181
17  com.apple.CoreData             0x00007fff28b13f8f _rawRowDataForXPCRequest + 63
18  com.apple.CoreData             0x00007fff28a12402 -[NSSQLXPCFetchRequestContext executeRequestCore:] + 18
19  com.apple.CoreData             0x00007fff28ac01ed -[NSSQLStoreRequestContext executeRequestUsingConnection:] + 205
20  com.apple.CoreData             0x00007fff28acfaeb __52-[NSSQLDefaultConnectionManager handleStoreRequest:]_block_invoke + 75
21  libdispatch.dylib              0x00007fff506eed50 _dispatch_client_callout + 8
22  libdispatch.dylib              0x00007fff507021d6 _dispatch_queue_barrier_sync_invoke_and_complete + 60
23  com.apple.CoreData             0x00007fff28acf9d3 -[NSSQLDefaultConnectionManager handleStoreRequest:] + 339
24  com.apple.CoreData             0x00007fff28a0cfc4 -[NSSQLCoreDispatchManager routeStoreRequest:] + 308
25  com.apple.CoreData             0x00007fff289d3305 -[NSSQLCore dispatchRequest:withRetries:] + 229
26  com.apple.CoreData             0x00007fff289cf080 -[NSSQLCore _newRowDataForXPCFetch:variables:context:error:] + 160
27  com.apple.CoreData             0x00007fff28b70f97 __94-[NSXPCStoreServerRequestHandlingPolicy _coreProcessFetchRequest:fromClientWithContext:error:]_block_invoke + 263
28  com.apple.CoreData             0x00007fff289bb012 gutsOfBlockToNSPersistentStoreCoordinatorPerform + 210
29  libdispatch.dylib              0x00007fff506eed50 _dispatch_client_callout + 8
30  libdispatch.dylib              0x00007fff507021d6 _dispatch_queue_barrier_sync_invoke_and_complete + 60
31  com.apple.CoreData             0x00007fff289a3f22 -[NSPersistentStoreCoordinator performBlockAndWait:] + 178
32  com.apple.CoreData             0x00007fff28b70927 -[NSXPCStoreServerRequestHandlingPolicy _coreProcessFetchRequest:fromClientWithContext:error:] + 551
33  com.apple.CoreData             0x00007fff28b69479 -[NSXPCStoreServer(InternalMethods) handleFetchRequest:inContext:error:] + 713
34  com.apple.CoreData             0x00007fff28b656a0 __40-[NSXPCStoreServer handleRequest:reply:]_block_invoke + 2016
35  libdispatch.dylib              0x00007fff50704e03 _dispatch_block_async_invoke2 + 102
36  libdispatch.dylib              0x00007fff506eed50 _dispatch_client_callout + 8
37  libdispatch.dylib              0x00007fff50701e76 _dispatch_continuation_pop + 472
38  libdispatch.dylib              0x00007fff506f96cb _dispatch_async_redirect_invoke + 703
39  libdispatch.dylib              0x00007fff506f0941 _dispatch_root_queue_drain + 515
40  libdispatch.dylib              0x00007fff506f06ed _dispatch_worker_thread3 + 101
41  libsystem_pthread.dylib        0x00007fff509b31ca _pthread_wqthread + 1387
42  libsystem_pthread.dylib        0x00007fff509b2c4d start_wqthread + 13
Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x7b7c4488ca7700af  rbx: 0x00007f9561f6e050  rcx: 0x0000000000000000  rdx: 0x0000000000000000
  rdi: 0x0000000000000000  rsi: 0x00007ffee30c6c58  rbp: 0x00007ffee30c6c40  rsp: 0x00007ffee30c6bc8
   r8: 0x000000000000000b   r9: 0x0000000000000000  r10: 0x000007f9561f1176  r11: 0x000000000000000c
  r12: 0x00007ffee30c6c58  r13: 0x00007f9561f6a658  r14: 0x0000000000000000  r15: 0x0000000000000000
  rip: 0x00007fff509b13d5  rfl: 0x0000000000010202  cr2: 0x00007fff39d17571
Logical CPU:     1
Error Code:      0x00000014
Trap Number:     14
Binary Images:
       0x10cb37000 -        0x10cb38fff  CalendarAgent (8.0 - 399.2.2) <084705E9-0DFB-3DE3-9141-3114AE6333FE> /System/Library/PrivateFrameworks/CalendarAgent.framework/Executables/CalendarAgent
       0x11864c000 -        0x11869698f  dyld (519.2.2) <6695F30B-4E88-3C0B-9867-7D738C44A3E6> /usr/lib/dyld
    0x7fff24ca4000 -     0x7fff24e7ffff  com.apple.avfoundation (2.0 - 1530.2) <30F528A4-548F-3E18-80A8-06481788E84C> /System/Library/Frameworks/AVFoundation.framework/Versions/A/AVFoundation
    0x7fff24e80000 -     0x7fff24f3aff7  com.apple.audio.AVFAudio (1.0 - ???) <891340D8-A1F3-37B5-A183-EE5BE81F5296> /System/Library/Frameworks/AVFoundation.framework/Versions/A/Frameworks/AVFAudio.framework/Versions/A/AVFAudio
    0x7fff25040000 -     0x7fff25040fff  com.apple.Accelerate (1.11 - Accelerate 1.11) <3468C044-F1F9-3A8F-B861-B1A9A0614EC3> /System/Library/Frameworks/Accelerate.framework/Versions/A/Accelerate
    0x7fff25059000 -     0x7fff257a7ffb  com.apple.vImage (8.1 - ???) <EF6110B2-8BE1-3078-811A-52DB7174494D> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vImage.framework/Versions/A/vImage
    0x7fff257a8000 -     0x7fff25949ffb  libBLAS.dylib (1211.30.1) <D5E70F72-E810-3AC1-897B-2B6FF0F7FFED> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBLAS.dylib
    0x7fff2594a000 -     0x7fff25984fef  libBNNS.dylib (37) <E65987ED-E7B1-3E74-BC3C-851CBFA20CC5> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libBNNS.dylib
    0x7fff25985000 -     0x7fff25d5eff7  libLAPACK.dylib (1211.30.1) <D5A466A3-F7B2-39EC-A5CA-0D2810A4AD47> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLAPACK.dylib
    0x7fff25d5f000 -     0x7fff25d75ff7  libLinearAlgebra.dylib (1211.30.1) <2029891F-DF9C-38E9-B32C-5B846F38453E> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libLinearAlgebra.dylib
    0x7fff25d76000 -     0x7fff25d7bff3  libQuadrature.dylib (3) <3D6BF66A-55B2-3692-BAC7-DEB0C676ED29> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libQuadrature.dylib
    0x7fff25d7c000 -     0x7fff25ddafff  libSparse.dylib (79.1.1) <7AD0F8A8-FD36-36FE-B83D-58648EBD0027> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparse.dylib
    0x7fff25ddb000 -     0x7fff25dedff7  libSparseBLAS.dylib (1211.30.1) <F7267913-EAFC-3185-9148-3B5DA42B7374> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libSparseBLAS.dylib
    0x7fff25dee000 -     0x7fff25f61fe7  libvDSP.dylib (622.20.8) <BFC9EDE8-3AB8-3FA4-BD4B-B5B36810A422> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvDSP.dylib
    0x7fff25f62000 -     0x7fff26017fe7  libvMisc.dylib (622.20.8) <35213FEE-B162-3428-B7FC-84E24D5589CF> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/libvMisc.dylib
    0x7fff26018000 -     0x7fff26018fff  com.apple.Accelerate.vecLib (3.11 - vecLib 3.11) <D140AD19-2E88-3D53-AA43-C56E269C01A4> /System/Library/Frameworks/Accelerate.framework/Versions/A/Frameworks/vecLib.framework/Versions/A/vecLib
    0x7fff26019000 -     0x7fff26070ff7  com.apple.Accounts (113 - 113) <02AC0AF3-D620-340E-BE52-CD31119F9CD8> /System/Library/Frameworks/Accounts.framework/Versions/A/Accounts
    0x7fff26071000 -     0x7fff26307fff  com.apple.AddressBook.framework (11.0 - 1808.6) <1BE4B8EF-60A4-320C-91A5-5241E2118B82> /System/Library/Frameworks/AddressBook.framework/Versions/A/AddressBook
    0x7fff26308000 -     0x7fff27165fff  com.apple.AppKit (6.9 - 1561.20.106) <D03AE413-C601-3B7C-A3D4-FC32F4C13940> /System/Library/Frameworks/AppKit.framework/Versions/C/AppKit
    0x7fff271b7000 -     0x7fff271b7fff  com.apple.ApplicationServices (48 - 50) <7627DBD6-497B-3AB7-9B63-F0532EDF09B8> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/ApplicationServices
    0x7fff271b8000 -     0x7fff2721efff  com.apple.ApplicationServices.ATS (377 - 445) <CDF5F6D7-4E7D-3D28-9FBA-1B53AD9FA8F8> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/ATS
    0x7fff272b7000 -     0x7fff273d9fff  libFontParser.dylib (222.1.2) <11BD5EEF-AF18-33FB-B114-DD611932E822> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontParser.dylib
    0x7fff273da000 -     0x7fff27424ff7  libFontRegistry.dylib (221) <A22F82C0-B4FE-3DB5-B968-79B28257DF2F> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ATS.framework/Versions/A/Resources/libFontRegistry.dylib
    0x7fff27566000 -     0x7fff2756aff3  com.apple.ColorSyncLegacy (4.13.0 - 1) <42C25E85-1CF3-3DEC-A434-BE69F68F4318> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/ColorSyncLegacy.framework/Versions/A/ColorSyncLegacy
    0x7fff2760a000 -     0x7fff2765cff7  com.apple.HIServices (1.22 - 622) <2E83CD6F-ED98-3C29-BD0A-8525E38AB5DB> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/HIServices.framework/Versions/A/HIServices
    0x7fff2765d000 -     0x7fff2766bfff  com.apple.LangAnalysis (1.7.0 - 1.7.0) <71A9C815-AC55-3E36-A618-F6778F5119AD> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/LangAnalysis.framework/Versions/A/LangAnalysis
    0x7fff2766c000 -     0x7fff276b8fff  com.apple.print.framework.PrintCore (13 - 503) <A69E2BAD-2B66-38CC-9D3A-0A0EBC41341D> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/PrintCore
    0x7fff276b9000 -     0x7fff276f3fff  com.apple.QD (3.12 - 403) <38D8106A-4FFA-3FE9-9999-714CADD7EE9C> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/QD.framework/Versions/A/QD
    0x7fff276f4000 -     0x7fff27700fff  com.apple.speech.synthesis.framework (7.4.1 - 7.4.1) <9ABE85D9-6E4A-3CEF-AA09-F81E52730598> /System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/SpeechSynthesis.framework/Versions/A/SpeechSynthesis
    0x7fff27701000 -     0x7fff2798cfff  com.apple.audio.toolbox.AudioToolbox (1.14 - 1.14) <46EDC245-5877-3438-805C-3AA0316E3F5C> /System/Library/Frameworks/AudioToolbox.framework/Versions/A/AudioToolbox
    0x7fff2798e000 -     0x7fff2798efff  com.apple.audio.units.AudioUnit (1.14 - 1.14) <0FC3B4FF-FFAB-3346-9A6A-C1DE033185EA> /System/Library/Frameworks/AudioUnit.framework/Versions/A/AudioUnit
    0x7fff27ca3000 -     0x7fff28035fff  com.apple.CFNetwork (893.13.1 - 893.13.1) <3ECC6AD0-B47D-38D2-BF26-496B34847D25> /System/Library/Frameworks/CFNetwork.framework/Versions/A/CFNetwork
    0x7fff28037000 -     0x7fff28049fff  com.apple.CalendarStore (8.0 - 1479) <AA42BD1B-4B14-324C-A236-ABFA140BC970> /System/Library/Frameworks/CalendarStore.framework/Versions/A/CalendarStore
    0x7fff2804a000 -     0x7fff2804afff  com.apple.Carbon (158 - 158) <FCD7A9FF-5E53-3B0C-8A52-691C11B2A0C5> /System/Library/Frameworks/Carbon.framework/Versions/A/Carbon
    0x7fff2804b000 -     0x7fff2804effb  com.apple.CommonPanels (1.2.6 - 98) <39C8EBA3-EEB2-335B-8A88-D6C64BAA112F> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/CommonPanels.framework/Versions/A/CommonPanels
    0x7fff2804f000 -     0x7fff28354ff7  com.apple.HIToolbox (2.1.1 - 910.4) <D1A799BC-146A-35E8-86FF-E36273123006> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/HIToolbox
    0x7fff28355000 -     0x7fff28358ffb  com.apple.help (1.3.8 - 64) <18D02016-119A-33E8-AEB0-E9466BA4AD56> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/Help.framework/Versions/A/Help
    0x7fff28359000 -     0x7fff2835efff  com.apple.ImageCapture (9.0 - 9.0) <2DE590E5-DF0E-3962-A2BE-06EBC79B3D72> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/ImageCapture.framework/Versions/A/ImageCapture
    0x7fff2835f000 -     0x7fff283f4ffb  com.apple.ink.framework (10.9 - 220) <D8AECAE3-9FD2-32E3-B659-026F33650BB2> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/Ink.framework/Versions/A/Ink
    0x7fff283f5000 -     0x7fff2840fff7  com.apple.openscripting (1.7 - 174) <66899B51-ADFD-360D-88E9-B52439F531B4> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/OpenScripting.framework/Versions/A/OpenScripting
    0x7fff28430000 -     0x7fff28431fff  com.apple.print.framework.Print (12 - 267) <E4CBAAFC-9045-38AC-9F93-8C931DDED9D8> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/Print.framework/Versions/A/Print
    0x7fff28432000 -     0x7fff28434ff7  com.apple.securityhi (9.0 - 55006) <E7668200-B4CB-3612-96B8-D57E94077787> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/SecurityHI.framework/Versions/A/SecurityHI
    0x7fff28435000 -     0x7fff2843bfff  com.apple.speech.recognition.framework (6.0.3 - 6.0.3) <1F10ED1F-12C7-39AC-88A0-43A1338F9316> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/SpeechRecognition.framework/Versions/A/SpeechRecognition
    0x7fff2843c000 -     0x7fff28547ff7  com.apple.cloudkit.CloudKit (735.4 - 735.4) <89BEB558-1CA4-3782-A0F2-0057E2BDFB99> /System/Library/Frameworks/CloudKit.framework/Versions/A/CloudKit
    0x7fff28548000 -     0x7fff28548fff  com.apple.Cocoa (6.11 - 22) <1553F279-3C7B-3A48-87E6-35237F7F452E> /System/Library/Frameworks/Cocoa.framework/Versions/A/Cocoa
    0x7fff28556000 -     0x7fff2860fff7  com.apple.ColorSync (4.13.0 - 546) <A5E013D9-7305-3026-879E-4D1F038A430D> /System/Library/Frameworks/ColorSync.framework/Versions/A/ColorSync
    0x7fff28610000 -     0x7fff286faff7  com.apple.contacts (1.0 - 2330.3) <CD2911E1-EA03-3D92-A97F-9143CC9BAA8F> /System/Library/Frameworks/Contacts.framework/Versions/A/Contacts
    0x7fff286fb000 -     0x7fff2879bff3  com.apple.ContactsUI (11.0 - 1808.6) <0524188A-7FA1-3D49-A111-321FEF07212C> /System/Library/Frameworks/ContactsUI.framework/Versions/A/ContactsUI
    0x7fff2879c000 -     0x7fff2882fff7  com.apple.audio.CoreAudio (4.3.0 - 4.3.0) <F91FDE26-0702-3E44-8931-E2CAD8E36F5A> /System/Library/Frameworks/CoreAudio.framework/Versions/A/CoreAudio
    0x7fff28896000 -     0x7fff288bfffb  com.apple.CoreBluetooth (1.0 - 1) <942F88A5-AD68-3359-90D5-6F1A3311C51A> /System/Library/Frameworks/CoreBluetooth.framework/Versions/A/CoreBluetooth
    0x7fff288c0000 -     0x7fff28c22ffb  com.apple.CoreData (120 - 849.2) <4A6F675D-6915-3530-8BB6-2A8C8F5A4F75> /System/Library/Frameworks/CoreData.framework/Versions/A/CoreData
    0x7fff28c23000 -     0x7fff28ceffff  com.apple.CoreDisplay (1.0 - 81.7) <D8030B81-097E-3FA2-A85C-AE1A3B8EBCFB> /System/Library/Frameworks/CoreDisplay.framework/Versions/A/CoreDisplay
    0x7fff28cf0000 -     0x7fff29189fff  com.apple.CoreFoundation (6.9 - 1451) <739D6558-3DF3-3181-AA07-BBE3882D3B7F> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
    0x7fff2918b000 -     0x7fff297b6ff7  com.apple.CoreGraphics (2.0 - 1129.5) <C3E052A6-B0EF-3017-A1EB-4C86CE986B8C> /System/Library/Frameworks/CoreGraphics.framework/Versions/A/CoreGraphics
    0x7fff297b8000 -     0x7fff29aa7fff  com.apple.CoreImage (13.0.0 - 579.2.9) <8AE143AB-6284-3B00-B56D-8C0C1826EF34> /System/Library/Frameworks/CoreImage.framework/Versions/A/CoreImage
    0x7fff29aa8000 -     0x7fff29b13ff7  com.apple.corelocation (2237.0.22) <EF931B7C-7656-389B-A161-FB6605869B0B> /System/Library/Frameworks/CoreLocation.framework/Versions/A/CoreLocation
    0x7fff29d36000 -     0x7fff29e17fff  com.apple.CoreMedia (1.0 - 2270.12.1) <B62CF714-D69A-38C3-9929-B70FF4CBB216> /System/Library/Frameworks/CoreMedia.framework/Versions/A/CoreMedia
    0x7fff29e18000 -     0x7fff29e66fff  com.apple.CoreMediaIO (812.0 - 4993) <CA47AF11-0EF4-33E1-9EC3-7A56661FFEE4> /System/Library/Frameworks/CoreMediaIO.framework/Versions/A/CoreMediaIO
    0x7fff29e67000 -     0x7fff29e67fff  com.apple.CoreServices (822.19 - 822.19) <44456ED2-59E4-34CB-B41B-C6A82B269949> /System/Library/Frameworks/CoreServices.framework/Versions/A/CoreServices
    0x7fff29e68000 -     0x7fff29edcffb  com.apple.AE (735.1 - 735.1) <D0C73200-90A7-3FD1-A6EC-97055AA367E2> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/AE.framework/Versions/A/AE
    0x7fff29edd000 -     0x7fff2a1b4ff7  com.apple.CoreServices.CarbonCore (1178.2 - 1178.2) <A1FE74F8-953B-371E-A8AC-E87B30FB79C6> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/CarbonCore
    0x7fff2a1b5000 -     0x7fff2a1e9ff7  com.apple.DictionaryServices (1.2 - 284) <3FCEE280-8DD0-37C9-BFD4-7BA87AAFC8EF> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/DictionaryServices.framework/Versions/A/DictionaryServices
    0x7fff2a1ea000 -     0x7fff2a1f2ff3  com.apple.CoreServices.FSEvents (1239 - 1239) <7BBC5CB7-DBC8-316B-99B0-781827159A2F> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/FSEvents.framework/Versions/A/FSEvents
    0x7fff2a1f3000 -     0x7fff2a3abff7  com.apple.LaunchServices (822.19 - 822.19) <2895A919-0445-3CE2-9696-40122B5A46C5> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/LaunchServices
    0x7fff2a3ac000 -     0x7fff2a45bff7  com.apple.Metadata (10.7.0 - 1191.2.6) <FB66B298-D55D-398A-BEDB-CB7B82956AE5> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/Metadata.framework/Versions/A/Metadata
    0x7fff2a45c000 -     0x7fff2a4b9ff7  com.apple.CoreServices.OSServices (822.19 - 822.19) <34BF1FAC-A0F7-37B4-950D-46408EBA9684> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/OSServices.framework/Versions/A/OSServices
    0x7fff2a4ba000 -     0x7fff2a528fff  com.apple.SearchKit (1.4.0 - 1.4.0) <14053F88-2C76-35CA-9FC1-2A9BC0B63F88> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SearchKit.framework/Versions/A/SearchKit
    0x7fff2a529000 -     0x7fff2a54dffb  com.apple.coreservices.SharedFileList (71.4 - 71.4) <4AA6DCF5-BAF8-36FA-A8B0-EDF518EFEF14> /System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/SharedFileList.framework/Versions/A/SharedFileList
    0x7fff2a54e000 -     0x7fff2a5a9ff3  com.apple.CoreSpotlight (1.0 - 231.2.2.2) <5EE0D800-96D5-31F3-80CA-C160FAEA42F4> /System/Library/Frameworks/CoreSpotlight.framework/Versions/A/CoreSpotlight
    0x7fff2a75f000 -     0x7fff2a7e7ff3  com.apple.CoreTelephony (113 - 5642) <675B73F8-C05E-3804-9539-6F9AB9ABF6A4> /System/Library/Frameworks/CoreTelephony.framework/Versions/A/CoreTelephony
    0x7fff2a7e8000 -     0x7fff2a936ffb  com.apple.CoreText (352.0 - 578.12) <DA0BC559-277A-32BA-91EA-FD2F02EA186F> /System/Library/Frameworks/CoreText.framework/Versions/A/CoreText
    0x7fff2a937000 -     0x7fff2a971ff3  com.apple.CoreVideo (1.8 - 279.2) <A8FC5325-D092-3A28-A1CF-5C94B8101F71> /System/Library/Frameworks/CoreVideo.framework/Versions/A/CoreVideo
    0x7fff2a972000 -     0x7fff2a9fcffb  com.apple.framework.CoreWLAN (13.0 - 1339) <16DDD47C-BBFE-3D49-8BDF-3652017240FD> /System/Library/Frameworks/CoreWLAN.framework/Versions/A/CoreWLAN
    0x7fff2ab8a000 -     0x7fff2ab95ff7  com.apple.DirectoryService.Framework (10.13 - 207) <79F272D7-48AE-3D1E-AC5B-3CEB5F16E7D5> /System/Library/Frameworks/DirectoryService.framework/Versions/A/DirectoryService
    0x7fff2ab96000 -     0x7fff2ac50fff  com.apple.DiscRecording (9.0.3 - 9030.4.5) <2094BAC4-4F37-3741-B9BD-60158A63C503> /System/Library/Frameworks/DiscRecording.framework/Versions/A/DiscRecording
    0x7fff2ac77000 -     0x7fff2ac7cfff  com.apple.DiskArbitration (2.7 - 2.7) <44836CE9-A9ED-3017-972A-7A0A3D6B472B> /System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration
    0x7fff2ac7d000 -     0x7fff2ae14fff  com.apple.ical.EventKit (3.0 - 751.2.3) <9E06A51E-BCE4-3F21-AB9B-1C11C415FB31> /System/Library/Frameworks/EventKit.framework/Versions/A/EventKit
    0x7fff2ae3d000 -     0x7fff2b202fff  com.apple.Foundation (6.9 - 1451) <B99F94E7-117E-39CC-A65D-B7AEA8998481> /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
    0x7fff2b272000 -     0x7fff2b2a2fff  com.apple.GSS (4.0 - 2.0) <3B4B4509-B5A3-396B-9C71-80BAE84476FA> /System/Library/Frameworks/GSS.framework/Versions/A/GSS
    0x7fff2b3b4000 -     0x7fff2b4b7ff3  com.apple.Bluetooth (6.0.2 - 6.0.2f2) <718565B3-6DA7-3DF9-BB51-391FFCAEAFA5> /System/Library/Frameworks/IOBluetooth.framework/Versions/A/IOBluetooth
    0x7fff2b517000 -     0x7fff2b5b2fff  com.apple.framework.IOKit (2.0.2 - 1445.40.1) <9CFA07B9-BA6E-31E4-AD4F-C47071A8C522> /System/Library/Frameworks/IOKit.framework/Versions/A/IOKit
    0x7fff2b5b4000 -     0x7fff2b5bbffb  com.apple.IOSurface (209.2.2 - 209.2.2) <6D35A601-1C47-37BE-AD31-F8EB88F67573> /System/Library/Frameworks/IOSurface.framework/Versions/A/IOSurface
    0x7fff2b5bc000 -     0x7fff2b611ff3  com.apple.ImageCaptureCore (7.0 - 7.0) <CBE349D4-2C8B-31F5-B5ED-B8E978DA9245> /System/Library/Frameworks/ImageCaptureCore.framework/Versions/A/ImageCaptureCore
    0x7fff2b612000 -     0x7fff2b78dfff  com.apple.ImageIO.framework (3.3.0 - 1713) <5388A825-2A1C-37C8-82DE-1B2265BE4244> /System/Library/Frameworks/ImageIO.framework/Versions/A/ImageIO
    0x7fff2b78e000 -     0x7fff2b792ffb  libGIF.dylib (1713) <A08E517F-2E9F-3A10-B180-592EDEB7A681> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libGIF.dylib
    0x7fff2b793000 -     0x7fff2b87afff  libJP2.dylib (1713) <50D10FDC-C470-3902-BDAC-D38BDDCFB81D> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJP2.dylib
    0x7fff2b87b000 -     0x7fff2b89efff  libJPEG.dylib (1713) <19935EFA-6B6D-3754-96CA-6F5F09BD4BB0> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libJPEG.dylib
    0x7fff2bb7b000 -     0x7fff2bba1ff3  libPng.dylib (1713) <0F84AF2F-7248-3D7D-B960-FFDA1A3F5837> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libPng.dylib
    0x7fff2bba2000 -     0x7fff2bba4ff3  libRadiance.dylib (1713) <09699DA5-3354-3D99-A544-CCC346BA4C1E> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libRadiance.dylib
    0x7fff2bba5000 -     0x7fff2bbf2fff  libTIFF.dylib (1713) <84580DD8-513F-383B-B1C2-2294D8B01969> /System/Library/Frameworks/ImageIO.framework/Versions/A/Resources/libTIFF.dylib
    0x7fff2bc0d000 -     0x7fff2bdabff7  com.apple.Intents (1.0 - 1) <B508FFA2-F6B8-3F22-AD8B-240EF59E3600> /System/Library/Frameworks/Intents.framework/Versions/A/Intents
    0x7fff2bdae000 -     0x7fff2c969fff  com.apple.JavaScriptCore (13604 - 13604.5.6) <5541E211-3248-3B89-9AE9-D5FE413F12D1> /System/Library/Frameworks/JavaScriptCore.framework/Versions/A/JavaScriptCore
    0x7fff2c981000 -     0x7fff2c99aff7  com.apple.Kerberos (3.0 - 1) <CAF075C0-4C24-3ACE-9AE6-77BEFDEA3622> /System/Library/Frameworks/Kerberos.framework/Versions/A/Kerberos
    0x7fff2c99b000 -     0x7fff2c9d0fff  com.apple.LDAPFramework (2.4.28 - 194.5) <8419BDB5-12DF-3BC9-871D-7A270E080249> /System/Library/Frameworks/LDAP.framework/Versions/A/LDAP
    0x7fff2c9d1000 -     0x7fff2c9f3ff7  com.apple.speech.LatentSemanticMappingFramework (2.12.3 - 2.12.3) <64C1B0E2-8EE6-35C5-9860-36775A12FCC4> /System/Library/Frameworks/LatentSemanticMapping.framework/Versions/A/LatentSemanticMapping
    0x7fff2ca4c000 -     0x7fff2cc50fff  com.apple.MapKit (1.0 - 1827.22.9.1.10) <3D4848CD-2894-3070-8D27-89C96DF9462B> /System/Library/Frameworks/MapKit.framework/Versions/A/MapKit
    0x7fff2cc52000 -     0x7fff2cc59fff  com.apple.MediaAccessibility (1.0 - 114) <D72C593A-AC32-3419-AB86-6B07217EBFD5> /System/Library/Frameworks/MediaAccessibility.framework/Versions/A/MediaAccessibility
    0x7fff2cc5a000 -     0x7fff2cc6eff7  com.apple.MediaLibrary (1.4.0 - 751) <A53903E5-3445-309F-A9B9-31282EB21729> /System/Library/Frameworks/MediaLibrary.framework/Versions/A/MediaLibrary
    0x7fff2ccfc000 -     0x7fff2d317fff  com.apple.MediaToolbox (1.0 - 2270.12.1) <40802884-1347-3CDC-AAD3-D046463A528E> /System/Library/Frameworks/MediaToolbox.framework/Versions/A/MediaToolbox
    0x7fff2d319000 -     0x7fff2d398fff  com.apple.Metal (124.7 - 124.7) <F161C177-80B4-3674-8147-04343702CF08> /System/Library/Frameworks/Metal.framework/Versions/A/Metal
    0x7fff2d3b5000 -     0x7fff2d3cafff  com.apple.MetalPerformanceShaders.MPSCore (1.0 - 1) <D4BCBA84-AD1B-33DC-99F3-16F9E5E50906> /System/Library/Frameworks/MetalPerformanceShaders.framework/Frameworks/MPSCore.framework/Versions/A/MPSCore
    0x7fff2d3cb000 -     0x7fff2d436fef  com.apple.MetalPerformanceShaders.MPSImage (1.0 - 1) <E504EC97-FAD7-36DC-B151-6F89AB911E3A> /System/Library/Frameworks/MetalPerformanceShaders.framework/Frameworks/MPSImage.framework/Versions/A/MPSImage
    0x7fff2d437000 -     0x7fff2d45afff  com.apple.MetalPerformanceShaders.MPSMatrix (1.0 - 1) <A5B6F6FC-A19A-32C0-A999-98B6688760C7> /System/Library/Frameworks/MetalPerformanceShaders.framework/Frameworks/MPSMatrix.framework/Versions/A/MPSMatrix
    0x7fff2d45b000 -     0x7fff2d4dbff7  com.apple.MetalPerformanceShaders.MPSNeuralNetwork (1.0 - 1) <D0D8F13F-ACD4-3B61-BE54-121CCB05ECF4> /System/Library/Frameworks/MetalPerformanceShaders.framework/Frameworks/MPSNeuralNetwork.framework/Versions/A/MPSNeuralNetwork
    0x7fff2d4dc000 -     0x7fff2d4dcff7  com.apple.MetalPerformanceShaders.MetalPerformanceShaders (1.0 - 1) <2E8723FC-AA53-3596-B6A4-220A378B7A5A> /System/Library/Frameworks/MetalPerformanceShaders.framework/Versions/A/MetalPerformanceShaders
    0x7fff2e4e5000 -     0x7fff2e4f1ffb  com.apple.NetFS (6.0 - 4.0) <81B22AE7-7094-30F2-BF41-84CA05EDB95B> /System/Library/Frameworks/NetFS.framework/Versions/A/NetFS
    0x7fff2e671000 -     0x7fff2e6a7ffb  com.apple.notificationcenter (1.0 - 639.10) <9333E690-C463-398B-9529-6E9ABCE63DC1> /System/Library/Frameworks/NotificationCenter.framework/Versions/A/NotificationCenter
    0x7fff312e2000 -     0x7fff3132fffb  com.apple.opencl (2.8.12 - 2.8.12) <7F9BF7F0-AFB2-349A-BF9B-2DE5288380C4> /System/Library/Frameworks/OpenCL.framework/Versions/A/OpenCL
    0x7fff31330000 -     0x7fff3134cffb  com.apple.CFOpenDirectory (10.13 - 207) <A229B355-337B-33F4-AAA8-C751BEF0B718> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/Frameworks/CFOpenDirectory.framework/Versions/A/CFOpenDirectory
    0x7fff3134d000 -     0x7fff31358fff  com.apple.OpenDirectory (10.13 - 207) <D8AA4C58-149E-3504-88CD-F5B59F882C25> /System/Library/Frameworks/OpenDirectory.framework/Versions/A/OpenDirectory
    0x7fff324d7000 -     0x7fff324d9fff  libCVMSPluginSupport.dylib (16.4.2) <A967BC8B-ABB3-393F-BF34-BD32B45831F7> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCVMSPluginSupport.dylib
    0x7fff324da000 -     0x7fff324deff7  libCoreFSCache.dylib (162.4) <B325B709-0C81-357A-B9F1-6E0027B64F9B> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreFSCache.dylib
    0x7fff324df000 -     0x7fff324e3fff  libCoreVMClient.dylib (162.4) <B129DB84-39BA-34E4-9FB7-20A020A1BB86> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libCoreVMClient.dylib
    0x7fff324e4000 -     0x7fff324ecfff  libGFXShared.dylib (16.4.2) <07F1D947-F79B-3608-9080-E4DBFE13AF1D> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGFXShared.dylib
    0x7fff324ed000 -     0x7fff324f8fff  libGL.dylib (16.4.2) <97D6871A-BAF1-33DD-9ED7-BE7BB437F378> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGL.dylib
    0x7fff324f9000 -     0x7fff32534fe7  libGLImage.dylib (16.4.2) <3E2802DF-4998-31DB-B3A2-65720DE919A5> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLImage.dylib
    0x7fff326a2000 -     0x7fff326e0ffb  libGLU.dylib (16.4.2) <ECABCFAB-E400-3667-8EE1-586C07E0E214> /System/Library/Frameworks/OpenGL.framework/Versions/A/Libraries/libGLU.dylib
    0x7fff33058000 -     0x7fff33066ffb  com.apple.opengl (16.4.2 - 16.4.2) <C8C31EF5-8DB4-336F-A87C-5D520C7EFDC5> /System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
    0x7fff332c5000 -     0x7fff332edff3  com.apple.frameworks.preferencepanes (16.0 - 16.0) <C6CBE836-253E-3761-980E-E58BF741E458> /System/Library/Frameworks/PreferencePanes.framework/Versions/A/PreferencePanes
    0x7fff333de000 -     0x7fff3352afff  com.apple.QTKit (7.7.3 - 3014) <CF2818E6-E215-3C24-A2AC-1B5F8AD7E398> /System/Library/Frameworks/QTKit.framework/Versions/A/QTKit
    0x7fff3352b000 -     0x7fff33790ff7  com.apple.imageKit (3.0 - 1040) <5A1261EA-76C5-386C-9F46-06EE804E2234> /System/Library/Frameworks/Quartz.framework/Versions/A/Frameworks/ImageKit.framework/Versions/A/ImageKit
    0x7fff33791000 -     0x7fff3387dff3  com.apple.PDFKit (1.0 - 677.4) <265C1FB2-4E62-3498-A86B-02C0F790F1ED> /System/Library/Frameworks/Quartz.framework/Versions/A/Frameworks/PDFKit.framework/Versions/A/PDFKit
    0x7fff3387e000 -     0x7fff33d90ff7  com.apple.QuartzComposer (5.1 - 364) <D99600D5-C066-3D49-8641-369A69300D16> /System/Library/Frameworks/Quartz.framework/Versions/A/Frameworks/QuartzComposer.framework/Versions/A/QuartzComposer
    0x7fff33d91000 -     0x7fff33db4fff  com.apple.quartzfilters (1.10.0 - 1.10.0) <8CBE10F3-828F-373C-B92A-A5289CBCF77A> /System/Library/Frameworks/Quartz.framework/Versions/A/Frameworks/QuartzFilters.framework/Versions/A/QuartzFilters
    0x7fff33db5000 -     0x7fff33eacff7  com.apple.QuickLookUIFramework (5.0 - 743.7) <B0352E2A-C582-3C86-A2ED-A30D4A3B785D> /System/Library/Frameworks/Quartz.framework/Versions/A/Frameworks/QuickLookUI.framework/Versions/A/QuickLookUI
    0x7fff33ead000 -     0x7fff33eadfff  com.apple.quartzframework (1.5 - 21) <3F3BE91F-A032-3E92-ACEA-E2DFEC999E93> /System/Library/Frameworks/Quartz.framework/Versions/A/Quartz
    0x7fff33eae000 -     0x7fff340f6fff  com.apple.QuartzCore (1.11 - 584.8.102) <4479AF33-E6EA-3037-A2C1-3C6F12B1260A> /System/Library/Frameworks/QuartzCore.framework/Versions/A/QuartzCore
    0x7fff340f7000 -     0x7fff3414eff7  com.apple.QuickLookFramework (5.0 - 743.7) <0CB6717C-CCE2-3591-A31E-A8EF01DD4FC8> /System/Library/Frameworks/QuickLook.framework/Versions/A/QuickLook
    0x7fff34313000 -     0x7fff34329fff  com.apple.SafariServices.framework (13604 - 13604.5.6) <D71DE52A-9B30-381B-82CA-57473E7A0E7E> /System/Library/Frameworks/SafariServices.framework/Versions/A/SafariServices
    0x7fff3492f000 -     0x7fff34c46ff7  com.apple.security (7.0 - 58286.41.2) <EB297497-884A-362F-B566-73A14A2F25FE> /System/Library/Frameworks/Security.framework/Versions/A/Security
    0x7fff34c47000 -     0x7fff34cd1ff7  com.apple.securityfoundation (6.0 - 55185.30.4) <65144003-B9E2-3DE3-8923-F2BAA68BBF4E> /System/Library/Frameworks/SecurityFoundation.framework/Versions/A/SecurityFoundation
    0x7fff34cd2000 -     0x7fff34d02ff3  com.apple.securityinterface (10.0 - 55109.30.2) <8BFD22AB-32C9-3AD1-9CB4-39EA2D18E865> /System/Library/Frameworks/SecurityInterface.framework/Versions/A/SecurityInterface
    0x7fff34d03000 -     0x7fff34d06ffb  com.apple.xpc.ServiceManagement (1.0 - 1) <B11C3C64-6FE7-3A78-B583-D790B7CCE95A> /System/Library/Frameworks/ServiceManagement.framework/Versions/A/ServiceManagement
    0x7fff34d07000 -     0x7fff34d51ff7  com.apple.sociald.Social (???) <BBA0FD17-179A-3078-8C06-1FBEEE4D13E1> /System/Library/Frameworks/Social.framework/Versions/A/Social
    0x7fff34f74000 -     0x7fff350aaffb  com.apple.syncservices (8.1 - 727) <2268DC1C-BC6F-3BE3-A8A2-7CD3A4FBD175> /System/Library/Frameworks/SyncServices.framework/Versions/A/SyncServices
    0x7fff350ab000 -     0x7fff3511bff3  com.apple.SystemConfiguration (1.17 - 1.17) <3C6B2BB9-43AB-39AD-8027-38E30A8A4186> /System/Library/Frameworks/SystemConfiguration.framework/Versions/A/SystemConfiguration
    0x7fff352d0000 -     0x7fff35649ff7  com.apple.VideoToolbox (1.0 - 2270.12.1) <77848D39-10DA-3AA3-88F4-2898F42FCA5A> /System/Library/Frameworks/VideoToolbox.framework/Versions/A/VideoToolbox
    0x7fff358f1000 -     0x7fff35e09fe7  libwebrtc.dylib (7604.5.6) <7C956A2C-8F14-310F-AE27-CCDA9F64AAE5> /System/Library/Frameworks/WebKit.framework/Versions/A/Frameworks/WebCore.framework/Versions/A/Frameworks/libwebrtc.dylib
    0x7fff35e0a000 -     0x7fff372f1ff7  com.apple.WebCore (13604 - 13604.5.6) <558072BF-6134-3C19-B32E-4889C5C06C5C> /System/Library/Frameworks/WebKit.framework/Versions/A/Frameworks/WebCore.framework/Versions/A/WebCore
    0x7fff372f2000 -     0x7fff374e5ff3  com.apple.WebKitLegacy (13604 - 13604.5.6) <E0177202-32F5-3D0F-9F80-85F41AFF049E> /System/Library/Frameworks/WebKit.framework/Versions/A/Frameworks/WebKitLegacy.framework/Versions/A/WebKitLegacy
    0x7fff374e6000 -     0x7fff378deff7  com.apple.WebKit (13604 - 13604.5.6) <5AB44A86-7C8C-3F24-A76B-80618F164763> /System/Library/Frameworks/WebKit.framework/Versions/A/WebKit
    0x7fff37926000 -     0x7fff37976ff7  com.apple.AOSAccounts (1.3.1 - 89.0.0.8) <00F3AE54-0952-303A-AF6B-3E92E3247849> /System/Library/PrivateFrameworks/AOSAccounts.framework/Versions/A/AOSAccounts
    0x7fff37979000 -     0x7fff37c3aff3  com.apple.AOSKit (1.07 - 264) <BFC6E041-CA51-3236-B926-431463057111> /System/Library/PrivateFrameworks/AOSKit.framework/Versions/A/AOSKit
    0x7fff37c3b000 -     0x7fff37c3bffb  com.apple.AOSMigrate (1.0 - 1) <933982ED-580B-39E1-A29C-BC7873917D9B> /System/Library/PrivateFrameworks/AOSMigrate.framework/Versions/A/AOSMigrate
    0x7fff37d4e000 -     0x7fff37ddaff7  com.apple.APFS (1.0 - 1) <9D67579C-7FB4-3AD9-AB4F-9174A552EB37> /System/Library/PrivateFrameworks/APFS.framework/Versions/A/APFS
    0x7fff383b8000 -     0x7fff38458fff  com.apple.accounts.AccountsDaemon (113 - 113) <0DC6C422-33A4-3B34-A6A9-A4378533DDE4> /System/Library/PrivateFrameworks/AccountsDaemon.framework/Versions/A/AccountsDaemon
    0x7fff38459000 -     0x7fff3848fffb  com.apple.framework.accountsui (1.0 - 62) <E5C44656-94CB-37B7-A490-00E929C0CEDD> /System/Library/PrivateFrameworks/AccountsUI.framework/Versions/A/AccountsUI
    0x7fff389a1000 -     0x7fff389bcff3  com.apple.AppContainer (4.0 - 360.30.1) <65A03D4F-1DD8-35CE-B228-BD470CD35689> /System/Library/PrivateFrameworks/AppContainer.framework/Versions/A/AppContainer
    0x7fff389bd000 -     0x7fff389cafff  com.apple.AppSandbox (4.0 - 360.30.1) <F333AEE2-379B-3C2E-BDB9-DFA63478F2E5> /System/Library/PrivateFrameworks/AppSandbox.framework/Versions/A/AppSandbox
    0x7fff389ed000 -     0x7fff38a15fff  com.apple.framework.Apple80211 (13.0 - 1345) <D1B8340A-41FB-35C3-88F0-3DDD9A2A1999> /System/Library/PrivateFrameworks/Apple80211.framework/Versions/A/Apple80211
    0x7fff38a17000 -     0x7fff38a26ff7  com.apple.AppleFSCompression (96.30.2 - 1.0) <CFA90544-68EB-3CCE-A8F7-1B03974F3B85> /System/Library/PrivateFrameworks/AppleFSCompression.framework/Versions/A/AppleFSCompression
    0x7fff38b1d000 -     0x7fff38b28ff7  com.apple.AppleIDAuthSupport (1.0 - 1) <6BEAA0D3-4B1B-3B59-B55D-7231BFBAD179> /System/Library/PrivateFrameworks/AppleIDAuthSupport.framework/Versions/A/AppleIDAuthSupport
    0x7fff38b29000 -     0x7fff38b61ff3  com.apple.AppleIDSSOAuthentication (1.0 - 1) <AADE96AD-FE15-3B9B-A5C3-3404DDAD6C32> /System/Library/PrivateFrameworks/AppleIDSSOAuthentication.framework/Versions/A/AppleIDSSOAuthentication
    0x7fff38b62000 -     0x7fff38baaff3  com.apple.AppleJPEG (1.0 - 1) <8BBD5180-5BF9-33DB-8B91-974B1D0AECFB> /System/Library/PrivateFrameworks/AppleJPEG.framework/Versions/A/AppleJPEG
    0x7fff38bab000 -     0x7fff38bbcffb  com.apple.AppleLDAP (10.13 - 46) <F7F893F5-A98A-39A1-9F28-61D7355B9280> /System/Library/PrivateFrameworks/AppleLDAP.framework/Versions/A/AppleLDAP
    0x7fff38bbd000 -     0x7fff38bdbff3  com.apple.aps.framework (4.0 - 4.0) <932274E4-DCD5-30D5-9AC7-58CEF4571EB3> /System/Library/PrivateFrameworks/ApplePushService.framework/Versions/A/ApplePushService
    0x7fff38bdc000 -     0x7fff38be4ff3  com.apple.AppleSRP (5.0 - 1) <4CEC34CF-63E3-3023-B61B-F8D133698534> /System/Library/PrivateFrameworks/AppleSRP.framework/Versions/A/AppleSRP
    0x7fff38be5000 -     0x7fff38c0dfff  com.apple.applesauce (1.0 - ???) <32FF4851-2F68-35BA-835F-91856A20C323> /System/Library/PrivateFrameworks/AppleSauce.framework/Versions/A/AppleSauce
    0x7fff38cd6000 -     0x7fff38cd9ff3  com.apple.AppleSystemInfo (3.1.5 - 3.1.5) <A6795AC1-D528-3A2F-9C43-14EBFC57B7D6> /System/Library/PrivateFrameworks/AppleSystemInfo.framework/Versions/A/AppleSystemInfo
    0x7fff38cda000 -     0x7fff38d2aff7  com.apple.AppleVAFramework (5.0.39 - 5.0.39) <4B830521-2A54-3116-8103-127E631FEDC8> /System/Library/PrivateFrameworks/AppleVA.framework/Versions/A/AppleVA
    0x7fff39038000 -     0x7fff392ceff3  com.apple.AuthKit (1.0 - 1) <A5675E95-FC6E-38BF-8345-9AF987884CEC> /System/Library/PrivateFrameworks/AuthKit.framework/Versions/A/AuthKit
    0x7fff393f9000 -     0x7fff39400ff7  com.apple.coreservices.BackgroundTaskManagement (1.0 - 57.1) <47B6301F-D908-3811-BB9E-DA16D9B29A34> /System/Library/PrivateFrameworks/BackgroundTaskManagement.framework/Versions/A/BackgroundTaskManagement
    0x7fff39401000 -     0x7fff39488ff7  com.apple.backup.framework (1.9.2 - 1.9.2) <40C6CFA8-4013-3AFB-97CE-8A09388FA343> /System/Library/PrivateFrameworks/Backup.framework/Versions/A/Backup
    0x7fff394fe000 -     0x7fff3953afff  com.apple.bom (14.0 - 194.2) <ED2FFB9D-4DA4-322B-89C0-102D790649CD> /System/Library/PrivateFrameworks/Bom.framework/Versions/A/Bom
    0x7fff39565000 -     0x7fff39565fff  com.apple.SafariDAVNotifier (1.1.1 - 1) <19DA25EA-2D46-3A67-B248-FF4582A7F070> /System/Library/PrivateFrameworks/BookmarkDAV.framework/Versions/A/Frameworks/SafariDAVNotifier.framework/Versions/A/SafariDAVNotifier
    0x7fff3978e000 -     0x7fff397e1fff  com.apple.CalDAV (8.0 - 257) <144FE212-0CD0-344E-993B-5DA2BFC5BC48> /System/Library/PrivateFrameworks/CalDAV.framework/Versions/A/CalDAV
    0x7fff39b10000 -     0x7fff39b57ffb  com.apple.CalendarAgent (8.0 - 399.2.2) <896399C9-8ADC-3000-A516-CCE032082E66> /System/Library/PrivateFrameworks/CalendarAgent.framework/Versions/A/CalendarAgent
    0x7fff39b58000 -     0x7fff39b85fff  com.apple.CalendarAgentLink (8.0 - 250) <55B60B04-2EAA-3F22-88EF-1602129D5294> /System/Library/PrivateFrameworks/CalendarAgentLink.framework/Versions/A/CalendarAgentLink
    0x7fff39b9e000 -     0x7fff39c06fff  com.apple.CalendarFoundation (8.0 - 552.2.3) <10489460-9476-3E7C-A3CE-8B85E37733DA> /System/Library/PrivateFrameworks/CalendarFoundation.framework/Versions/A/CalendarFoundation
    0x7fff39c07000 -     0x7fff39c2eff3  com.apple.CalendarNotification (1.0 - ???) <0D60B3B2-AC31-3CC3-AE05-66B3926E9801> /System/Library/PrivateFrameworks/CalendarNotification.framework/Versions/A/CalendarNotification
    0x7fff39c2f000 -     0x7fff39f61fff  com.apple.CalendarPersistence (8.0 - 470.2.2) <8DD2E437-574F-3DA0-8D0C-993975381438> /System/Library/PrivateFrameworks/CalendarPersistence.framework/Versions/A/CalendarPersistence
    0x7fff3a0cb000 -     0x7fff3a109ff3  com.apple.CalendarUIKit (1.0 - 233) <85359A2C-9447-3286-8B65-6466D7B2D56B> /System/Library/PrivateFrameworks/CalendarUIKit.framework/Versions/A/CalendarUIKit
    0x7fff3a24c000 -     0x7fff3a29bff3  com.apple.ChunkingLibrary (188 - 188) <84E024A0-5F95-3A30-A60B-102343AE46B7> /System/Library/PrivateFrameworks/ChunkingLibrary.framework/Versions/A/ChunkingLibrary
    0x7fff3a2a2000 -     0x7fff3a332ff7  com.apple.CloudDocs (1.0 - 572.2) <7549FED5-7C41-3E3E-ABB4-34DCB312F8A2> /System/Library/PrivateFrameworks/CloudDocs.framework/Versions/A/CloudDocs
    0x7fff3ad75000 -     0x7fff3ad87ff3  com.apple.CloudPhotoServicesConfiguration (3.0 - 3251.12.190) <31A0CC51-4131-32A2-9CAD-41B9BB55C4DA> /System/Library/PrivateFrameworks/CloudPhotoServices.framework/Versions/A/Frameworks/CloudPhotoServicesConfiguration.framework/Versions/A/CloudPhotoServicesConfiguration
    0x7fff3ad88000 -     0x7fff3ad88ff7  com.apple.CloudPhotosConfigurationXPC (3.0 - 3251.12.190) <9374AEBB-B9D2-3B0C-8E6A-A1CCFA0CEE7A> /System/Library/PrivateFrameworks/CloudPhotoServices.framework/Versions/A/Frameworks/CloudPhotosConfigurationXPC.framework/Versions/A/CloudPhotosConfigurationXPC
    0x7fff3ad89000 -     0x7fff3ada2fff  com.apple.CloudServices (1.0 - 297.20.1) <04107C8D-5DCD-33EA-850E-E7A617D17E8C> /System/Library/PrivateFrameworks/CloudServices.framework/Versions/A/CloudServices
    0x7fff3ae0c000 -     0x7fff3ae17ffb  com.apple.CommerceCore (1.0 - 654.8) <8774AB32-E5A1-3257-A631-F0C96CB56824> /System/Library/PrivateFrameworks/CommerceKit.framework/Versions/A/Frameworks/CommerceCore.framework/Versions/A/CommerceCore
    0x7fff3ae18000 -     0x7fff3ae21ff3  com.apple.CommonAuth (4.0 - 2.0) <11B2D184-36B8-3624-B1AD-7B6037D76160> /System/Library/PrivateFrameworks/CommonAuth.framework/Versions/A/CommonAuth
    0x7fff3ae36000 -     0x7fff3ae4bff7  com.apple.commonutilities (8.0 - 900) <518FBC2C-0AF3-39DF-B454-526311796A49> /System/Library/PrivateFrameworks/CommonUtilities.framework/Versions/A/CommonUtilities
    0x7fff3ae4c000 -     0x7fff3ae50ff7  com.apple.communicationsfilter (10.0 - 1000) <ABDFD1A2-ABBB-380E-A390-8316466BB597> /System/Library/PrivateFrameworks/CommunicationsFilter.framework/Versions/A/CommunicationsFilter
    0x7fff3af68000 -     0x7fff3af9efff  com.apple.contacts.ContactsAutocomplete (1.0 - 90) <5C5058A3-4436-3AE9-910C-0A35CF473645> /System/Library/PrivateFrameworks/ContactsAutocomplete.framework/Versions/A/ContactsAutocomplete
    0x7fff3afb2000 -     0x7fff3afc9fff  com.apple.contacts.donation (1.0 - 1) <FC3D227F-12EC-3FBF-A6B2-A0BA2F42A8DC> /System/Library/PrivateFrameworks/ContactsDonation.framework/Versions/A/ContactsDonation
    0x7fff3afcf000 -     0x7fff3b02ffff  com.apple.AddressBook.ContactsFoundation (8.0 - ???) <9B39DF3E-5B7F-3467-AC17-23258F76A376> /System/Library/PrivateFrameworks/ContactsFoundation.framework/Versions/A/ContactsFoundation
    0x7fff3b030000 -     0x7fff3b052fff  com.apple.contacts.ContactsPersistence (1.0 - 2330.3) <EE8079D4-E1E2-3EFF-A7E1-E97BD81F6C6F> /System/Library/PrivateFrameworks/ContactsPersistence.framework/Versions/A/ContactsPersistence
    0x7fff3b053000 -     0x7fff3b0a8ffb  com.apple.Contacts.ContactsUICore (1.0 - 999.9.9) <01BFDE8F-019C-3E59-AE80-CA410F15C439> /System/Library/PrivateFrameworks/ContactsUICore.framework/Versions/A/ContactsUICore
    0x7fff3b0b2000 -     0x7fff3b492fff  com.apple.CoreAUC (249.0.0 - 249.0.0) <7F406271-A215-3C75-BDAA-B9611BA2822E> /System/Library/PrivateFrameworks/CoreAUC.framework/Versions/A/CoreAUC
    0x7fff3b493000 -     0x7fff3b4c3ff7  com.apple.CoreAVCHD (5.9.0 - 5900.4.1) <D0879D8F-488A-3D46-BC50-AD7B0642C0E8> /System/Library/PrivateFrameworks/CoreAVCHD.framework/Versions/A/CoreAVCHD
    0x7fff3b4da000 -     0x7fff3b507fff  com.apple.CoreAnalytics.CoreAnalytics (1.0 - 1) <2D7C7B8C-C9C8-3F32-B43E-E792CD88A77D> /System/Library/PrivateFrameworks/CoreAnalytics.framework/Versions/A/CoreAnalytics
    0x7fff3b580000 -     0x7fff3b596ffb  com.apple.CoreCDP-OSX (1.0 - 1) <F2101EC5-0B66-391F-BA19-7244186B1C5C> /System/Library/PrivateFrameworks/CoreCDP.framework/Versions/A/CoreCDP
    0x7fff3b649000 -     0x7fff3b6bdff7  com.apple.coredav (1.0.1 - 332.2.2) <15F1309E-2D9D-3D5C-B86B-96AF40BA1FAF> /System/Library/PrivateFrameworks/CoreDAV.framework/Versions/A/CoreDAV
    0x7fff3b6be000 -     0x7fff3b6c7ff7  com.apple.frameworks.CoreDaemon (1.3 - 1.3) <0A87A91C-A2CF-3BB1-9038-7F610111BC30> /System/Library/PrivateFrameworks/CoreDaemon.framework/Versions/B/CoreDaemon
    0x7fff3b6c8000 -     0x7fff3b7f4ff7  com.apple.CoreDuet (1.0 - 1) <A42B66EA-4E96-36DF-AF09-3AC6D6F7053E> /System/Library/PrivateFrameworks/CoreDuet.framework/Versions/A/CoreDuet
    0x7fff3b7f5000 -     0x7fff3b813ff7  com.apple.coreduetcontext (1.0 - 1) <FA27BD07-8FB3-37A6-8C45-D1F41AA57E80> /System/Library/PrivateFrameworks/CoreDuetContext.framework/Versions/A/CoreDuetContext
    0x7fff3b814000 -     0x7fff3b826fff  com.apple.CoreDuetDaemonProtocol (1.0 - 1) <3E2ECA73-6CEE-3B31-B9BD-C2C2280DAFE9> /System/Library/PrivateFrameworks/CoreDuetDaemonProtocol.framework/Versions/A/CoreDuetDaemonProtocol
    0x7fff3b829000 -     0x7fff3b82bfff  com.apple.CoreDuetDebugLogging (1.0 - 1) <34AF678F-6F75-325E-86E3-55F23889BB27> /System/Library/PrivateFrameworks/CoreDuetDebugLogging.framework/Versions/A/CoreDuetDebugLogging
    0x7fff3b82e000 -     0x7fff3b83eff7  com.apple.CoreEmoji (1.0 - 69.3) <A4357F5C-0C38-3A61-B456-D7321EB2CEE5> /System/Library/PrivateFrameworks/CoreEmoji.framework/Versions/A/CoreEmoji
    0x7fff3b83f000 -     0x7fff3b852ff3  com.apple.CoreFollowUp-OSX (1.0 - 1) <54F4027A-77AF-3A72-AB95-7C330C08E9B1> /System/Library/PrivateFrameworks/CoreFollowUp.framework/Versions/A/CoreFollowUp
    0x7fff3c0f9000 -     0x7fff3c10fff7  com.apple.CoreMediaAuthoring (2.2 - 956) <A5003E93-68B3-3A0E-8252-454CC4CFEC4D> /System/Library/PrivateFrameworks/CoreMediaAuthoring.framework/Versions/A/CoreMediaAuthoring
    0x7fff3c251000 -     0x7fff3c2aeff7  com.apple.CoreNLP (1.0 - 130.4.1) <1F329E38-E7D0-3455-981B-48F434C9E8CC> /System/Library/PrivateFrameworks/CoreNLP.framework/Versions/A/CoreNLP
    0x7fff3c437000 -     0x7fff3c43cff7  com.apple.CoreOptimization (1.0 - 1) <785B622B-8F7D-3B4D-83AF-EB98CB79FFF6> /System/Library/PrivateFrameworks/CoreOptimization.framework/Versions/A/CoreOptimization
    0x7fff3c43d000 -     0x7fff3c4e2ff7  com.apple.CorePDF (4.0 - 414) <2F0447DB-826B-3CEC-A98C-B8A8DDEA1B26> /System/Library/PrivateFrameworks/CorePDF.framework/Versions/A/CorePDF
    0x7fff3c4e3000 -     0x7fff3c581ff7  com.apple.siri.parsec.CoreParsec (1.0 - 184.2.12) <D4987453-A6D7-3CD1-83B9-6861606391B5> /System/Library/PrivateFrameworks/CoreParsec.framework/Versions/A/CoreParsec
    0x7fff3c582000 -     0x7fff3c58aff7  com.apple.CorePhoneNumbers (1.0 - 1) <B782FEA6-ACAD-34C6-811F-635F319C3B71> /System/Library/PrivateFrameworks/CorePhoneNumbers.framework/Versions/A/CorePhoneNumbers
    0x7fff3c58b000 -     0x7fff3c5e7fff  com.apple.CorePrediction (1.0 - 1) <B1E687BA-4E62-321B-8C87-D37C540A5A68> /System/Library/PrivateFrameworks/CorePrediction.framework/Versions/A/CorePrediction
    0x7fff3c610000 -     0x7fff3c61affb  com.apple.corerecents (1.0 - 1) <958F2F6C-E22D-3406-8DBE-4EE7FE3681A3> /System/Library/PrivateFrameworks/CoreRecents.framework/Versions/A/CoreRecents
    0x7fff3c7e0000 -     0x7fff3c84bffb  com.apple.CoreSuggestions (1.0 - 680.1.5) <1BEDE63D-4067-31F3-B6F5-994D18727847> /System/Library/PrivateFrameworks/CoreSuggestions.framework/Versions/A/CoreSuggestions
    0x7fff3ca94000 -     0x7fff3cb25fff  com.apple.CoreSymbolication (63075) <2458D96C-1C31-34F8-93F0-73DB0042CB30> /System/Library/PrivateFrameworks/CoreSymbolication.framework/Versions/A/CoreSymbolication
    0x7fff3cba8000 -     0x7fff3ccdcfff  com.apple.coreui (2.1 - 492.2) <FE0B32BC-958D-3445-866E-A3905626396D> /System/Library/PrivateFrameworks/CoreUI.framework/Versions/A/CoreUI
    0x7fff3ccdd000 -     0x7fff3cde8fff  com.apple.CoreUtils (5.3 - 530.60) <D74B16CC-E2C5-3D8A-B9F4-6D1DB23092B1> /System/Library/PrivateFrameworks/CoreUtils.framework/Versions/A/CoreUtils
    0x7fff3ce3d000 -     0x7fff3cea1fff  com.apple.framework.CoreWiFi (13.0 - 1339) <7EBA8BA5-C650-3067-A805-8DD08FC816FD> /System/Library/PrivateFrameworks/CoreWiFi.framework/Versions/A/CoreWiFi
    0x7fff3cea2000 -     0x7fff3ceb1ff7  com.apple.CrashReporterSupport (10.13 - 1) <6B5B2B78-15D2-363F-BE34-8F9226E79E24> /System/Library/PrivateFrameworks/CrashReporterSupport.framework/Versions/A/CrashReporterSupport
    0x7fff3cf2e000 -     0x7fff3cf3dff7  com.apple.framework.DFRFoundation (1.0 - 191.1) <87B83349-C317-3E07-894C-5BC5C20AE08B> /System/Library/PrivateFrameworks/DFRFoundation.framework/Versions/A/DFRFoundation
    0x7fff3cf40000 -     0x7fff3cf44ffb  com.apple.DSExternalDisplay (3.1 - 380) <BEC07C7C-F3AC-3CF3-B13E-3EBFD6224C0D> /System/Library/PrivateFrameworks/DSExternalDisplay.framework/Versions/A/DSExternalDisplay
    0x7fff3cf89000 -     0x7fff3cfc9ffb  com.apple.datadetectors (5.0 - 376) <612C677D-BB71-35D2-BBF3-EA9EC51ADF12> /System/Library/PrivateFrameworks/DataDetectors.framework/Versions/A/DataDetectors
    0x7fff3cfca000 -     0x7fff3d040fff  com.apple.datadetectorscore (7.0 - 590.3) <792593BC-21C7-3E0C-929E-067CD725FCBD> /System/Library/PrivateFrameworks/DataDetectorsCore.framework/Versions/A/DataDetectorsCore
    0x7fff3d08e000 -     0x7fff3d0ceff7  com.apple.DebugSymbols (141 - 141) <99562E28-0E56-3F6F-93A1-EF997A5E1F87> /System/Library/PrivateFrameworks/DebugSymbols.framework/Versions/A/DebugSymbols
    0x7fff3d0cf000 -     0x7fff3d1fefff  com.apple.desktopservices (1.12.2 - 1.12.2) <8E068AF7-FB72-38E2-948E-6C13F359C69D> /System/Library/PrivateFrameworks/DesktopServicesPriv.framework/Versions/A/DesktopServicesPriv
    0x7fff3d25e000 -     0x7fff3d25fff7  com.apple.diagnosticlogcollection (10.0 - 1000) <DFD57FDB-CB78-3FB0-B49B-4586F9E0DFC1> /System/Library/PrivateFrameworks/DiagnosticLogCollection.framework/Versions/A/DiagnosticLogCollection
    0x7fff3d2f1000 -     0x7fff3d3c3ff7  com.apple.DiskImagesFramework (480.30.2 - 480.30.2) <4A7D01B5-0A30-3425-AC35-D349EC10454F> /System/Library/PrivateFrameworks/DiskImages.framework/Versions/A/DiskImages
    0x7fff3d3c4000 -     0x7fff3d43bff7  com.apple.DiskManagement (11.2 - 1339) <0B17E79A-0BF6-3FD8-8AD0-A8498545030F> /System/Library/PrivateFrameworks/DiskManagement.framework/Versions/A/DiskManagement
    0x7fff3d43c000 -     0x7fff3d447ff7  com.apple.DisplayServicesFW (3.1 - 380) <FB1FBB24-7FD5-3B57-8D3C-39F953F8A9BB> /System/Library/PrivateFrameworks/DisplayServices.framework/Versions/A/DisplayServices
    0x7fff3d4cf000 -     0x7fff3d4d1ff7  com.apple.EFILogin (2.0 - 2) <5B6E5A73-C2B2-3188-ADE9-C29CE29BB225> /System/Library/PrivateFrameworks/EFILogin.framework/Versions/A/EFILogin
    0x7fff3d4e3000 -     0x7fff3d4edfff  com.apple.Email (11.0 - 3445.5.20) <934D28B3-DEEF-3688-8B3D-E985AD6A6AC8> /System/Library/PrivateFrameworks/Email.framework/Versions/A/Email
    0x7fff3d4ee000 -     0x7fff3d4f5ff7  com.apple.EmailAddressing (11.0 - 3445.5.20) <ED77EFA8-8293-333D-BFF4-3F8709F3B4C8> /System/Library/PrivateFrameworks/EmailAddressing.framework/Versions/A/EmailAddressing
    0x7fff3d4f6000 -     0x7fff3d4fcff3  com.apple.EmailCore (11.0 - 3445.5.20) <CF767899-F439-3C97-933E-BFBCB94C40CF> /System/Library/PrivateFrameworks/EmailCore.framework/Versions/A/EmailCore
    0x7fff3dc1d000 -     0x7fff3dc35fff  com.apple.Engram (1.0 - 1) <93DC8AE3-B3F7-326E-93DB-C85A2BFD3C3F> /System/Library/PrivateFrameworks/Engram.framework/Versions/A/Engram
    0x7fff3de0c000 -     0x7fff3de5eff7  com.apple.ExchangeWebServices (8.0 - 287.1.1) <02290D8C-95D9-3E07-8F00-A6CAB345CC38> /System/Library/PrivateFrameworks/ExchangeWebServices.framework/Versions/A/ExchangeWebServices
    0x7fff3de5f000 -     0x7fff3de7eff7  com.apple.icloud.FMCore (1.0 - 1) <C5C31F82-F206-3164-BAF7-32D64DDED78B> /System/Library/PrivateFrameworks/FMCore.framework/Versions/A/FMCore
    0x7fff3de7f000 -     0x7fff3de95ff7  com.apple.icloud.FMCoreLite (1.0 - 1) <E35C902D-17B9-3D0E-AE7C-F640BFDF17B3> /System/Library/PrivateFrameworks/FMCoreLite.framework/Versions/A/FMCoreLite
    0x7fff3de96000 -     0x7fff3de9bff7  com.apple.icloud.FMCoreUI (1.0 - 1) <7B74CFDC-6868-301D-89DE-3071715F774B> /System/Library/PrivateFrameworks/FMCoreUI.framework/Versions/A/FMCoreUI
    0x7fff3de9c000 -     0x7fff3dec3ff3  com.apple.icloud.FMF (1.0 - 1) <63FD1CAC-566C-3262-A97B-B73C8713255D> /System/Library/PrivateFrameworks/FMF.framework/Versions/A/FMF
    0x7fff3dec4000 -     0x7fff3ded7ff3  com.apple.icloud.FMFUI (1.0 - 1) <2A0D68F9-206E-3F7F-8173-200A1962A48F> /System/Library/PrivateFrameworks/FMFUI.framework/Versions/A/FMFUI
    0x7fff3df73000 -     0x7fff3e3a1fff  com.apple.vision.FaceCore (3.3.2 - 3.3.2) <80C97AD7-D5C2-311A-B268-4AA60CAD6CED> /System/Library/PrivateFrameworks/FaceCore.framework/Versions/A/FaceCore
    0x7fff3e3ab000 -     0x7fff3e3cdfff  com.apple.framework.familycontrols (4.1 - 410) <E1B7D26E-5A27-3844-868C-3609621124FF> /System/Library/PrivateFrameworks/FamilyControls.framework/Versions/A/FamilyControls
    0x7fff3e3d4000 -     0x7fff3e45dff7  com.apple.FileProvider (124.1 - 124.1) <5BF434E7-7675-3877-B761-0281418BC633> /System/Library/PrivateFrameworks/FileProvider.framework/Versions/A/FileProvider
    0x7fff3e45e000 -     0x7fff3e46dff7  com.apple.icloud.FindMyDevice (1.0 - 1) <534BBB1D-A62C-3E8E-A31A-D6DB3FC6B357> /System/Library/PrivateFrameworks/FindMyDevice.framework/Versions/A/FindMyDevice
    0x7fff3e864000 -     0x7fff3e889ffb  com.apple.FlightUtilities (1.0 - 115.4) <E91E94FF-201C-3C60-BC70-4262A119EBA1> /System/Library/PrivateFrameworks/FlightUtilities.framework/Versions/A/FlightUtilities
    0x7fff41a11000 -     0x7fff41a36ff3  com.apple.GenerationalStorage (2.0 - 285) <07E7BC5F-8EF2-34FC-9EEC-B4E61EAAFA9A> /System/Library/PrivateFrameworks/GenerationalStorage.framework/Versions/A/GenerationalStorage
    0x7fff41a4e000 -     0x7fff42372fff  com.apple.GeoServices (1.0 - 1359.23.11.28.2) <4248D1A7-816A-361B-9B74-76074C599001> /System/Library/PrivateFrameworks/GeoServices.framework/Versions/A/GeoServices
    0x7fff423b5000 -     0x7fff423c4fff  com.apple.GraphVisualizer (1.0 - 5) <0A93C5DE-0D28-312E-8764-6B0FB805ED91> /System/Library/PrivateFrameworks/GraphVisualizer.framework/Versions/A/GraphVisualizer
    0x7fff4243c000 -     0x7fff424b0fff  com.apple.Heimdal (4.0 - 2.0) <ACC132E5-97F1-3B36-AD7B-4E6CC077E691> /System/Library/PrivateFrameworks/Heimdal.framework/Versions/A/Heimdal
    0x7fff4254a000 -     0x7fff4265cff3  com.apple.ids (10.0 - 1000) <F85463DF-6D86-33C6-B740-498B29D7ECC6> /System/Library/PrivateFrameworks/IDS.framework/Versions/A/IDS
    0x7fff4265d000 -     0x7fff4273bfff  com.apple.idsfoundation (10.0 - 1000) <BE207DF9-8609-3A80-9132-87FEB8E817B6> /System/Library/PrivateFrameworks/IDSFoundation.framework/Versions/A/IDSFoundation
    0x7fff4274a000 -     0x7fff427d3fff  com.apple.IMAP (11.0 - 3445.5.20) <546B4F30-ED0D-323D-96B4-6E4A22372C71> /System/Library/PrivateFrameworks/IMAP.framework/Versions/A/IMAP
    0x7fff42c42000 -     0x7fff42ca8ff3  com.apple.imfoundation (10.0 - 1000) <32799DD1-91A2-3093-8D8C-4F4988560FC2> /System/Library/PrivateFrameworks/IMFoundation.framework/Versions/A/IMFoundation
    0x7fff42d62000 -     0x7fff42d69ffb  com.apple.IOAccelerator (376.6 - 376.6) <A47129CC-F386-3C31-AD66-C19A70615A50> /System/Library/PrivateFrameworks/IOAccelerator.framework/Versions/A/IOAccelerator
    0x7fff42d6c000 -     0x7fff42d6cfff  com.apple.IOPlatformPluginFamily (1.0 - 1) <5CC6785B-7D50-3DD2-8DB1-0FA94F236852> /System/Library/PrivateFrameworks/IOPlatformPluginFamily.framework/Versions/A/IOPlatformPluginFamily
    0x7fff42d6d000 -     0x7fff42d84fff  com.apple.IOPresentment (1.0 - 32.1) <B95F06EA-9D5D-311D-9912-978AE42ECFCE> /System/Library/PrivateFrameworks/IOPresentment.framework/Versions/A/IOPresentment
    0x7fff4314e000 -     0x7fff43173fff  com.apple.IconServices (97.4 - 97.4) <C84A6CA5-8C50-3288-8572-CC3DABF5BBCE> /System/Library/PrivateFrameworks/IconServices.framework/Versions/A/IconServices
    0x7fff4317b000 -     0x7fff43180ffb  com.apple.incomingcallfilter (10.0 - 1000) <2530DF50-38FA-3272-B5CB-FB3587C147EE> /System/Library/PrivateFrameworks/IncomingCallFilter.framework/Versions/A/IncomingCallFilter
    0x7fff4327a000 -     0x7fff4327afff  com.apple.IntentsFoundation (1.0 - 1) <12D23AE9-B8E3-3A1B-B7B7-277AE5D97859> /System/Library/PrivateFrameworks/IntentsFoundation.framework/Versions/A/IntentsFoundation
    0x7fff4327d000 -     0x7fff43280ff3  com.apple.InternationalSupport (1.0 - 1) <5AB382FD-BF81-36A1-9565-61F1FD398ECA> /System/Library/PrivateFrameworks/InternationalSupport.framework/Versions/A/InternationalSupport
    0x7fff43281000 -     0x7fff43283fff  com.apple.InternationalTextSearch (1.0 - 1) <2FFBEFFD-C977-3D25-8C84-9574D14A45EC> /System/Library/PrivateFrameworks/InternationalTextSearch.framework/Versions/A/InternationalTextSearch
    0x7fff43284000 -     0x7fff432edfff  com.apple.framework.internetaccounts (2.1 - 210) <8C98785D-4CE4-3935-AE0C-245170BED1A3> /System/Library/PrivateFrameworks/InternetAccounts.framework/Versions/A/InternetAccounts
    0x7fff432ee000 -     0x7fff432feffb  com.apple.IntlPreferences (2.0 - 227) <0FF7209B-0E4D-3411-B325-03938B7C0938> /System/Library/PrivateFrameworks/IntlPreferences.framework/Versions/A/IntlPreferences
    0x7fff43399000 -     0x7fff433a6fff  com.apple.KerberosHelper (4.0 - 1.0) <06271ABF-0798-3947-86F7-6FAF863BC56C> /System/Library/PrivateFrameworks/KerberosHelper.framework/Versions/A/KerberosHelper
    0x7fff433f4000 -     0x7fff43407ff3  com.apple.security.KeychainCircle.KeychainCircle (1.0 - 1) <0BE63918-1F6F-3366-B8E2-901DA1D6071B> /System/Library/PrivateFrameworks/KeychainCircle.framework/Versions/A/KeychainCircle
    0x7fff43408000 -     0x7fff434fdfff  com.apple.LanguageModeling (1.0 - 159.3.1) <9B08E18E-69B4-3413-A03A-EF5AE4BE6277> /System/Library/PrivateFrameworks/LanguageModeling.framework/Versions/A/LanguageModeling
    0x7fff434fe000 -     0x7fff43540ff7  com.apple.Lexicon-framework (1.0 - 33.2) <5CC5E8EE-62A1-3EA5-B300-A39ABD0CF12D> /System/Library/PrivateFrameworks/Lexicon.framework/Versions/A/Lexicon
    0x7fff43544000 -     0x7fff4354bff7  com.apple.LinguisticData (1.0 - 238.3) <228AF7CA-649A-3E24-BBC7-8A24B39B3FC4> /System/Library/PrivateFrameworks/LinguisticData.framework/Versions/A/LinguisticData
    0x7fff43632000 -     0x7fff43637fff  com.apple.LoginUICore (4.0 - 4.0) <33D9089B-ECDB-361D-973A-1FDDACC5BFDF> /System/Library/PrivateFrameworks/LoginUIKit.framework/Versions/A/Frameworks/LoginUICore.framework/Versions/A/LoginUICore
    0x7fff43683000 -     0x7fff436a0ff3  com.apple.LookupFramework (1.2 - 239) <ACECB15D-1C21-3D3D-A4C5-F9736090E741> /System/Library/PrivateFrameworks/Lookup.framework/Versions/A/Lookup
    0x7fff438d1000 -     0x7fff43be9ff3  com.apple.Mail.framework (11.0 - 3445.5.20) <07790DD2-130C-32D8-9330-D0B39A97D899> /System/Library/PrivateFrameworks/Mail.framework/Versions/A/Mail
    0x7fff43bea000 -     0x7fff43cbbffb  com.apple.MailCore (11.0 - 3445.5.20) <B94B4B84-A68E-304E-BB05-07F320DA1A2D> /System/Library/PrivateFrameworks/MailCore.framework/Versions/A/MailCore
    0x7fff43cbc000 -     0x7fff43cc4ff7  com.apple.MailService (11.0 - 3445.5.20) <7ECB175F-3441-3723-9F97-DC8872204F83> /System/Library/PrivateFrameworks/MailService.framework/Versions/A/MailService
    0x7fff43cc5000 -     0x7fff43cdeff7  com.apple.MailSupport (11.0 - 3445.5.20) <EBE50F8E-8B28-3464-8935-E2C73B035C0D> /System/Library/PrivateFrameworks/MailSupport.framework/Versions/A/MailSupport
    0x7fff43d6f000 -     0x7fff43d72fff  com.apple.Mangrove (1.0 - 1) <13832222-8A6B-3790-8914-BE874B5ED4DD> /System/Library/PrivateFrameworks/Mangrove.framework/Versions/A/Mangrove
    0x7fff43dfb000 -     0x7fff43dfdff7  com.apple.marco (10.0 - 1000) <86E99D5C-D56D-3874-B465-833A9B40DE88> /System/Library/PrivateFrameworks/Marco.framework/Versions/A/Marco
    0x7fff43e8b000 -     0x7fff43ebeff7  com.apple.MediaKit (16 - 871) <47A5B872-4139-315D-A047-581F9C485D15> /System/Library/PrivateFrameworks/MediaKit.framework/Versions/A/MediaKit
    0x7fff441ad000 -     0x7fff44216ff7  com.apple.gpusw.MetalTools (1.0 - 1) <F77943BC-0466-3700-BEDF-CDD13125D36A> /System/Library/PrivateFrameworks/MetalTools.framework/Versions/A/MetalTools
    0x7fff4421e000 -     0x7fff44233ff3  com.apple.MobileAssets (1.0 - 437.40.1) <589D6786-0C0F-3CDB-8093-3834402CE083> /System/Library/PrivateFrameworks/MobileAsset.framework/Versions/A/MobileAsset
    0x7fff4438b000 -     0x7fff443a3ff3  com.apple.MobileKeyBag (2.0 - 1.0) <4C70F92C-BB58-393E-B613-EAB16790CCED> /System/Library/PrivateFrameworks/MobileKeyBag.framework/Versions/A/MobileKeyBag
    0x7fff4442f000 -     0x7fff44457fff  com.apple.MultitouchSupport.framework (1204.13 - 1204.13) <6C5D778D-4AB7-39A4-989B-2E8D2D57B3A0> /System/Library/PrivateFrameworks/MultitouchSupport.framework/Versions/A/MultitouchSupport
    0x7fff44564000 -     0x7fff44657fff  com.apple.Navigation (1.0 - 1) <C6B87818-DC8C-33AE-B624-960698ECEAEC> /System/Library/PrivateFrameworks/Navigation.framework/Versions/A/Navigation
    0x7fff446bc000 -     0x7fff446c7fff  com.apple.NetAuth (6.2 - 6.2) <5C6F492A-28EF-3A0E-B573-6F3D60CFF0C7> /System/Library/PrivateFrameworks/NetAuth.framework/Versions/A/NetAuth
    0x7fff446d4000 -     0x7fff447a1ff7  com.apple.Network (1.0 - 1) <0221BFB3-3174-3D87-B6F2-4557367B770B> /System/Library/PrivateFrameworks/Network.framework/Versions/A/Network
    0x7fff44f09000 -     0x7fff44f0bfff  com.apple.OAuth (25 - 25) <F1914BCB-72E8-33D7-BCB1-B710ACB2AD9B> /System/Library/PrivateFrameworks/OAuth.framework/Versions/A/OAuth
    0x7fff45a65000 -     0x7fff45d39ffb  com.apple.PassKitCore (1.0 - 1) <75896636-AE98-3BBC-8583-5D60E384D64B> /System/Library/PrivateFrameworks/PassKitCore.framework/Versions/A/PassKitCore
    0x7fff45ec9000 -     0x7fff45f09ffb  com.apple.PerformanceAnalysis (1.183.1 - 183.1) <2BE359F2-DCE6-3E33-BA99-964507A3F540> /System/Library/PrivateFrameworks/PerformanceAnalysis.framework/Versions/A/PerformanceAnalysis
    0x7fff45f0a000 -     0x7fff45f35fff  com.apple.persistentconnection (1.0 - 1.0) <DDFDC8D2-8C12-382D-9D26-2BC9A5DC50F1> /System/Library/PrivateFrameworks/PersistentConnection.framework/Versions/A/PersistentConnection
    0x7fff45f36000 -     0x7fff45f47ffb  com.apple.PersonaKit (1.0 - 1) <B86A7F74-DA6E-3191-8211-FC0B635026BF> /System/Library/PrivateFrameworks/PersonaKit.framework/Versions/A/PersonaKit
    0x7fff45f48000 -     0x7fff45f56ffb  com.apple.PersonaUI (1.0 - 1) <B76F88BC-848E-3AE6-B955-E189F590FD68> /System/Library/PrivateFrameworks/PersonaUI.framework/Versions/A/PersonaUI
    0x7fff45f81000 -     0x7fff45f81ff7  com.apple.PhoneNumbers (1.0 - 1) <F8D10107-164F-39BE-8A3D-638E90678DDD> /System/Library/PrivateFrameworks/PhoneNumbers.framework/Versions/A/PhoneNumbers
    0x7fff4796d000 -     0x7fff47992fff  com.apple.pluginkit.framework (1.0 - 1) <D627D78B-BEDA-3E53-B5F8-B3805F1C4D2A> /System/Library/PrivateFrameworks/PlugInKit.framework/Versions/A/PlugInKit
    0x7fff47aa6000 -     0x7fff47abaffb  com.apple.proactive.support.ProactiveEventTracker (1.0 - 123.3) <B7D03EC7-E569-363D-AEC8-7220E0E1757F> /System/Library/PrivateFrameworks/ProactiveEventTracker.framework/Versions/A/ProactiveEventTracker
    0x7fff47b78000 -     0x7fff47ba6fff  com.apple.proactive.support.ProactiveSupport (1.0 - 123.3) <9F81A893-84B3-365B-86B6-6680AAA06FBB> /System/Library/PrivateFrameworks/ProactiveSupport.framework/Versions/A/ProactiveSupport
    0x7fff47c91000 -     0x7fff47ce4fff  com.apple.ProtectedCloudStorage (1.0 - 1) <BA92C805-2AAD-3C71-A8EC-C2A4DAFD5E7C> /System/Library/PrivateFrameworks/ProtectedCloudStorage.framework/Versions/A/ProtectedCloudStorage
    0x7fff47ce5000 -     0x7fff47d03fff  com.apple.ProtocolBuffer (1 - 259) <D047A3FE-C7A8-3CAA-9891-6232BA88C247> /System/Library/PrivateFrameworks/ProtocolBuffer.framework/Versions/A/ProtocolBuffer
    0x7fff47e70000 -     0x7fff47e85ff3  com.apple.QuickLookThumbnailing (1.0 - 1) <0771325B-BBAD-3865-B2A3-BE7089641C52> /System/Library/PrivateFrameworks/QuickLookThumbnailing.framework/Versions/A/QuickLookThumbnailing
    0x7fff47ed8000 -     0x7fff47ee3fff  com.apple.xpc.RemoteServiceDiscovery (1.0 - 1205.30.29) <E2AEA0D7-23E6-3198-96EF-2F1F8ACD98B4> /System/Library/PrivateFrameworks/RemoteServiceDiscovery.framework/Versions/A/RemoteServiceDiscovery
    0x7fff47ee4000 -     0x7fff47f07ffb  com.apple.RemoteViewServices (2.0 - 125) <AEDBCE8C-88B7-315A-9F81-3E068F0D3EDC> /System/Library/PrivateFrameworks/RemoteViewServices.framework/Versions/A/RemoteViewServices
    0x7fff47f08000 -     0x7fff47f19ffb  com.apple.xpc.RemoteXPC (1.0 - 1205.30.29) <1FD53EC6-A9E9-3AEC-86A7-C52729D1A135> /System/Library/PrivateFrameworks/RemoteXPC.framework/Versions/A/RemoteXPC
    0x7fff4959a000 -     0x7fff496c9ff3  com.apple.SearchFoundation (1.0 - 184.2.12) <FE97FD5A-E3D7-335C-B829-B85FBE409AED> /System/Library/PrivateFrameworks/SearchFoundation.framework/Versions/A/SearchFoundation
    0x7fff496ca000 -     0x7fff496cdff3  com.apple.SecCodeWrapper (4.0 - 360.30.1) <B57C5AE8-273E-37EA-BB26-B7671BF41E10> /System/Library/PrivateFrameworks/SecCodeWrapper.framework/Versions/A/SecCodeWrapper
    0x7fff49824000 -     0x7fff49935fff  com.apple.Sharing (972.14 - 972.14) <964AEC2D-8A2A-33BE-9334-98A7CBE7CC51> /System/Library/PrivateFrameworks/Sharing.framework/Versions/A/Sharing
    0x7fff4995f000 -     0x7fff49960fff  com.apple.performance.SignpostNotification (1.0 - 1) <D4C967BA-92C6-3BAB-AFB5-59F98B35F921> /System/Library/PrivateFrameworks/SignpostNotification.framework/Versions/A/SignpostNotification
    0x7fff4a65d000 -     0x7fff4a8f7fff  com.apple.SkyLight (1.600.0 - 312.23.4) <455CE6F6-CD58-3E08-8300-CA8BDD3377FC> /System/Library/PrivateFrameworks/SkyLight.framework/Versions/A/SkyLight
    0x7fff4b0a8000 -     0x7fff4b0b5ff7  com.apple.SpeechRecognitionCore (4.0.13 - 4.0.13) <AC026FB9-78F8-31F9-BB80-619D5378DB70> /System/Library/PrivateFrameworks/SpeechRecognitionCore.framework/Versions/A/SpeechRecognitionCore
    0x7fff4b168000 -     0x7fff4b3a3fff  com.apple.spotlight.index (10.7.0 - 1191.2.6) <236BC19C-F361-344B-91DE-1B40C6E31878> /System/Library/PrivateFrameworks/SpotlightIndex.framework/Versions/A/SpotlightIndex
    0x7fff4b54e000 -     0x7fff4b575ff7  com.apple.StreamingZip (1.0 - 1) <F06F4CDC-138C-3991-A987-7C7332BE4508> /System/Library/PrivateFrameworks/StreamingZip.framework/Versions/A/StreamingZip
    0x7fff4b576000 -     0x7fff4b5e7ffb  com.apple.Suggestions (6.0 - 213) <36ED2166-7EE5-3114-9EA7-FEA0616DF0DE> /System/Library/PrivateFrameworks/Suggestions.framework/Versions/A/Suggestions
    0x7fff4bc47000 -     0x7fff4bccbfe7  com.apple.Symbolication (9.0 - 63079.1) <177BC9CA-E6AE-3B40-806F-0080C0CDFF29> /System/Library/PrivateFrameworks/Symbolication.framework/Versions/A/Symbolication
    0x7fff4bcdc000 -     0x7fff4bce3fff  com.apple.SymptomDiagnosticReporter (1.0 - 820.30.7) <0188BA8D-0C88-3413-A199-666C7FBE4DF1> /System/Library/PrivateFrameworks/SymptomDiagnosticReporter.framework/Versions/A/SymptomDiagnosticReporter
    0x7fff4bf8d000 -     0x7fff4bfb9fff  com.apple.framework.SystemAdministration (1.0 - 1.0) <6AE03DE8-E5C1-38E5-B788-0A763CAACDC2> /System/Library/PrivateFrameworks/SystemAdministration.framework/Versions/A/SystemAdministration
    0x7fff4c1bd000 -     0x7fff4c1c4ff3  com.apple.TCC (1.0 - 1) <C807D3F0-FE20-3FC0-8D61-306477ABEBC4> /System/Library/PrivateFrameworks/TCC.framework/Versions/A/TCC
    0x7fff4c1ce000 -     0x7fff4c275ff7  com.apple.TelephonyUtilities (1.0 - 1.0) <4BA81802-B803-3F64-BEC4-808461F02921> /System/Library/PrivateFrameworks/TelephonyUtilities.framework/Versions/A/TelephonyUtilities
    0x7fff4c3d1000 -     0x7fff4c48eff7  com.apple.TextureIO (3.7 - 3.7) <C98BFACA-7807-3DCA-945D-58EBA2B723C8> /System/Library/PrivateFrameworks/TextureIO.framework/Versions/A/TextureIO
    0x7fff4c4df000 -     0x7fff4c4faffb  com.apple.ToneKit (1.0 - 1) <A0242052-983B-3726-B5BD-D2E672CEFE2F> /System/Library/PrivateFrameworks/ToneKit.framework/Versions/A/ToneKit
    0x7fff4c4fb000 -     0x7fff4c522fff  com.apple.ToneLibrary (1.0 - 1) <7EEABD7F-D0CB-327E-A772-ACF406B9C8F2> /System/Library/PrivateFrameworks/ToneLibrary.framework/Versions/A/ToneLibrary
    0x7fff4c532000 -     0x7fff4c533fff  com.apple.TrustEvaluationAgent (2.0 - 31) <39F533B2-211E-3635-AF47-23F27749FF4A> /System/Library/PrivateFrameworks/TrustEvaluationAgent.framework/Versions/A/TrustEvaluationAgent
    0x7fff4c539000 -     0x7fff4c6e8ff3  com.apple.UIFoundation (1.0 - 546.1.1) <D2DB451C-56CD-3249-B8EE-AF21DBDCFBF8> /System/Library/PrivateFrameworks/UIFoundation.framework/Versions/A/UIFoundation
    0x7fff4c8b1000 -     0x7fff4cf7ffff  com.apple.VectorKit (1.0 - 1355.22.9.1.12) <860C7D72-D7D1-3643-B1BB-8CDD023369C7> /System/Library/PrivateFrameworks/VectorKit.framework/Versions/A/VectorKit
    0x7fff4d3b1000 -     0x7fff4d480ff7  com.apple.ViewBridge (343.2 - 343.2) <B23D10F2-A5E8-30EF-964A-6E968F4017A1> /System/Library/PrivateFrameworks/ViewBridge.framework/Versions/A/ViewBridge
    0x7fff4d557000 -     0x7fff4d55efff  com.apple.WeatherKit (2.0 - 153.2.2) <4B4724D0-1A57-3238-9AFF-CAB0426534D8> /System/Library/PrivateFrameworks/WeatherKit.framework/Versions/A/WeatherKit
    0x7fff4d6a3000 -     0x7fff4d706ffb  com.apple.WhitePagesFramework (10.7.0 - 141.0) <2598BE6B-B613-368A-94F8-B826D23B5248> /System/Library/PrivateFrameworks/WhitePages.framework/Versions/A/WhitePages
    0x7fff4d845000 -     0x7fff4da6bff7  libAWDSupportFramework.dylib (749.13) <B8711178-CE8C-3786-A269-E83710A1B71D> /System/Library/PrivateFrameworks/WirelessDiagnostics.framework/Versions/A/Libraries/libAWDSupportFramework.dylib
    0x7fff4da6c000 -     0x7fff4da7dfff  libprotobuf-lite.dylib (749.13) <9344E035-93AA-3197-8033-93348005AFF5> /System/Library/PrivateFrameworks/WirelessDiagnostics.framework/Versions/A/Libraries/libprotobuf-lite.dylib
    0x7fff4da7e000 -     0x7fff4dae0fff  libprotobuf.dylib (749.13) <EA5D182D-2E4B-3DF3-BEED-ADEF1FE4EBC5> /System/Library/PrivateFrameworks/WirelessDiagnostics.framework/Versions/A/Libraries/libprotobuf.dylib
    0x7fff4dae1000 -     0x7fff4db24ff7  com.apple.awd (1.0 - 919) <60D12A36-C579-348F-B84C-E25F20C422E0> /System/Library/PrivateFrameworks/WirelessDiagnostics.framework/Versions/A/WirelessDiagnostics
    0x7fff4dc73000 -     0x7fff4dca8ffb  com.apple.iCalendar (7.0 - 275) <77B8CC80-6639-30FA-83EA-FFB3489466D3> /System/Library/PrivateFrameworks/iCalendar.framework/Versions/A/iCalendar
    0x7fff4de2c000 -     0x7fff4de2effb  com.apple.loginsupport (1.0 - 1) <5E2C4AA7-066D-3FDB-B0E1-4CDAF287392C> /System/Library/PrivateFrameworks/login.framework/Versions/A/Frameworks/loginsupport.framework/Versions/A/loginsupport
    0x7fff4de2f000 -     0x7fff4de44fff  com.apple.login (3.0 - 3.0) <140118A7-8FB9-35C5-92FA-BD79C9C32309> /System/Library/PrivateFrameworks/login.framework/Versions/A/login
    0x7fff4de6f000 -     0x7fff4de9cfff  com.apple.contacts.vCard (1.0 - 2330.3) <0417A3AE-7A95-37DB-895B-33EAAC7A1422> /System/Library/PrivateFrameworks/vCard.framework/Versions/A/vCard
    0x7fff4df92000 -     0x7fff4dfc5fff  libclosured.dylib (519.2.2) <48051216-5647-3643-B979-B77D0FD20011> /usr/lib/closure/libclosured.dylib
    0x7fff4e065000 -     0x7fff4e09eff7  libCRFSuite.dylib (41) <AB2DA745-F22C-30CF-81D4-35DD716463B8> /usr/lib/libCRFSuite.dylib
    0x7fff4e09f000 -     0x7fff4e0aafff  libChineseTokenizer.dylib (28) <D30A7DB6-058F-3286-9583-60C9EEB77A6E> /usr/lib/libChineseTokenizer.dylib
    0x7fff4e0ab000 -     0x7fff4e138ff7  libCoreStorage.dylib (546) <B495D87B-FE21-3C14-9017-F02D9CA226D5> /usr/lib/libCoreStorage.dylib
    0x7fff4e13c000 -     0x7fff4e13dff3  libDiagnosticMessagesClient.dylib (104) <9712E980-76EE-3A89-AEA6-DF4BAF5C0574> /usr/lib/libDiagnosticMessagesClient.dylib
    0x7fff4e174000 -     0x7fff4e33eff3  libFosl_dynamic.dylib (17.7) <B2476843-7FA7-3E62-B79F-2B15FE557E63> /usr/lib/libFosl_dynamic.dylib
    0x7fff4e35e000 -     0x7fff4e365fff  libMatch.1.dylib (31) <74AB4815-11D1-3930-A559-BD6550CE5865> /usr/lib/libMatch.1.dylib
    0x7fff4e376000 -     0x7fff4e376fff  libOpenScriptingUtil.dylib (174) <203D2C39-61BB-3713-A502-2D17B04A42AC> /usr/lib/libOpenScriptingUtil.dylib
    0x7fff4e49f000 -     0x7fff4e4a3ffb  libScreenReader.dylib (562.13) <21638ECC-87BF-3CC5-B2D8-6F7883F5FDCA> /usr/lib/libScreenReader.dylib
    0x7fff4e4a4000 -     0x7fff4e4a5ff3  libSystem.B.dylib (1252) <47329E26-DC23-3EBA-9461-37755368327D> /usr/lib/libSystem.B.dylib
    0x7fff4e4a6000 -     0x7fff4e523fff  libTelephonyUtilDynamic.dylib (3086.3) <22323382-134A-3F4E-94EB-5DA2914081A4> /usr/lib/libTelephonyUtilDynamic.dylib
    0x7fff4e524000 -     0x7fff4e525fff  libThaiTokenizer.dylib (2.2) <04AC7CD4-BD9A-3B1D-8CEB-9E2030B6B667> /usr/lib/libThaiTokenizer.dylib
    0x7fff4e538000 -     0x7fff4e538fff  libapple_crypto.dylib (109.40.1) <32252490-B1E9-363F-AEED-3EC97D919348> /usr/lib/libapple_crypto.dylib
    0x7fff4e539000 -     0x7fff4e54fff7  libapple_nghttp2.dylib (1.24) <01402BC4-4822-3676-9C80-50D83F816424> /usr/lib/libapple_nghttp2.dylib
    0x7fff4e550000 -     0x7fff4e57aff3  libarchive.2.dylib (54) <8FC28DD8-E315-3C3E-95FE-D1D2CBE49888> /usr/lib/libarchive.2.dylib
    0x7fff4e57b000 -     0x7fff4e67efe7  libate.dylib (1.13.1) <DABEA1B1-806C-34C9-8AFF-DEB6AB2829E3> /usr/lib/libate.dylib
    0x7fff4e682000 -     0x7fff4e682ff3  libauto.dylib (187) <A05C7900-F8C7-3E75-8D3F-909B40C19717> /usr/lib/libauto.dylib
    0x7fff4e683000 -     0x7fff4e73afff  libboringssl.dylib (109.40.1) <75F5F125-B919-3318-BD12-29CB5E868475> /usr/lib/libboringssl.dylib
    0x7fff4e73b000 -     0x7fff4e74bff3  libbsm.0.dylib (39) <770B341F-3BB7-3123-B53C-F2D58868A963> /usr/lib/libbsm.0.dylib
    0x7fff4e74c000 -     0x7fff4e759ffb  libbz2.1.0.dylib (38) <0A5086BB-4724-3C14-979D-5AD4F26B5B45> /usr/lib/libbz2.1.0.dylib
    0x7fff4e75a000 -     0x7fff4e7b0fff  libc++.1.dylib (400.9) <FCF5E1F6-2B04-3545-8004-F3AB32FED172> /usr/lib/libc++.1.dylib
    0x7fff4e7b1000 -     0x7fff4e7d5ff7  libc++abi.dylib (400.7) <217656D5-BC40-37FF-B322-91CB2AAD4F34> /usr/lib/libc++abi.dylib
    0x7fff4e7d7000 -     0x7fff4e7e7fff  libcmph.dylib (6) <A5509EE8-7E00-3224-8814-015B077A3CF5> /usr/lib/libcmph.dylib
---
travis_time:end:034893d0:start=1541421819180681000,finish=1541421820693281000,duration=1512600000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b341e2a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1811cfdc
travis_time:start:1811cfdc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:21d64d83
$ dmesg | grep -i kill
$ dmesg | grep -i kill
Unable to obtain kernel buffer: Operation not permitted
usage: sudo dmesg
travis_fold:end:after_failure.6

Done. Your build exited with 1.
