plain

[00:03:47] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:47] tidy error: /checkout/src/libcore/num/mod.rs:2048: line longer than 100 chars
[00:03:49] some tidy checks failed
[00:03:49] 
[00:03:49] 
[00:03:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:49] 
[00:03:49] 
[00:03:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:49] Build completed unsuccessfully in 0:00:51
[00:03:49] Build completed unsuccessfully in 0:00:51
[00:03:49] make: *** [tidy] Error 1
[00:03:49] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05fe009e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:10286649
$ sudo tail -n 500 /var/log/syslog
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] kvm-clock: using sched offset of 1805037536 cycles
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Zone ranges:
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   Device   empty
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Movable zone start for each node
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Early memory node ranges
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Policy zone: Normal
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] console [ttyS0] enabled
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.472539] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.475194] pid_max: default: 32768 minimum: 301
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.476755] ACPI: Core revision 20150930
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.484146] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.486686] Security Framework initialized
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.488354] Yama: becoming mindful.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.489622] AppArmor: AppArmor disabled by boot time parameter
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.493585] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.505524] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.511855] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.515317] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.518461] Initializing cgroup subsys io
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.520363] Initializing cgroup subsys memory
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.522226] Initializing cgroup subsys devices
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.524760] Initializing cgroup subsys freezer
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.526969] Initializing cgroup subsys net_cls
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.529928] Initializing cgroup subsys perf_event
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.532066] Initializing cgroup subsys net_prio
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.534087] Initializing cgroup subsys hugetlb
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.536522] Initializing cgroup subsys pids
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.538449] CPU: Physical Processor ID: 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.540714] CPU: Processor Core ID: 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.543666] mce: CPU supports 32 MCE banks
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.545364] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.547615] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.552274] Freeing SMP alternatives memory: 32K
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.561980] ftrace: allocating 32185 entries in 126 pages
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.613847] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.616786] smpboot: Max logical packages: 2
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.619692] x2apic enabled
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.621972] Switched APIC routing to physical x2apic.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.627206] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.734798] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.739449] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.745261] x86: Booting SMP configuration:
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.746912] .... node  #0, CPUs:      #1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.748554] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.754378]  #2
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.755375] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.761388]  #3
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.762353] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.768171] x86: Booted up 1 node, 4 CPUs
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.769681] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.773526] devtmpfs: initialized
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.779124] evm: security.selinux
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.780374] evm: security.SMACK64
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.781530] evm: security.SMACK64EXEC
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.782675] evm: security.SMACK64TRANSMUTE
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.784181] evm: security.SMACK64MMAP
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.785812] evm: security.ima
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.786928] evm: security.capability
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.788984] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.792108] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.794457] pinctrl core: initialized pinctrl subsystem
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.796569] RTC time: 21:44:17, date: 08/08/18
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.799038] NET: Registered protocol family 16
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.805328] cpuidle: using governor ladder
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.817358] cpuidle: using governor menu
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.818734] PCCT header not found.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.820264] ACPI: bus type PCI registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.822095] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.824888] PCI: Using configuration type 1 for base access
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.835473] ACPI: Added _OSI(Module Device)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.837226] ACPI: Added _OSI(Processor Device)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.838866] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.840571] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.845673] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.871371] ACPI: Interpreter enabled
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.872776] ACPI: (supports S0 S3 S4 S5)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.873965] ACPI: Using IOAPIC for interrupt routing
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.875626] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.907273] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.909583] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.912542] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.915211] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.920183] PCI host bridge to bus 0000:00
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.921650] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.923997] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.926428] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.929476] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.931923] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.934305] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.934758] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.955878] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.978588] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.981671] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.991356] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    0.997954] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.018497] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.027042] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.033903] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.055421] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.059749] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.064337] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.069766] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.075427] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.099722] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.102348] vgaarb: loaded
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.103773] SCSI subsystem initialized
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.105414] libata version 3.00 loaded.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.105440] ACPI: bus type USB registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.107162] usbcore: registered new interface driver usbfs
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.109615] usbcore: registered new interface driver hub
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.112142] usbcore: registered new device driver usb
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.114651] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.118067] dmi: Firmware registration failed.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.120562] PCI: Using ACPI for IRQ routing
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.122081] PCI: pci_cache_line_size set to 64 bytes
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.122201] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.122204] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.122351] NetLabel: Initializing
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.123761] NetLabel:  domain hash size = 128
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.125681] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.127796] NetLabel:  unlabeled traffic allowed by default
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.130594] amd_nb: Cannot enumerate AMD northbridges
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.133934] clocksource: Switched to clocksource kvm-clock
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.143768] pnp: PnP ACPI init
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.145604] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.145671] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.145714] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.145761] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.145801] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.145838] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.145891] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.146070] pnp: PnP ACPI: found 7 devices
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.154841] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.158805] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.158808] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.158809] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.158811] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.158847] NET: Registered protocol family 2
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.160886] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.163905] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.166512] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.169276] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.171476] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.175031] NET: Registered protocol family 1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.176870] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.179086] PCI: CLS 0 bytes, default 64
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    1.179159] Unpacking initramfs...
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.407488] Freeing initrd memory: 21432K
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.409349] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.412141] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.415744] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.419843] hw unit of domain pp0-core 2^-0 Joules
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.422068] hw unit of domain package 2^-0 Joules
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.424176] hw unit of domain dram 2^-16 Joules
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.426179] Scanning for low memory corruption every 60 seconds
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.429339] audit: initializing netlink subsys (disabled)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.431641] audit: type=2000 audit(1533764660.206:1): initialized
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.434568] Initialise system trusted keyring
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.436367] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.438418] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.441495] zbud: loaded
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.442943] VFS: Disk quotas dquot_6.6.0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.444280] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.447265] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.449694] fuse init (API version 7.23)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.451441] Key type big_key registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.452972] Allocating IMA MOK and blacklist keyrings.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.458349] Key type asymmetric registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.460396] Asymmetric key parser 'x509' registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.462475] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.465738] io scheduler noop registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.467460] io scheduler deadline registered (default)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.469061] io scheduler cfq registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.470604] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.472954] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.475485] intel_idle: does not run on family 6 model 63
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.475611] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.479328] ACPI: Power Button [PWRF]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.480662] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.483379] ACPI: Sleep Button [SLPF]
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.485194] GHES: HEST is not enabled!
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.489753] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.491951] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.500741] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.503645] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.512797] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.536467] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.560807] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.584757] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.609367] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.613696] Linux agpgart interface v0.103
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.618125] loop: module loaded
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.619168] libphy: Fixed MDIO Bus: probed
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.620072] tun: Universal TUN/TAP device driver, 1.6
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.621056] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.667088] PPP generic driver version 2.4.2
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.668426] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.670586] ehci-pci: EHCI PCI platform driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.671780] ehci-platform: EHCI generic platform driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.673184] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.674857] ohci-pci: OHCI PCI platform driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.675926] ohci-platform: OHCI generic platform driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.677278] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.679177] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.682094] i8042: Warning: Keylock active
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.684401] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.685743] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.687342] mousedev: PS/2 mouse device common for all mice
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.689262] rtc_cmos 00:00: RTC can wake from S4
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.690936] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.693194] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.694900] i2c /dev entries driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.696148] device-mapper: uevent: version 1.0.3
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.697434] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.699501] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.701565] NET: Registered protocol family 10
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.703039] NET: Registered protocol family 17
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.704085] Key type dns_resolver registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.705492] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.707087] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.708447] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.710135] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.711713] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.714625] registered taskstats version 1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.715677] Loading compiled-in X.509 certificates
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.717514] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.720163] zswap: loaded using pool lzo/zbud
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.723333] Key type trusted registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.727779] Key type encrypted registered
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.728877] ima: No TPM chip found, activating TPM-bypass!
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.730330] evm: HMAC attrs: 0x1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.731482]   Magic number: 14:525:752
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.732247] tty ttyS3: hash matches
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.733126] acpi LNXCPU:77: hash matches
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.734228] acpi LNXCPU:4a: hash matches
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.735440] rtc_cmos 00:00: setting system clock to 2018-08-08 21:44:20 UTC (1533764660)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.737373] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.738923] EDD information not available.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.739769] PM: Hibernation image not present or could not be loaded.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.741264] Freeing unused kernel memory: 1496K
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.742725] Write protecting the kernel read-only data: 14336k
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.744932] Freeing unused kernel memory: 1956K
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.746140] Freeing unused kernel memory: 92K
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.762833] systemd-udevd[119]: starting version 204
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.818011] AVX2 version of gcm_enc/dec engaged.
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.824072] AES CTR mode by8 optimization enabled
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.832167] scsi host0: Virtio SCSI HBA
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.835658] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.870140] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.870168] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.872401] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.873361] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.874394] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.874542] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.877760]  sda: sda1
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.879319] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    3.890616] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    4.426060] tsc: Refined TSC clocksource calibration: 2299.796 MHz
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    4.428131] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2126747b124, max_idle_ns: 440795298764 ns
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    4.727928] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    6.810136] floppy0: no floppy controllers found
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    7.969993] raid6: sse2x1   gen()  9054 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.037968] raid6: sse2x1   xor()  6996 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.105980] raid6: sse2x2   gen() 11421 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.173964] raid6: sse2x2   xor()  8010 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.241991] raid6: sse2x4   gen() 12441 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.310134] raid6: sse2x4   xor()  8283 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.378001] raid6: avx2x1   gen() 17685 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.445988] raid6: avx2x2   gen() 19904 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.513985] raid6: avx2x4   gen() 21316 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.515495] raid6: using algorithm avx2x4 gen() 21316 MB/s
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.517845] raid6: using avx2x2 recovery algorithm
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.521576] xor: automatically using best checksumming function:
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.561985]    avx       : 21797.000 MB/sec
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.577419] Btrfs loaded
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.636533] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.639598] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.765019] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.775054] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.776856] EXT4-fs (sda1): recovery complete
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    8.785128] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    9.073037] random: init: uninitialized urandom read (12 bytes read, 25 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    9.243874] random: mountall: uninitialized urandom read (12 bytes read, 29 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    9.309470] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [    9.582196] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.290344] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.459946] systemd-udevd[703]: starting version 204
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.604776] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.668869] intel_rapl: no valid rapl domains found in package 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.727080] intel_rapl: no valid rapl domains found in package 0
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.729085] ppdev: user-space parallel port driver
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.854506] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.918298] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   10.997165] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   11.182723] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   11.479433] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   11.571795] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   11.670847] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   11.727951] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   12.024870] init: failsafe main process (1095) killed by TERM signal
Aug  8 21:44:28 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Running set_multiqueue.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Set channels for eth0 to 4.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Starting Google Accounts daemon.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for me.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account me.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for henrik.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account henrik.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for emma.
Aug  8 21:44:29 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account emma.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for igor.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.278701] random: nonblocking pool is initialized
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account igor.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account konstantinhaase.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for aj.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account aj.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for solarce.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account solarce.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for asari.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account asari.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for bogdana.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.679143] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.682254] floppy0: no floppy controllers found
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.683421] Bridge firewalling registered
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.697918] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account bogdana.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for konstantin.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.771644] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account konstantin.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for carmen.
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 21:44:30 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.869459] Initializing XFRM netlink socket
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.877688] Netfilter messages via NETLINK v0.30.
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account carmen.
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   13.880963] ctnetlink v0.93: registering with nfnetlink.
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Creating a new user account for maria.
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Created user account maria.
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db google-accounts: INFO Removing user packer.
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db cron[1712]: (CRON) INFO (pidfile fd = 3)
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db cron[1747]: (CRON) STARTUP (fork ok)
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db cron[1747]: (CRON) INFO (Running @reboot jobs)
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db acpid: starting up with netlink and the input layer
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db acpid: 1 rule loaded
Aug  8 21:44:31 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db acpid: waiting for events: event logging is off
Aug  8 21:44:32 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db haveged: haveged starting up
Aug  8 21:44:32 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   15.017366] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1847]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: proto: precision = 0.105 usec
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen normally on 3 eth0 10.20.2.92 UDP 123
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: peers refreshed
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listening on routing socket on fd #21 for interface updates
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [   20.245582] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db startup-script: INFO Found startup-script in metadata.
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db startup-script: INFO startup-script: job 1 at Thu Aug  9 00:54:00 2018
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db startup-script: INFO startup-script: Return code 0.
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db startup-script: INFO startup-script: Return code 0.
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db startup-script: INFO Finished running startup scripts.
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: 
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: #############################################################
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: 1024 72:8f:4d:ef:45:d7:03:bc:fa:ad:4b:12:74:b2:fd:1f  root@travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db (DSA)
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: 256 70:d3:50:9a:de:a8:f6:aa:a1:f5:35:c9:04:23:4d:b0  root@travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db (ECDSA)
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: 256 ec:40:47:fe:ea:65:2b:22:1e:4e:af:56:00:27:5b:d4  root@travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db (ED25519)
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: 2048 07:71:ee:d2:42:ac:33:6a:05:20:07:d1:ec:0c:35:1e  root@travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db (RSA)
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 21:44:37 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ec2: #############################################################
Aug  8 21:44:45 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpdate[2242]: the NTP socket is in use, exiting
Aug  8 21:46:49 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  152.521208] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 21:47:42 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.638428] device veth2239683 entered promiscuous mode
Aug  8 21:47:42 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.638492] docker0: port 1(veth2239683) entered forwarding state
Aug  8 21:47:42 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.638499] docker0: port 1(veth2239683) entered forwarding state
Aug  8 21:47:42 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.638899] docker0: port 1(veth2239683) entered disabled state
Aug  8 21:47:42 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.736912] cgroup: docker-runc (4847) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 21:47:42 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.736922] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 21:47:42 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.825450] eth0: renamed from vetha40a060
Aug  8 21:47:43 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.865792] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 21:47:43 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.867253] docker0: port 1(veth2239683) entered forwarding state
Aug  8 21:47:43 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.867269] docker0: port 1(veth2239683) entered forwarding state
Aug  8 21:47:43 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  205.867289] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 21:47:46 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen normally on 5 docker0 fe80::42:84ff:fee8:8bf5 UDP 123
Aug  8 21:47:46 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug  8 21:47:46 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 21:47:46 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: peers refreshed
Aug  8 21:47:46 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db ntpd[1848]: new interface(s) found: waking up resolver
Aug  8 21:47:58 travis-job-476ab4f4-8317-47af-8dcb-73275f04d2db kernel: [  220.911846] docker0: port 1(veth2239683) entered forwarding state
---
travis_time:end:083599e2:start=1533765040590513378,finish=1533765040598423865,duration=7910487
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3084e4a4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:044c2fdf
travis_time:start:044c2fdf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:041c6565
$ dmesg | grep -i kill
