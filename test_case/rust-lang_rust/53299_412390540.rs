plain

[00:04:00] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:00] tidy error: /checkout/src/libcore/macros.rs: missing trailing newline
[00:04:02] some tidy checks failed
[00:04:02] 
[00:04:02] 
[00:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:02] 
[00:04:02] 
[00:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:02] Build completed unsuccessfully in 0:00:52
[00:04:02] Build completed unsuccessfully in 0:00:52
[00:04:02] Makefile:79: recipe for target 'tidy' failed
[00:04:02] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:061d0e29
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:264b347e
$ sudo tail -n 500 /var/log/syslog
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] kvm-clock: using sched offset of 1774955375 cycles
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Zone ranges:
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   Device   empty
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Movable zone start for each node
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Early memory node ranges
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Policy zone: Normal
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] console [ttyS0] enabled
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.514694] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.517828] pid_max: default: 32768 minimum: 301
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.520138] ACPI: Core revision 20150930
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.527689] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.530520] Security Framework initialized
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.531722] Yama: becoming mindful.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.533088] AppArmor: AppArmor disabled by boot time parameter
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.537535] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.550674] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.556957] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.560560] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.563754] Initializing cgroup subsys io
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.565652] Initializing cgroup subsys memory
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.567044] Initializing cgroup subsys devices
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.569402] Initializing cgroup subsys freezer
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.571391] Initializing cgroup subsys net_cls
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.573150] Initializing cgroup subsys perf_event
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.575262] Initializing cgroup subsys net_prio
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.577584] Initializing cgroup subsys hugetlb
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.579221] Initializing cgroup subsys pids
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.580829] CPU: Physical Processor ID: 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.582327] CPU: Processor Core ID: 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.584750] mce: CPU supports 32 MCE banks
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.586576] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.588526] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.593669] Freeing SMP alternatives memory: 32K
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.606029] ftrace: allocating 32185 entries in 126 pages
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.667995] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.670168] smpboot: Max logical packages: 2
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.672226] x2apic enabled
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.675010] Switched APIC routing to physical x2apic.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.680687] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.789282] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.793179] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.798168] x86: Booting SMP configuration:
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.799855] .... node  #0, CPUs:      #1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.801700] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.807600]  #2
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.808509] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.814360]  #3
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.815371] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.821586] x86: Booted up 1 node, 4 CPUs
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.823195] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.827404] devtmpfs: initialized
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.833035] evm: security.selinux
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.834166] evm: security.SMACK64
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.835474] evm: security.SMACK64EXEC
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.837136] evm: security.SMACK64TRANSMUTE
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.840091] evm: security.SMACK64MMAP
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.841679] evm: security.ima
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.842700] evm: security.capability
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.844714] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.847874] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.850677] pinctrl core: initialized pinctrl subsystem
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.852758] RTC time:  1:48:32, date: 08/13/18
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.855288] NET: Registered protocol family 16
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.869363] cpuidle: using governor ladder
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.881371] cpuidle: using governor menu
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.883497] PCCT header not found.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.885469] ACPI: bus type PCI registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.887197] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.889684] PCI: Using configuration type 1 for base access
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.907133] ACPI: Added _OSI(Module Device)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.908907] ACPI: Added _OSI(Processor Device)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.910691] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.912557] ACPI: Added _OSI(Processor Aggregator Device)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.917474] ACPI: Executed 2 blocks of module-level executable AML code
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.943693] ACPI: Interpreter enabled
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.945023] ACPI: (supports S0 S3 S4 S5)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.946632] ACPI: Using IOAPIC for interrupt routing
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.948264] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.982756] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.984652] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.986984] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.989217] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.994028] PCI host bridge to bus 0000:00
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.995549] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    0.998531] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.000766] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.003940] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.006516] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.008708] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.009187] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.037436] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.061693] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.064714] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.074736] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.081920] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.106146] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.117482] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.126465] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.151115] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.154994] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.159862] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.163937] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.168435] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.191118] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.193411] vgaarb: loaded
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.194585] SCSI subsystem initialized
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.196179] libata version 3.00 loaded.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.196220] ACPI: bus type USB registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.197962] usbcore: registered new interface driver usbfs
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.199736] usbcore: registered new interface driver hub
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.201598] usbcore: registered new device driver usb
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.203750] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.206605] dmi: Firmware registration failed.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.208794] PCI: Using ACPI for IRQ routing
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.210151] PCI: pci_cache_line_size set to 64 bytes
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.210259] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.210261] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.210449] NetLabel: Initializing
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.212109] NetLabel:  domain hash size = 128
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.213732] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.215618] NetLabel:  unlabeled traffic allowed by default
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.217748] amd_nb: Cannot enumerate AMD northbridges
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.219829] clocksource: Switched to clocksource kvm-clock
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.229580] pnp: PnP ACPI init
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.230893] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.230965] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.231009] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.231059] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.231110] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.231149] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.231189] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.231358] pnp: PnP ACPI: found 7 devices
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.240156] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.243862] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.243864] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.243866] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.243867] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.243916] NET: Registered protocol family 2
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.246367] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.250423] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.254009] TCP: Hash tables configured (established 131072 bind 65536)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.257552] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.261950] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.266581] NET: Registered protocol family 1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.268707] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.271028] PCI: CLS 0 bytes, default 64
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    1.271105] Unpacking initramfs...
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.480911] Freeing initrd memory: 21432K
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.482887] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.488183] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.493945] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.498898] hw unit of domain pp0-core 2^-0 Joules
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.500947] hw unit of domain package 2^-0 Joules
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.503481] hw unit of domain dram 2^-0 Joules
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.506008] Scanning for low memory corruption every 60 seconds
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.510524] audit: initializing netlink subsys (disabled)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.512848] audit: type=2000 audit(1534124915.033:1): initialized
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.517243] Initialise system trusted keyring
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.519451] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.522412] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.526967] zbud: loaded
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.528764] VFS: Disk quotas dquot_6.6.0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.530642] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.534591] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.537710] fuse init (API version 7.23)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.540214] Key type big_key registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.542360] Allocating IMA MOK and blacklist keyrings.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.547961] Key type asymmetric registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.549217] Asymmetric key parser 'x509' registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.552524] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.557060] io scheduler noop registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.558558] io scheduler deadline registered (default)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.560952] io scheduler cfq registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.563000] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.565668] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.568093] intel_idle: does not run on family 6 model 62
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.568284] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.572926] ACPI: Power Button [PWRF]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.574675] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.577887] ACPI: Sleep Button [SLPF]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.580471] GHES: HEST is not enabled!
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.585262] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.588558] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.597319] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.599880] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.612647] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.638028] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.664653] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.690716] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.716699] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.722660] Linux agpgart interface v0.103
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.728605] loop: module loaded
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.730522] libphy: Fixed MDIO Bus: probed
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.733514] tun: Universal TUN/TAP device driver, 1.6
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.736267] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.794024] PPP generic driver version 2.4.2
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.796579] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.800959] ehci-pci: EHCI PCI platform driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.803108] ehci-platform: EHCI generic platform driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.805572] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.810180] ohci-pci: OHCI PCI platform driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.812525] ohci-platform: OHCI generic platform driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.814867] uhci_hcd: USB Universal Host Controller Interface driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.817758] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.822677] i8042: Warning: Keylock active
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.825927] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.828106] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.832191] mousedev: PS/2 mouse device common for all mice
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.835252] rtc_cmos 00:00: RTC can wake from S4
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.837914] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.842486] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.845789] i2c /dev entries driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.847858] device-mapper: uevent: version 1.0.3
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.850980] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.856123] ledtrig-cpu: registered to indicate activity on CPUs
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.861072] NET: Registered protocol family 10
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.864571] NET: Registered protocol family 17
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.867244] Key type dns_resolver registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.870108] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.873595] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.877002] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.880059] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.882933] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.888700] registered taskstats version 1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.891754] Loading compiled-in X.509 certificates
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.896075] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.901181] zswap: loaded using pool lzo/zbud
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.906854] Key type trusted registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.915085] Key type encrypted registered
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.917294] ima: No TPM chip found, activating TPM-bypass!
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.920798] evm: HMAC attrs: 0x1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.923698]   Magic number: 14:877:811
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.926112] acpi LNXCPU:8d: hash matches
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.928683] rtc_cmos 00:00: setting system clock to 2018-08-13 01:48:35 UTC (1534124915)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.933377] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.936456] EDD information not available.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.938917] PM: Hibernation image not present or could not be loaded.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.940872] Freeing unused kernel memory: 1496K
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.943390] Write protecting the kernel read-only data: 14336k
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.947586] Freeing unused kernel memory: 1956K
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.950761] Freeing unused kernel memory: 92K
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    3.971530] systemd-udevd[119]: starting version 204
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.045266] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.045841] scsi host0: Virtio SCSI HBA
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.056535] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.066566] AVX version of gcm_enc/dec engaged.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.068116] AES CTR mode by8 optimization enabled
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.122877] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.122911] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.122913] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.123442] sd 0:0:1:0: [sda] Write Protect is off
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.123444] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.123513] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.126835]  sda: sda1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.128437] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.503959] tsc: Refined TSC clocksource calibration: 2499.777 MHz
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.504991] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24086721a4d, max_idle_ns: 440795248663 ns
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    4.889032] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    7.056089] floppy0: no floppy controllers found
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.235832] raid6: sse2x1   gen()  9247 MB/s
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.303830] raid6: sse2x1   xor()  6805 MB/s
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.371833] raid6: sse2x2   gen() 11395 MB/s
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.439828] raid6: sse2x2   xor()  7850 MB/s
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.507831] raid6: sse2x4   gen() 12420 MB/s
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.575828] raid6: sse2x4   xor()  7964 MB/s
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.576751] raid6: using algorithm sse2x4 gen() 12420 MB/s
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.577844] raid6: .... xor() 7964 MB/s, rmw enabled
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.578653] raid6: using ssse3x2 recovery algorithm
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.580927] xor: automatically using best checksumming function:
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.619848]    avx       : 22153.000 MB/sec
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.635723] Btrfs loaded
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.681270] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.683071] EXT4-fs (sda1): write access will be enabled during recovery
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.770437] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.784906] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.786618] EXT4-fs (sda1): recovery complete
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    8.794397] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    9.041934] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    9.192609] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    9.254668] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [    9.492893] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   10.165183] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   10.324625] systemd-udevd[702]: starting version 204
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   10.464443] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   10.554216] ppdev: user-space parallel port driver
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   10.678316] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   10.748697] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   10.827229] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   11.003970] random: cloud-init: uninitialized urandom read (32 bytes read, 54 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   11.305184] random: mktemp: uninitialized urandom read (12 bytes read, 57 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   11.396366] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   11.495984] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   11.552159] EXT4-fs (sda1): resized filesystem to 7864064
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   11.979405] init: failsafe main process (1095) killed by TERM signal
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Running set_multiqueue.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Set channels for eth0 to 4.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 13 01:48:43 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Starting Google Accounts daemon.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for me.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account me.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for henrik.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account henrik.
Aug 13 01:48:44 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for emma.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-clock-skew: INFO Synced system time with hardware clock.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account emma.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for igor.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account igor.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.214869] random: nonblocking pool is initialized
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account konstantinhaase.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for aj.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account aj.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for solarce.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account solarce.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for asari.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account asari.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.474849] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.481074] Bridge firewalling registered
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for bogdana.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.495478] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.537247] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account bogdana.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.556242] floppy0: no floppy controllers found
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for konstantin.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account konstantin.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.650607] Initializing XFRM netlink socket
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for carmen.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.665515] Netfilter messages via NETLINK v0.30.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   13.669578] ctnetlink v0.93: registering with nfnetlink.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account carmen.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Creating a new user account for maria.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Created user account maria.
Aug 13 01:48:45 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 google-accounts: INFO Removing user packer.
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 cron[1709]: (CRON) INFO (pidfile fd = 3)
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 cron[1743]: (CRON) STARTUP (fork ok)
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 cron[1743]: (CRON) INFO (Running @reboot jobs)
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 acpid: starting up with netlink and the input layer
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 acpid: 1 rule loaded
Aug 13 01:48:46 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 acpid: waiting for events: event logging is off
Aug 13 01:48:47 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 haveged: haveged starting up
Aug 13 01:48:47 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   15.206712] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1843]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: proto: precision = 0.205 usec
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen normally on 3 eth0 10.20.0.57 UDP 123
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: peers refreshed
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listening on routing socket on fd #21 for interface updates
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   20.428611] init: plymouth-upstart-bridge main process ended, respawning
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 startup-script: INFO Found startup-script in metadata.
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 startup-script: INFO startup-script: job 1 at Mon Aug 13 04:58:00 2018
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 startup-script: INFO startup-script: Return code 0.
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 startup-script: INFO startup-script: Return code 0.
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 startup-script: INFO Finished running startup scripts.
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: 
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: #############################################################
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: 1024 d2:35:d6:b1:0f:31:cf:49:e0:f9:44:c6:88:8f:ed:94  root@travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 (DSA)
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: 256 ce:58:98:55:57:2f:b8:7c:f9:16:a9:2c:69:25:b4:27  root@travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 (ECDSA)
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: 256 f5:5c:16:7f:3b:54:e0:97:23:86:b1:8b:94:44:6e:b6  root@travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 (ED25519)
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: 2048 70:da:fc:cf:4a:a6:fa:d1:2d:9a:f3:21:12:a2:f7:56  root@travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 (RSA)
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 13 01:48:52 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ec2: #############################################################
Aug 13 01:49:00 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpdate[2456]: the NTP socket is in use, exiting
Aug 13 01:49:27 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [   55.206255] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.427942] device veth68dfd44 entered promiscuous mode
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.532165] cgroup: docker-runc (4832) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.532170] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.619448] eth0: renamed from veth64cd94b
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.663591] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.664984] docker0: port 1(veth68dfd44) entered forwarding state
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.665000] docker0: port 1(veth68dfd44) entered forwarding state
Aug 13 01:50:33 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  121.665019] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 13 01:50:37 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 13 01:50:37 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen normally on 6 docker0 fe80::42:86ff:fece:aa9 UDP 123
Aug 13 01:50:37 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 13 01:50:37 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: peers refreshed
Aug 13 01:50:37 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 ntpd[1844]: new interface(s) found: waking up resolver
Aug 13 01:50:48 travis-job-bbac92d9-b320-42dd-9e86-ed0781424c22 kernel: [  136.707237] docker0: port 1(veth68dfd44) entered forwarding state
---
travis_time:end:0c53d494:start=1534125211069911965,finish=1534125211075888302,duration=5976337
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09a19b21
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f5f28b4
travis_time:start:0f5f28b4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:017e679d
$ dmesg | grep -i kill
