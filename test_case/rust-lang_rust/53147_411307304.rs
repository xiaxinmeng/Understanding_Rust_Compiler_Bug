plain

[00:03:52] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:72: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:100: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:108: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:136: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:164: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:172: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:195: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:283: TODO is deprecated; use FIXME
[00:03:52] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:318: TODO is deprecated; use FIXME
[00:03:53] some tidy checks failed
[00:03:53] 
[00:03:53] 
[00:03:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:53] 
[00:03:53] 
[00:03:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:53] Build completed unsuccessfully in 0:00:53
[00:03:53] Build completed unsuccessfully in 0:00:53
[00:03:53] make: *** [tidy] Error 1
[00:03:53] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:083d84e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:2d085d88
$ sudo tail -n 500 /var/log/syslog
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] kvm-clock: using sched offset of 1399453855 cycles
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Zone ranges:
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   Device   empty
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Movable zone start for each node
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Early memory node ranges
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Policy zone: Normal
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] console [ttyS0] enabled
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.310925] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.312074] pid_max: default: 32768 minimum: 301
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.312898] ACPI: Core revision 20150930
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.319075] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.320137] Security Framework initialized
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.320723] Yama: becoming mindful.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.321251] AppArmor: AppArmor disabled by boot time parameter
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.323664] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.332735] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.337125] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.338297] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.340118] Initializing cgroup subsys io
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.340729] Initializing cgroup subsys memory
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.341451] Initializing cgroup subsys devices
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.342063] Initializing cgroup subsys freezer
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.342956] Initializing cgroup subsys net_cls
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.343708] Initializing cgroup subsys perf_event
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.344535] Initializing cgroup subsys net_prio
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.345354] Initializing cgroup subsys hugetlb
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.346008] Initializing cgroup subsys pids
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.346765] CPU: Physical Processor ID: 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.347368] CPU: Processor Core ID: 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.347919] mce: CPU supports 32 MCE banks
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.348607] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.349610] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.352471] Freeing SMP alternatives memory: 32K
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.360703] ftrace: allocating 32185 entries in 126 pages
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.407575] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.408911] smpboot: Max logical packages: 2
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.410111] x2apic enabled
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.411941] Switched APIC routing to physical x2apic.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.415438] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.522236] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.524031] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.527959] x86: Booting SMP configuration:
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.528581] .... node  #0, CPUs:      #1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.529485] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.532834]  #2
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.533277] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.536860]  #3
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.537361] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.540783] x86: Booted up 1 node, 4 CPUs
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.541520] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.543757] devtmpfs: initialized
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.547850] evm: security.selinux
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.548561] evm: security.SMACK64
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.549025] evm: security.SMACK64EXEC
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.549625] evm: security.SMACK64TRANSMUTE
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.550200] evm: security.SMACK64MMAP
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.550850] evm: security.ima
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.551353] evm: security.capability
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.552298] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.553710] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.555017] pinctrl core: initialized pinctrl subsystem
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.556185] RTC time:  6:47:40, date: 08/08/18
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.557881] NET: Registered protocol family 16
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.570398] cpuidle: using governor ladder
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.582272] cpuidle: using governor menu
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.582862] PCCT header not found.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.583444] ACPI: bus type PCI registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.584069] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.585221] PCI: Using configuration type 1 for base access
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.599130] ACPI: Added _OSI(Module Device)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.599862] ACPI: Added _OSI(Processor Device)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.600606] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.601389] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.604637] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.627488] ACPI: Interpreter enabled
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.628130] ACPI: (supports S0 S3 S4 S5)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.628750] ACPI: Using IOAPIC for interrupt routing
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.629530] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.658551] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.659681] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.660832] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.661742] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.664111] PCI host bridge to bus 0000:00
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.665001] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.666027] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.667002] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.668041] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.669178] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.670135] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.670543] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.685045] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.699191] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.700663] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.705672] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.711078] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.725144] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.730265] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.734303] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.747229] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.750011] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.752312] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.754817] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.757635] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.777811] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.779202] vgaarb: loaded
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.779806] SCSI subsystem initialized
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.780613] libata version 3.00 loaded.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.780648] ACPI: bus type USB registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.781274] usbcore: registered new interface driver usbfs
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.782083] usbcore: registered new interface driver hub
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.782841] usbcore: registered new device driver usb
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.783855] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.785335] dmi: Firmware registration failed.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.786353] PCI: Using ACPI for IRQ routing
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.787007] PCI: pci_cache_line_size set to 64 bytes
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.787099] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.787100] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.787244] NetLabel: Initializing
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.787742] NetLabel:  domain hash size = 128
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.788495] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.789318] NetLabel:  unlabeled traffic allowed by default
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.790631] amd_nb: Cannot enumerate AMD northbridges
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.791562] clocksource: Switched to clocksource kvm-clock
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799094] pnp: PnP ACPI init
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799665] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799736] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799780] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799831] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799873] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799914] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.799960] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.800119] pnp: PnP ACPI: found 7 devices
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.807815] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.809527] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.809529] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.809531] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.809532] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.809565] NET: Registered protocol family 2
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.810421] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.812461] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.813721] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.814859] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.815810] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.817494] NET: Registered protocol family 1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.818192] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.819298] PCI: CLS 0 bytes, default 64
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    0.819348] Unpacking initramfs...
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.814530] Freeing initrd memory: 21432K
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.815356] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.816399] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.817914] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.819381] hw unit of domain pp0-core 2^-0 Joules
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.820059] hw unit of domain package 2^-0 Joules
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.820750] hw unit of domain dram 2^-0 Joules
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.821443] Scanning for low memory corruption every 60 seconds
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.822725] audit: initializing netlink subsys (disabled)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.823534] audit: type=2000 audit(1533710862.389:1): initialized
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.824637] Initialise system trusted keyring
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.825497] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.826611] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.828732] zbud: loaded
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.829426] VFS: Disk quotas dquot_6.6.0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.830058] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.831235] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.832585] fuse init (API version 7.23)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.833436] Key type big_key registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.834000] Allocating IMA MOK and blacklist keyrings.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.835488] Key type asymmetric registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.836123] Asymmetric key parser 'x509' registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.836923] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.838318] io scheduler noop registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.838930] io scheduler deadline registered (default)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.839766] io scheduler cfq registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.840499] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.841327] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.842390] intel_idle: does not run on family 6 model 45
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.842484] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.843543] ACPI: Power Button [PWRF]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.844409] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.845553] ACPI: Sleep Button [SLPF]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.846525] GHES: HEST is not enabled!
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.848850] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.849747] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.854097] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.855052] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.859652] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.881823] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.904665] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.927877] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.950857] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.953763] Linux agpgart interface v0.103
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.956698] loop: module loaded
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.957665] libphy: Fixed MDIO Bus: probed
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.958329] tun: Universal TUN/TAP device driver, 1.6
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.959575] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.990684] PPP generic driver version 2.4.2
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.991679] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.993041] ehci-pci: EHCI PCI platform driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.993736] ehci-platform: EHCI generic platform driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.994688] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.995369] ohci-pci: OHCI PCI platform driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.996242] ohci-platform: OHCI generic platform driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.996975] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.997820] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    2.999312] i8042: Warning: Keylock active
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.000723] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.001496] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.002939] mousedev: PS/2 mouse device common for all mice
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.004073] rtc_cmos 00:00: RTC can wake from S4
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.005053] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.006676] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.007798] i2c /dev entries driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.008417] device-mapper: uevent: version 1.0.3
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.009020] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.010804] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.012241] NET: Registered protocol family 10
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.013282] NET: Registered protocol family 17
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.014376] Key type dns_resolver registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.016009] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.016978] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.018380] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.020207] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.021803] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.024418] registered taskstats version 1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.025299] Loading compiled-in X.509 certificates
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.027503] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.029934] zswap: loaded using pool lzo/zbud
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.032844] Key type trusted registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.036924] Key type encrypted registered
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.038275] ima: No TPM chip found, activating TPM-bypass!
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.039895] evm: HMAC attrs: 0x1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.041147]   Magic number: 14:482:771
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.042138] acpi LNXCPU:5f: hash matches
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.043354] rtc_cmos 00:00: setting system clock to 2018-08-08 06:47:42 UTC (1533710862)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.045353] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.047352] EDD information not available.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.048513] PM: Hibernation image not present or could not be loaded.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.049928] Freeing unused kernel memory: 1496K
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.050593] Write protecting the kernel read-only data: 14336k
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.052252] Freeing unused kernel memory: 1956K
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.053394] Freeing unused kernel memory: 92K
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.066640] systemd-udevd[119]: starting version 204
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.116661] scsi host0: Virtio SCSI HBA
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.120408] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.125653] AVX version of gcm_enc/dec engaged.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.126598] AES CTR mode by8 optimization enabled
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.162376] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.162398] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.162400] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.162580] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.162582] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.162624] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.164165]  sda: sda1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.164858] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.207884] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.819738] tsc: Refined TSC clocksource calibration: 2600.001 MHz
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    3.820712] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3ce1c4c, max_idle_ns: 440795206275 ns
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    4.040684] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    6.111716] floppy0: no floppy controllers found
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.267599] raid6: sse2x1   gen()  9056 MB/s
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.335586] raid6: sse2x1   xor()  6727 MB/s
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.403588] raid6: sse2x2   gen() 11091 MB/s
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.471584] raid6: sse2x2   xor()  7407 MB/s
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.539583] raid6: sse2x4   gen() 12983 MB/s
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.607583] raid6: sse2x4   xor()  9136 MB/s
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.608346] raid6: using algorithm sse2x4 gen() 12983 MB/s
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.609283] raid6: .... xor() 9136 MB/s, rmw enabled
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.609958] raid6: using ssse3x2 recovery algorithm
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.612081] xor: automatically using best checksumming function:
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.651570]    avx       : 27736.000 MB/sec
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.664876] Btrfs loaded
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.701792] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.702964] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.770680] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.776505] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.777386] EXT4-fs (sda1): recovery complete
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.781499] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    7.966675] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    8.065422] random: mountall: uninitialized urandom read (12 bytes read, 31 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    8.118196] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    8.289844] random: cloud-init: uninitialized urandom read (32 bytes read, 38 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    8.776080] random: cloud-init: uninitialized urandom read (32 bytes read, 46 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    8.890714] systemd-udevd[702]: starting version 204
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    8.983953] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.050454] intel_rapl: no valid rapl domains found in package 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.089227] intel_rapl: no valid rapl domains found in package 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.125020] intel_rapl: no valid rapl domains found in package 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.128352] ppdev: user-space parallel port driver
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.188513] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.233594] random: mktemp: uninitialized urandom read (6 bytes read, 58 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.298488] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.462063] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.707431] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.778149] random: mktemp: uninitialized urandom read (6 bytes read, 62 bits of entropy available)
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.842787] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [    9.873245] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   10.078439] init: failsafe main process (1096) killed by TERM signal
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Running set_multiqueue.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Set channels for eth0 to 4.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 06:47:49 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   10.800943] random: nonblocking pool is initialized
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Starting Google Accounts daemon.
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 cron[1387]: (CRON) INFO (pidfile fd = 3)
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 cron[1432]: (CRON) STARTUP (fork ok)
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for me.
Aug  8 06:47:50 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 cron[1432]: (CRON) INFO (Running @reboot jobs)
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 acpid: starting up with netlink and the input layer
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 acpid: 1 rule loaded
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 acpid: waiting for events: event logging is off
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account me.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for henrik.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account henrik.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 haveged: haveged starting up
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for emma.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account emma.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.332096] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.346815] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for igor.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account igor.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account konstantinhaase.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for aj.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.454888] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.458110] Bridge firewalling registered
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.467985] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account aj.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for solarce.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account solarce.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.542108] Initializing XFRM netlink socket
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for asari.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.550847] Netfilter messages via NETLINK v0.30.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   11.553719] ctnetlink v0.93: registering with nfnetlink.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account asari.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for bogdana.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account bogdana.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for konstantin.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account konstantin.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for carmen.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account carmen.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Creating a new user account for maria.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Created user account maria.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 google-accounts: INFO Removing user packer.
Aug  8 06:47:51 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   12.055688] floppy0: no floppy controllers found
Aug  8 06:48:14 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpdate[1853]: adjust time server 169.254.169.254 offset 0.004252 sec
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1888]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: proto: precision = 0.107 usec
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen normally on 3 eth0 10.20.1.72 UDP 123
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: peers refreshed
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listening on routing socket on fd #21 for interface updates
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [   41.506106] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 startup-script: INFO Found startup-script in metadata.
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 startup-script: INFO startup-script: job 1 at Wed Aug  8 09:58:00 2018
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 startup-script: INFO startup-script: Return code 0.
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 startup-script: INFO startup-script: Return code 0.
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 startup-script: INFO Finished running startup scripts.
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: 
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: #############################################################
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: 1024 8e:ad:c8:9d:ff:e4:59:2d:5a:49:d1:04:54:b7:ac:b9  root@travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 (DSA)
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: 256 f4:1b:4d:44:02:c6:b9:38:bd:b5:a2:9c:89:b6:10:b4  root@travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 (ECDSA)
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: 256 c0:93:a3:a7:13:8e:05:b4:fd:62:14:45:af:4a:e0:5e  root@travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 (ED25519)
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: 2048 44:2c:0f:6a:70:b7:ea:e0:4d:ea:9d:ee:d9:47:f2:3e  root@travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 (RSA)
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 06:48:21 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ec2: #############################################################
Aug  8 06:50:18 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  159.020509] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  226.860812] device veth7dc78be entered promiscuous mode
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  226.860896] docker0: port 1(veth7dc78be) entered forwarding state
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  226.860906] docker0: port 1(veth7dc78be) entered forwarding state
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  226.861204] docker0: port 1(veth7dc78be) entered disabled state
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  226.956766] cgroup: docker-runc (4879) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  226.956770] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  227.031893] eth0: renamed from veth54f1e81
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  227.066276] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  227.067229] docker0: port 1(veth7dc78be) entered forwarding state
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  227.067245] docker0: port 1(veth7dc78be) entered forwarding state
Aug  8 06:51:26 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  227.067273] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 06:51:30 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  8 06:51:30 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen normally on 6 docker0 fe80::42:ebff:fef1:1f4c UDP 123
Aug  8 06:51:30 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 06:51:30 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: peers refreshed
Aug  8 06:51:30 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 ntpd[1889]: new interface(s) found: waking up resolver
Aug  8 06:51:41 travis-job-7316140f-cdb9-4fe5-9163-987a5769b583 kernel: [  242.067708] docker0: port 1(veth7dc78be) entered forwarding state
---
travis_time:end:308c6a90:start=1533711254031200789,finish=1533711254036770480,duration=5569691
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06fee290
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13141b70
travis_time:start:13141b70
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:18f6cb48
$ dmesg | grep -i kill
