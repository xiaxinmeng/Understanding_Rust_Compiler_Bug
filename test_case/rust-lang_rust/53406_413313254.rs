plain

[00:04:55] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:55] tidy error: /checkout/src/test/ui/issues/issue-53348.rs:15: trailing whitespace
[00:04:55] tidy error: /checkout/src/test/ui/issues/issue-53348.rs:17: trailing whitespace
[00:04:55] tidy error: /checkout/src/test/ui/issues/issue-53348.rs: missing trailing newline
[00:04:56] some tidy checks failed
[00:04:56] 
[00:04:56] 
[00:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:56] 
[00:04:56] 
[00:04:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:56] Build completed unsuccessfully in 0:00:50
[00:04:56] Build completed unsuccessfully in 0:00:50
[00:04:56] Makefile:79: recipe for target 'tidy' failed
[00:04:56] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01052c64
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:0373188f
$ sudo tail -n 500 /var/log/syslog
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] kvm-clock: using sched offset of 1616012934 cycles
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Zone ranges:
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   Device   empty
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Movable zone start for each node
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Early memory node ranges
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Policy zone: Normal
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.484628] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.487980] pid_max: default: 32768 minimum: 301
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.490295] ACPI: Core revision 20150930
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.498090] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.501455] Security Framework initialized
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.503187] Yama: becoming mindful.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.504781] AppArmor: AppArmor disabled by boot time parameter
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.508782] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.522249] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.529280] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.531729] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.534551] Initializing cgroup subsys io
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.536397] Initializing cgroup subsys memory
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.538686] Initializing cgroup subsys devices
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.540856] Initializing cgroup subsys freezer
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.542405] Initializing cgroup subsys net_cls
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.544293] Initializing cgroup subsys perf_event
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.545957] Initializing cgroup subsys net_prio
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.548188] Initializing cgroup subsys hugetlb
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.550031] Initializing cgroup subsys pids
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.551738] CPU: Physical Processor ID: 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.553717] CPU: Processor Core ID: 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.556709] mce: CPU supports 32 MCE banks
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.559121] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.562226] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.568072] Freeing SMP alternatives memory: 32K
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.581432] ftrace: allocating 32185 entries in 126 pages
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.643649] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.646718] smpboot: Max logical packages: 2
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.649597] x2apic enabled
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.652560] Switched APIC routing to physical x2apic.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.658254] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.766343] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.770614] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.776659] x86: Booting SMP configuration:
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.778785] .... node  #0, CPUs:      #1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.780831] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.786865]  #2
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.788188] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.794527]  #3
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.795398] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.801564] x86: Booted up 1 node, 4 CPUs
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.803322] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.807289] devtmpfs: initialized
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.812719] evm: security.selinux
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.814463] evm: security.SMACK64
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.815584] evm: security.SMACK64EXEC
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.817264] evm: security.SMACK64TRANSMUTE
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.819203] evm: security.SMACK64MMAP
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.820539] evm: security.ima
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.821457] evm: security.capability
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.823336] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.826964] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.829628] pinctrl core: initialized pinctrl subsystem
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.832056] RTC time: 19:32:04, date: 08/15/18
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.835033] NET: Registered protocol family 16
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.846410] cpuidle: using governor ladder
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.858420] cpuidle: using governor menu
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.860177] PCCT header not found.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.862091] ACPI: bus type PCI registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.863696] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.867130] PCI: Using configuration type 1 for base access
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.883826] ACPI: Added _OSI(Module Device)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.885722] ACPI: Added _OSI(Processor Device)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.887439] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.889870] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.896241] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.922873] ACPI: Interpreter enabled
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.924252] ACPI: (supports S0 S3 S4 S5)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.925688] ACPI: Using IOAPIC for interrupt routing
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.927716] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.960677] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.963501] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.966718] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.969724] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.975644] PCI host bridge to bus 0000:00
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.977722] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.980644] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.983590] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.986736] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.989755] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.991752] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    0.992266] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.017872] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.042470] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.046333] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.058398] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.065561] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.086498] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.095573] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.103061] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.127843] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.133170] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.139191] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.143568] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.148209] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.172147] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.174623] vgaarb: loaded
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.175670] SCSI subsystem initialized
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.177671] libata version 3.00 loaded.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.177702] ACPI: bus type USB registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.179579] usbcore: registered new interface driver usbfs
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.181967] usbcore: registered new interface driver hub
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.183988] usbcore: registered new device driver usb
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.187170] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.190275] dmi: Firmware registration failed.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.192397] PCI: Using ACPI for IRQ routing
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.193638] PCI: pci_cache_line_size set to 64 bytes
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.193749] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.193752] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.193933] NetLabel: Initializing
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.195507] NetLabel:  domain hash size = 128
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.197246] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.200106] NetLabel:  unlabeled traffic allowed by default
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.202971] amd_nb: Cannot enumerate AMD northbridges
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.205408] clocksource: Switched to clocksource kvm-clock
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.215575] pnp: PnP ACPI init
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217268] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217344] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217387] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217457] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217497] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217537] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217581] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.217775] pnp: PnP ACPI: found 7 devices
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.226819] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.230115] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.230119] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.230120] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.230122] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.230175] NET: Registered protocol family 2
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.232123] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.235684] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.239452] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.243208] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.245684] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.249097] NET: Registered protocol family 1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.251101] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.253684] PCI: CLS 0 bytes, default 64
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    1.253766] Unpacking initramfs...
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.488035] Freeing initrd memory: 21432K
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.489970] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.492722] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.496283] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.499416] hw unit of domain pp0-core 2^-0 Joules
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.501042] hw unit of domain package 2^-0 Joules
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.502431] hw unit of domain dram 2^-0 Joules
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.503873] Scanning for low memory corruption every 60 seconds
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.506470] audit: initializing netlink subsys (disabled)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.508342] audit: type=2000 audit(1534361526.391:1): initialized
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.510541] Initialise system trusted keyring
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.512981] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.515366] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.518847] zbud: loaded
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.519987] VFS: Disk quotas dquot_6.6.0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.521804] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.524428] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.527131] fuse init (API version 7.23)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.528883] Key type big_key registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.530292] Allocating IMA MOK and blacklist keyrings.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.536356] Key type asymmetric registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.537778] Asymmetric key parser 'x509' registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.539210] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.542329] io scheduler noop registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.543608] io scheduler deadline registered (default)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.545513] io scheduler cfq registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.546765] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.548577] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.550910] intel_idle: does not run on family 6 model 62
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.551141] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.553407] ACPI: Power Button [PWRF]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.554807] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.557623] ACPI: Sleep Button [SLPF]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.559736] GHES: HEST is not enabled!
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.563639] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.566279] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.574005] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.576240] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.584905] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.608856] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.633476] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.658177] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.683005] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.689208] Linux agpgart interface v0.103
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.694872] loop: module loaded
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.696729] libphy: Fixed MDIO Bus: probed
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.698362] tun: Universal TUN/TAP device driver, 1.6
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.699968] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.752436] PPP generic driver version 2.4.2
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.754859] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.756917] ehci-pci: EHCI PCI platform driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.758727] ehci-platform: EHCI generic platform driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.760617] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.762795] ohci-pci: OHCI PCI platform driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.764323] ohci-platform: OHCI generic platform driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.766403] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.768740] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.772023] i8042: Warning: Keylock active
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.775229] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.777707] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.780411] mousedev: PS/2 mouse device common for all mice
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.783586] rtc_cmos 00:00: RTC can wake from S4
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.786810] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.790163] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.793371] i2c /dev entries driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.795918] device-mapper: uevent: version 1.0.3
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.798253] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.802805] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.807464] NET: Registered protocol family 10
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.809525] NET: Registered protocol family 17
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.811175] Key type dns_resolver registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.813272] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.816863] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.818826] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.821764] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.824493] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.828138] registered taskstats version 1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.829700] Loading compiled-in X.509 certificates
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.832609] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.836032] zswap: loaded using pool lzo/zbud
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.840382] Key type trusted registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.848117] Key type encrypted registered
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.849901] ima: No TPM chip found, activating TPM-bypass!
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.852729] evm: HMAC attrs: 0x1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.854594]   Magic number: 14:632:546
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.856428] tty ttyS22: hash matches
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.858541] mem null: hash matches
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.860042] rtc_cmos 00:00: setting system clock to 2018-08-15 19:32:07 UTC (1534361527)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.864267] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.867145] EDD information not available.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.869689] PM: Hibernation image not present or could not be loaded.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.872009] Freeing unused kernel memory: 1496K
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.874462] Write protecting the kernel read-only data: 14336k
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.878292] Freeing unused kernel memory: 1956K
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.880984] Freeing unused kernel memory: 92K
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.901752] systemd-udevd[120]: starting version 204
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.944129] AVX version of gcm_enc/dec engaged.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.946031] AES CTR mode by8 optimization enabled
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.987678] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.987848] scsi host0: Virtio SCSI HBA
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    3.998564] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.074275] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.074544] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.074546] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.075064] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.075066] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.075192] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.077954]  sda: sda1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.080661] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.501644] tsc: Refined TSC clocksource calibration: 2499.793 MHz
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.504152] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24087658c32, max_idle_ns: 440795284564 ns
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    4.839663] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    6.973678] floppy0: no floppy controllers found
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.165476] raid6: sse2x1   gen()  8212 MB/s
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.233457] raid6: sse2x1   xor()  6646 MB/s
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.301459] raid6: sse2x2   gen() 10464 MB/s
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.369469] raid6: sse2x2   xor()  7675 MB/s
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.437506] raid6: sse2x4   gen() 12419 MB/s
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.505452] raid6: sse2x4   xor()  8395 MB/s
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.507797] raid6: using algorithm sse2x4 gen() 12419 MB/s
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.510089] raid6: .... xor() 8395 MB/s, rmw enabled
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.511855] raid6: using ssse3x2 recovery algorithm
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.515009] xor: automatically using best checksumming function:
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.553420]    avx       : 21445.000 MB/sec
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.570433] Btrfs loaded
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.627177] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.629992] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.706986] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.721543] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.723482] EXT4-fs (sda1): recovery complete
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.731369] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    8.970742] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    9.097764] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    9.153511] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    9.366072] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [    9.985463] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.130871] systemd-udevd[702]: starting version 204
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.255842] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.330459] ppdev: user-space parallel port driver
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.441918] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.493498] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.558751] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.724579] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   10.989382] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   11.066805] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   11.152893] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   11.199713] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   11.918585] init: failsafe main process (1093) killed by TERM signal
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Running set_multiqueue.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 19:32:15 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Starting Google Accounts daemon.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for me.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account me.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account me.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for henrik.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account henrik.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for emma.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account emma.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for igor.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account igor.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account konstantinhaase.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.013676] random: nonblocking pool is initialized
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for aj.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account aj.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for solarce.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account solarce.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for asari.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account asari.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for bogdana.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.309515] floppy0: no floppy controllers found
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.310573] work still pending
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account bogdana.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for konstantin.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.383473] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.387824] Bridge firewalling registered
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.402344] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account konstantin.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for carmen.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.442294] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account carmen.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Creating a new user account for maria.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.526609] Initializing XFRM netlink socket
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.535204] Netfilter messages via NETLINK v0.30.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.538698] ctnetlink v0.93: registering with nfnetlink.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Created user account maria.
Aug 15 19:32:16 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 google-accounts: INFO Removing user packer.
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 cron[1707]: (CRON) INFO (pidfile fd = 3)
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 cron[1743]: (CRON) STARTUP (fork ok)
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 cron[1743]: (CRON) INFO (Running @reboot jobs)
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 acpid: starting up with netlink and the input layer
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 acpid: 1 rule loaded
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 acpid: waiting for events: event logging is off
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 haveged: haveged starting up
Aug 15 19:32:17 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   13.976976] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1854]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: proto: precision = 0.104 usec
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen normally on 3 eth0 10.20.0.78 UDP 123
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: peers refreshed
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listening on routing socket on fd #21 for interface updates
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   19.148170] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 startup-script: INFO Found startup-script in metadata.
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 startup-script: INFO startup-script: job 1 at Wed Aug 15 22:42:00 2018
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 startup-script: INFO startup-script: Return code 0.
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 startup-script: INFO startup-script: Return code 0.
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 startup-script: INFO Finished running startup scripts.
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: 
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: #############################################################
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: 1024 d3:28:0f:1e:1b:cc:d5:1f:07:8a:8d:ad:d1:2a:fd:1c  root@travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 (DSA)
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: 256 96:e0:8d:1d:2d:f5:df:ee:af:76:5d:58:1b:6c:d4:9d  root@travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 (ECDSA)
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: 256 33:36:1e:f3:9a:c6:5a:78:8b:72:25:48:c8:a3:d4:a7  root@travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 (ED25519)
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: 2048 92:d0:85:9b:8d:a2:8f:38:ea:dc:ad:b3:fd:1a:ee:bd  root@travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 (RSA)
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 19:32:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ec2: #############################################################
Aug 15 19:32:31 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpdate[2508]: the NTP socket is in use, exiting
Aug 15 19:33:00 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [   57.229264] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.459329] device vethfe4a70f entered promiscuous mode
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.459374] docker0: port 1(vethfe4a70f) entered forwarding state
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.459379] docker0: port 1(vethfe4a70f) entered forwarding state
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.460207] docker0: port 1(vethfe4a70f) entered disabled state
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.549702] cgroup: docker-runc (4953) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.549705] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.625211] eth0: renamed from vetha01a7c9
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.666365] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.667614] docker0: port 1(vethfe4a70f) entered forwarding state
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.667642] docker0: port 1(vethfe4a70f) entered forwarding state
Aug 15 19:35:06 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  183.667660] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 19:35:10 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 15 19:35:10 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen normally on 6 docker0 fe80::42:efff:fe9d:bb86 UDP 123
Aug 15 19:35:10 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 19:35:10 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: peers refreshed
Aug 15 19:35:10 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 ntpd[1855]: new interface(s) found: waking up resolver
Aug 15 19:35:22 travis-job-22c8b454-9773-470b-9274-d8b4f3347b87 kernel: [  198.676655] docker0: port 1(vethfe4a70f) entered forwarding state
---
travis_time:end:2217fc63:start=1534361879093275238,finish=1534361879101126450,duration=7851212
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0867b3ea
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05b8b334
travis_time:start:05b8b334
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:16264d29
$ dmesg | grep -i kill
