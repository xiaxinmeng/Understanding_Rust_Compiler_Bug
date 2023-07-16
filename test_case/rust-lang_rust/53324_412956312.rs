plain

[00:04:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:24] tidy error: /checkout/src/librustc_resolve/lib.rs:2987: line longer than 100 chars
[00:04:25] some tidy checks failed
[00:04:25] 
[00:04:25] 
[00:04:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:25] 
[00:04:25] 
[00:04:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:25] Build completed unsuccessfully in 0:00:55
[00:04:25] Build completed unsuccessfully in 0:00:55
[00:04:26] make: *** [tidy] Error 1
[00:04:26] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06df803f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:07790d9a
$ sudo tail -n 500 /var/log/syslog
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] kvm-clock: using sched offset of 1426572331 cycles
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Zone ranges:
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   Device   empty
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Movable zone start for each node
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Early memory node ranges
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Policy zone: Normal
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.319981] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.321454] pid_max: default: 32768 minimum: 301
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.322103] ACPI: Core revision 20150930
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.328617] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.330151] Security Framework initialized
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.330915] Yama: becoming mindful.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.331534] AppArmor: AppArmor disabled by boot time parameter
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.334187] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.343354] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.347781] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.348995] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.350351] Initializing cgroup subsys io
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.351018] Initializing cgroup subsys memory
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.351921] Initializing cgroup subsys devices
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.352731] Initializing cgroup subsys freezer
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.353682] Initializing cgroup subsys net_cls
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.354299] Initializing cgroup subsys perf_event
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.355017] Initializing cgroup subsys net_prio
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.355888] Initializing cgroup subsys hugetlb
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.356530] Initializing cgroup subsys pids
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.357377] CPU: Physical Processor ID: 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.358136] CPU: Processor Core ID: 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.358686] mce: CPU supports 32 MCE banks
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.359538] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.360437] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.363691] Freeing SMP alternatives memory: 32K
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.373557] ftrace: allocating 32185 entries in 126 pages
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.427564] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.428578] smpboot: Max logical packages: 2
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.430093] x2apic enabled
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.431612] Switched APIC routing to physical x2apic.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.435029] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.543115] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.544955] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.547222] x86: Booting SMP configuration:
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.547995] .... node  #0, CPUs:      #1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.549235] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.552900]  #2
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.553354] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.557304]  #3
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.557898] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.561276] x86: Booted up 1 node, 4 CPUs
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.561964] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.564550] devtmpfs: initialized
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.568680] evm: security.selinux
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.569320] evm: security.SMACK64
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.569909] evm: security.SMACK64EXEC
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.570483] evm: security.SMACK64TRANSMUTE
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.571359] evm: security.SMACK64MMAP
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.571991] evm: security.ima
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.572634] evm: security.capability
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.573598] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.574971] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.576556] pinctrl core: initialized pinctrl subsystem
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.577821] RTC time: 17:32:48, date: 08/14/18
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.579428] NET: Registered protocol family 16
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.591145] cpuidle: using governor ladder
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.603141] cpuidle: using governor menu
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.603781] PCCT header not found.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.604639] ACPI: bus type PCI registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.605241] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.606596] PCI: Using configuration type 1 for base access
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.620182] ACPI: Added _OSI(Module Device)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.620914] ACPI: Added _OSI(Processor Device)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.621666] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.622374] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.625963] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.649040] ACPI: Interpreter enabled
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.649662] ACPI: (supports S0 S3 S4 S5)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.650262] ACPI: Using IOAPIC for interrupt routing
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.651092] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.680511] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.681988] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.682952] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.684223] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.687151] PCI host bridge to bus 0000:00
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.687897] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.688972] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.690088] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.691856] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.692897] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.693770] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.694184] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.707087] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.721726] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.723056] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.727935] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.731821] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.744629] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.749950] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.754430] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.768051] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.770267] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.772626] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.774958] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.776987] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.796800] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.797803] vgaarb: loaded
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.798393] SCSI subsystem initialized
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.799021] libata version 3.00 loaded.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.799042] ACPI: bus type USB registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.799633] usbcore: registered new interface driver usbfs
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.800395] usbcore: registered new interface driver hub
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.801468] usbcore: registered new device driver usb
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.802410] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.803422] dmi: Firmware registration failed.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.804205] PCI: Using ACPI for IRQ routing
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.804852] PCI: pci_cache_line_size set to 64 bytes
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.804959] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.804961] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.805095] NetLabel: Initializing
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.805686] NetLabel:  domain hash size = 128
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.806389] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.807317] NetLabel:  unlabeled traffic allowed by default
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.808534] amd_nb: Cannot enumerate AMD northbridges
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.809483] clocksource: Switched to clocksource kvm-clock
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.816707] pnp: PnP ACPI init
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817338] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817413] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817460] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817537] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817584] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817626] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817668] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.817828] pnp: PnP ACPI: found 7 devices
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.825689] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.827254] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.827257] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.827259] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.827261] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.827295] NET: Registered protocol family 2
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.828144] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.829726] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.831035] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.832212] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.833264] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.834990] NET: Registered protocol family 1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.835672] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.836599] PCI: CLS 0 bytes, default 64
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    0.836647] Unpacking initramfs...
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.833612] Freeing initrd memory: 21432K
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.834433] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.835350] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.837305] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.838738] hw unit of domain pp0-core 2^-0 Joules
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.839478] hw unit of domain package 2^-0 Joules
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.840118] hw unit of domain dram 2^-0 Joules
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.840810] Scanning for low memory corruption every 60 seconds
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.842159] audit: initializing netlink subsys (disabled)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.843046] audit: type=2000 audit(1534267970.271:1): initialized
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.844340] Initialise system trusted keyring
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.845621] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.846662] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.848960] zbud: loaded
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.849610] VFS: Disk quotas dquot_6.6.0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.850275] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.851496] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.852680] fuse init (API version 7.23)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.853672] Key type big_key registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.854350] Allocating IMA MOK and blacklist keyrings.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.856426] Key type asymmetric registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.857106] Asymmetric key parser 'x509' registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.858001] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.859197] io scheduler noop registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.859828] io scheduler deadline registered (default)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.860728] io scheduler cfq registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.861429] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.862333] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.863325] intel_idle: does not run on family 6 model 45
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.863430] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.864590] ACPI: Power Button [PWRF]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.865153] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.866436] ACPI: Sleep Button [SLPF]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.867448] GHES: HEST is not enabled!
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.870500] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.871400] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.875695] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.876810] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.881920] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.904669] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.929180] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.952618] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.976389] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.980634] Linux agpgart interface v0.103
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.983828] loop: module loaded
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.984782] libphy: Fixed MDIO Bus: probed
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.985616] tun: Universal TUN/TAP device driver, 1.6
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    2.986715] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.019115] PPP generic driver version 2.4.2
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.020132] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.021590] ehci-pci: EHCI PCI platform driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.022836] ehci-platform: EHCI generic platform driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.023981] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.025783] ohci-pci: OHCI PCI platform driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.026903] ohci-platform: OHCI generic platform driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.027816] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.029259] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.031653] i8042: Warning: Keylock active
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.033320] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.034168] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.035641] mousedev: PS/2 mouse device common for all mice
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.037256] rtc_cmos 00:00: RTC can wake from S4
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.039159] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.041068] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.043317] i2c /dev entries driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.044600] device-mapper: uevent: version 1.0.3
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.046503] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.049002] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.051766] NET: Registered protocol family 10
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.053157] NET: Registered protocol family 17
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.054559] Key type dns_resolver registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.056159] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.057917] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.059813] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.061453] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.062688] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.065426] registered taskstats version 1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.066609] Loading compiled-in X.509 certificates
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.068097] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.071318] zswap: loaded using pool lzo/zbud
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.074659] Key type trusted registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.079831] Key type encrypted registered
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.080683] ima: No TPM chip found, activating TPM-bypass!
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.081829] evm: HMAC attrs: 0x1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.082913]   Magic number: 14:345:542
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.084187] rtc_cmos 00:00: setting system clock to 2018-08-14 17:32:50 UTC (1534267970)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.086203] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.088144] EDD information not available.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.089467] PM: Hibernation image not present or could not be loaded.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.090924] Freeing unused kernel memory: 1496K
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.091875] Write protecting the kernel read-only data: 14336k
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.093520] Freeing unused kernel memory: 1956K
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.094650] Freeing unused kernel memory: 92K
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.108736] systemd-udevd[118]: starting version 204
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.162440] scsi host0: Virtio SCSI HBA
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.166430] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.174209] AVX version of gcm_enc/dec engaged.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.176047] AES CTR mode by8 optimization enabled
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.211235] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.211246] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.214246] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.215635] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.216544] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.216673] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.220039]  sda: sda1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.221095] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.233855] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.837574] tsc: Refined TSC clocksource calibration: 2600.003 MHz
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    3.838720] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ef07ae, max_idle_ns: 440795268508 ns
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    4.066634] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    6.161673] floppy0: no floppy controllers found
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.325508] raid6: sse2x1   gen()  8898 MB/s
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.393501] raid6: sse2x1   xor()  6713 MB/s
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.461495] raid6: sse2x2   gen() 10999 MB/s
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.529494] raid6: sse2x2   xor()  7381 MB/s
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.597493] raid6: sse2x4   gen() 12840 MB/s
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.665490] raid6: sse2x4   xor()  9143 MB/s
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.666254] raid6: using algorithm sse2x4 gen() 12840 MB/s
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.667273] raid6: .... xor() 9143 MB/s, rmw enabled
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.668105] raid6: using ssse3x2 recovery algorithm
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.670139] xor: automatically using best checksumming function:
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.709492]    avx       : 27799.000 MB/sec
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.723363] Btrfs loaded
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.770934] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.772047] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.852074] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.859634] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.860618] EXT4-fs (sda1): recovery complete
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    7.866509] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    8.097091] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    8.247589] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    8.306058] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    8.555953] random: cloud-init: uninitialized urandom read (32 bytes read, 36 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.229434] random: cloud-init: uninitialized urandom read (32 bytes read, 44 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.373585] systemd-udevd[702]: starting version 204
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.516811] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.563845] intel_rapl: no valid rapl domains found in package 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.617699] ppdev: user-space parallel port driver
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.723143] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.778499] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [    9.853137] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   10.021356] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   10.300496] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   10.388915] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   10.477365] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   10.538825] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   10.796921] init: failsafe main process (1094) killed by TERM signal
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Running set_multiqueue.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 17:32:58 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for me.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account me.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for henrik.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account henrik.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for emma.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account emma.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for igor.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account igor.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account konstantinhaase.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for aj.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account aj.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for solarce.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account solarce.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for asari.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account asari.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.236642] random: nonblocking pool is initialized
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for bogdana.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.266224] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.270268] Bridge firewalling registered
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.283237] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account bogdana.
Aug 14 17:32:59 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for konstantin.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.327176] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account konstantin.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for carmen.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.404267] Initializing XFRM netlink socket
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.414209] Netfilter messages via NETLINK v0.30.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.417511] ctnetlink v0.93: registering with nfnetlink.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account carmen.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Creating a new user account for maria.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Created user account maria.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 google-accounts: INFO Removing user packer.
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   12.577668] floppy0: no floppy controllers found
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 17:33:00 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 cron[1711]: (CRON) INFO (pidfile fd = 3)
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 cron[1742]: (CRON) STARTUP (fork ok)
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 cron[1742]: (CRON) INFO (Running @reboot jobs)
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 acpid: starting up with netlink and the input layer
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 acpid: 1 rule loaded
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 acpid: waiting for events: event logging is off
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 haveged: haveged starting up
Aug 14 17:33:01 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   13.918426] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1845]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: proto: precision = 0.125 usec
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen normally on 3 eth0 10.20.1.39 UDP 123
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: peers refreshed
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listening on routing socket on fd #21 for interface updates
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [   19.134441] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 startup-script: INFO Found startup-script in metadata.
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 startup-script: INFO startup-script: job 1 at Tue Aug 14 20:43:00 2018
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 startup-script: INFO startup-script: Return code 0.
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 startup-script: INFO startup-script: Return code 0.
Aug 14 17:33:06 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 startup-script: INFO Finished running startup scripts.
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: 
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: #############################################################
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: 1024 e5:a9:eb:9c:0a:5c:12:6f:5c:8d:e5:12:97:73:f0:ae  root@travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 (DSA)
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: 256 db:83:98:fd:c2:1d:e0:d9:26:6f:ed:93:0c:63:be:47  root@travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 (ECDSA)
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: 256 4b:63:3d:fe:98:55:cd:53:3e:15:9a:5e:88:a4:46:c5  root@travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 (ED25519)
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: 2048 2f:e9:b9:0b:94:af:36:e2:d3:6a:b2:b2:9e:de:b8:9c  root@travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 (RSA)
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 17:33:07 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ec2: #############################################################
Aug 14 17:33:15 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpdate[1925]: the NTP socket is in use, exiting
Aug 14 17:35:51 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  183.985161] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.748374] device veth9b12aeb entered promiscuous mode
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.854676] cgroup: docker-runc (4846) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.854688] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.924984] eth0: renamed from veth41a81fa
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.966098] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.967355] docker0: port 1(veth9b12aeb) entered forwarding state
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.967389] docker0: port 1(veth9b12aeb) entered forwarding state
Aug 14 17:36:54 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  246.967421] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 17:36:57 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen normally on 5 docker0 fe80::42:6bff:fe03:6821 UDP 123
Aug 14 17:36:57 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 14 17:36:57 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 17:36:57 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: peers refreshed
Aug 14 17:36:57 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 ntpd[1846]: new interface(s) found: waking up resolver
Aug 14 17:37:09 travis-job-a44da1ef-6433-40e7-9025-c033f7e109d4 kernel: [  261.980204] docker0: port 1(veth9b12aeb) entered forwarding state
---
travis_time:end:0fb5825e:start=1534268419406571285,finish=1534268419414830791,duration=8259506
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2e4b6158
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1046cfdc
travis_time:start:1046cfdc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0dcc8664
$ dmesg | grep -i kill
