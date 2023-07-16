
Process:               ld [63654]
Path:                  /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/ld
Identifier:            ld
Version:               278.4
Code Type:             X86-64 (Native)
Parent Process:        ??? [63653]
Responsible:           ld [63654]
User ID:               501

Date/Time:             2017-11-15 13:03:39.865 +0000
OS Version:            Mac OS X 10.12.6 (16G29)
Report Version:        12
Anonymous UUID:        A91A3A70-3ADA-D7A9-5891-C1A67A5778E7


Time Awake Since Boot: 8100 seconds

System Integrity Protection: enabled

Crashed Thread:        0  Dispatch queue: com.apple.main-thread

Exception Type:        EXC_BAD_ACCESS (SIGBUS)
Exception Codes:       KERN_PROTECTION_FAILURE at 0x00000001084cb020
Exception Note:        EXC_CORPSE_NOTIFY

Termination Signal:    Bus error: 10
Termination Reason:    Namespace SIGNAL, Code 0xa
Terminating Process:   exc handler [0]

VM Regions Near 0x1084cb020:
    MALLOC metadata        0000000108499000-000000010849a000 [    4K] r--/rwx SM=PRV  
--> __TEXT                 000000010849a000-000000010850a000 [  448K] r-x/rwx SM=COW  /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libtapi.dylib
    __DATA                 000000010850a000-000000010850f000 [   20K] rw-/rwx SM=COW  /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libtapi.dylib

Application Specific Information:
ld64-278.4
ld -demangle -lto_library /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/libLTO.dylib -dynamic -arch x86_64 -macosx_version_min 10.8.0 -o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out -L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib -L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib -L/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out0.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out1.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out2.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out3.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out4.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out5.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out6.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out7.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out8.rcgu.o /var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.crate.allocator.rcgu.o -dead_strip -lstd-369c1f8e5b16bb0f /Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-21e0a7e843257473.rlib -lSystem -lresolv -lpthread -lc -lm 

Thread 0 Crashed:: Dispatch queue: com.apple.main-thread
0   libsystem_c.dylib             	0x00007fffdd69d143 __cxa_finalize_ranges + 279

Thread 1:
0   libsystem_c.dylib             	0x00007fffdd63eb52 strlen + 18
1   libsystem_c.dylib             	0x00007fffdd6849fc __vfprintf + 5701
2   libsystem_c.dylib             	0x00007fffdd6ad423 __v2printf + 699
3   libsystem_c.dylib             	0x00007fffdd682f46 _vasprintf + 554
4   libsystem_c.dylib             	0x00007fffdd67a7a3 asprintf + 186
5   ld                            	0x0000000108352f78 ld::tool::InputFiles::parseWorkerThread() + 224
6   libsystem_pthread.dylib       	0x00007fffdd82193b _pthread_body + 180
7   libsystem_pthread.dylib       	0x00007fffdd821887 _pthread_start + 286
8   libsystem_pthread.dylib       	0x00007fffdd82108d thread_start + 13

Thread 0 crashed with X86 Thread State (64-bit):
  rax: 0x00000001084cb020  rbx: 0xffffffff00000000  rcx: 0x00001d0000001d00  rdx: 0x0000000000000000
  rdi: 0x0000000000001aff  rsi: 0x00001d0000001e03  rbp: 0x00007fff579192bf  rsp: 0x00007fff57919280
   r8: 0x00007fffe65560c8   r9: 0x000000000005c297  r10: 0x00007fffe65560d0  r11: 0xffffffff00000000
  r12: 0x0000000108495030  r13: 0x00000000fffec895  r14: 0x0000000000001b00  r15: 0x0000000000035fe0
  rip: 0x00007fffdd69d143  rfl: 0x0000000000010246  cr2: 0xfffffffffffffff0
  
Logical CPU:     1
Error Code:      0x00000004
Trap Number:     14
