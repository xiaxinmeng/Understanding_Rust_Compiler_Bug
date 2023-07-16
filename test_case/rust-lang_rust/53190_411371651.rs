plain

[00:03:45] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:45] tidy error: /checkout/src/bootstrap/test.rs:986: trailing whitespace
[00:03:47] some tidy checks failed
[00:03:47] 
[00:03:47] 
[00:03:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:47] 
[00:03:47] 
[00:03:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:47] Build completed unsuccessfully in 0:00:49
[00:03:47] Build completed unsuccessfully in 0:00:49
[00:03:47] Makefile:79: recipe for target 'tidy' failed
[00:03:47] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00a12c14
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:07530a92
$ sudo tail -n 500 /var/log/syslog
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] kvm-clock: using sched offset of 1613389554 cycles
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Zone ranges:
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   Device   empty
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Movable zone start for each node
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Early memory node ranges
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Policy zone: Normal
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] console [ttyS0] enabled
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.344268] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.345847] pid_max: default: 32768 minimum: 301
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.346668] ACPI: Core revision 20150930
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.353405] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.355212] Security Framework initialized
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.356049] Yama: becoming mindful.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.356587] AppArmor: AppArmor disabled by boot time parameter
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.358989] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.369073] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.373769] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.374897] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.376774] Initializing cgroup subsys io
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.377376] Initializing cgroup subsys memory
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.378112] Initializing cgroup subsys devices
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.378872] Initializing cgroup subsys freezer
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.379684] Initializing cgroup subsys net_cls
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.380583] Initializing cgroup subsys perf_event
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.381431] Initializing cgroup subsys net_prio
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.382353] Initializing cgroup subsys hugetlb
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.383116] Initializing cgroup subsys pids
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.384052] CPU: Physical Processor ID: 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.384736] CPU: Processor Core ID: 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.386511] mce: CPU supports 32 MCE banks
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.387463] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.388503] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.391536] Freeing SMP alternatives memory: 32K
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.400677] ftrace: allocating 32185 entries in 126 pages
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.450668] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.452116] smpboot: Max logical packages: 2
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.453734] x2apic enabled
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.455805] Switched APIC routing to physical x2apic.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.459586] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.567407] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.568898] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.572364] x86: Booting SMP configuration:
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.572976] .... node  #0, CPUs:      #1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.573761] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.578208]  #2
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.578681] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.584028]  #3
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.584516] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.588830] x86: Booted up 1 node, 4 CPUs
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.589592] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.591911] devtmpfs: initialized
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.596385] evm: security.selinux
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.596881] evm: security.SMACK64
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.597386] evm: security.SMACK64EXEC
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.597894] evm: security.SMACK64TRANSMUTE
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.598469] evm: security.SMACK64MMAP
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.599161] evm: security.ima
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.599608] evm: security.capability
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.600799] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.602169] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.603303] pinctrl core: initialized pinctrl subsystem
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.604312] RTC time: 11:02:26, date: 08/08/18
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.605799] NET: Registered protocol family 16
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.615438] cpuidle: using governor ladder
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.627438] cpuidle: using governor menu
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.628160] PCCT header not found.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.628748] ACPI: bus type PCI registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.629426] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.630554] PCI: Using configuration type 1 for base access
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.644337] ACPI: Added _OSI(Module Device)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.645090] ACPI: Added _OSI(Processor Device)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.645762] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.646424] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.650097] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.673410] ACPI: Interpreter enabled
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.674114] ACPI: (supports S0 S3 S4 S5)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.674712] ACPI: Using IOAPIC for interrupt routing
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.675565] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.705433] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.706575] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.707649] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.708609] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.711022] PCI host bridge to bus 0000:00
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.711826] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.712764] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.713718] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.714828] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.716046] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.716933] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.717404] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.732575] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.749214] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.750846] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.756461] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.761859] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.786076] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.791711] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.796380] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.810634] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.812803] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.814911] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.817188] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.819786] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.840680] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.842315] vgaarb: loaded
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.843292] SCSI subsystem initialized
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.844017] libata version 3.00 loaded.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.844040] ACPI: bus type USB registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.844736] usbcore: registered new interface driver usbfs
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.846087] usbcore: registered new interface driver hub
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.846970] usbcore: registered new device driver usb
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.848202] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.849374] dmi: Firmware registration failed.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.850626] PCI: Using ACPI for IRQ routing
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.851537] PCI: pci_cache_line_size set to 64 bytes
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.851638] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.851640] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.851781] NetLabel: Initializing
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.852471] NetLabel:  domain hash size = 128
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.853244] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.854372] NetLabel:  unlabeled traffic allowed by default
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.855441] amd_nb: Cannot enumerate AMD northbridges
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.856435] clocksource: Switched to clocksource kvm-clock
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.865109] pnp: PnP ACPI init
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866399] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866464] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866506] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866553] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866591] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866630] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866670] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.866838] pnp: PnP ACPI: found 7 devices
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.874963] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.876914] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.876916] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.876918] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.876920] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.876957] NET: Registered protocol family 2
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.878763] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.880608] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.882092] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.884401] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.885591] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.887712] NET: Registered protocol family 1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.888634] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.889741] PCI: CLS 0 bytes, default 64
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    0.889802] Unpacking initramfs...
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.000311] Freeing initrd memory: 21432K
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.001211] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.002212] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.003756] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.005054] hw unit of domain pp0-core 2^-0 Joules
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.005895] hw unit of domain package 2^-0 Joules
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.006658] hw unit of domain dram 2^-0 Joules
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.007466] Scanning for low memory corruption every 60 seconds
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.009217] audit: initializing netlink subsys (disabled)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.010235] audit: type=2000 audit(1533726149.107:1): initialized
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.011427] Initialise system trusted keyring
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.012487] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.013512] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.015888] zbud: loaded
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.016653] VFS: Disk quotas dquot_6.6.0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.017296] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.018715] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.020148] fuse init (API version 7.23)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.020960] Key type big_key registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.021570] Allocating IMA MOK and blacklist keyrings.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.023759] Key type asymmetric registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.024494] Asymmetric key parser 'x509' registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.025402] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.026798] io scheduler noop registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.027436] io scheduler deadline registered (default)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.028204] io scheduler cfq registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.029011] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.029889] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.031036] intel_idle: does not run on family 6 model 62
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.031151] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.032267] ACPI: Power Button [PWRF]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.033112] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.034272] ACPI: Sleep Button [SLPF]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.035177] GHES: HEST is not enabled!
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.037885] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.039016] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.044133] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.045298] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.050812] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.073355] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.096701] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.120118] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.143582] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.147241] Linux agpgart interface v0.103
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.150689] loop: module loaded
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.151551] libphy: Fixed MDIO Bus: probed
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.152314] tun: Universal TUN/TAP device driver, 1.6
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.153305] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.187444] PPP generic driver version 2.4.2
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.188722] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.190377] ehci-pci: EHCI PCI platform driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.191278] ehci-platform: EHCI generic platform driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.192190] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.193387] ohci-pci: OHCI PCI platform driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.194226] ohci-platform: OHCI generic platform driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.195306] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.196576] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.198875] i8042: Warning: Keylock active
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.200847] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.201763] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.202853] mousedev: PS/2 mouse device common for all mice
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.204610] rtc_cmos 00:00: RTC can wake from S4
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.205982] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.207425] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.208830] i2c /dev entries driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.209484] device-mapper: uevent: version 1.0.3
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.210680] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.212585] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.214328] NET: Registered protocol family 10
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.215778] NET: Registered protocol family 17
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.217004] Key type dns_resolver registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.218123] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.219264] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.220718] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.222282] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.223829] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.225872] registered taskstats version 1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.226770] Loading compiled-in X.509 certificates
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.228413] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.229994] zswap: loaded using pool lzo/zbud
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.232983] Key type trusted registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.237349] Key type encrypted registered
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.238554] ima: No TPM chip found, activating TPM-bypass!
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.239487] evm: HMAC attrs: 0x1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.240405]   Magic number: 14:738:23
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.241173] memory memory63: hash matches
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.242218] rtc_cmos 00:00: setting system clock to 2018-08-08 11:02:29 UTC (1533726149)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.243936] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.244879] EDD information not available.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.245667] PM: Hibernation image not present or could not be loaded.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.247353] Freeing unused kernel memory: 1496K
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.248026] Write protecting the kernel read-only data: 14336k
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.250538] Freeing unused kernel memory: 1956K
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.251599] Freeing unused kernel memory: 92K
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.266890] systemd-udevd[118]: starting version 204
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.325938] scsi host0: Virtio SCSI HBA
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.330867] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.337039] AVX version of gcm_enc/dec engaged.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.337908] AES CTR mode by8 optimization enabled
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.371981] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.373288] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.373290] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.375683] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.376933] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.377114] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.380483]  sda: sda1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.381527] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    3.405052] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    4.004586] tsc: Refined TSC clocksource calibration: 2499.789 MHz
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    4.006112] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2408728afa5, max_idle_ns: 440795312487 ns
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    4.242245] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    6.316638] floppy0: no floppy controllers found
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.484450] raid6: sse2x1   gen()  9143 MB/s
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.552447] raid6: sse2x1   xor()  7006 MB/s
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.620449] raid6: sse2x2   gen() 11456 MB/s
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.688448] raid6: sse2x2   xor()  7834 MB/s
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.756443] raid6: sse2x4   gen() 12584 MB/s
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.824447] raid6: sse2x4   xor()  8562 MB/s
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.825256] raid6: using algorithm sse2x4 gen() 12584 MB/s
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.826069] raid6: .... xor() 8562 MB/s, rmw enabled
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.826750] raid6: using ssse3x2 recovery algorithm
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.828782] xor: automatically using best checksumming function:
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.868449]    avx       : 22261.000 MB/sec
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.883278] Btrfs loaded
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.921844] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.923196] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.985565] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.996730] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    7.997778] EXT4-fs (sda1): recovery complete
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    8.002053] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    8.194126] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    8.301753] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    8.356068] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    8.537400] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.036352] random: cloud-init: uninitialized urandom read (32 bytes read, 47 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.170191] systemd-udevd[700]: starting version 204
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.269939] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.359344] ppdev: user-space parallel port driver
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.493797] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.547577] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.615591] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [    9.775505] random: cloud-init: uninitialized urandom read (32 bytes read, 59 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   10.025702] random: mktemp: uninitialized urandom read (12 bytes read, 62 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   10.101813] random: mktemp: uninitialized urandom read (6 bytes read, 63 bits of entropy available)
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   10.166502] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   10.195277] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   10.772698] init: failsafe main process (1093) killed by TERM signal
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Running set_multiqueue.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Set channels for eth0 to 4.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   11.438838] random: nonblocking pool is initialized
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Starting Google Accounts daemon.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for me.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account me.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for henrik.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 11:02:37 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account henrik.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for emma.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account emma.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for igor.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account igor.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e cron[1429]: (CRON) INFO (pidfile fd = 3)
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e cron[1471]: (CRON) STARTUP (fork ok)
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e cron[1471]: (CRON) INFO (Running @reboot jobs)
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account konstantinhaase.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for aj.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e acpid: starting up with netlink and the input layer
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e acpid: 1 rule loaded
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e acpid: waiting for events: event logging is off
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account aj.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for solarce.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e haveged: haveged starting up
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account solarce.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for asari.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   11.968535] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   11.978495] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account asari.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for bogdana.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account bogdana.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for konstantin.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account konstantin.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for carmen.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   12.142006] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account carmen.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   12.146158] Bridge firewalling registered
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   12.155638] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Creating a new user account for maria.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Created user account maria.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e google-accounts: INFO Removing user packer.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   12.221627] Initializing XFRM netlink socket
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   12.229423] Netfilter messages via NETLINK v0.30.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   12.232181] ctnetlink v0.93: registering with nfnetlink.
Aug  8 11:02:38 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   12.332588] floppy0: no floppy controllers found
Aug  8 11:03:01 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpdate[1841]: adjust time server 169.254.169.254 offset 0.004575 sec
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1868]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: proto: precision = 0.108 usec
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen normally on 3 eth0 10.20.0.212 UDP 123
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: peers refreshed
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listening on routing socket on fd #21 for interface updates
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [   42.150366] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e startup-script: INFO Found startup-script in metadata.
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e startup-script: INFO startup-script: job 1 at Wed Aug  8 14:13:00 2018
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e startup-script: INFO startup-script: Return code 0.
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e startup-script: INFO startup-script: Return code 0.
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e startup-script: INFO Finished running startup scripts.
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: 
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: #############################################################
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: 1024 87:f5:48:27:11:d8:ba:1e:42:89:5c:8d:4c:cc:2a:3b  root@travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e (DSA)
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: 256 67:3a:72:91:ce:b7:11:fa:e9:7a:19:b5:ab:38:d3:44  root@travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e (ECDSA)
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: 256 38:db:e4:77:28:22:54:85:36:f9:70:90:4d:dc:41:b4  root@travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e (ED25519)
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: 2048 fa:c9:55:7e:41:3e:a4:c3:bd:1d:37:60:88:30:6c:ef  root@travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e (RSA)
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 11:03:08 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ec2: #############################################################
Aug  8 11:04:28 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  121.975115] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.677040] device veth37d1023 entered promiscuous mode
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.764082] cgroup: docker-runc (4864) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.764085] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.847393] eth0: renamed from veth2834acb
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.881950] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.883551] docker0: port 1(veth37d1023) entered forwarding state
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.883575] docker0: port 1(veth37d1023) entered forwarding state
Aug  8 11:05:33 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  186.883600] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 11:05:36 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  8 11:05:36 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen normally on 6 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 11:05:36 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: Listen normally on 7 docker0 fe80::42:78ff:fea0:4412 UDP 123
Aug  8 11:05:36 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: peers refreshed
Aug  8 11:05:36 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e ntpd[1869]: new interface(s) found: waking up resolver
Aug  8 11:05:48 travis-job-0cb977cb-3fcd-4b5a-866b-05eb69bb512e kernel: [  201.919160] docker0: port 1(veth37d1023) entered forwarding state
---
travis_time:end:1ab1eb98:start=1533726497148974936,finish=1533726497155826775,duration=6851839
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b23d0c9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08351342
travis_time:start:08351342
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:12e35032
$ dmesg | grep -i kill
