plain
[01:16:35] Verifying status of rust-by-example...
[01:16:35] Verifying status of rls...
[01:16:35] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:16:35] 
[01:16:35] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:16:35] 
[01:16:35] If you do intend to update 'rls', please check the error messages above and
[01:16:35] commit another update.
[01:16:35] 
[01:16:35] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:16:35] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:16:35] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:316c6fec
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0009185f
$ sudo tail -n 500 /var/log/syslog
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] kvm-clock: using sched offset of 1380030198 cycles
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Zone ranges:
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   Device   empty
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Movable zone start for each node
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Early memory node ranges
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Policy zone: Normal
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Hierarchical RCU implementation.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] console [ttyS0] enabled
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.305057] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.306197] pid_max: default: 32768 minimum: 301
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.306831] ACPI: Core revision 20150930
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.312955] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.314006] Security Framework initialized
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.314735] Yama: becoming mindful.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.315283] AppArmor: AppArmor disabled by boot time parameter
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.317716] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.326607] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.330632] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.331631] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.332910] Initializing cgroup subsys io
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.333565] Initializing cgroup subsys memory
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.334198] Initializing cgroup subsys devices
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.334809] Initializing cgroup subsys freezer
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.335440] Initializing cgroup subsys net_cls
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.336050] Initializing cgroup subsys perf_event
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.336778] Initializing cgroup subsys net_prio
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.337434] Initializing cgroup subsys hugetlb
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.338042] Initializing cgroup subsys pids
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.338692] CPU: Physical Processor ID: 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.339433] CPU: Processor Core ID: 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.340829] mce: CPU supports 32 MCE banks
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.341575] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.342453] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.345429] Freeing SMP alternatives memory: 32K
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.353579] ftrace: allocating 32185 entries in 126 pages
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.398378] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.399337] smpboot: Max logical packages: 2
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.400575] x2apic enabled
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.402173] Switched APIC routing to physical x2apic.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.405731] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.512390] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.514358] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.516888] x86: Booting SMP configuration:
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.517482] .... node  #0, CPUs:      #1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.518341] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.522659]  #2
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.523082] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.527019]  #3
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.527572] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.531611] x86: Booted up 1 node, 4 CPUs
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.532286] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.535043] devtmpfs: initialized
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.539016] evm: security.selinux
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.539707] evm: security.SMACK64
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.540338] evm: security.SMACK64EXEC
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.540855] evm: security.SMACK64TRANSMUTE
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.541472] evm: security.SMACK64MMAP
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.542016] evm: security.ima
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.542573] evm: security.capability
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.543633] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.545040] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.546202] pinctrl core: initialized pinctrl subsystem
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.547154] RTC time:  7:25:55, date: 08/07/18
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.548603] NET: Registered protocol family 16
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.560430] cpuidle: using governor ladder
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.572438] cpuidle: using governor menu
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.573198] PCCT header not found.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.573813] ACPI: bus type PCI registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.574481] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.575730] PCI: Using configuration type 1 for base access
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.589557] ACPI: Added _OSI(Module Device)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.590620] ACPI: Added _OSI(Processor Device)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.591631] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.592942] ACPI: Added _OSI(Processor Aggregator Device)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.596298] ACPI: Executed 2 blocks of module-level executable AML code
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.618981] ACPI: Interpreter enabled
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.619819] ACPI: (supports S0 S3 S4 S5)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.620434] ACPI: Using IOAPIC for interrupt routing
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.621327] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.650262] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.651677] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.653401] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.654841] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.657584] PCI host bridge to bus 0000:00
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.658408] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.659357] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.660545] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.661873] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.663216] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.664222] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.664655] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.679971] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.696150] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.697679] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.703838] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.708438] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.721634] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.727433] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.731793] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.746460] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.748681] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.750768] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.752960] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.755134] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.774877] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.775900] vgaarb: loaded
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.776556] SCSI subsystem initialized
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.777278] libata version 3.00 loaded.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.777303] ACPI: bus type USB registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.778105] usbcore: registered new interface driver usbfs
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.778950] usbcore: registered new interface driver hub
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.779785] usbcore: registered new device driver usb
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.780663] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.781767] dmi: Firmware registration failed.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.782656] PCI: Using ACPI for IRQ routing
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.783286] PCI: pci_cache_line_size set to 64 bytes
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.783378] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.783380] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.783489] NetLabel: Initializing
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.783969] NetLabel:  domain hash size = 128
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.784680] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.785476] NetLabel:  unlabeled traffic allowed by default
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.786553] amd_nb: Cannot enumerate AMD northbridges
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.787343] clocksource: Switched to clocksource kvm-clock
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.794535] pnp: PnP ACPI init
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795074] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795143] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795186] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795235] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795276] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795316] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795371] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.795532] pnp: PnP ACPI: found 7 devices
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.803372] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.804743] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.804745] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.804747] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.804748] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.804783] NET: Registered protocol family 2
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.805743] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.807767] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.808938] TCP: Hash tables configured (established 131072 bind 65536)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.809902] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.811260] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.813135] NET: Registered protocol family 1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.813832] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.814723] PCI: CLS 0 bytes, default 64
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    0.814777] Unpacking initramfs...
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.822849] Freeing initrd memory: 21432K
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.823710] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.824616] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.826109] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.827634] hw unit of domain pp0-core 2^-0 Joules
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.828411] hw unit of domain package 2^-0 Joules
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.829157] hw unit of domain dram 2^-16 Joules
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.830337] Scanning for low memory corruption every 60 seconds
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.831746] audit: initializing netlink subsys (disabled)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.832594] audit: type=2000 audit(1533626757.468:1): initialized
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.833886] Initialise system trusted keyring
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.834824] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.835775] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.838125] zbud: loaded
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.838844] VFS: Disk quotas dquot_6.6.0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.839486] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.840813] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.842210] fuse init (API version 7.23)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.843039] Key type big_key registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.843911] Allocating IMA MOK and blacklist keyrings.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.845848] Key type asymmetric registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.846798] Asymmetric key parser 'x509' registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.848127] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.849706] io scheduler noop registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.850484] io scheduler deadline registered (default)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.851526] io scheduler cfq registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.852394] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.853232] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.854486] intel_idle: does not run on family 6 model 63
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.854589] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.855780] ACPI: Power Button [PWRF]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.856381] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.858423] ACPI: Sleep Button [SLPF]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.859712] GHES: HEST is not enabled!
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.862129] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.863395] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.868737] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.869875] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.875117] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.897493] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.920617] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.943405] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.966609] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.969962] Linux agpgart interface v0.103
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.972929] loop: module loaded
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.973677] libphy: Fixed MDIO Bus: probed
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.974535] tun: Universal TUN/TAP device driver, 1.6
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    2.975526] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.004412] PPP generic driver version 2.4.2
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.005427] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.007206] ehci-pci: EHCI PCI platform driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.008078] ehci-platform: EHCI generic platform driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.009205] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.010278] ohci-pci: OHCI PCI platform driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.011288] ohci-platform: OHCI generic platform driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.012113] uhci_hcd: USB Universal Host Controller Interface driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.013145] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.015581] i8042: Warning: Keylock active
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.017315] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.017899] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.018691] mousedev: PS/2 mouse device common for all mice
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.019793] rtc_cmos 00:00: RTC can wake from S4
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.020643] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.021623] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.022502] i2c /dev entries driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.023376] device-mapper: uevent: version 1.0.3
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.024216] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.025833] ledtrig-cpu: registered to indicate activity on CPUs
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.027521] NET: Registered protocol family 10
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.028423] NET: Registered protocol family 17
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.029610] Key type dns_resolver registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.030730] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.031981] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.033039] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.034284] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.035826] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.038023] registered taskstats version 1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.039110] Loading compiled-in X.509 certificates
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.040572] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.042642] zswap: loaded using pool lzo/zbud
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.045343] Key type trusted registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.049314] Key type encrypted registered
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.050286] ima: No TPM chip found, activating TPM-bypass!
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.051465] evm: HMAC attrs: 0x1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.052594]   Magic number: 14:651:419
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.053479] acpi LNXCPU:de: hash matches
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.054600] rtc_cmos 00:00: setting system clock to 2018-08-07 07:25:57 UTC (1533626757)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.056105] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.057340] EDD information not available.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.058167] PM: Hibernation image not present or could not be loaded.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.059543] Freeing unused kernel memory: 1496K
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.060227] Write protecting the kernel read-only data: 14336k
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.061890] Freeing unused kernel memory: 1956K
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.062798] Freeing unused kernel memory: 92K
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.075385] systemd-udevd[118]: starting version 204
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.128034] scsi host0: Virtio SCSI HBA
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.131736] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.136955] AVX2 version of gcm_enc/dec engaged.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.137908] AES CTR mode by8 optimization enabled
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.172102] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.172972] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.174148] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.175211] sd 0:0:1:0: [sda] Write Protect is off
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.175923] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.176057] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.179023]  sda: sda1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.180396] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.219642] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.827779] tsc: Refined TSC clocksource calibration: 2300.000 MHz
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    3.834508] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x212735223b2, max_idle_ns: 440795277976 ns
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    4.052486] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    6.127676] floppy0: no floppy controllers found
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.291365] raid6: sse2x1   gen()  8737 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.359363] raid6: sse2x1   xor()  6512 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.427359] raid6: sse2x2   gen() 10615 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.495364] raid6: sse2x2   xor()  6975 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.563363] raid6: sse2x4   gen() 12402 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.631361] raid6: sse2x4   xor()  8808 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.699363] raid6: avx2x1   gen() 16965 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.767364] raid6: avx2x2   gen() 19754 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.835362] raid6: avx2x4   gen() 22327 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.837310] raid6: using algorithm avx2x4 gen() 22327 MB/s
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.840012] raid6: using avx2x2 recovery algorithm
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.844366] xor: automatically using best checksumming function:
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.883391]    avx       : 26986.000 MB/sec
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.899212] Btrfs loaded
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.952391] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    7.956108] EXT4-fs (sda1): write access will be enabled during recovery
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.048335] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.058912] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.062923] EXT4-fs (sda1): recovery complete
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.072991] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.322982] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.478020] random: mountall: uninitialized urandom read (12 bytes read, 28 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.541042] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    8.762618] random: cloud-init: uninitialized urandom read (32 bytes read, 35 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.388811] random: cloud-init: uninitialized urandom read (32 bytes read, 43 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.550890] systemd-udevd[704]: starting version 204
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.678269] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.744011] intel_rapl: no valid rapl domains found in package 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.799995] intel_rapl: no valid rapl domains found in package 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.814051] ppdev: user-space parallel port driver
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.928225] random: mktemp: uninitialized urandom read (6 bytes read, 54 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [    9.989396] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   10.063992] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   10.240265] random: cloud-init: uninitialized urandom read (32 bytes read, 55 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   10.535848] random: mktemp: uninitialized urandom read (12 bytes read, 58 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   10.612663] random: mktemp: uninitialized urandom read (6 bytes read, 59 bits of entropy available)
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   10.695417] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   10.743865] EXT4-fs (sda1): resized filesystem to 7864064
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   11.159152] init: failsafe main process (1095) killed by TERM signal
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Running set_multiqueue.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Set channels for eth0 to 4.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  7 07:26:05 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Starting Google Accounts daemon.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for me.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account me.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for henrik.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account henrik.
Aug  7 07:26:06 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for emma.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-clock-skew: INFO Synced system time with hardware clock.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account emma.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for igor.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account igor.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account konstantinhaase.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for aj.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account aj.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for solarce.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.400699] random: nonblocking pool is initialized
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account solarce.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for asari.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account asari.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for bogdana.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account bogdana.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for konstantin.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.629011] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.638414] Bridge firewalling registered
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.650146] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account konstantin.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for carmen.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.690857] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account carmen.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Creating a new user account for maria.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.768356] Initializing XFRM netlink socket
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.777796] Netfilter messages via NETLINK v0.30.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.781409] ctnetlink v0.93: registering with nfnetlink.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Created user account maria.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   12.807742] floppy0: no floppy controllers found
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 google-accounts: INFO Removing user packer.
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 07:26:07 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 pollinate: To re-seed this system again, use the -r|--reseed option
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 cron[1710]: (CRON) INFO (pidfile fd = 3)
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 cron[1748]: (CRON) STARTUP (fork ok)
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 cron[1748]: (CRON) INFO (Running @reboot jobs)
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 acpid: starting up with netlink and the input layer
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 acpid: 1 rule loaded
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 acpid: waiting for events: event logging is off
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 haveged: haveged starting up
Aug  7 07:26:08 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   13.698316] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1849]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: proto: precision = 0.114 usec
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen normally on 3 eth0 10.20.1.188 UDP 123
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: peers refreshed
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listening on routing socket on fd #21 for interface updates
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [   18.890836] init: plymouth-upstart-bridge main process ended, respawning
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 startup-script: INFO Found startup-script in metadata.
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 startup-script: INFO startup-script: job 1 at Tue Aug  7 10:36:00 2018
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 startup-script: INFO startup-script: Return code 0.
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 startup-script: INFO startup-script: Return code 0.
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 startup-script: INFO Finished running startup scripts.
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: 
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: #############################################################
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: 1024 72:dd:2b:05:54:96:44:95:95:e2:e0:11:f5:00:e0:4e  root@travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 (DSA)
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: 256 92:76:14:14:2c:6e:4d:5c:2e:59:c4:d3:b0:9d:15:e6  root@travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 (ECDSA)
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: 256 00:cc:ae:63:93:11:a0:74:f1:f5:df:0b:69:22:fd:f6  root@travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 (ED25519)
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: 2048 c1:cd:2c:5f:10:44:05:41:c6:3a:07:f3:52:59:6a:c4  root@travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 (RSA)
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  7 07:26:13 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ec2: #############################################################
Aug  7 07:26:22 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpdate[2250]: the NTP socket is in use, exiting
Aug  7 07:27:43 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  109.088459] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.424775] device veth820fd8d entered promiscuous mode
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.528505] cgroup: docker-runc (4868) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.528508] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.600054] eth0: renamed from vethe7a4223
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.647400] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.648643] docker0: port 1(veth820fd8d) entered forwarding state
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.648680] docker0: port 1(veth820fd8d) entered forwarding state
Aug  7 07:28:23 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  148.648710] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  7 07:28:26 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  7 07:28:26 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen normally on 6 docker0 fe80::42:7eff:feb5:efb2 UDP 123
Aug  7 07:28:26 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  7 07:28:26 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: peers refreshed
Aug  7 07:28:26 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 ntpd[1850]: new interface(s) found: waking up resolver
Aug  7 07:28:38 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 kernel: [  163.655443] docker0: port 1(veth820fd8d) entered forwarding state
Aug  7 08:17:01 travis-job-6058efd4-6ccc-4a9c-999d-bafec9c9b196 CRON[2739]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:02743dd6:start=1533631462503942401,finish=1533631462512986845,duration=9044444
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18fb9cf4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b57e167
travis_time:start:1b57e167
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0271bb4e
$ dmesg | grep -i kill
