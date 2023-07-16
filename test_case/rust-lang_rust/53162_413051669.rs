plain
[00:52:34] ....................................................................................................
[00:52:38] ...............................................................................................i....
[00:52:40] ....................................................................................................
[00:52:44] ....................................................................................................
[00:52:46] ............................................iiiiiiiii...............................................
[00:52:52] ....................................................................................................
[00:52:56] ....................................................................................................
[00:52:58] .......................i............................................................................
[00:53:01] ..........................i...............................................i.i..ii...................
---
travis_time:start:test_rustdoc
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:52] 
[01:05:52] running 255 tests
[01:07:11] ......................i.................................................................F...........
00] thread '[rustdoc] rustdoc/issue-13698.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3166:9
[01:09:00] 
[01:09:00] 
[01:09:00] failures:
[01:09:00] failures:
[01:09:00]     [rustdoc] rustdoc/issue-13698.rs
[01:09:00] 
[01:09:00] test result: FAILED. 252 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out
[01:09:00] 
[01:09:00] 
[01:09:00] 
[01:09:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:00] 
[01:09:00] 
[01:09:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:00] Build completed unsuccessfully in 0:20:37
[01:09:00] Build completed unsuccessfully in 0:20:37
[01:09:00] Makefile:58: recipe for target 'check' failed
[01:09:00] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13a17387
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2c4b717d
$ sudo tail -n 500 /var/log/syslog
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] kvm-clock: using sched offset of 1850470068 cycles
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Zone ranges:
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   Device   empty
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Movable zone start for each node
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Early memory node ranges
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000]  [mem 0x00000000-0x00000fff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Policy zone: Normal
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
4942] Security Framework initialized
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.778784] Yama: becoming mindful.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.782286] AppArmor: AppArmor disabled by boot time parameter
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.789798] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.807048] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.819847] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.826976] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.831844] Initializing cgroup subsys io
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.835448] Initializing cgroup subsys memory
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.840782] Initializing cgroup subsys devices
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.844182] Initializing cgroup subsys freezer
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.847760] Initializing cgroup subsys net_cls
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.851975] Initializing cgroup subsys perf_event
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.855685] Initializing cgroup subsys net_prio
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.859941] Initializing cgroup subsys hugetlb
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.864023] Initializing cgroup subsys pids
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.867915] CPU: Physical Processor ID: 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.872010] CPU: Processor Core ID: 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.875480] mce: CPU supports 32 MCE banks
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.879490] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.885810] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.893825] Freeing SMP alternatives memory: 32K
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.907453] ftrace: allocating 32185 entries in 126 pages
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.967095] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.972918] smpboot: Max logical packages: 2
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.977099] x2apic enabled
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.983689] Switched APIC routing to physical x2apic.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    0.994996] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.108405] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.117362] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.130602] x86: Booting SMP configuration:
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.133540] .... node  #0, CPUs:      #1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.136722] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.142899]  #2
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.144689] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.151605]  #3
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.153293] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.162865] x86: Booted up 1 node, 4 CPUs
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.167572] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.176868] devtmpfs: initialized
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.183527] evm: security.selinux
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.185761] evm: security.SMACK64
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.188381] evm: security.SMACK64EXEC
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.191029] evm: security.SMACK64TRANSMUTE
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.194710] evm: security.SMACK64MMAP
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.198034] evm: security.ima
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.200182] evm: security.capability
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.202969] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.214173] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.220683] pinctrl core: initialized pinctrl subsystem
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.224356] RTC time: 22:40:02, date: 08/14/18
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.228404] NET: Registered protocol family 16
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.244602] cpuidle: using governor ladder
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.254914] cpuidle: using governor menu
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.257947] PCCT header not found.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.259682] ACPI: bus type PCI registered
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.263354] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.272570] PCI: Using configuration type 1 for base access
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.293626] ACPI: Added _OSI(Module Device)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.296825] ACPI: Added _OSI(Processor Device)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.299553] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.302961] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.313324] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.342318] ACPI: Interpreter enabled
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.345222] ACPI: (supports S0 S3 S4 S5)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.347789] ACPI: Using IOAPIC for interrupt routing
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.351287] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.386547] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.391875] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.396415] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.402599] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.411787] PCI host bridge to bus 0000:00
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.415256] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.419274] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.424993] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.430015] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.434950] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.438626] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.439126] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.474878] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.526199] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.531992] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.543992] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.552108] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.577939] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.588303] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.597216] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.622033] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.630167] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.638025] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.644094] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.652402] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.678925] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.684171] vgaarb: loaded
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.686161] SCSI subsystem initialized
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.688794] libata version 3.00 loaded.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.688831] ACPI: bus type USB registered
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.691350] usbcore: registered new interface driver usbfs
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.697399] usbcore: registered new interface driver hub
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.701845] usbcore: registered new device driver usb
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.705218] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.709923] dmi: Firmware registration failed.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.712442] PCI: Using ACPI for IRQ routing
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.716409] PCI: pci_cache_line_size set to 64 bytes
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.716511] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.716514] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.716701] NetLabel: Initializing
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.720513] NetLabel:  domain hash size = 128
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.724038] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.727908] NetLabel:  unlabeled traffic allowed by default
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.732803] amd_nb: Cannot enumerate AMD northbridges
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.736891] clocksource: Switched to clocksource kvm-clock
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.750304] pnp: PnP ACPI init
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.754819] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.754898] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.754946] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.754997] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.755044] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.755089] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.755167] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.755343] pnp: PnP ACPI: found 7 devices
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.767647] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.778517] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.778520] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.778522] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.778523] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.778557] NET: Registered protocol family 2
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.783120] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.790254] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.797407] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.803093] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.808309] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.814986] NET: Registered protocol family 1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.819204] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.823755] PCI: CLS 0 bytes, default 64
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    1.823814] Unpacking initramfs...
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.927360] Freeing initrd memory: 21432K
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.930118] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.935676] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.943550] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.951026] hw unit of domain pp0-core 2^-0 Joules
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.955303] hw unit of domain package 2^-0 Joules
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.958332] hw unit of domain dram 2^-0 Joules
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.961924] Scanning for low memory corruption every 60 seconds
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.967898] audit: initializing netlink subsys (disabled)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.971037] audit: type=2000 audit(1534286404.949:1): initialized
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.975273] Initialise system trusted keyring
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.979618] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    3.9840:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.110177] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.137688] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.164955] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.192682] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.221252] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.228808] Linux agpgart interface v0.103
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.237491] loop: module loaded
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.239954] libphy: Fixed MDIO Bus: probed
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.242576] tun: Universal TUN/TAP device driver, 1.6
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.246298] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.304563] PPP generic driver version 2.4.2
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.307039] ehci_hc-4d17-b690-cef3539c9a1b kernel: [    4.412587] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.416033] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.422195] registered taskstats version 1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.426881] Loading compiled-in X.509 certificates
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.431853] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.437818] zswap: loaded using pool lzo/zbud
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.444210] Key type trusted registered
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.452334] Key type encrypted registered
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.455016] ima: No TPM chip found, activating TPM-bypass!
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.459757] evm: HMAC attrs: 0x1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.462574]   Magic number: 14:531:704
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.465177] acpi LNXCPU:41: hash matches
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.468321]  cpu: hash matches
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.470478] rtc_cmos 00:00: setting system clock to 2018-08-14 22:40:06 UTC (1534286406)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.475909] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.480587] EDD information not available.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.483482] PM: Hibernation image not present or could not be loaded.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.485399] Freeing unused kernel memory: 1496K
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.488907] Write protecting the kernel read-only data: 14336k
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.493380] Freeing unused kernel memory: 1956K
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.497687] Freeing unused kernel memory: 92K
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.517949] systemd-udevd[119]: starting version 204
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.561811] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.593865] scsi host0: Virtio SCSI HBA
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.601227] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.616984] AVX version of gcm_enc/dec engaged.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.621088] AES CTR mode by8 optimization enabled
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.707625] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.707972] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.707974] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.709437] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.709439] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.709683] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.712509]  sda: sda1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.713855] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.961091] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    4.965582] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275avx       : 27713.000 MB/sec
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.151659] Btrfs loaded
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.192653] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.193964] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.281682] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.289199] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.290085] EXT4-fs (sda1): recovery complete
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.295115] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.510229] random: init: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.629504] random: mountall: uninitialized urandom read (12 bytes read, 32 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.680749] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [    9.882228] random: cloud-init: uninitialized urandom read (32 bytes read, 39 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   10.497720] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   10.652669] systemd-udevd[704]: starting version 204
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   10.804832] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   10.861230] intel_rapl: no valid rapl domains found in package 0
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   10.921392] ppdev: user-space parallel port driver
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   11.063517] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   11.130541] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   11.214635] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   11.396307] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   11.702240] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug 14 22:4up: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 22:40:14 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Starting Google Accounts daemon.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Creating a new user account for me.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Created user account me.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Creating a new user account for aj.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Created user account aj.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Creating a new user account for carmen.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Created user account carmen.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-accounts: INFO Removing user packer.
Aug 14 22:40:15 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   13.901054] floppy0: no floppy controllers found
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   13.975509] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   13.981598] Bridge firewalling registered
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   13.991767] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   14.032498] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   14.114093] Initializing XFRM netlink socket
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   14.123592] Netfilter messages via NETLINK v0.30.
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   14.127037] ctnetlink v0.93: registering with nfnetlink.
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 22:40:16 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   14.374429] random: nonblocking pool is initialized
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b cron[1622]: (CRON) INFO (pidfile fd = 3)
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b cron[1658]: (CRON) STARTUP (fork ok)
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b cron[1658]: (CRON) INFO (Running @reboot jobs)
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b acpid: starting up with netlink and the input layer
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b acpid: 1 rule loaded
Aug 14 22:40:19 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b acpid: waiting for events: event logging is off
Aug 14 22:40:20 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b haveged: haveged starting up
Aug 14 22:40:20 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   18.359249] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1757]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: proto: precision = 0.115 usec
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: Listen normally on 3 eth0 10.20.0.59 UDP 123
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: peers refreshed
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpd[1758]: Listening on routing socket on fd #21 for interface updates
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [   23.581406] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b startup-script: INFO Found startup-script in metadata.
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b startup-script: INFO startup-script: job 1 at Wed Aug 15 01:50:00 2018
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b startup-script: INFO startup-script: Return code 0.
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b startup-script: INFO startup-script: Return code 0.
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b startup-script: INFO Finished running startup scripts.
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: 
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: #############################################################
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: 1024 bc:c0:04:ed:09:63:c0:ee:56:0d:c3:5a:2f:bf:97:81  root@travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b (DSA)
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: 256 23:4b:e2:de:3d:ec:12:a2:7a:94:56:66:61:b9:7a:de  root@travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b (ECDSA)
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: 256 66:f7:4d:47:c6:d8:21:90:ca:f3:38:0a:21:49:48:e5  root@travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b (ED25519)
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: 2048 c5:af:d7:3c:21:66:4c:ed:dc:9f:86:d8:bd:bd:d4:5b  root@travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b (RSA)
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 22:40:25 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ec2: #############################################################
Aug 14 22:40:30 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b ntpdate[2151]: the NTP socket is in use, exiting
Aug 14 22:42:01 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  119.301229] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  190.898698] device veth45da92b entered promiscuous mode
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  190.898777] docker0: port 1(veth45da92b) entered forwarding state
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  190.898786] docker0: port 1(veth45da92b) entered forwarding state
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  190.899262] docker0: port 1(veth45da92b) entered disabled state
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  191.012665] cgroup: docker-runc (4751) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  191.012668] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  191.092332] eth0: renamed from veth49aa32d
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  191.135096] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  191.136470] docker0: port 1(veth45da92b) entered forwarding state
Aug 14 22:43:12 travis-job-da213d42-41a7-4d17-b690-cef3539c9a1b kernel: [  0Ktravis_time:start:1cb98355
