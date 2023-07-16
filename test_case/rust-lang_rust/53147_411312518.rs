plain

[00:04:39] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:72: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:100: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:108: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:136: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:164: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:172: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:195: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:283: TODO is deprecated; use FIXME
[00:04:39] tidy error: /checkout/src/test/ui/suggestions/dont-suggest-ref.rs:318: TODO is deprecated; use FIXME
[00:04:41] some tidy checks failed
[00:04:41] 
[00:04:41] 
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:41] 
[00:04:41] 
[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:00:52
[00:04:41] Build completed unsuccessfully in 0:00:52
[00:04:41] make: *** [tidy] Error 1
[00:04:41] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0297cf31
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:3702a88a
$ sudo tail -n 500 /var/log/syslog
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] kvm-clock: using sched offset of 1779594883 cycles
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Zone ranges:
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   Device   empty
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Movable zone start for each node
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Early memory node ranges
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] On node 0 totalpages: 3932049
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Policy zone: Normal
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Hierarchical RCU implementation.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] Console: colour VGA+ 80x25
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] console [ttyS0] enabled
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.000000] tsc: Detected 2300.000 MHz processor
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.393693] Calibrating delay loop (skipped) preset value.. 4600.00 BogoMIPS (lpj=9200000)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.395665] pid_max: default: 32768 minimum: 301
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.396540] ACPI: Core revision 20150930
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.403482] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.404997] Security Framework initialized
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.405614] Yama: becoming mindful.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.406466] AppArmor: AppArmor disabled by boot time parameter
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.409729] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.420279] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.426214] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.427643] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.429006] Initializing cgroup subsys io
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.429860] Initializing cgroup subsys memory
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.430466] Initializing cgroup subsys devices
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.431089] Initializing cgroup subsys freezer
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.431769] Initializing cgroup subsys net_cls
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.432409] Initializing cgroup subsys perf_event
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.433433] Initializing cgroup subsys net_prio
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.434141] Initializing cgroup subsys hugetlb
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.434825] Initializing cgroup subsys pids
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.435527] CPU: Physical Processor ID: 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.436254] CPU: Processor Core ID: 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.438194] mce: CPU supports 32 MCE banks
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.439100] Last level iTLB entries: 4KB 1024, 2MB 1024, 4MB 1024
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.440031] Last level dTLB entries: 4KB 1024, 2MB 1024, 4MB 1024, 1GB 4
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.443158] Freeing SMP alternatives memory: 32K
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.451901] ftrace: allocating 32185 entries in 126 pages
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.501314] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.502567] smpboot: Max logical packages: 2
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.503990] x2apic enabled
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.506320] Switched APIC routing to physical x2apic.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.510114] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.616804] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.30GHz (family: 0x6, model: 0x3f, stepping: 0x0)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.620529] Performance Events: unsupported p6 CPU model 63 no PMU driver, software events only.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.625529] x86: Booting SMP configuration:
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.627062] .... node  #0, CPUs:      #1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.628528] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.635122]  #2
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.636249] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.642292]  #3
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.643174] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.649190] x86: Booted up 1 node, 4 CPUs
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.651569] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.656697] devtmpfs: initialized
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.662664] evm: security.selinux
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.664000] evm: security.SMACK64
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.665088] evm: security.SMACK64EXEC
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.666404] evm: security.SMACK64TRANSMUTE
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.668094] evm: security.SMACK64MMAP
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.669678] evm: security.ima
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.670608] evm: security.capability
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.672159] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.676819] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.679052] pinctrl core: initialized pinctrl subsystem
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.680666] RTC time:  7:10:00, date: 08/08/18
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.683018] NET: Registered protocol family 16
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.695260] cpuidle: using governor ladder
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.708928] cpuidle: using governor menu
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.710902] PCCT header not found.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.713089] ACPI: bus type PCI registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.714994] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.720791] PCI: Using configuration type 1 for base access
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.736401] ACPI: Added _OSI(Module Device)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.738265] ACPI: Added _OSI(Processor Device)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.740180] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.741952] ACPI: Added _OSI(Processor Aggregator Device)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.747010] ACPI: Executed 2 blocks of module-level executable AML code
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.773052] ACPI: Interpreter enabled
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.774712] ACPI: (supports S0 S3 S4 S5)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.775838] ACPI: Using IOAPIC for interrupt routing
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.777714] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.810601] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.813003] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.815299] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.817622] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.823019] PCI host bridge to bus 0000:00
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.824894] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.827123] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.829159] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.832315] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.834811] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.836786] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.837242] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.862910] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.888965] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.891903] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.901372] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.910232] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.934620] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.944947] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.953046] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.977544] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.981794] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.985894] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.989966] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    0.994308] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.017923] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.019861] vgaarb: loaded
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.020970] SCSI subsystem initialized
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.022373] libata version 3.00 loaded.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.022393] ACPI: bus type USB registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.023886] usbcore: registered new interface driver usbfs
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.025871] usbcore: registered new interface driver hub
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.028079] usbcore: registered new device driver usb
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.030165] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.032495] dmi: Firmware registration failed.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.034353] PCI: Using ACPI for IRQ routing
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.035914] PCI: pci_cache_line_size set to 64 bytes
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.036022] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.036025] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.036168] NetLabel: Initializing
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.037533] NetLabel:  domain hash size = 128
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.039762] NetLabel:  protocols = UNLABELED CIPSOv4
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.041994] NetLabel:  unlabeled traffic allowed by default
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.044217] amd_nb: Cannot enumerate AMD northbridges
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.046591] clocksource: Switched to clocksource kvm-clock
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.056920] pnp: PnP ACPI init
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.059948] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.060039] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.060089] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.060141] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.060180] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.060249] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.060288] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.060489] pnp: PnP ACPI: found 7 devices
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.070675] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.074522] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.074526] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.074527] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.074529] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.074572] NET: Registered protocol family 2
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.076732] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.079741] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.082745] TCP: Hash tables configured (established 131072 bind 65536)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.084945] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.086830] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.090549] NET: Registered protocol family 1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.092523] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.094982] PCI: CLS 0 bytes, default 64
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    1.095067] Unpacking initramfs...
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.354195] Freeing initrd memory: 21432K
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.356581] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.359615] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.363689] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.366577] hw unit of domain pp0-core 2^-0 Joules
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.368630] hw unit of domain package 2^-0 Joules
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.371891] hw unit of domain dram 2^-16 Joules
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.373631] Scanning for low memory corruption every 60 seconds
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.376775] audit: initializing netlink subsys (disabled)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.378891] audit: type=2000 audit(1533712203.192:1): initialized
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.381273] Initialise system trusted keyring
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.383333] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.385558] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.389265] zbud: loaded
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.390804] VFS: Disk quotas dquot_6.6.0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.392046] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.394993] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.397883] fuse init (API version 7.23)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.399598] Key type big_key registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.400976] Allocating IMA MOK and blacklist keyrings.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.405483] Key type asymmetric registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.407028] Asymmetric key parser 'x509' registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.408669] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.411705] io scheduler noop registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.413204] io scheduler deadline registered (default)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.414785] io scheduler cfq registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.416409] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.418964] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.421386] intel_idle: does not run on family 6 model 63
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.421492] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.424913] ACPI: Power Button [PWRF]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.426414] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.429207] ACPI: Sleep Button [SLPF]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.431360] GHES: HEST is not enabled!
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.436273] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.439103] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.448219] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.450498] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.459863] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.483530] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.508454] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.532612] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.558689] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.565000] Linux agpgart interface v0.103
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.570845] loop: module loaded
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.572229] libphy: Fixed MDIO Bus: probed
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.573951] tun: Universal TUN/TAP device driver, 1.6
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.575891] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.633798] PPP generic driver version 2.4.2
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.635673] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.638540] ehci-pci: EHCI PCI platform driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.639994] ehci-platform: EHCI generic platform driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.642021] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.644535] ohci-pci: OHCI PCI platform driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.646194] ohci-platform: OHCI generic platform driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.648165] uhci_hcd: USB Universal Host Controller Interface driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.650247] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.654473] i8042: Warning: Keylock active
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.657088] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.658724] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.661908] mousedev: PS/2 mouse device common for all mice
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.664584] rtc_cmos 00:00: RTC can wake from S4
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.667144] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.670105] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.672827] i2c /dev entries driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.674572] device-mapper: uevent: version 1.0.3
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.676529] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.679661] ledtrig-cpu: registered to indicate activity on CPUs
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.683071] NET: Registered protocol family 10
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.685697] NET: Registered protocol family 17
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.687213] Key type dns_resolver registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.689476] microcode: CPU0 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.692178] microcode: CPU1 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.694649] microcode: CPU2 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.697255] microcode: CPU3 sig=0x306f0, pf=0x1, revision=0x1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.699577] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.703800] registered taskstats version 1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.705135] Loading compiled-in X.509 certificates
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.707241] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.711033] zswap: loaded using pool lzo/zbud
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.716167] Key type trusted registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.723931] Key type encrypted registered
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.725788] ima: No TPM chip found, activating TPM-bypass!
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.728201] evm: HMAC attrs: 0x1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.730100]   Magic number: 14:967:166
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.732020] rtc_cmos 00:00: setting system clock to 2018-08-08 07:10:03 UTC (1533712203)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.736441] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.740445] EDD information not available.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.742367] PM: Hibernation image not present or could not be loaded.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.744253] Freeing unused kernel memory: 1496K
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.746688] Write protecting the kernel read-only data: 14336k
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.750565] Freeing unused kernel memory: 1956K
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.753523] Freeing unused kernel memory: 92K
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.775727] systemd-udevd[119]: starting version 204
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.854652] scsi host0: Virtio SCSI HBA
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.861136] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.864975] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.875935] AVX2 version of gcm_enc/dec engaged.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.877837] AES CTR mode by8 optimization enabled
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.941290] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.941371] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.941373] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.941604] sd 0:0:1:0: [sda] Write Protect is off
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.941605] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.941745] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.943364]  sda: sda1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    3.944965] sd 0:0:1:0: [sda] Attached SCSI disk
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    4.370814] tsc: Refined TSC clocksource calibration: 2299.815 MHz
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    4.373610] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x2126863038f, max_idle_ns: 440795225368 ns
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    4.706755] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    6.834832] floppy0: no floppy controllers found
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    7.994609] raid6: sse2x1   gen()  9227 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.062605] raid6: sse2x1   xor()  7070 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.130604] raid6: sse2x2   gen() 11361 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.198605] raid6: sse2x2   xor()  7865 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.266609] raid6: sse2x4   gen() 12539 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.334605] raid6: sse2x4   xor()  8569 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.402608] raid6: avx2x1   gen() 17888 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.470605] raid6: avx2x2   gen() 20065 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.538604] raid6: avx2x4   gen() 22038 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.539334] raid6: using algorithm avx2x4 gen() 22038 MB/s
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.540147] raid6: using avx2x2 recovery algorithm
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.542242] xor: automatically using best checksumming function:
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.582604]    avx       : 22197.000 MB/sec
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.597124] Btrfs loaded
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.647141] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.648122] EXT4-fs (sda1): write access will be enabled during recovery
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.724394] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.729275] EXT4-fs (sda1): 6 orphan inodes deleted
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.730069] EXT4-fs (sda1): recovery complete
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.736022] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    8.944314] random: init: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    9.065089] random: mountall: uninitialized urandom read (12 bytes read, 30 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    9.113644] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    9.330858] random: cloud-init: uninitialized urandom read (32 bytes read, 37 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [    9.863525] random: cloud-init: uninitialized urandom read (32 bytes read, 45 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.003845] systemd-udevd[701]: starting version 204
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.113548] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.169507] intel_rapl: no valid rapl domains found in package 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.219162] ppdev: user-space parallel port driver
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.302700] random: mktemp: uninitialized urandom read (6 bytes read, 56 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.352943] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.415615] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.581546] random: cloud-init: uninitialized urandom read (32 bytes read, 58 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.860457] random: mktemp: uninitialized urandom read (12 bytes read, 61 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   10.939614] random: mktemp: uninitialized urandom read (6 bytes read, 61 bits of entropy available)
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   11.034777] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   11.094649] EXT4-fs (sda1): resized filesystem to 7864064
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   11.367870] init: failsafe main process (1095) killed by TERM signal
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Running set_multiqueue.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Set channels for eth0 to 4.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Starting Google Accounts daemon.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for me.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-clock-skew: INFO Clock drift token has changed: 0.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account me.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account me.
Aug  8 07:10:11 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for henrik.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account henrik.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for emma.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account emma.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for igor.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account igor.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for konstantinhaase.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.464319] random: nonblocking pool is initialized
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account konstantinhaase.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for aj.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account aj.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for solarce.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account solarce.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for asari.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account asari.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for bogdana.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.824762] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account bogdana.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.828531] Bridge firewalling registered
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.842511] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for konstantin.
Aug  8 07:10:12 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.880025] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-clock-skew: INFO Synced system time with hardware clock.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account konstantin.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for carmen.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.970175] Initializing XFRM netlink socket
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.979764] Netfilter messages via NETLINK v0.30.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   12.983132] ctnetlink v0.93: registering with nfnetlink.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account carmen.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Creating a new user account for maria.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Created user account maria.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c google-accounts: INFO Removing user packer.
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   13.166791] floppy0: no floppy controllers found
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   13.167047] work still pending
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c pollinate: To re-seed this system again, use the -r|--reseed option
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c cron[1703]: (CRON) INFO (pidfile fd = 3)
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c cron[1737]: (CRON) STARTUP (fork ok)
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c cron[1737]: (CRON) INFO (Running @reboot jobs)
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c acpid: starting up with netlink and the input layer
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c acpid: 1 rule loaded
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c acpid: waiting for events: event logging is off
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c haveged: haveged starting up
Aug  8 07:10:13 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   13.643715] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug  8 07:10:36 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpdate[1850]: adjust time server 169.254.169.254 offset 0.009101 sec
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1885]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: proto: precision = 0.101 usec
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen normally on 3 eth0 10.20.0.5 UDP 123
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: peers refreshed
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listening on routing socket on fd #21 for interface updates
Aug  8 07:10:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [   43.838993] init: plymouth-upstart-bridge main process ended, respawning
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c startup-script: INFO Found startup-script in metadata.
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c startup-script: INFO startup-script: job 1 at Wed Aug  8 10:20:00 2018
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c startup-script: INFO startup-script: Return code 0.
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c startup-script: INFO startup-script: Return code 0.
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c startup-script: INFO Finished running startup scripts.
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: 
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: #############################################################
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: 1024 ce:81:70:2d:85:6f:09:54:51:c5:14:cb:87:59:3f:90  root@travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c (DSA)
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: 256 ad:d2:38:40:0a:3b:5d:8a:76:ba:db:ae:71:8c:bc:b8  root@travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c (ECDSA)
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: 256 75:30:2e:61:3c:b4:77:d6:22:83:25:3d:e2:70:76:1f  root@travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c (ED25519)
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: 2048 24:f0:c1:8e:ed:a5:c1:14:96:48:e8:2f:37:ed:41:12  root@travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c (RSA)
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug  8 07:10:44 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ec2: #############################################################
Aug  8 07:12:06 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  126.700316] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.372845] device veth122d58e entered promiscuous mode
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.372961] docker0: port 1(veth122d58e) entered forwarding state
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.372970] docker0: port 1(veth122d58e) entered forwarding state
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.374414] docker0: port 1(veth122d58e) entered disabled state
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.481150] cgroup: docker-runc (4880) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.481153] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.568644] eth0: renamed from veth8706227
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.614237] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.615835] docker0: port 1(veth122d58e) entered forwarding state
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.615859] docker0: port 1(veth122d58e) entered forwarding state
Aug  8 07:13:43 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  223.615883] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug  8 07:13:46 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen normally on 5 docker0 fe80::1 UDP 123
Aug  8 07:13:46 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen normally on 6 docker0 fe80::42:35ff:fea9:e766 UDP 123
Aug  8 07:13:46 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug  8 07:13:46 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: peers refreshed
Aug  8 07:13:46 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c ntpd[1886]: new interface(s) found: waking up resolver
Aug  8 07:13:58 travis-job-34cc63df-91c1-4a30-a828-5642cb8a715c kernel: [  238.628125] docker0: port 1(veth122d58e) entered forwarding state
---
travis_time:end:1b2f7214:start=1533712609898669837,finish=1533712609907253795,duration=8583958
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0483b490
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c952c41
travis_time:start:1c952c41
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:179e3fe4
$ dmesg | grep -i kill
