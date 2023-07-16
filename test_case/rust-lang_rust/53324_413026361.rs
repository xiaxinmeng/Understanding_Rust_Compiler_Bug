plain

[00:04:19] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:19] tidy error: /checkout/src/test/compile-fail/feature-gate-self-in-typedefs.rs:15: trailing whitespace
[00:04:20] some tidy checks failed
[00:04:20] 
[00:04:20] 
[00:04:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:20] 
[00:04:20] 
[00:04:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:20] Build completed unsuccessfully in 0:00:56
[00:04:20] Build completed unsuccessfully in 0:00:56
[00:04:20] Makefile:79: recipe for target 'tidy' failed
[00:04:20] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01a8bbe0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:145c602a
$ sudo tail -n 500 /var/log/syslog
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] kvm-clock: using sched offset of 1629146040 cycles
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Zone ranges:
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   Device   empty
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Movable zone start for each node
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Early memory node ranges
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Policy zone: Normal
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] console [ttyS0] enabled
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.627934] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.633365] pid_max: default: 32768 minimum: 301
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.635542] ACPI: Core revision 20150930
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.643547] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.647402] Security Framework initialized
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.649899] Yama: becoming mindful.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.651935] AppArmor: AppArmor disabled by boot time parameter
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.657367] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.669314] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.676931] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.682481] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.688163] Initializing cgroup subsys io
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.690496] Initializing cgroup subsys memory
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.693559] Initializing cgroup subsys devices
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.695902] Initializing cgroup subsys freezer
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.699141] Initializing cgroup subsys net_cls
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.701753] Initializing cgroup subsys perf_event
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.704336] Initializing cgroup subsys net_prio
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.706796] Initializing cgroup subsys hugetlb
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.709984] Initializing cgroup subsys pids
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.713182] CPU: Physical Processor ID: 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.715455] CPU: Processor Core ID: 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.717763] mce: CPU supports 32 MCE banks
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.720936] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.724101] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.730362] Freeing SMP alternatives memory: 32K
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.742515] ftrace: allocating 32185 entries in 126 pages
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.799925] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.804200] smpboot: Max logical packages: 2
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.807337] x2apic enabled
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.810238] Switched APIC routing to physical x2apic.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.816141] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.925182] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.931292] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.937134] x86: Booting SMP configuration:
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.939254] .... node  #0, CPUs:      #1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.942946] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.948785]  #2
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.949907] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.955724]  #3
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.956820] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.962596] x86: Booted up 1 node, 4 CPUs
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.964721] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.970613] devtmpfs: initialized
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.976354] evm: security.selinux
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.978628] evm: security.SMACK64
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.980464] evm: security.SMACK64EXEC
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.982062] evm: security.SMACK64TRANSMUTE
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.984218] evm: security.SMACK64MMAP
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.986257] evm: security.ima
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.987925] evm: security.capability
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.990541] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.996050] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    0.999294] pinctrl core: initialized pinctrl subsystem
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.001921] RTC time: 21:30:13, date: 08/14/18
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.005225] NET: Registered protocol family 16
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.017212] cpuidle: using governor ladder
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.029212] cpuidle: using governor menu
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.031298] PCCT header not found.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.033230] ACPI: bus type PCI registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.035438] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.039282] PCI: Using configuration type 1 for base access
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.055221] ACPI: Added _OSI(Module Device)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.057476] ACPI: Added _OSI(Processor Device)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.060478] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.063827] ACPI: Added _OSI(Processor Aggregator Device)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.070083] ACPI: Executed 2 blocks of module-level executable AML code
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.096191] ACPI: Interpreter enabled
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.098305] ACPI: (supports S0 S3 S4 S5)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.100305] ACPI: Using IOAPIC for interrupt routing
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.103117] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.137080] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.142206] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.146895] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.150445] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.157458] PCI host bridge to bus 0000:00
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.160493] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.164542] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.168301] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.172394] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.176137] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.179020] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.179445] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.207062] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.232809] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.237173] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.246979] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.254768] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.275888] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.285382] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.293470] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.315280] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.321487] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.326864] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.332264] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.337274] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.359623] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.362920] vgaarb: loaded
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.364891] SCSI subsystem initialized
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.367066] libata version 3.00 loaded.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.367093] ACPI: bus type USB registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.369125] usbcore: registered new interface driver usbfs
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.372163] usbcore: registered new interface driver hub
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.375037] usbcore: registered new device driver usb
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.378185] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.382183] dmi: Firmware registration failed.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.384600] PCI: Using ACPI for IRQ routing
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.387120] PCI: pci_cache_line_size set to 64 bytes
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.387243] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.387245] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.387374] NetLabel: Initializing
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.388966] NetLabel:  domain hash size = 128
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.390970] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.393277] NetLabel:  unlabeled traffic allowed by default
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.396423] amd_nb: Cannot enumerate AMD northbridges
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.399556] clocksource: Switched to clocksource kvm-clock
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.409092] pnp: PnP ACPI init
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.410907] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.410986] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.411032] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.411083] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.411126] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.411167] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.411211] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.411378] pnp: PnP ACPI: found 7 devices
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.421353] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.425798] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.425801] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.425802] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.425804] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.425841] NET: Registered protocol family 2
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.428020] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.432563] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.435841] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.438851] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.442137] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.445847] NET: Registered protocol family 1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.448121] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.451753] PCI: CLS 0 bytes, default 64
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    1.451834] Unpacking initramfs...
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.491983] Freeing initrd memory: 21432K
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.494572] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.497917] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.503397] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.508084] hw unit of domain pp0-core 2^-0 Joules
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.510368] hw unit of domain package 2^-0 Joules
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.513014] hw unit of domain dram 2^-0 Joules
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.515537] Scanning for low memory corruption every 60 seconds
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.520071] audit: initializing netlink subsys (disabled)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.523240] audit: type=2000 audit(1534282215.464:1): initialized
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.527941] Initialise system trusted keyring
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.530848] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.534865] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.539401] zbud: loaded
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.541604] VFS: Disk quotas dquot_6.6.0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.544314] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.548999] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.552899] fuse init (API version 7.23)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.555753] Key type big_key registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.558264] Allocating IMA MOK and blacklist keyrings.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.565980] Key type asymmetric registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.568120] Asymmetric key parser 'x509' registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.570948] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.577186] io scheduler noop registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.579527] io scheduler deadline registered (default)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.582560] io scheduler cfq registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.584993] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.588808] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.593407] intel_idle: does not run on family 6 model 45
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.593500] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.597570] ACPI: Power Button [PWRF]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.599766] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.603673] ACPI: Sleep Button [SLPF]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.607034] GHES: HEST is not enabled!
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.612419] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.616118] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.626194] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.629509] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.639359] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.664971] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.691758] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.719450] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.746480] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.755601] Linux agpgart interface v0.103
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.762422] loop: module loaded
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.764821] libphy: Fixed MDIO Bus: probed
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.767009] tun: Universal TUN/TAP device driver, 1.6
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.770619] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.820676] PPP generic driver version 2.4.2
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.823726] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.828315] ehci-pci: EHCI PCI platform driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.830775] ehci-platform: EHCI generic platform driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.833877] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.837035] ohci-pci: OHCI PCI platform driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.839097] ohci-platform: OHCI generic platform driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.841645] uhci_hcd: USB Universal Host Controller Interface driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.845623] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.850181] i8042: Warning: Keylock active
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.853428] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.855931] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.858764] mousedev: PS/2 mouse device common for all mice
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.862720] rtc_cmos 00:00: RTC can wake from S4
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.865581] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.869339] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.872476] i2c /dev entries driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.874391] device-mapper: uevent: version 1.0.3
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.877323] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.881122] ledtrig-cpu: registered to indicate activity on CPUs
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.886121] NET: Registered protocol family 10
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.889047] NET: Registered protocol family 17
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.891782] Key type dns_resolver registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.894731] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.897961] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.901824] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.905656] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.908537] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.914986] registered taskstats version 1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.917499] Loading compiled-in X.509 certificates
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.922356] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.928502] zswap: loaded using pool lzo/zbud
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.933657] Key type trusted registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.941364] Key type encrypted registered
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.943986] ima: No TPM chip found, activating TPM-bypass!
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.947533] evm: HMAC attrs: 0x1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.950045]   Magic number: 14:769:550
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.952539] tty ttyS26: hash matches
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.954740] clockevents clockevent1: hash matches
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.958095] acpi PNP0C0F:01: hash matches
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.961257] rtc_cmos 00:00: setting system clock to 2018-08-14 21:30:16 UTC (1534282216)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.966527] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.970364] EDD information not available.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.974113] PM: Hibernation image not present or could not be loaded.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.975844] Freeing unused kernel memory: 1496K
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.978943] Write protecting the kernel read-only data: 14336k
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.983080] Freeing unused kernel memory: 1956K
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    3.985762] Freeing unused kernel memory: 92K
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.006101] systemd-udevd[119]: starting version 204
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.065055] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.065157] scsi host0: Virtio SCSI HBA
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.073025] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.086543] AVX version of gcm_enc/dec engaged.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.088909] AES CTR mode by8 optimization enabled
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.163470] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.163507] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.171513] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.175891] sd 0:0:1:0: [sda] Write Protect is off
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.179717] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.180053] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.190732]  sda: sda1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.194724] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.511672] tsc: Refined TSC clocksource calibration: 2600.009 MHz
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.515820] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a44c4d56, max_idle_ns: 440795316911 ns
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    4.926283] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    7.047755] floppy0: no floppy controllers found
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.215566] raid6: sse2x1   gen()  8733 MB/s
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.283566] raid6: sse2x1   xor()  6512 MB/s
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.351572] raid6: sse2x2   gen() 10621 MB/s
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.419572] raid6: sse2x2   xor()  7141 MB/s
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.487570] raid6: sse2x4   gen() 12440 MB/s
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.555568] raid6: sse2x4   xor()  8743 MB/s
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.557950] raid6: using algorithm sse2x4 gen() 12440 MB/s
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.560910] raid6: .... xor() 8743 MB/s, rmw enabled
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.563381] raid6: using ssse3x2 recovery algorithm
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.567539] xor: automatically using best checksumming function:
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.607567]    avx       : 27051.000 MB/sec
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.623951] Btrfs loaded
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.683449] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.687373] EXT4-fs (sda1): write access will be enabled during recovery
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.772860] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.782943] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.785620] EXT4-fs (sda1): recovery complete
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    8.793936] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    9.034817] random: init: uninitialized urandom read (12 bytes read, 24 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    9.178931] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    9.235187] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [    9.480871] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.148432] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.298531] systemd-udevd[702]: starting version 204
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.426160] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.481586] intel_rapl: no valid rapl domains found in package 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.528639] intel_rapl: no valid rapl domains found in package 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.534499] ppdev: user-space parallel port driver
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.638906] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.696459] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.762612] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   10.933720] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   11.203879] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   11.287774] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   11.374955] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   11.433575] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   11.890365] init: failsafe main process (1093) killed by TERM signal
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Running set_multiqueue.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 14 21:30:24 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for me.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account me.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for henrik.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account henrik.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for emma.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account emma.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for igor.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account igor.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account konstantinhaase.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for aj.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account aj.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for solarce.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account solarce.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for asari.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account asari.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for bogdana.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account bogdana.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.278120] random: nonblocking pool is initialized
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for konstantin.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account konstantin.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for carmen.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account carmen.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.386455] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.390630] Bridge firewalling registered
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Creating a new user account for maria.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.404693] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.437142] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Created user account maria.
Aug 14 21:30:25 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-accounts: INFO Removing user packer.
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.497196] Initializing XFRM netlink socket
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.503649] floppy0: no floppy controllers found
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.506915] Netfilter messages via NETLINK v0.30.
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 google-clock-skew: INFO Synced system time with hardware clock.
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.511586] ctnetlink v0.93: registering with nfnetlink.
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 cron[1702]: (CRON) INFO (pidfile fd = 3)
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 cron[1744]: (CRON) STARTUP (fork ok)
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 cron[1744]: (CRON) INFO (Running @reboot jobs)
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 acpid: starting up with netlink and the input layer
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 acpid: 1 rule loaded
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 acpid: waiting for events: event logging is off
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 haveged: haveged starting up
Aug 14 21:30:26 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   13.936020] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 14 21:30:49 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpdate[1871]: adjust time server 169.254.169.254 offset 0.003455 sec
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1907]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: proto: precision = 0.107 usec
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen normally on 3 eth0 10.20.2.149 UDP 123
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: peers refreshed
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listening on routing socket on fd #21 for interface updates
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   44.137222] init: plymouth-upstart-bridge main process ended, respawning
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 startup-script: INFO Found startup-script in metadata.
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 startup-script: INFO startup-script: job 1 at Wed Aug 15 00:40:00 2018
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 startup-script: INFO startup-script: Return code 0.
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 startup-script: INFO startup-script: Return code 0.
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 startup-script: INFO Finished running startup scripts.
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: 
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: #############################################################
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: 1024 0d:44:e5:9d:54:bd:14:2e:8e:5a:0d:53:e4:37:9e:75  root@travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 (DSA)
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: 256 ca:0e:ee:d0:5a:5a:27:62:0f:8b:4d:1b:b6:e0:eb:60  root@travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 (ECDSA)
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: 256 a8:fb:3f:d7:7c:4e:27:a2:a0:6e:6e:26:45:4e:bb:65  root@travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 (ED25519)
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: 2048 90:10:14:e7:6f:99:7b:41:53:d9:8b:8e:7b:8d:fe:a9  root@travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 (RSA)
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 21:30:56 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ec2: #############################################################
Aug 14 21:31:30 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [   78.112768] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.039185] device vethd8a78af entered promiscuous mode
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.039301] docker0: port 1(vethd8a78af) entered forwarding state
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.039311] docker0: port 1(vethd8a78af) entered forwarding state
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.039691] docker0: port 1(vethd8a78af) entered disabled state
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.139335] cgroup: docker-runc (4889) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.139339] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.214864] eth0: renamed from vethb985088
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.260669] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.261924] docker0: port 1(vethd8a78af) entered forwarding state
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.261953] docker0: port 1(vethd8a78af) entered forwarding state
Aug 14 21:32:37 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  145.261974] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 14 21:32:40 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug 14 21:32:40 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen normally on 6 docker0 fe80::42:5bff:fed8:a6b7 UDP 123
Aug 14 21:32:40 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 14 21:32:40 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: peers refreshed
Aug 14 21:32:40 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 ntpd[1908]: new interface(s) found: waking up resolver
Aug 14 21:32:52 travis-job-fb6f3572-f96c-4605-8eda-4f52a6842050 kernel: [  160.290497] docker0: port 1(vethd8a78af) entered forwarding state
---
travis_time:end:3214e61e:start=1534282552716344161,finish=1534282552723782708,duration=7438547
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04cd88b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0187b594
travis_time:start:0187b594
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:09569a66
$ dmesg | grep -i kill
