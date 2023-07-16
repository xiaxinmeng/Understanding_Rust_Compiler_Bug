plain
[00:48:31] ....................................................................................................
[00:48:33] ....................................................................................................
[00:48:36] ....................................................................................................
[00:48:39] ....................................................................................................
[00:48:42] .......iiiiiiiii....................................................................................
[00:48:47] ....................................................................................................
[00:48:51] ............i.......................................................................................
[00:48:54] .....................i..............................................................................
[00:48:57] ....................................................................................................
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:44] 
[00:56:44] running 110 tests
[00:56:55] iiii.......i..i........i..i.i.............i..........iiii...........i....i...F......ii.i.i.......ii.
[00:56:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:56:56] failures:
[00:56:56] 
[00:56:56] ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
[00:56:56] ---- [debuginfo-gdb] debuginfo/pretty-std-collections.rs stdout ----
[00:56:56] NOTE: compiletest thinks it is using GDB without native rust support
[00:56:56] NOTE: compiletest thinks it is using GDB version 7011001
[00:56:56] 
[00:56:56] error: line not found in debugger output: $1 = BTreeSet<i32> with 3 elements = {[0] = 3, [1] = 5, [2] = 7}
[00:56:56] status: exit code: 0
[00:56:56] command: "/usr/bin/gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/pretty-std-collections.debugger.script"
[00:56:56] ------------------------------------------
[00:56:56] GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.5) 7.11.1
[00:56:56] Copyright (C) 2016 Free Software Foundation, Inc.
[00:56:56] Copyright (C) 2016 Free Software Foundation, Inc.
[00:56:56] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:56:56] This is free software: you are free to change and redistribute it.
[00:56:56] There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
[00:56:56] and "show warranty" for details.
[00:56:56] This GDB was configured as "x86_64-linux-gnu".
[00:56:56] Type "show configuration" for configuration details.
[00:56:56] For bug reporting instructions, please see:
[00:56:56] <http://www.gnu.org/software/gdb/bugs/>.
[00:56:56] Find the GDB manual and other documentation resources online at:
[00:56:56] <http://www.gnu.org/software/gdb/documentation/>.
[00:56:56] For help, type "help".
[00:56:56] Type "apropos word" to search for commands related to "word".
[00:56:56] Breakpoint 1 at 0xb162: file /checkout/src/test/debuginfo/pretty-std-collections.rs, line 47.
[00:56:56] [Thread debugging using libthread_db enabled]
[00:56:56] Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".
[00:56:56] 
[00:56:56] Breakpoint 1, pretty_std_collections::main::h617474859dc5b54e () at /checkout/src/test/debuginfo/pretty-std-collections.rs:47
[00:56:56] 47     zzz(); // #break
[00:56:56] $1 = BTreeSet<i32> with 3 elements = {[0] = 
[00:56:56] stderr:
[00:56:56] ------------------------------------------
[00:56:56] ------------------------------------------
[00:56:56] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/pretty-std-collections/pretty-std-collections.debugger.script:9: Error in sourced command file:
[00:56:56] Cannot access memory at address 0x0
[00:56:56] ------------------------------------------
[00:56:56] 
[00:56:56] thread '[debuginfo-gdb] debuginfo/pretty-std-collections.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:56:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:56:56] test result: FAILED. 85 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[00:56:56] 
[00:56:56] 
[00:56:56] 
[00:56:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "debuginfo-gdb" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:56] 
[00:56:56] 
[00:56:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:56] Build completed unsuccessfully in 0:11:09
[00:56:56] Build completed unsuccessfully in 0:11:09
[00:56:56] Makefile:58: recipe for target 'check' failed
[00:56:56] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e99a84a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Aug  7 14:25:02 UTC 2018
 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] kvm-clock: using sched offset of 1376977137 cycles
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] Zone ranges:
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   Device   empty
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] Movable zone start for each node
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] Early memory node ranges
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] ACPI: PM-Timer ravis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.305712] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.307045] pid_max: default: 32768 minimum: 301
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.307725] ACPI: Core revision 2015093082c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.336291] Initializing cgroup subsys net_cls
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.336930] Initializing cgroup subsys perf_event
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.337573] Initializing cgroup subsys net_prio
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.338197] Initializing cgroup subsys hugetlb
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.338806] Initializing cgroup subsys pids
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.339454] CPU: Physical Processor ID: 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.340006] CPU: Processor Core ID: 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.340524] mce: CPU supports 32 MCE banks
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.341237] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.341982] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.344572] Freeing SMP alternatives memory: 32K
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.352732] ftrace: allocating 32185 entries in 126 pages
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.399104] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.400045] smpboot: Max logical packages: 2
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.401387] x2apic enabled
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.403035] Switched APIC routing to physical x2apic.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.406672] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.512813] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.514729] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.517088] x86: Booting SMP configuration:
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.517840] .... node  #0, CPUs:      #1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.518944] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.522392]  #2
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.522891] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.526186]  #3
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.526718] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.530027] x86: Booted up 1 node, 4 CPUs
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.530845] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.533186] devtmpfs: initialized
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.537368] evm: security.selinux
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.537852] evm: security.SMACK64
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.538438] evm: security.SMACK64EXEC
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.539006] evm: security.SMACK64TRANSMUTE
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.539706] evm: security.SMACK64MMAP
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.540274] evm: security.ima
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.541051] evm: security.capability
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.541914] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.543946] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.545199] pinctrl core: initialized pinctrl subsystem
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.546145] RTC time: 13:25:53, date: 08/07/18
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.547781] NET: Registered protocol family 16
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.556840] cpuidle: using governor ladder
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.568842] cpuidle: using governor menu
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.569526] PCCT header not found.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.570116] ACPI: bus type PCI registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.570709] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.571829] PCI: Using configuration type 1 for base access
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.585815] ACPI: Added _OSI(Module Device)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.586612] ACPI: Added _OSI(Processor Device)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.587373] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.588199] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.591458] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.614249] ACPI: Interpreter enabled
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.614896] ACPI: (supports S0 S3 S4 S5)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.615580] ACPI: Using IOAPIC for interrupt routing
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.616365] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.645554] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.646554] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.647703] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.648729] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.651055] PCI host bridge to bus 0000:00
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.651887] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.652952] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.653978] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.655028] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.656443] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.657291] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.657682] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.670915] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.683441] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.685019] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.690999] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.695260] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.706317] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.710896] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.714393] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.726253] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.728457] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.730720] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.732827] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.734739] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.754694] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.755736] vgaarb: loaded
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.756594] SCSI subsystem initialized
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.757495] libata version 3.00 loaded.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.757521] ACPI: bus type USB registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.758167] usbcore: registered new interface driver usbfs
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.759197] usbcore: registered new interface driver hub
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.760018] usbcore: registered new device driver usb
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.761025] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.762204] dmi: Firmware registration failed.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.763136] PCI: Using ACPI for IRQ routing
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.763786] PCI: pci_cache_line_size set to 64 bytes
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.763882] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.763884] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.764009] NetLabel: Initializing
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.764571] NetLabel:  domain hash size = 128
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.765316] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.766032] NetLabel:  unlabeled traffic allowed by default
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.766957] amd_nb: Cannot enumerate AMD northbridges
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.767814]64
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    0.795029] Unpacking initramfs...
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.741224] Freeing initrd memory: 21432K
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.741881] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.742843] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.744587] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.745891] hw unit of domain pp0-core 2^-0 Joules
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.746668] hw unit of domain package 2^-0 Joules
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.747331] hw unit of domain dram 2^-0 Joules
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.748038] Scanning for low memory corruption every 60 seconds
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.749385] audit: initializing netlink subsys (disabled)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.750259] audit: type=2000 audit(1533648355.857:1): initialized
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.751370] Initialise system trusted keyring
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.752238] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.753136] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.755157] zbud: loaded
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.755802] VFS: Disk quotas dquot_6.6.0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.756546] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.758006] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.759351] fuse init (API version 7.23)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.760391] Key type big_key registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.761401] Allocating IMA MOK and blacklist keyrings.
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.763380] Key type asymmetric registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.764118] Asymmetric key parser 'x509' registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.764847] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.766151] io scheduler noop registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.766767] io scheduler deadline registered (default)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.767591] io scheduler cfq registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.768263] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.769066] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.770289] intel_idle: does not run on family 6 model 45
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.770408] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.771553] ACPI: Power Button [PWRF]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.772282] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.773529] ACPI: Sleep Button [SLPF]
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.774416] GHES: HEST is not enabled!
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.776815] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.777909] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 13:2alcomm.com>
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.918284] PPP generic driver version 2.4.2
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.919625] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.921506] ehci-pci: EHCI PCI platform driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.922968] ehci-platform: EHCI generic platform driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.924696] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.926592] ohci-pci: OHCI PCI platform driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.927965] ohci-platform: OHCI generic platform driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.929237] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.931226] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.934221] i8042: Warning: Keylock active
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.936440] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.937552] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 13:26:03 travis-job-c5758icrocode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.959613] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.960542] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.962058] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.963613] registered taskstats version 1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.964239] Loading compiled-in X.509 certificates
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.965712] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.967401] zswap: loaded using pool lzo/zbud
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.970068] Key type trusted registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.973652] Key type encrypted registered
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.974416] ima: No TPM chip found, activating TPM-bypass!
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.975482] evm: HMAC attrs: 0x1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    2.976322]   Magic number: 14:290:432
Aug  7  mode by8 optimization enabled
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.055717] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.093293] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.093320] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.095339] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.096210] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.096899] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.096943] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.099853]  sda: sda1
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.100710] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.140158] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.747986] tsc: Refined TSC clocksource calibration: 2600.000 MHz
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.749003] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3c3232d, max_idle_ns: 440795236700 ns
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    3.972908] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    6.056029] floppy0: no floppy controllers found
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.207825] raid6: sse2x1   gen()  8992 MB/s
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.275825] raid6: sse2x1   xor()  6763 MB/s
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.343822] raid6: sse2x2   gen() 11266 MB/s
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.411831] raid6: sse2x2   xor()  7382 MB/s
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.479836] raid6: sse2x4   gen() 12743 MB/s
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.547831] raid6: sse2x4   xor()  8881 MB/s
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.548606] raid6: using algorithm sse2x4 gen() 12743 MB/s
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.549522] raid6: .... xor() 8881 MB/s, rmw enabled
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.550398] raid6: using ssse3x2 recovery algorithm
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.552554] xor: automatically using best checksumming function:
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.591830]    avx       : 27857.000 MB/sec
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.605679] Btrfs loaded
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.644548] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.645603] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.715808] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.722425] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.723422] EXT4-fs (sda1): recovery complete
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.727435] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    7.912610] random: init: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    8.015622] random: mountall: uninitialized urandom read (12 bytes read, 33 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    8.065160] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    8.243855] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    8.736017] random: cloud-init: uninitialized urandom read (32 bytes read, 50 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    8.868699] systemd-udevd[703]: starting version 204
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    8.967690] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.022499] intel_rapl: no valid rapl domains found in package 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.072459] intel_rapl: no valid rapl domains found in package 0
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.105813] ppdev: user-space parallel port driver
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.172586] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.213896] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.270062] random: cloud-init: uninitialized urandom read (32 bytes read, 63 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.429054] random: cloud-init: uninitialized urandom read (32 bytes read, 63 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.677095] random: mktemp: uninitialized urandom read (12 bytes read, 65 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.739180] random: mktemp: uninitialized urandom read (6 bytes read, 66 bits of entropy available)
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.803893] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [    9.837748] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   10.390521] init: failsafe main process (1095) killed by TERM signal
Aug  7 13:26:03 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Running set_multiqueue.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   11.015344] random: nonblocking pool is initialized
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 google-accounts: INFO Creating a new user account for me.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 google-accounts: INFO Created user account me.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 google-accounts: INFO Removing user packer.
Aug  7 13:26:04 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 google-clock-skew: INFO Starting Google Clock Skew(C) 2000-2006 Netfilter Core Team
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   11.759588] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   11.762683] Bridge firewalling registered
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   11.771212] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   11.830938] Initializing XFRM netlink socket
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   11.836246] Netfilter messages via NETLINK v0.30.
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   11.838592] ctnetlink v0.93: registering with nfnetlink.
Aug  7 13:26:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [   12.055909] floppy0: no floppy controllers found
Aug  7 13:26:28 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpdate[1762]: adjust time server 169.254.169.254 offset 0.018132 sec
Aug  7 13:26:34 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpd[1796]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 13:26:34 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpd[1797]: proto: precision = 0.114 use2018
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 startup-script: INFO startup-script: Return code 0.
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 startup-script: INFO startup-script: Return code 0.
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 startup-script: INFO Finished running startup scripts.
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: 
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: #############################################################
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: 1024 81:c7:04:e5:3b:a6:37:9e:de:17:fb:14:6d:62:18:ea  root@travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 (DSA)
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: 256 f4:36:41:21:a2:16:ba:9b:9a:4f:de:34:0b:31:eb:03  root@travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 (ECDSA)
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: 256 5a:fc:9a:98:32:3b:20:ca:e9:92:b9:76:d6:4c:9e:a5  root@travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 (ED25519)
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: 2048 da:12:78:86:64:6c:ab:96:86:a6:fb:c2:27:39:b3:30  root@travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 (RSA)
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 13:26:35 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ec2: #############################################################
Aug  7 13:28:05 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  131.784621] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  226.808953] device vethaa95c2f entered promiscuous mode
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  226.809038] docker0: port 1(vethaa95c2f) entered forwarding state
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  226.809047] docker0: port 1(vethaa95c2f) entered forwarding state
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  226.809478] docker0: port 1(vethaa95c2f) entered disabled state
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  226.899692] cgroup: docker-runc (4791) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  226.899695] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  226.971355] eth0: renamed from vethc9a77e8
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  227.009613] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  227.010687] docker0: port 1(vethaa95c2f) entered forwarding state
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  227.010707] docker0: port 1(vethaa95c2f) entered forwarding state
Aug  7 13:29:40 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  227.010736] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 13:29:42 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpd[1797]: Listen normally on 5 docker0 fe80::42:5eff:fe13:4568 UDP 123
Aug  7 13:29:42 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpd[1797]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  7 13:29:42 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpd[1797]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 13:29:42 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpd[1797]: peers refreshed
Aug  7 13:29:42 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 ntpd[1797]: new interface(s) found: waking up resolver
Aug  7 13:29:55 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [  242.054674] docker0: port 1(vethaa95c2f) entered forwarding state
Aug  7 14:17:01 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 CRON[4310]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug  7 14:17:12 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3079.486027] traps: a[5773] trap invalid opcode ip:55eb731b1c1b sp:7fff26b28940 error:0 in a[55eb731ae000+6000]
Aug  7 14:17:27 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3094.466968] traps: a[8789] trap invalid opcode ip:7fc70f261ee1 sp:7ffd8b02b570 error:0 in libstd-e054c7a28f8831a7.so[7fc70f206000+172000]
Aug  7 14:17:27 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3094.494300] traps: a[8790] trap invalid opcode ip:7fd3a7d45ee1 sp:7ffca2275640 error:0 in libstd-e054c7a28f8831a7.so[7fd3a7cea000+172000]
Aug  7 14:18:50 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3177.628286] traps: a[24542] trap invalid opcode ip:56395c9d3e68 sp:7ffdbee99b10 error:0 in a[56395c9d1000+4000]
Aug  7 14:21:36 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3343.591388] a[22389]: segfault at 0 ip 000055f219757658 sp 00007ffeddc80670 error 6 in a[55f219754000+5000]
Aug  7 14:21:46 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3352.911882] a[23172]: segfault at 1 ip 0000559331bbcc6c sp 00007ffcbcbf3fc0 error 6 in a[559331bba000+4000]
Aug  7 14:21:50 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3356.961110] traps: a[23576] trap invalid opcode ip:5635162925bc sp:7fffa3a8c390 error:0 in a[56351628f000+7000]
Aug  7 14:25:02 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3549.007041] vethc9a77e8: renamed from eth0
Aug  7 14:25:02 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3549.054300] docker0: port 1(vethaa95c2f) entered disabled state
Aug  7 14:25:02 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3549.079872] docker0: port 1(vethaa95c2f) entered disabled state
Aug  7 14:25:02 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3549.081438] device vethaa95c2f left promiscuous mode
Aug  7 14:25:02 travis-job-c57582c1-a2b3-4bd4-98d9-da370c9091a3 kernel: [ 3549.081441] docker0: port 1(vethaa95c2f) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1a534d48
