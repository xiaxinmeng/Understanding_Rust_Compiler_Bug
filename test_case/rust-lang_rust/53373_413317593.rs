plain
[00:48:03] ....................................................................................................
[00:48:06] ..........................................................................................i.........
[00:48:09] .........................................................i..........................................
[00:48:12] ....................................................................................................
[00:48:15] .......................F............................................................................
[00:48:21] ....................................................................................................
[00:48:24] ....................................................................................................
[00:48:27] ....................................................................................................
[00:48:30] ....................................................................................................
---
[00:49:32] 
[00:49:32] ---- [ui] ui/issue-10636-1.rs stdout ----
[00:49:32] diff of stderr:
[00:49:32] 
[00:49:32] 8    | ^ incorrect close delimiter
[00:49:32] 10 error: aborting due to previous error
[00:49:32] + 
[00:49:32] 11 
[00:49:32] 
[00:49:32] 
[00:49:32] 
[00:49:32] The actual stderr differed from the expected stderr.
[00:49:32] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-10636-1/issue-10636-1.stderr
[00:49:32] To update references, rerun the tests and pass the `--bless` flag
[00:49:32] To only update this specific test, also pass `--test-args issue-10636-1.rs`
[00:49:32] error: 1 errors occurred comparing output.
[00:49:32] status: exit code: 1
[00:49:32] status: exit code: 1
[00:49:32] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-10636-1.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-10636-1/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-10636-1/auxiliary" "-A" "unused"
[00ntest.rs:3166:9
[00:49:32] 
[00:49:32] 
[00:49:32] failures:
[00:49:32]     [ui] ui/issue-10636-1.rs
[00:49:32]     [ui] ui/issue-10636-1.rs
[00:49:32] 
[00:49:32] test result: FAILED. 4108 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
[00:49:32] 
[00:49:32] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:497:22
[00:49:32] 
[00:49:32] 
[00:49:32] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:32] 
[00:49:32] 
[00:49:32] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:32] Build completed unsuccessfully in 0:03:15
[00:49:32] Build completed unsuccessfully in 0:03:15
[00:49:32] Makefile:58: recipe for target 'check' failed
[00:49:32] make: *** [check] Error 1
cb9 kernel: [    0.000000]   5 disabled
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   6 disabled
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   7 disabled
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] x86/PAT: Configuration [0-7]: WB  WC  UC- UC  WB  WC  UC- WT  
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] e820: last_pfn = 0xbfff3 max_arch_pfn = 0x400000000
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] found SMP MP-table at [mem 0x000f2800-0x000f280f] mapped at [ffff8800000f2800]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Scanning 1 areas for low memory corruption
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Using GB pages for direct mapping
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a87G 00000001)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] kvm-clock: using sched offset of 1863876186 cycles
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Zone ranges:
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   Device   empty
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Movable zone start for each node
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Early memory node ranges
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] PM: Registered nosave memory: [8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Policy zone: Normal
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.483007] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.487292] pid_max: default: 32768 minimum: 301
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.489197] ACPI: Core revision 20150930
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.497105] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.500386] Security Framework initialized
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.501937] Yama: becoming mindful.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.503521] AppArmor: AppArmor disabled by boot time parameter
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.507152] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.518578] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.525214] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.527398] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.529973] Initializing cgroup subsys io
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.531580] Initializing cgroup subsys memory
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.533379] Initializing cgroup subsys devices
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.535567] Initializing cgroup subsys freezer
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.538292] Initializing cgroup subsys net_cls
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.540231] Initializing cgroup subsys perf_event
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.542349] Initializing cgroup subsys net_prio
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.544347] Initializing cgroup subsys hugetlb
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.546498] Initializing cgroup subsys pids
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.548279] CPU: Physical Processor ID: 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.549940] CPU: Processor Core ID: 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.552280] mce: CPU supports 32 MCE banks
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.554177] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.555967] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.560739] Freeing SMP alternatives memory: 32K
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.573362] ftrace: allocating 32185 entries in 126 pages
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.633830] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.636463] smpboot: Max logical packages: 2
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.638984] x2apic enabled
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.641355] Switched APIC routing to physical x2apic.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.646053] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.753291] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.757657] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.763413] x86: Booting SMP configuration:
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.765238] .... node  #0, CPUs:      #1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.767052] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.772980]  #2
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.774023] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.779968]  #3
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.780788] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.787229] x86: Booted up 1 node, 4 CPUs
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.789185] smpboot: Total of 4 09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.833363] cpuidle: using governor ladder
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.845366] cpuidle: using governor menu
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.846872] PCCT header not found.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.848195] ACPI: bus type PCI registered
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.849438] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.851840] PCI: Using configuration type 1 for base access
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.867649] ACPI: Added _OSI(Module Device)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.869266] ACPI: Added _OSI(Processor Device)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.870777] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.873207] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.878911] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.906317] ACPI: Interpreter enabled
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.908237] ACPI: (supports S0 S3 S4 S5)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.909835] ACPI: Using IOAPIC for interrupt routing
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.911474] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.945384] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.947498] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.949814] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.953102] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.957885] PCI host bridge to bus 0000:00
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.959497] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.961641] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.964577] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.966948] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.970069] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.972175] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    0.972645] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.000268] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.028319] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.031415] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.041073] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.049241] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.073377] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.082251] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.090605] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.115809] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.121036] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.126401] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.130656] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.135715] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.160313] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.162452] vgaarb: loaded
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.163704] SCSI subsystem initialized
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.165545] libata version 3.00 loaded.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.165576] ACPI: bus type USB registered
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.167168] usbcore: registered new interface driver usbfs
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.169249] usbcore: registered new interface driver hub
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.172032] usbcore: registered new device driver usb
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.174411] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.176887] dmi: Firmware registration failed.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.180245] PCI: Using ACPI for IRQ routing
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.182134] PCI: pci_cache_line_size set to 64 bytes
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.182244] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.182246] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.182440] NetLabel: Initializing
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.184113] NetLabel:  domain hash size = 128
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.185865] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.187911] NetLabel:  unlabeled traffic allowed by default
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.190017] amd_nb: Cannot enumerate AMD northbridges
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.192225] clocksource: Switched to clocksource kvm-clock
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.202126] pnp: PnP ACPI init
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   x000a0000-0x000bffff window]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.221551] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.221593] NET: Registered protocol family 2
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.224375] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.229477] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.234150] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.237404] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.240680] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.243591] NET: Registered protocol family 1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.245778] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.248778] PCI: CLS 0 bytes, default 64
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    1.248853] Unpacking initramfs...
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.445472] Freeing initrd memory: 21432K
: [    3.498773] io scheduler cfq registered
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.500168] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.502270] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.504678] intel_idle: does not run on family 6 model 62
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.504808] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.507803] ACPI: Power Button [PWRF]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.509444] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.512121] ACPI: Sleep Button [SLPF]
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.514260] GHES: HEST is not enabled!
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.520082] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.522533] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.530862] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.533135] virtio-pci 0000:008-a875-ffd166310cb9 kernel: [    3.774561] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.777833] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.781461] registered taskstats version 1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.784774] Loading compiled-in X.509 certificates
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.788123] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.795240] zswap: loaded using pool lzo/zbud
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.801570] Key type trusted registered
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.808746] Key type encrypted registered
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.810775] ima: No TPM chip found, activating TPM-bypass!
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.813323] evm: HMAC attrs: 0x1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.815511]   Magic number: 14:770:192
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.817764] acpi device:1e: hash matches
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.819962] rtc_cmos 00:00: setting system clock to 2018-08-15 19:09:09 UTC (1534360149)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.824004] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.826995] EDD information not available.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.829430] PM: Hibernation image not present or could not be loaded.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.831282] Freeing unused kernel memory: 1496K
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.833402] Write protecting the kernel read-only data: 14336k
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.837508] Freeing unused kernel memory: 1956K
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.840285] Freeing unused kernel memory: 92K
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.860838] systemd-udevd[119]: starting version 204
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.940358] scsi host0: Virtio SCSI HBA
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.940515] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.951141] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    3.957712] AVX version of gcm_enc/dec eric Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    6.908518] floppy0: no floppy controllers found
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.100280] raid6: sse2x1   gen()  8607 MB/s
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.168308] raid6: sse2x1   xor()  6730 MB/s
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.236311] raid6: sse2x2   gen() 10925 MB/s
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.304303] raid6: sse2x2   xor()  7282 MB/s
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.372295] raid6: sse2x4   gen() 12322 MB/s
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.440333] raid6: sse2x4   xor()  8116 MB/s
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.442153] raid6: using algorithm sse2x4 gen() 12322 MB/s
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.444434] raid6: .... xor() 8116 MB/s, rmw enabled
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.446133] raid6: using ssse3x2 recovery algorithm
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.449951] xor: automatically using best checksumming function:
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.488310]    avx       : 21658.000 MB/sec
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.506539] Btrfs loaded
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.559940] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.562850] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.658294] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.674698] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.677192] EXT4-fs (sda1): recovery complete
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.685000] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    8.936897] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    9.080735] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    9.137006] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    9.353782] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [    9.948604] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.088264] systemd-udevd[702]: starting version 204
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.192642] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.310978] ppdev: user-space parallel port driver
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.407949] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.462042] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.528922] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.690034] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   10.933473] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   11.010324] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   11.095198] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   11.136291] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   11.400343] init: failsafe main process (1094) killed by TERM signal
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Running set_multiqueue.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 19:09:17 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 19:09:17 travis-job-cin, use the -r|--reseed option
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 acpid: starting up with netlink and the input layer
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 acpid: 1 rule loaded
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 acpid: waiting for events: event logging is off
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 haveged: haveged starting up
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.650654] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.661544] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.823774] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.828715] Bridge firewalling registered
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.839484] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.921489] Initializing XFRM netlink socket
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.929636] Netfilter messages via NETLINK v0.30.
Aug 15 19:09:18 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   12.933421] ctnetlink v0.93: registering with nfnetlink.
Aug 15 19:09:19 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 19:09:19 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   13.252392] floppy0: no floppy controllers found
Aug 15 19:09:41 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpdate[1730]: adjust time server 169.254.169.254 offset 0.003602 sec
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1763]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: proto: precision = 0.103 usec
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen normally on 3 eth0 10.20.0.68 UDP 123
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: peers refreshed
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listening on routing socket on fd #21 for interface updates
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [   42.863510] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 startup-script: INFO Found startup-script in metadata.
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 startup-script: INFO startup-script: job 1 at Wed Aug 15 22:19:00 2018
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 startup-script: INFO startup-script: Return code 0.
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 startup-script: INFO startup-script: Return code 0.
Aug 15 19:09:48 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 startup-script: INFO Finished running startup scripts.
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: 
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: #############################################################
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: 1024 b8:3d:e0:ca:82:0e:bd:54:e9:33:95:cb:1c:da:68:25  root@travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 (DSA)
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: 256 58:6a:0a:9f:9b:76:bc:9f:a2:d9:bf:da:71:e7:2b:89  root@travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 (ECDSA)
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: 256 f8:83:e6:13:10:b2:95:82:a3:ff:52:47:07:d8:fe:59  root@travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 (ED25519)
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: 2048 e1:ba:e6:af:bc:90:f3:e8:66:a6:ac:7e:6f:09:52:53  root@travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 (RSA)
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 19:09:49 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ec2: #############################################################
Aug 15 19:11:42 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  156.950430] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.066826] device veth398b8b5 entered promiscuous mode
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.066892] docker0: port 1(veth398b8b5) entered forwarding state
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.066899] docker0: port 1(veth398b8b5) entered forwarding state
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.067418] docker0: port 1(veth398b8b5) entered disabled state
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.168181] cgroup: docker-runc (4855) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.168184] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.251883] eth0: renamed from veth4855169
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.309266] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.310872] docker0: port 1(veth398b8b5) entered forwarding state
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.310890] docker0: port 1(veth398b8b5) entered forwarding state
Aug 15 19:13:55 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  289.310911] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 19:13:58 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen normally on 5 docker0 fe80::42:6dff:fe14:92eb UDP 123
Aug 15 19:13:58 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 15 19:13:58 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 19:13:58 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: peers refreshed
Aug 15 19:13:58 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 ntpd[1764]: new interface(s) found: waking up resolver
Aug 15 19:14:10 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [  304.335360] docker0: port 1(veth398b8b5) entered forwarding state
Aug 15 19:17:01 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 CRON[6435]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 15 20:01:16 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [ 3130.011248] docker0: port 1(veth398b8b5) entered disabled state
Aug 15 20:01:16 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [ 3130.011374] veth4855169: renamed from eth0
Aug 15 20:01:16 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [ 3130.095984] docker0: port 1(veth398b8b5) entered disabled state
Aug 15 20:01:16 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [ 3130.097950] device veth398b8b5 left promiscuous mode
Aug 15 20:01:16 travis-job-c01358b3-81a3-45a8-a875-ffd166310cb9 kernel: [ 3130.097953] docker0: port 1(veth398b8b5) entered disabled state
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:0b93fc78
---
travis_time:end:0a1294e8:start=1534363277914688454,finish=1534363277929313437,duration=14624983
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ce6f2b9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:02b4b101
$ dmesg | grep -i kill
